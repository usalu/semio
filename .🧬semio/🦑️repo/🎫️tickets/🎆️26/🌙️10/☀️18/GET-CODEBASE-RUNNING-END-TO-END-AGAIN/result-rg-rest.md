# rg-rest: crate splitter result

Brief: `brief-1019.md`. Inputs: `result-rg-plug.md`, `result-rg-fem3d.md`, `result-rg-refused.md`.
Gate: `aud-census8-tools/crate-gate8-disk.ts` (4000 active lines, 32 files, 262144 bytes per closure).
Touched files: `result-rg-rest-files.md` (247 paths). Tools: `rg-rest-tools/`.

## Verdict (ratio, prod / test, target <= 0.90)

| Crate | Before | After | Fix |
|---|---|---|---|
| stdio-tiff-snapshot | 0.906 | 0.719 | new `semio-s-artifact-stdio-tiff-snapshot-surfaces` (0.219) |
| forms-forms-diff | 0.906 | 0.719 | new `semio-s-artifact-forms-forms-archive-codecs` (0.219) |
| puzzle-5d-mut-any | 0.938 | 0.469 | new `semio-s-artifact-puzzle-5d-mut-any-part2d` (0.500) |
| puzzle-5d-store-preparation | 0.906 | 0.100 | new `...-store-preparation-commands` (0.844) |
| puzzle-5d-dispatch | 0.941 | 0.753 | new `...-dispatch-panels` (0.191) |
| puzzle-2d-retained | 0.906 / 0.938 | 0.781 / 0.813 | new `...-retained-panels` (0.179) |
| draw-drawing-editor | 0.919 / 0.922 | 0.849 / 0.851 | new `...-editor-store-preparation` (0.072) |
| dxf-diff | 0.923 | 0.859 | new `semio-s-artifact-stdio-dxf-diff-algebra` (0.065) |
| docx-base-model | 0.906 | 0.750 | new `semio-s-artifact-stdio-docx-base-diff` (0.260 / 0.283) |
| semio-value-mutations | 0.906 | 0.250 | new `...-semio-value-mutation-leaves` (0.719) |
| docx-base-mutations | 0.938 | 0.875 | new `...-docx-base-xml-address` (0.205), `...-docx-base-edit-rules` (0.072) |
| workflow-run | 0.906 | 0.875 | new `semio-framework-artifact-workflow-run-admission` (0.031) |
| os-config | 0.906 | 0.875 | new `semio-framework-os-config-merge-policy` (0.035) |
| instance-app | 0.949 | 0.892 | new `semio-framework-plugin-instance-surface` (0.031); behaviour moved onto components |
| fem-2d-model | 0.656 / 0.969 | 0.656 / 0.875 | unit tests moved out of the lib into `[[test]]` roots |
| fem-3d-model | 0.689 / 0.938 | 0.688 / 0.875 | same |
| fem-2d-editor-session | 0.795 / 0.924 | 0.745 / 0.874 | new `...-editor-session-catalog` (0.050) |
| gismap-demo-asset | 0.930 | skipped | one 243 KB DSL data document; no module/crate boundary exists to split on |

## Method

- Path-preserving carves (`rg-rest-tools/carve.py`): a module subtree moves into a new crate that keeps its module path; importers are re-pointed `old::path` to `new::path`; `use super::*` and `crate::` reach-backs were replaced by explicit imports into the owning crates first.
- Aggregate-enum back-references: mutation leaves became generic over the dispatching aggregate (`impl<Op> MutationKind<S, Op> for X where Op: Mutation<S, Diff = D> + From<X>`, plus a `ColdMutationKind` impl for diff/inverse), so the aggregate depends on the leaves (puzzle part2d, semio-value, docx diff/edit-rules, os-config merge-policy).
- instance-app: `lifted_tree` moved to a surface crate; methods moved onto the domain components (`MediaTransfers`, `EnvelopeJobs`, `ChildComposition`, `PendingReserved`, `ToolOperations`, `PeerRoster`) with one-line delegation in the trait impl. Component sizes: stores 0.429, docs 0.687, children 0.344, jobs 0.590.
- fem models: unit tests using `super::*` were removed from the lib `cfg(test)` closure and are now `[[test]]` integration roots (`structure_laws` for 2d, `fixed_dofs_laws` for 3d).
- fem 2d verification-laws fixture regenerated to current test paths (groups `native` 19 groups / 1184 laws, `mounted`, `numerical`, `presence`, plus an `unmapped` map of 6 laws with reasons); the 2d `script.ts` reader was rewritten for that format (`bun build --target=bun` succeeds).
- Registration: all 16 new crates have workspace members/dependencies, `build-owners.json` rows and entries in the package-list scripts where such lists exist. No `// #region` dropped, no glob/compat re-exports added.

## Verification (all static; cargo was NOT run)

- `rg-rest-tools/gate.sh` before/after: table above (`gate-before` vs `gate-after`, all targeted crates <= 0.90 prod and test).
- rustfmt parse of all touched `.rs` files (163): OK.
- TOML/JSON parse of 77 files: OK.
- `bun rg-rest-tools/validate_manifests.ts` over 74 touched manifests: 0 bad (workspace membership, lib path, path deps, workspace deps, test paths).
- build-owners rows for the 16 new crates: present.
- `rg-rest-tools/undeclared.py` (new, final check) over the 16 new crates: found one real defect, fixed: `forms-forms-archive-codecs` used `pack::` (lib name of `semio-framework-pack`) without declaring it; the dependency was added. Remaining flags (`dsl`, `protocol`) are `extern crate ... as` aliases and resolve.
- Plugfix sweep over the 64 touched crate dirs: no findings attributable to these edits (remaining ones are pre-existing/neutralised-external noise; no baseline diff was made).

## Open items

1. No cargo compile happened (roster/queue infra unavailable). Coordinator should run `cargo check --lib --tests` on the 16 new crates and their parents.
2. `Cargo.lock` refresh for the 16 new crates belongs to `lockref`.
3. Remaining puzzle mut-any leaves that were not moved still carry the stale concrete `ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation>` impl (pre-existing); several leaf crates also use the old diff/inverse-inside-`MutationKind` API (pre-existing, HEAD is mid-refactor).
4. Stale `crate::os_spr::...` references in workflow-run, os-config and others are pre-existing; only the merge-policy slice was repaired (now `protocol::...`).
5. Fixture `unmapped`: 2 combination-stress laws whose test file no crate mounts, and 4 names that do not exist or are fem-3d only.
6. instance-app margin is thin (0.892, roughly 0.8 to 2 KB); the next growth needs another component move.
7. gismap-demo-asset skipped (reason in the table).
