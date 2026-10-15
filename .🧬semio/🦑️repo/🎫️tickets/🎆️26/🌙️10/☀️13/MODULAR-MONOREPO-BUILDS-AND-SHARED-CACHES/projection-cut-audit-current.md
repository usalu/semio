# Current Composition Admission Cut Audit

Read-only production audit, 2026-10-15. Only this report was written. No Git mutations or production edits. This audit inspected current files with rg/cat/sed; it did not execute tests or native compilers. Parent-provided qualification context says ownership checks pass in both modes (3 tests/40 assertions), strict types pass, and native attempts stop before compilation at the unrelated Framework brep construction workspace rename inconsistency. Those results are context, not independently rerun results.

## Ownership and Dependencies

The actual body is at `🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🦀️.rs`, with its Cargo owner at `📦️packages/🦀️rust/Cargo.toml`. Framework workspace membership is explicit at `🧰️framework/Cargo.toml:180`. There is one production definition of `ChildRestoreProjection` and one of `ChildRestoreProjectionError` across repository Rust Source excluding ticket archives. No production `store::ChildRestoreProjection` or `os_store::ChildRestoreProjection` occurrence was found.

The owner has exactly two direct production dependencies: schema-composition and artifact-reference. Schema-composition has no production dependencies. Artifact-reference points downward into value, DSL record, pack JSON, and schema registry. No owner Source reference to Kernel/Store/Plugin was observed. This is source/manifest inspection rather than a resolved Cargo graph proof.

Kernel directly depends on the canonical crate (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:66`), and Plugin does too (`…/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml:31`). Store privately imports canonical types at line 19. Plugin child, declarations, ingress, dispatch, publication, and test-support Source use the canonical crate path. Repository-wide callers include Hub compositions and editor/plugin artifacts; inspected occurrences name the canonical owner. This audit did not map every caller to its nearest Cargo manifest, so exhaustive direct-dependency coverage remains a qualification gap.

Kernel's root `pub use semio_framework_composition_admission;` at line 332 exposes the crate name for exported macro hygiene. Store macro methods at lines 10052 and 10055 resolve canonical projection/error types through `$crate::semio_framework_composition_admission`, avoiding a caller dependency or the old Store type façade. No duplicated production body or legacy Store type reexport was found.

## Actionable Qualification Gaps

1. Native owner and real caller compilation remain unproven until the Framework workspace inconsistency is resolved. Text ownership and strict TypeScript checks cannot establish Rust lifetime/type correctness or exported macro expansion correctness. Run the existing native owner route, Kernel/Plugin compilation, and an external macro consumer once workspace discovery succeeds.
2. The third ownership contract only inspects Kernel/Plugin manifests, Store's main file, and Plugin child Source. It does not enforce canonical paths or direct dependencies for the complete monorepo caller set. Add a fixture-driven caller inventory with nearest manifest ownership and prohibit old qualified paths throughout actual Source, excluding ticket archives.
3. The neutral corpus covers six identity cases. The SQLite oracle checks empty child/artifact IDs, kind equality, and duplicates, but omits standard/subset emptiness and byte bounds, unlike Rust admission. Its agreement with the current corpus therefore does not qualify those laws independently. Extend the shared corpus/oracle for every field and byte boundary, invalid/duplicate slot metadata, singular slots, capability selection, and complete-set substitution/duplicate/overflow behavior.
4. Source caps are manifest metadata (65,536 bytes, eight files, 1,000 active lines) and contract Source walks cover macOS/Linux/Windows with and without native tests. They do not establish transitive dependency Source headroom or native compiler resource qualification. Retain separately reported frontier results for those claims.

## Current Assessment

No concrete extraction ownership or macro-path regression was found in inspected Source. The cut is structurally coherent. Native execution and exhaustive caller/dependency qualification remain the material open evidence, and the independent neutral oracle is narrower than the full admission contract. Existing project targets route through the package's `📜️script.ts`; launch entries exist for admission, source frontiers, and types in both modes.
