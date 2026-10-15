# Result: architect registers split (seam map D)

Executed 2026-10-15 by the Sonnet executor. `P` = `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program`, `R` = `P/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers`.
Crate `semio-s-artifact-architect-program` (lib root `P/🦀️.rs`). Pure structural move: no behaviour change, no visibility widened, public paths stable through `pub use`.

## 1. Taxonomy change (D.8)

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, textual insertion only (+21 lines, nothing reformatted; the file also carries unrelated peer edits). Applied by `🗑️generated/registers-split/patch_tax.py`, which asserts that the JSON is identical to the original after removing the two new entries.

1. `semanticDirectoryKinds.registers` (placed directly before `retained`, same shape as the schema-child precedents `retained`, `schema-catalogue`, `repo-schema-inference`):
   `{ emoji: "🗄️", slugPattern: "^registers$", allowEmojiOnly: false, parentKindIds: ["schema"] }`
2. `semanticDirectoryMemberKinds.members-of-registers` (placed before `members-of-repo`, alphabetical by owner): `ownerKindIds: ["registers"]`, `source: "registry"`, `memberNames`: `🧭️program-structure`, `🔁️operations-requirements`, `🌐️context-requirements`, `📋️decisions`, `🗃️records`.

Decisions and evidence:
- `🧱️kernel` is deliberately NOT in `members-of-registers`: `matchDirectoryKind` (`🧹️normalization/🟦️.ts`) also resolves a member name through ancestor kinds, and `🧱️kernel` is already a `members-of-schema` name (owner `schema` is an ancestor), exactly like the sibling `🧬️schema/🧱️kernel`. The kernel child therefore keeps the same kind id as its sibling.
- `🗄️registers` stays in `members-of-schema.memberNames`. The new kind only wins for a `🗄️registers` directory directly under a `schema` kind directory. `find` shows exactly one `🗄️registers` directory in the repo, so the new kind and `members-of-registers` can only affect this subtree.
- `📋️decisions` also matches the global kind `asset-table-subject`, but that kind is restricted to parents `assets`/`members-of-assets`, so it cannot compete (only `members-of-registers` applies).
- All five names: RGI emoji followed by U+FE0F, already used as folder prefixes elsewhere in the taxonomy (`🧭️` 85, `🔁️` 36, `🌐️` 161, `📋️` 119, `🗃️` 15 occurrences), distinct from each other and from the siblings `🗂️catalog`, `🧱️kernel`, `🧪️tests`, and no member-kind lists any of them (checked, only `🧱️kernel` is listed, in `members-of-schema`).
- Validator: `bun ./📜️script.ts verify taxonomy report --scope R` (`runTaxonomy` in the root `📜️script.ts`):
  - baseline before any change: `clean=true errors=0 warnings=0`, 14 directories, 4 files (`🗑️generated/reg-taxonomy-baseline.log`);
  - after registering, before the split: `clean=true errors=0 warnings=0` (`reg-taxonomy-after-registration.log`), which also proves the shared loader accepts the edited JSON (also `json.load` of the file);
  - after the split: `clean=true errors=0 warnings=0`, 28 directories, 14 files (`reg-taxonomy-after-split.log`).
- Repo-wide validator run: attempted (`reg-taxonomy-baseline-repo.log`) and aborted by me. Inventory alone took about 10 minutes and the 101,340 reference digests advanced at about 20 files per minute at load average 55, so it could not finish. The repo-wide claim therefore rests on the scope runs plus the blast-radius argument above (one directory in the repo is affected). No repo-wide finding count was compared.

## 2. Line counts (R)

| File | Before | After |
|---|---|---|
| `🦀️.rs` (component) | 7,727 | 107 |
| `🧱️kernel/🦀️.rs` | - | 500 |
| `🧭️program-structure/🦀️.rs` | - | 1,100 |
| `🔁️operations-requirements/🦀️.rs` | - | 1,540 |
| `🌐️context-requirements/🦀️.rs` | - | 1,652 |
| `📋️decisions/🦀️.rs` | - | 1,429 |
| `🗃️records/🦀️.rs` | - | 1,451 |
| `🧪️tests/🔬️unit/🦀️.rs` | 4,182 | removed (empty dirs removed) |
| `🧭️program-structure/🧪️tests/🔬️unit/🦀️.rs` | - | 570 (8 tests) |
| `🔁️operations-requirements/🧪️tests/🔬️unit/🦀️.rs` | - | 944 (14 tests) |
| `🌐️context-requirements/🧪️tests/🔬️unit/🦀️.rs` | - | 1,010 (16 tests) |
| `📋️decisions/🧪️tests/🔬️unit/🦀️.rs` | - | 876 (14 tests) |
| `🗃️records/🧪️tests/🔬️unit/🦀️.rs` | - | 832 (13 tests) |

Largest child 1,652 lines (limit about 2,500). Test total 65, as before. Longest new path 220 UTF-8 bytes from the repo root (`🔁️operations-requirements/🧪️tests/🔬️unit/🦀️.rs`; limit 240).

## 3. What moved where

- Component `🦀️.rs` keeps lines 1-89 of the original verbatim (docs, imports, `trait PatchRow` and its impls, both `macro_rules!`, the `TraceLink` `impl_patchable!`) and then declares, after the macros so their textual scope reaches the children: `#[path = "<child>/🦀️.rs"] mod <snake>; pub use <snake>::*;` for `kernel`, `program_structure`, `operations_requirements`, `context_requirements`, `decisions`, `records`. The child modules are private, only the glob re-exports are `pub`, so `registers::*` (used by `schema/🦀️.rs`, `snapshot`, `io`, `editor`, catalog) exposes exactly the same names. `P/🦀️.rs` mount (line 105-111) is unchanged.
- `🧱️kernel`: the 29 shared enums (`SharedEnums` region, original lines 90-587). No imports needed. Rust module name `kernel` is private in `registers`, so it neither shadows nor collides with `schema::kernel` (`schema/🦀️.rs` globs `registers::*` and only sees pub names).
- Families (original line ranges, each cut at a `// #region` boundary): program-structure 589-1680 (ProgramMeta ... Adjacency), operations-requirements 1682-3213 (Process ... RegulatoryRequirement), context-requirements 3215-4858 (SiteContext ... Conflict), decisions 4860-6280 (Requirement ... SearchFilter), records 6282-7724 (StatusRecord ... Governance). Each opens with `//! <emoji>` docstring and `use super::*;`, ends with the `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` mount (the original single mount was removed from the component).
- Tests: every test moved verbatim to its family (the split verifies that each test's `let mut item = X {` type is a record of that family's regions). The two test macros are re-declared only where used: `patch_through_program!` in all five test files (4 extra copies), `patch_through_table!` only in `🗃️records` (knowledge and benchmark tests). Each test file opens with a unique-emoji `//!` docstring (`🏗️ ⚙️ 🗺️ ⚖️ 📒️`) and `use super::*;`.
- Created: 12 new `.rs` files above plus 11 helper scripts/logs in `🪓️monolith-split/🗑️generated/registers-split/` (traceability copies, not repo tooling).
- Edited outside R: `P/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (one doc comment: "mirrored ... from `🗄️registers/🦀️.rs`" now says `🗄️registers/<family>/🦀️.rs` families) and `taxonomy.json` (section 1).

## 4. Widenings

None. Every moved struct, enum, field and method was already `pub`. `trait PatchRow` stays private in the component and is reached by the children through `use super::*` (descendants may see private parent items); the `macro_rules!` are visible by textual scope. No qualified paths introduced.

## 5. Verification (all commands actually run)

Cargo is gated; I used only analysis-only `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_s_artifact_architect_program --emit=metadata -o /dev/null --error-format=json --json=diagnostic-short [--test] [--target wasm32-wasip2] <lib root>` (writes nothing). I created no cargo workspace, manifest, target dir or `RUSTC` override.

1. Differential on the real tree, unpatched, from the real lib root `P/🦀️.rs` (baseline taken before my first edit): native 284 errors, `--test` 321, wasm32-wasip2 284, wasm32-wasip2 `--test` 321; after the split the diagnostic multisets (level, code, message, primary file with line numbers stripped, label; family/kernel/test files folded into one `REGISTERS/*` token) are identical in all four configurations (0 missing, 0 extra). This pass is nearly blind to the registers files (every error is E0432/E0433/E0463 from missing external crates, and E0463 stops late resolution), which is why step 2 exists.
2. Strong differential per `📓️RESOLUTION-ORACLE.md` sandbox patches. Sandboxes are copies of the whole crate (rsync, no target) with the repo `🧰️framework` symlinked so out-of-crate `#[path]`/`include!` resolve; one sandbox has the ORIGINAL two files restored, the other the split files; both patched identically (`patchsbx.py`: remove the 4 `extern crate` lines, replace external-crate glob imports with `const _: () = ();` in the 2 files that had them: lib root and the sqlite snapshot test). Results, original vs split: native 4,484 vs 4,484; `--test` 7,103 vs 7,103; wasm32-wasip2 4,484 vs 4,484; wasm32-wasip2 `--test` 7,103 vs 7,103; 0 missing and 0 extra in all four. The error population (`--test`): E0433 5,323, E0432 305, E0425 253, E0422 14, plus 1,198 code-less (macro/environment), all identical, none is E0603/E0616/E0624. (An earlier strong run that applied only the `extern crate` patch, before I added the glob patch, gave the same equality at 4,484 and 7,117 errors.)
3. Sensitivity (seeded regressions, all in scratch copies, never in the repo):
   - removing `use super::*;` from the `decisions` family: +394 new E0425 (`EntityHeader`, `EntityId`, `TextField`, `ValidationStatus`, ...) in the strong oracle; the same removal was invisible to the unpatched oracle (284 vs 284), proving why the strong one is needed;
   - removing `pub use records::*;`: new E0432/E0425 at 8 consumers (inferences, mutations, editor, catalog, io);
   - making `pub struct Stakeholder` private: +9 new errors (E0432 at 3 importers, E0425/E0422 at 6 uses);
   - definition scan (`privscan.py`: top-level items, struct fields in every family file lacking `pub`): real tree 0 findings; with `pub role: String` made private it reports the private field (and the private struct in the seeded copy), so the scan is sensitive. Field and method privacy (E0616/E0624) cannot be reached by rustc here because typeck never runs (missing crates), so this scan is the only check for them. The scan also confirmed the only non-pub items outside the component are `mod tests;` and trait-impl `fn id` bodies.
4. Losslessness (non-blank line multisets, `🗑️generated/registers-split/lossless.txt`): source original 7,414 vs new 7,455 and tests original 4,115 vs new 4,156, with 0 missing lines; the 41 extra lines each are exactly the wiring (18 lines: 6 `#[path]`, 6 `mod`, 6 `pub use` in the component), 6 `//!` docstrings, 5 `use super::*;`, 4 times `#[cfg(test)]`/`#[path]`/`mod tests;`, and for tests 5 docstrings, 4 extra `use super::*;` and 4 extra copies of the 9-line `patch_through_program!` macro. Test count 65 (16+14+14+13+8 `async_test` attributes in the five files).
5. rustfmt (`rustfmt +nightly-2026-07-20 --edition 2021 --config-path rustfmt.toml --check`, which follows the `#[path]` mods): original tree 97 pre-existing diffs (derive lines), split tree 97 diffs (29+16+14+14+14+10 across kernel and families, 0 in the tests files), no parse error. The split introduces no new formatting finding.
6. Path resolution: real-tree and sandbox runs resolved every `#[path]` (a missing child would have produced `couldn't find file`, fatal in `--test`, which happened in my first sandbox attempt and not in the final ones). Emoji admission: section 1 (taxonomy scope run clean over the new directories).
7. Source-text readers: `git grep -n -I -F '🗄️registers'` over all tracked files (excluding `.🧬semio`, the catalog): only the lib-root mount line (path of the component, unchanged and still valid), `📸️snapshot/🔣️.json` and TS `🟦️.ts` imports (all point to `🗂️catalog`, untouched), the edited TS doc comment, two `.cursor/plans` notes and the taxonomy member name. Nothing reads the Rust file or the unit test as text (`include_str!`/`readFileSync` hits are the unrelated io fixtures). A plain `grep -r` I started first timed out with no output and is not evidence; `git grep` is. No TS test touches the moved files, so none was run.
8. Not run: any `cargo check` / `cargo test` (gate), so the 65 round-trip tests have not executed after the move; repo-wide taxonomy validator (section 1); the lint `unused_imports`/`unused_macros` on the split files is unobserved (rustc stops before lints). Risk accepted: `use super::*` in the family tests relies on a three-link private-glob chain (tests -> family -> component), the same mechanism the original two-link chain used; the strong oracle resolved it without errors.
