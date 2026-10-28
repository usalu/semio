# IFC Host Contextual Derivation Follow-Up

The later P1/P2 cancellation custody and GlobalId corrections are documented in `host-custody-fixes.md`. Subsequent owned receiving, native value editing, Store wire publication, explicit progressive-preimage refusal and corrected contiguous-owner fixture authority are documented in `host-wire-progress.md`; the next sparse original-source design is in `host-sparse-owner-plan.md`. These later reports supersede the earlier publication details below. No native compile or execution has succeeded.

The contextual marker repair is implemented in the shared IFC details provider and reducer. Native type checking and execution remain unverified because authentic compiler ingress refuses this session. The retained preparation cursor is implemented and connected to both IFC editors, but the entire P2 physical accounting and cancellation contract is **not closed**.

## Contextual Markers

The provider collects effective derived redeclarations from the complete retained component set, including the primary declaration. It caches this declaration metadata for one provider request and no longer truncates component inspection at the value depth limit. Declaration queries borrow the catalog's inherited attribute slices.

Primary type selection, same-primary selection, constituent type selection or rename, and constituent insertion or removal normalize the explicit slots in every affected retained component. Starting an override writes `Derived` into the ancestor's explicit slot; ending an override supplies the declared editable template, or `Unset` when optional or no template is available. Sparse primary corrections and the changed complex list are emitted together through the existing central mutation route. The parent's bitwise argument and complex equality helpers and signed-zero behavior are preserved.

`derived.json` records seven language-neutral cases for the `IfcNamedUnit.Dimensions` override: same primary, both SI/conversion rename directions, both variant directions, SI insertion and SI removal. Native tests compare the direct reduction and source-owned retained preparation, then apply central diffs and inverse diffs. Another test places the overriding declaration beyond 80 retained components. These tests are written but have not executed.

These records exercise retained component repair. They do not establish ISO 10303-21 physical conformance for a single-leaf entity represented by external partial records. `complex-mapping-research.md` remains the physical mapping boundary.

## Compact Requests And Retained Preparation

Entity and constituent type rows carry only `{ "$ifcType": "DECLARATION" }`, rather than copied arguments or complex constituents. Indexed type selection remains available for the complete catalog. Tests require each type request to remain below 256 encoded bytes even when the current entity owns a 1 MB text value.

The domain-neutral snapshot editor trait now supplies optional source-owned work and retained capacity hooks. Default editors keep their existing preparation; IFC4 and IFC4x3 install `IfcDetailsWork`. The common factory uses the same domain-declared capacity for admission, payload decoding and work construction. IFC capacity derives from the catalog's maximum attribute count, allowing sparse multi-mutation type changes to pass the existing Store row admission.

`🔎️details/work.rs` captures the original snapshot `Arc` through `RetainedCloneSource`, projects either the owning argument or entity, and advances the actual native clone cursor with at most 4096 body bytes per step. It reports actual clone progress and checks the original cancellation token between turns. Existing argument edits change the owned native argument rather than converting the entire document to DSL values. Type changes move retained same-declaration fields into the result. The source is installed in retained custody before receipt consumption can fail. Refused sources, partial values, copied entities, displaced values and pending mutation owners remain available for controlled close.

`retained.json` specifies a 2000-item typed text list, its last-item edit, an eight-turn cancellation point and zero publication on cancellation. The written native test captures the original snapshot, uses the source-owned factory hook, cancels before completion, requires unchanged source and drives controlled close. Written contextual tests also drive `IfcDetailsWork` under the original fixture caller before applying emitted diffs. This is work preparation coverage; its fixture Store read is deliberately unavailable, so it is not an executed complete retained command / Store publication test.

The profile has an optional `detail_variants(snapshot, path, locale)` hook, consulted consistently by list, count and indexed lookup. A native unit test checks all three methods in English and German. The parent owns IFC4x3 MVD header presets and their neutral fixtures. The hook itself does not alter field admission or the existing header mutation rule.

## Verification

Executed `py -3.13 -X utf8 <ticket>/details-oracle.py` using the retained IfcOpenShell 0.9.0.post1 oracle through its existing test boundary. The script uses the Windows short package path needed by the cp313 native wheel. Results:

- All seven contextual expectations match the independent declaration's `derived()` metadata.
- Ordinary SI and conversion unit populations report zero EXPRESS errors.
- The 2000-value `IfcPropertyListValue` and 1 MB `IfcText` population report zero EXPRESS errors.
- Existing five inherited-attribute cases, three GlobalId vectors, and the 301-entity / 257-reference population pass; the reference population reports zero EXPRESS errors.
- The oracle explicitly reports native cursor execution as not executed and physical complex mapping conformance as not evaluated.

Rustfmt accepts the shared provider, cursor and IFC4x3 native test source. This proves parsing and formatting only.

The latest focused command was `cargo test --manifest-path <IFC4x3 Rust package>/Cargo.toml --lib contextual_derived`. Manifest resolution progressed past the earlier missing GLTF workspace inheritance entry, then the repository compiler ingress refused its `-vV` probe: **“Original compiler ingress requires the repository launcher and authentic enforcing store.”** No enforcement variables or scripts were changed. No Rust test, native type check, actual retained command publication, scheduler execution or heap receipt test ran.

## Remaining Native Boundaries

The cursor eliminates the known full-payload type-row clones and makes native source copying interruptible. It is not proof of complete retained execution closure. Final mutation vector allocation, temporary path/catalog metadata allocation, constituent list shifts / rotations and large native collection insertion or removal remain synchronous. Full reference and GlobalId scans and complete component derivation collection are still linear metadata operations in the final phase; cancellation callbacks are checked but they are not serialized resumable scan positions. A large newly supplied replacement is bounded by the existing wire limit but its DSL-to-native conversion is synchronous. These final-phase allocations are not yet individually demonstrated against original retained grants and heap receipts.

The new work object has in-memory turn resumability; it does not add process checkpoint serialization or restoration. The existing common factory's checkpoint policy remains authoritative. Source/partial-owner close paths and failed receipt paths need execution under the authentic retained scheduler, including tight capacity/depth/release grants and cancellation before publication. Passing the independent schema oracle cannot replace those checks.

No AGENTS, IDE, git history, worktree, global build or compiler enforcement files were changed. The shared framework edits are limited to the editor work/capacity extension points and the existing factory's use of them. No runtime external dependency was added. Shared oracle dependencies remain under `🗑️generated` for the parent and other workers; this subtask does not delete them.
