# RESULT: Counter cfg-selection fix (`selectRustSourceV1`)

Status: fixed, tested and measured. `semio-framework-os-kernel-db` is no longer refused. It is still oversized, which is a policy matter. No limits, policy or admission code were touched.

## Root cause

The canonical source-closure counter is `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`. `selectRustSourceV1` removes a cfg-false construct with `rustFindTopLevel(tokens, pairs, attrs.next, tokens.length, {";", ",", "{"})`. That call has three defects. The audit named the first. Measuring the repo showed the other two, which are more common.

1. **Runs past the enclosing closer.** The search range is the whole file and `rustFindTopLevel` does not stop at an unpaired `}`, `)` or `]`. When the cfg-false construct is the last one in its group (last struct-literal field, last call argument, last match arm, tail expression), there is no `,` or `;` before the closer. The boundary then becomes the first terminator after the group. The edit deletes the enclosing `}`, which unbalances the file. This is the `os-kernel-db` refusal: `Unsupported Rust compile module body`. A group that never reaches any terminator threw `Unsupported cfg-owned Rust item`. The four triggers are in `🛢️db/🗄️storage/🦀️.rs` (3 struct-literal fields and 1 tail expression).
2. **Commas inside generics end the removal.** `,` was a top-level terminator, but `<A, B>` is not a delimiter pair. `#[cfg(test)] fn f<A, B>() -> Result<X, Y> { ... }` was cut at the first comma and left `B>() -> ... { ... }` behind. Production selections were silently corrupted, but the delimiters stayed balanced, so no refusal followed. The same happened for `let m: HashMap<K, V> = ...;` and for fields such as `m: HashMap<K, V>,`. This inflated line and byte counts.
3. **A body `}` was treated as the end of an expression.** `let v = if c { 1 } else { 2 };`, `match x { .. }.max(3)` and match arms with struct patterns (`P { a } => ..`, `P { a } | Q { b } => ..`) were cut after the first `}`, leaving `else {..}`, `.max(3)`, `=> ..` or `| ..` behind.

## Fix

The change is confined to `🟦️.ts` in the source module (about 45 lines).

- `selectRustSourceV1` now computes `limits[i]`, the closer index of the innermost delimiter group around each token (`()`, `[]` and `{}`). It passes that bound to a new helper, `rustCfgOwnedEnd(tokens, pairs, start, limit)`. The helper docstring starts with 🪚️.
- `rustCfgOwnedEnd` scans only inside the group, so it can never run past the enclosing closer. It ends at:
  - a top-level `;` or `,`, which it consumes, or
  - the body `}` of the first `{`, plus a directly trailing `;` or `,`, or
  - the last token before the closer. This is the tail expression, last field, last argument or last arm, which has no terminator.
- Commas never end items or `let` statements, which are recognised by their head keyword. Commas inside generic arguments never end any construct. `rustOpensGenerics` (docstring 🔺️) decides whether a `<` opens generics: after `::`, after a CamelCase type name, or in prefix position, and never after an operand.
- After a body `}`, the construct continues over `else`, `.`, `?`, `=>`, `=` and `|`.
- It fails closed. A cfg-false attribute with nothing to own throws `Unsupported cfg-owned Rust item`, and so does a `<` that is never closed.
- `rustFindTopLevel` and the syntax module are unchanged, so other callers are unaffected. Strings, chars, raw strings, comments and lifetimes were already handled by `rustTokens`, and the new vectors lock that in.

## Tests (test-driven)

Suite: `📥️source/🧪️tests/🟦️.ts`, with the language-neutral corpus `🧪️tests/🧫️fixtures/🔣️.json` and `schema.json` (new optional `oracle` flag per case).

- **Vectors added:** 21 `cases` (all flagged `oracle`) and 2 `closures`. The cases cover:
  - a cfg-false last struct-literal field, with and without `;` after the group;
  - a tail expression, a nested-block tail and a tail block;
  - the last call argument and last array element;
  - the last match arm, struct-pattern arms and alternated struct-pattern arms;
  - a last enum variant and last struct field with generic commas;
  - an item with generic commas, `let` with generics, `let ... else`, and a method chain after a block;
  - braces inside strings, chars, raw strings and comments;
  - lifetimes (`'a`) next to chars;
  - nested-paren cfg predicates and `cfg_attr`;
  - a cfg-true last field, which only drops the attribute;
  - an inline module as the last item.
  
  The two closures put the `os-kernel-db` shapes (struct-literal field and tail expression) inside an inline `mod` that has a `mod child;` after it.
- **Independent oracle:** a new test runs `rustc -Zunpretty=expanded` (`RUSTC_BOOTSTRAP=1`, `--cfg` flags from the vector) on the original vector. It runs it again, without cfg flags, on our selected text. The two expansions must be identical for every `oracle` vector. This is a real compiler's cfg-stripping and not our own logic, in the same spirit as the existing Ajv and SQLite oracles. One more test checks that a cfg-false attribute with nothing to own fails closed (it throws).
- **Red before the fix** (the vectors against the HEAD version of the function, by a throwaway runner that was deleted afterwards): 14 of the 20 corpus cases that existed then failed (`bun ./🩹️old-run.tmp.ts`). An earlier probe of 7 shapes against rustc showed 7 of 7 mismatches. The oracle test and the closure cases were not run against the old code.
- **Green after the fix:** `cd 📥️source && bun test ./🧪️tests/🟦️.ts` gives 43 pass, 0 fail, 85 expect calls. This includes all pre-existing laws.
- Other suites under `🏗️native-build`:
  - `📥️source/🎟️admission/📖️read` passes (2 pass) with `SEMIO_TEST_ARTIFACT_DIR` set to the scratchpad and `SEMIO_BUILD_MODE=prototype`.
  - `🕸️units/📥️capture` passes (1 pass).
  - The other suites there fail or error without script-invocation authority, a cargo launcher or fixture env vars. I did not analyse them further and attribute none to this change.

## Repo-wide validation (tree-sitter-rust oracle)

`🗑️generated/cfgfix-audit.ts` selects every git-tracked `.rs` file that contains a cfg attribute, under host cfg plus `debug_assertions`, and under host plus `test`. It parses the original and the selection with tree-sitter-rust (`web-tree-sitter`, from `node_modules`; used only in this audit, not in the suite). Files that already have parse errors in the original are skipped.

| | files with cfg | original not parseable (skipped) | selection has syntax error | threw |
|---|---|---|---|---|
| before (HEAD) | 7565 | 79 | **278** (ticket scratch files included) | 0 |
| after | 7560 | 78 | **0** | 0 |

The two checked-in file counts differ only because peers add and remove files. A mid-fix run found one remaining corruption (`🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/📤️staging/🦀️.rs`, a cfg-false match arm `A { .. } | B { .. } | C { .. } => ..`). Adding `|` to the continuation set fixed it, and a vector for it was added.

## Census (item 4)

- **`ONLY=semio-framework-os-kernel-db bun 🗑️generated/census.ts`:** `error: ""`, 29 files, 2,614,222 bytes, 51,618 active lines, worst ratio 12.905 (binding: lines), `overLimit: true`. The refusal is gone and the crate is still oversized. The earlier in-memory patch gave 29 / 2,614,229 / 52,301; the difference is the four source sites that patch rewrote, which the fixed counter now handles without any edit.
- **Full census, old counter vs fixed counter on the same tree.** `🗑️generated/cfgfix-census.ts` is a copy of `census.ts` with the counter module and CSV path injected; the shared `crate-census.csv` and `census-summary.json` were not touched. Outputs are `cfgfix-census-old.csv` and `cfgfix-census-new.csv`. Both runs saw 277 crates and 120 members.
  - Old counter: 17 oversized, 162 compilable, 1 error (`os-kernel-db`).
  - Fixed counter: 17 oversized, 162 compilable, 0 errors.
  - No crate flipped `overLimit` or `closureCompilable`. Only the `os-kernel-db` refusal disappeared.
- **Changed counts: 36 crates.** Every change except `os-kernel-db` is a line count (`plugin` also loses one file). Mostly the counts drop, because leftover tails of cfg-false items are no longer counted. The increases are crates where the old counter over-removed live code after a tail field or expression.

| crate | files | bytes | active lines (old > new) |
|---|---|---|---|
| os-kernel-db | 0 > 29 | 0 > 2614222 | 0 (refused) > 51618 |
| plugin | 185 > 184 | 6002505 > 6000811 | 83363 > 83090 |
| os-renderer-wgpu | 397 | 5961929 | 102532 > 102079 |
| os-kernel | 180 | 3676554 | 54848 > 54677 |
| os-infinite | 67 | 2211384 | 41842 > 41749 |
| os-mcp | 51 | 1822586 | 29297 > 29291 |
| os-flow | 59 | 1296035 | 19232 > 19230 |
| ui-contract | 37 | 756785 | 14968 > 14919 |
| replication | 57 | 1039702 | 16470 > 16464 |
| pack | 33 | 804976 | 11174 > 11184 (up) |
| plugin-host | 31 | 1022464 | 14443 > 14461 (up) |
| repo-dashboard-terminal | 6 | 128443 | 2133 > 2153 (up) |
| async | 12 | 193774 | 3246 > 3252 (up) |
| ui-render-packet | 5 | 99839 | 2021 > 1954 |
| ui-backend-metal | 10 | 192699 | 3290 > 3256 |
| os-command | 2 | 57264 | 1082 > 1033 |
| os-identity | 1 | 7851 | 176 > 128 |

The remaining 19 changed crates (`actor-vocabulary`, `canvas-assets`, `canvas-scene`, `compiler`, `job-worker`, `os`, `os-directory-client`, `os-host-media-raster`, `os-vcs`, `plugin-describe`, `plugin-guest-contract`, `plugin-ownership`, `ui-host`, `ui-render`, `ui-runtime-reconcile`, `ui-runtime-reconcile-primitives`, `value-ordered`, `value-paged-list`, `value-retained-map`) have identical files and bytes. Their line counts move by 2 to 37 and are listed in `cfgfix-census-*.csv`. No change crosses a limit.

- **`semio-framework-plugin` lost one embedded file:** `🔌️plugin/🧪️tests/🧫️fixtures/🔬️tool-factory-proof.json` (1,694 bytes). It was reached only through `include_str!` in a `#[cfg(test)] pub(crate) fn test_retained_factory_proof_join<A, F, G, W>(...)` in `👷️job/🦀️.rs`. The old counter cut that item at its first generic comma and kept the body; the fixed counter removes it as cfg-false.
- **Spot check of the file-level diff:** `🗑️generated/cfgfix-filediff.ts` (log `cfgfix-filediff.log`, taken before the `|` tweak) shows 186 of the repo's cfg-bearing files change under host cfg plus `debug_assertions`. All shown cases are cfg-false generic functions, struct-literal tails or tail expressions. Only ticket scratch files were excluded.
- **Stale baseline:** the checked-in `🗑️generated/crate-census.csv` has 224 rows and predates peer work (the tree now has 277). Against it, 37 crates differ, but 15 of those differ already with the old counter on today's tree. The like-for-like comparison is the old-vs-new table above.

## Files

- Changed: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`, `…/📥️source/🧪️tests/🟦️.ts`, `…/📥️source/🧪️tests/🧫️fixtures/🔣️.json`, `…/📥️source/🧪️tests/🧫️fixtures/schema.json`.
- Ticket scripts and outputs in `🪓️monolith-split/🗑️generated/`: `cfgfix-probe.ts`, `cfgfix-audit.ts`, `cfgfix-census.ts`, `cfgfix-filediff.ts`, `cfgfix-one.ts`, `cfgfix-err.ts`, plus `cfgfix-*.log`, `cfgfix-*.csv` and `cfgfix-plugin-*.txt` outputs.
- Method notes: the throwaway copy of the HEAD module used as the "old counter" was removed from the source folder after the runs. No git write command was used. No `cargo` or `rustc` type-check of repo crates was run; `rustc -Zunpretty=expanded` ran only on small inline vectors.

## Remaining caveats

- `rustOpensGenerics` is a heuristic: a CamelCase identifier directly before `<` counts as a type. A comparison such as `CamelCase < x` inside a comma-terminated construct that never closes the `<` throws `Unsupported cfg-owned Rust item` (fail closed). Such a `<` before a `;` heals itself. The repo audit found no such case.
- Const-generic braces in an item head (`impl X where T: Foo<{ N }> {`) are still skipped as ordinary angle content only when the `<` is recognised as generic. A match-arm guard (`P { a } if c => ..`) is not kept in one piece, because `if` after a `}` usually starts the next statement. The tree-sitter audit found no corrupted selection from either shape, but they are not covered by a vector.
