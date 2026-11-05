# Complete Typed Setter Payloads

The parent independent census found two EN1999 routes whose declared setter/value/selector keys did not complete the canonical native leaf payload: bucklingLengthZ omitted axis and bolts.rows omitted newBoltsPerRow. The actual family resolver delegates to the shared edit-rules payload owner.

The shared pure vocabulary now exposes:
- NormEditRules.set_field_arguments: a static roster of SetFieldArgument records.
- SetFieldArgument fields: path, kind, argument, source.
- Closed SetFieldArgumentSource variants: Text for an authored string, Sibling for the actual current value at a path relative to the matched setter boundary's immediate parent.

The generic payload owner matches both path and kind, appends companions after canonical selectors and the supplied typed value, refuses duplicate/empty payload keys, and refuses missing sibling fields. A nearest-parent replacement uses the matched parent segments for its companions. No operation leaf, default zero, resolver exemption or coercion was introduced.

EN1999 declares axis=Text("z") for members[].bucklingLengthZ/change-member-buckling-length and newBoltsPerRow=Sibling("boltsPerRow") for connections[].bolts.rows/change-bolt-count. The other fourteen family rosters and all existing literal test owners explicitly declare empty companions. Existing selector/value mappings and operation identities remain intact.

Native generic tests consume the parent's closed neutral payload-bindings corpus and check exact vocabulary, complete payloads, missing siblings, collision refusal and nearest-parent sibling semantics. serde_json JSON Pointer supplies independent sibling and replacement values.

Actual EN1999 native tests are mounted beside its mutation edit-rules owner. They resolve and apply the canonical ChangeMemberBucklingLength and ChangeBoltCount operations to full_showcase(), verify buckling Y/T and bolts-per-row preservation, compare complete results with independent JSON Pointer writes and prove the original remains unchanged. A paired set first changes bolts-per-row, then rows; the second canonical payload must carry the newly admitted sibling from the typed projection. serde_json was added only as an existing workspace dev-dependency.

Owned source changes:
- app-surface/✏️edit-rules/🦀️.rs
- app-surface/✏️edit-rules/🎒️payload-arguments/🦀️.rs (new production owner)
- app-surface/✏️edit-rules/🧪️tests/🦀️.rs
- all fifteen canonical mutation edit-rule roster owners (only explicit empty roster additions outside EN1999)
- app-surface/🌲️outliner/🧪️tests/🔬️unit/🦀️.rs (empty roster)
- app-surface/🧪️tests/🔬️unit/🦀️.rs (empty rosters)
- EN1999 mutation edit-rules/🧪️tests/🦀️.rs (new native business tests)
- EN1999 mutation package Cargo.toml (test-only serde_json)

The parent owns the independent Bun census, strict neutral schema/corpus/feature and the separate EN1993 field-spelling corrections. None of those sources was edited here.

Execution of these new native tests is pending the framework source guard refactor. No new native acceptance or completed cohort result is claimed. The prior strict nested-edit oracle result was 2 passed/12 assertions; it does not stand in for this new payload feature's execution.

## Latest Native Evidence

The real EN1999 mutation package gate 62028 terminated before compiler admission on its existing 407,413-byte oracle manifest dependency (remaining allowance 180,165 bytes). The manifest is included by the kinds-catalog law. Parent owns a coherent registry/source ownership repair; no minification, test removal, or raised budget was attempted. Consequently the new real-family business assertions remain pending native execution. Parent reports the shared payload Bun suite and its strict TypeScript closure green.
