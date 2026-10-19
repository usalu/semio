# Result cg3: bim-model unmirror completed, generator hardened (static only, nothing compiled)

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Tools: `C/cg3-tools/` (stage.ts, probe*.ts, owners.ts, macro_*.py, sqlite_names.py, restore_and_edit.py, sqlite-by.json). No cargo, nx or `bun repo` ran; no git modifying command (git was only read with `show`/`archive`/`diff`).

## 1. State found

`unmirror` had written nothing. Evidence: newest mtime under the bim plugin was the carve apply (04:46:52), all 23 `🪞️mirror/🦀️.rs` files still held `pub use semio_s_artifact…` re-exports, and a dry run planned 1,794 file edits. The tree was "applied, unmirror never written". The 23 mirror dirs that remain after the fix are not leftovers: they hold only `#[path]` mounts of physical files (0 re-export lines), which is valid carve output.

## 2. Root cause of the exit 133

Generator bug, not a tooling crash. `Universe.member/leafHits` and the `Unmirror` edge/namespace memos refused to memoise any result that had met a cycle. The carved crates re-export each other in dense rings (schema ↔ snapshot ↔ …), so the first file (the facade root) was resolved once per path: millions of `member` calls, 18.6 GB footprint, swap thrash, bun abort. A second latent defect: `primary` ↔ `forwardOf` recursed without bound on a ring.

Fix: new `L/🏗️authoring/📦️crate-tree/🪞️unmirror/🔁️cycle-memo/🟦️.ts` (`CycleMemo`): a lookup that reaches its own in-progress computation gets the fallback, a result that never met a cut is final, a result that did stays reusable while the outermost computation it was cut by is alive. Resolver, edge memo, namespace memo and `forwardOf` all use it (`universe.cycles`). Dry run of the whole bim artifact: 53 s, 3 GB peak (was: not finishing, 18 GB).

## 3. Generator changes (all in `L/🏗️authoring/…`)

- `🪞️unmirror/🟦️.ts`: public glob over a module with items of its own keeps the glob and imports privately the names it only reached through a deleted edge; a glob dropped over a foreign re-exported module imports the names it provided straight from that module (they were lost: `KeyedDelta`, all `*Patch`, `Project`…); glob over an enum/item is re-pointed; chains inside macro invocation arguments keep a bare module head whose import survives and still reaches the owner (macro matchers such as `$m:ident :: $t:ident` keep matching), everything else is rewritten; a module prefix is re-pointed even when the last segment is macro-generated; an import with an unknown last segment is re-pointed by its resolvable prefix or through the only opaque glob target; forwarding modules whose only own children are `cfg(test)` mounts count as pure forwarders; `primary` always prefers the final forwarded owner; audit tolerates tails into modules a macro fills; dead `referencedIdents` field removed (tsc error).
- `🪞️unmirror/🔍️resolver/🟦️.ts`: `CycleMemo`; names declared inside `macro_rules!` bodies are attributed to the invocations in the same file (`UTILITIES`, `ROOT`, `from_snapshot`); path-qualified invocations are opaque; prelude words (`Some`…) are never guessed as items (a bogus `use …::Some;` was generated); an unresolved import of an outside crate still binds its name; `opaqueModule`.
- `🪞️unmirror/🏃️run/🟦️.ts`: files mounted by `#[path]` from outside their package dir (hub composition root) get their manifest dependencies from the owning crate; debug logging removed.
- `🎮️command/🟦️.ts`: `crateBudgetFlags` (parses `--target`/`--hard`), regressions are printed and fail the run.
- Inline `{NAME}` format captures inside macro arguments count as uses when a deleted glob is replaced by imports.

## 4. Tests added

- `L/🧪️tests/🪞️unmirror/🟦️.ts` (12 → 30): enum-variant glob, dense 24-crate ring (bounded time, exact result), `CycleMemo` unit tests, macro-argument head kept / crate-rooted path rewritten / local-forwarder rewritten / shell-with-forwarding-glob rewritten, macro-generated tail, macro-declared names, pure-forwarder with `cfg(test)` mount, opaque-glob import, outside-crate re-export, public kept glob, dropped glob over foreign module, inline format capture, audit tolerance.
- `L/🧪️tests/📦️crate-tree/🚧️hard-budget/🟦️.ts` (new, registered in `🧪️tests/📦️crate-tree/🟦️.ts`): `--target`/`--hard` parsing (independent, 5 malformed forms refused) and a plan that fits the default hard gate but is refused ("exceeds the hard budget") under a smaller `--hard`.
- Run: `bun test ./🧪️tests/🪞️unmirror/🟦️.ts ./🧪️tests/📦️crate-tree/🟦️.ts` in `L`: 149 pass, 0 fail. `tsc --noEmit` over the unmirror module closure: clean.

## 5. bim source changes (hand fixes the generator cannot do)

The unmirror planned from the HEAD tree (auto-commit 12:04 = post-carve, pre-unmirror; HEAD was used to restore after each oracle iteration). Hand edits are scripted and idempotent in `C/cg3-tools/restore_and_edit.py` (`restore` / `pre` before the unmirror, `post` after):
1. `🧩️entities` macros `set!`/`delete!` used `crate::…::mutations::$module::$leaf`, which cannot be re-pointed (the 112 leaf modules live in 190 crates). The macros now take `$module::$leaf` and the 16 files that call them (314 call sites) import the mutation modules explicitly (`macro_imports.py`).
2. `🔙️cascade-restore`: same pattern in `restore!` (`create_*` modules imported explicitly).
3. Three tests used `crate::editor::bim::BIM_TOOL_IDS` (macro-generated, lives in the `bim` crate): now `semio_s_artifact_bim_model_bim::…`.
4. `io/🪶️sqlite/📸️snapshot/🦀️.rs`: `use crate::*;` replaced by two explicit imports (80 names from the schema crate `snapshot::values`, 63 from the snapshot crate); names found by token scan minus local/imported/enumeration-variant names, each located by its `pub` declaration (no name missing, 6 ambiguous names resolved to the snapshot entity). `📏️cells` and `🧪️tests` use `super::*` and inherit them. Not compiled: if a name is unused it is a warning, if one is missing it is E0412.
5. `🧪️tests/🏗️export-bim-1-ifc/🦀️.rs`: `library_to_part21` does not exist anywhere (stale at HEAD); now `model_to_part21(Schema::Ifc4, &snapshot)?.0` with `Schema` from the examples crate.
6. Facade `🏢️model/🦀️.rs` is 11 lines: only `pub use semio_s_artifact_bim_model_v1::{BimApplication, artifact, definition};` plus the unit-test mount. No mirror module, no `render` re-export (nobody used it), no `extern crate` aliases. Its unit test imports the schema/snapshot crates explicitly. Consumers import carved crates: bridge (`mut_any_190`, `snapshot`; facade dependency removed), hub composition (`bim`, `schema`, `viewer`; it keeps the facade only for `artifact()`/`definition`), hub test (`BIM_MODEL_DOCUMENT_SCHEMA` from the schema crate), and about 1,790 bim sources. `grep semio_s_artifact_bim_model::` over `✏️s`, `🌎️hub`, `🧰️framework`: 0 hits.

## 6. Verification (static)

| Check | Result |
| --- | --- |
| `new crates <pkg> --unmirror` (apply) | 1,796 files, 3 manifests (bridge, hub bim composition, facade), 3,241 mirror re-exports removed, 14,686 paths re-pointed, 1,027 imports added, 108 modules removed, exit 0, 0 regressions, 0 unresolved, 0 cycles |
| same, dry run on the result | 0 files (idempotent) |
| `new crates <pkg> --check-applied --base=HEAD --samples=2000 --sandbox=<dir>` | exit 0: native `onlyCombined` 0 (base 8061, combined 8119), `--test` 0 (10427 / 10445), admission, shared, sharedForeign empty |
| owners scan `F/owners-tools/scan.ts` | 3509 packages, 3651 owners, 0 missing |
| `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` | 1 pass, 2 fail; no bim entry in either failure (process3d, energy, layout, norm, procedural: peers) |
| launch.json / launch seed parse (JSONC) | ok (13,803 configurations) |

Oracle iterations (each ~15 min) that shaped the generator: 1 error (command table macro matcher), 144 (macro-arg skip too broad), 126 (glob drop lost direct members), then 0. Intermediate applies were undone from HEAD; nothing outside the listed files was touched.

## 7. Compile executor (do not skip the lock)

1. Refresh locks offline first (the facade manifest gained 3 stdio deps, bridge and hub composition changed, the flow lock still lacks `unit-tests-2`): `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml` (and the same for the flow artifact manifest and `🌎️hub/Cargo.toml`; retry online if `zip` or a new crate is missing).
2. bim: `bun repo cargo --store=shared --configuration=prototype -- check --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml --workspace --lib --tests` (start narrow: `-p semio-s-artifact-bim-model-schema -p …-snapshot -p …-diff -p …-patches -p …-entities -p …-inferences -p …-v1 -p …-bim -p …-io`).
3. Then `-p semio-bim-mutation-bridge` and `-p semio-hub-bim` (hub workspace), and flow: `-p semio-s-artifact-flow-flow-editor-leaves --lib --tests`, `-p semio-s-artifact-flow-flow --lib --tests`.
4. Compile risks the oracle cannot see: the 143 explicit names in `sqlite/snapshot`; macro-generated names (`BimCommand`, `BimLabels`, `UTILITIES`, `BIM_TOOL_IDS`) now reached through re-pointed paths; the 16 entities files that import mutation modules for macro call sites; `{NAME}` format captures.

## 8. Open

- The facade `Cargo.toml` still lists all 236 component crates (and their `testkit` features); the root only needs `…-v1`. Pruning changes feature unification for the `testkit` runs, so it was left; trim it once the compile executor confirms the runner does not rely on it.
- No launch.json routes are added for the 236 carved crates (precedent R3/R4).
- `lib-wiring-budget` failures of other plugins and the other modified files under `L/🏗️authoring` (`🧰️testkit`, `🗿️artifact-tree`, `🪶️artifact-empty-facet-authoring` test) are peers' work, untouched.
