# Result comments-b: comments inside definition bodies in `✏️s/**`, `🌎️hub/**`, `🎓️teaching/**`, `🏢️semio-tech/**`, `♻️mit-bestand/**`

Executor `comments-b` (foreground, no sub-agents, no git writes, no cargo, no dependency sync). `C` = this ticket folder. Rule: AGENTS.md "You MUST NOT comment inside definitions". HARD FREEZE respected: nothing under `📚️library/`, no root `📜️script.ts`, no `package.json`/`bun.lock`, no `🔣️taxonomy.json` touched (only `.ts/.tsx/.rs` sources).

## Outcome

* 10,232 comments (10,232 lines) removed from 1,642 files (9,245 Rust lines in 1,513 `.rs` files, 987 TS/TSX lines in 129 files). The audit's 4,439 lines were an indented-`//` heuristic over recently touched files; this run covers every non-generated source file in the five trees.
* Final dry re-scan after the last batch (`--tag final2`, 34,332 files incl. `📜️script.ts`): 0 removable comments left, 0 verification failures.

| Tree | Rust | TS/TSX |
| --- | ---: | ---: |
| `✏️s/🔌️plugins` | 8,768 (1,441 files) | 654 (80 files) |
| `✏️s/🔨️modules` | 69 (14) | 0 |
| `✏️s/🧑‍💻dev` | 218 (30) | 178 (22) |
| `✏️s/🧪️tests` | 0 | 10 (1) |
| `🌎️hub` | 190 (28) | 118 (18) |
| `🏢️semio-tech` | 0 | 15 (3) |
| `♻️mit-bestand` | 0 | 12 (5) |
| `🎓️teaching` | 0 (2 files via lexer fallback, nothing to remove) | 0 |

The full text of every removed comment (path, line, text) is in `C/🗑️generated/comments-b/all-removed.jsonl`; list of changed files: `C/🗑️generated/comments-b/changed-final.txt`.

## Scope and keep-rules

Removed: every non-docstring comment strictly inside a definition. TS: anything inside a top-level statement other than `import`/`export {…}` (functions, classes, interfaces, enums, object/array literals, variable initialisers, top-level `if`/`for`/`try`, call arguments, JSX `{/* */}` containers removed whole); `namespace` bodies count as top level. Rust: any comment whose ancestors include something other than `source_file`/`mod` blocks (functions, closures, impl/trait/struct/enum bodies, match arms, `const`/`static` initialisers, macro bodies).

Kept (same rules as `comments-a`, verified: running comments-a's directive/SAFETY patterns over my 10,232 removed texts finds 1 hit, the prose "Safety net: …", which is correctly removed): `///`, `//!`, `/** */`, `/*! */`, `// #region`/`#endregion`, any comment containing `🔖` (`mod x { // 🔖️Name`), anything starting with `@`, `#` or `!` (`@ts-*`, `@vite-ignore`, `#__PURE__`, `/// <reference`, licenses), `eslint-*`, `biome-ignore`, `prettier-ignore`, `istanbul`/`c8`/`v8`, `tslint:`, `oxlint`, `deno-lint`, `cspell:`, `rustfmt::`, `clippy::`, `SAFETY` comments (whole contiguous `//` run). 15,877 comments inside definitions were kept by these rules. Comments outside any definition (file header, between top-level items, trailing a top-level statement) are untouched by scope.

Differences to `comments-a` worth knowing: mine also removes comments inside const/static initialisers, object/array literals and top-level control-flow blocks (comments-a leaves "module-level objects" and `const` initialisers alone); mine keeps every `/** */` block (comments-a removes 35 unattached ones). No other divergence in what is kept.

Skipped files: 1,052 with a "generated / do not edit" header in the first 600 bytes (dry-run with that check off found 0 removals in all of them), 45 by path (`node_modules`, `dist`, `jcoprobe`, `.d.ts`, `.min.js`), 1 vanished, 0 recently edited at the end (files younger than 15 minutes were skipped per pass and caught by later passes: pass 4 applied 94 such files). Python (220 `.py`), CSS, `.go`, shell not touched (no `//`-rule scope in the audit).

## Decisions to review

* 1,762 of the removed Rust lines were `// 🚫️async: E1/U1 ...` exemption notes (inside `impl` bodies; same family as comments-a's 1,642). Not read by any tool (grepped `.ts/.json/.rs` outside tickets; the only TS reference is a Rust-source string fixture in the `mutation-leaf-generics` test). The ones between a docstring and a top-level `pub fn` (about 11,000 more) stay because they are outside a definition. If the exemption register is wanted, it must move to a docstring or a schema field.
* `✏️s/…/🎬️presentation/…/✏️editor/🧰️support/🦀️.rs` hashes `concat!(file!(), line!())` as a per-call-site seed; removing comment lines changes the seed value (comments-a skipped its three `line!()` files; I did not, because the value is only an identity seed).
* Emptied blocks are left as `{\n}` (comments-a collapses to `{}`); a blank line next to a removed full-line comment is dropped when it would double up or hug a brace.
* Not touched, count not measured: top-level non-doc comments (e.g. the `// 🚫️async` lines above `pub fn`). Policy needed: convert to docstring or delete.

## Tool

`C/comments-b-tools/` (Bun). `keep.ts` (keep-rules, line/inline removal and blank-line tidy), `ts-engine.ts` (TypeScript compiler API: token walk with leading and trailing trivia, JSX text and template literals aware), `rs-engine.ts` (web-tree-sitter + `tree-sitter-wasms`, as `C/rs-sweep-tools/parse.ts`), `rs-lexer.ts` (independent hand-written Rust lexer: strings, raw strings, nested block comments, chars vs lifetimes, brace stack with `mod` detection), `run.ts` (`--mode dry|apply --prefix a,b --tag T [--scripts] [--gen-ok]`; file list cached in `C/🗑️generated/comments-b/files.nul` because `git ls-files -co` takes 50-75 s), `verify.sh`, `fx/` (TSX and Rust fixtures with strings, templates, JSX text, raw strings, SAFETY run, `mod` markers; `bun C/comments-b-tools/fx/try.ts`).

Per file the tool proves equivalence before writing: token-and-node-kind signature of old and new (TS: comment-free leaf tokens plus every node kind, JSX text normalised with the React whitespace rule and empty JSX containers ignored, which also catches the `return /* a\n b */ x` ASI trap; Rust: tree-sitter leaf tokens plus node types plus surviving doc/SAFETY/marker comments), then a second plan on the new text must find 0 removals, then mtime must be unchanged at write time. Files tree-sitter rejects (154 with `async |x|` closures, `unsafe extern`, etc.) are planned and verified by the lexer instead.

## Verification (exact commands, outcome)

* Dry-run, all five trees: `bun C/comments-b-tools/run.ts --mode dry --tag dry-all2` (33,877 files parsed in 4:40): 1,624 files / 10,013 removals, signature+idempotence failures 0. Cross-check of two independent Rust planners (tree-sitter vs lexer) on all ~26,000 Rust files: removal sets identical in every file (0 mismatches).
* Review: 50 random removals from the final log (25 TS, 25 Rust) plus ~30 from the dry samples: all prose rationale or dead notes, none a directive, marker, SAFETY or docstring. Two defects found in the fixture phase and fixed before any write: JSX `{/* */}` removal must not eat the surrounding space (signature check flagged it), and JSX text whitespace normalisation.
* Anchor check: every `"// …"` string literal used as a source anchor in repo TS (48) and Rust (14) was matched against the 10,232 removed texts: 0 hits that point at a removed comment (the two substring hits are a Rust-source fixture string and an anchor still present at top level).
* Apply in batches with the same tool: `--mode apply --prefix 🌎️hub,🎓️teaching,🏢️semio-tech,♻️mit-bestand` (54 files), `✏️s/🧪️tests,✏️s/🧑‍💻dev,✏️s/🔨️modules` (64), four groups of plugins (211, 153, 877, 172 files), a retry pass for files that were too young or newly edited (94), and `--scripts` for 17 `📜️script.ts` generator/probe scripts under fem/stdio/note.
* TS syntax gate: `bun build --no-bundle FILE --outfile=…` over all changed TS/TSX files via `C/comments-b-tools/verify.sh LIST`: 129 files, 0 failures.
* Rust syntax gate: `rustfmt --edition 2021 --emit stdout < FILE` over all 1,513 changed `.rs` files: 0 parse errors (with `--edition 2024` 15 files report `gen` as reserved keyword, a pre-existing identifier, not a comment effect).
* `bun C/rs-sweep-tools/main.ts --since 2026-10-20T00:30:00 --tag cb --out …` (after the first 1,531 files were applied): 4,843 findings over 1,202 touched packages, kinds undeclared-dependency/unresolved-*/private-item/duplicate-name only, no syntax-error kind. Findings were not compared against a pre-change baseline (none exists for this tree state), so this shows no parse regression, not equality.
* One changed `📜️script.ts` smoke-run: `bun <script> --help` prints the generator's own usage error (loads fine).
* Not run: `cargo check`/tests, vitest/bun test suites, dependency sync (all forbidden or out of scope for this shard).

## Open items

* `🎓️teaching`: nothing to remove in the current tree (the codex course ticket edits there; re-run `bun C/comments-b-tools/run.ts --mode dry --prefix 🎓️teaching` to confirm).
* Peers keep creating commented code in `✏️s` (pass 4 found 94 new files in 40 minutes); a gate would keep it clean. A cheap one: rerun `run.ts --mode dry` and fail on `removed > 0`.
* Decisions: the 🚫️async notes, top-level comment policy, the `line!()` seed file.
* Scratch under `C/🗑️generated/comments-b/` (logs, `files.nul`, removal log) is deleted when the coordinator removes the generated folder; keep `all-removed.jsonl` until the async-note decision is made.
