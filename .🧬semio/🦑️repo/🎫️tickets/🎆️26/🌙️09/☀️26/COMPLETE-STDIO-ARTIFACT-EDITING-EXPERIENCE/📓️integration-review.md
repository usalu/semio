# Integration Review

## Shared Editing Boundary

The first shared Rust reducer now defines typed set, insert, remove, move, rename, and source-replacement events. It works on a detached value and decodes back to the artifact's own snapshot before producing a mutation. Invalid edits therefore need not modify the live document. Typed re-encoding comparison is included to prevent silently discarded edits.

The following checks were sent to the implementation owner before integration:

- Action argument controls and parsing must agree on types. A text argument copied as a string cannot change a boolean, number, or collection. Use an explicit JSON argument or actual typed controls, without guessing whether an ordinary string should be parsed.
- Typed re-encoding must compare object contents independently of map iteration order. Appending a lexically earlier map key is valid even when the snapshot stores a sorted map. Duplicate keys must still fail.
- Required field removal, unknown fields, fractional integer edits, pointer escaping, move-into-descendant, large numbers, and source decode failures need rejection and preservation fixtures.
- Expensive conversion and large collection rendering need bounded work and paging. Hidden collection entries must remain reachable.

## Native Mutation Publication

All new actions must join the editor's command id, retained tool roster, factory keys, owner proof, publication lane, and app declaration. A new native `SetSnapshot` variant also needs schema, text and binary codecs, diff, inverse, and persisted event replay coverage. Preview windows must not advertise write commands that return a no-op.

## Current Scope

These are implementation review findings, not passing acceptance results. The family agents are building complete JSON and PNG pilots before applying the integration to the remaining subset surfaces.

## Component Reachability

The committed default component exposes only nine text/data editors. The full fleet comments cite an unverified function-ceiling refusal, while the earlier play audit measured only the nine-editor component (387787 unoptimized functions, 50514 optimized); it did not reproduce the full release fleet failure. Added an Nx command to build the full optimized component and validate its extracted module independently with the native WebAssembly parser before changing shipped exposure. Library catalog tests alone cannot establish end-user availability.

## Direct Manipulation and Draft Conflicts

The framework table host only understood text, number, stepper and button cells, so declaring `set-cell` did not make CSV grid cells editable. Assigned a typed text-input cell protocol and both renderer implementations to the shared execution owner; the text/office owner will emit it from table windows. Draft Apply also needs a conflict rule because retaining a local draft across a changed persisted buffer must not silently overwrite a collaborator or a newly opened document. Assigned conflict handling to the same owner.

## Current Integration Milestone

All 88 artifact editor roots now declare the shared Details window. This is a source inventory result, not runtime acceptance: generated native mutation leaves are still being compiled, all-root non-no-op mutation cases are being authored, and the full browser component has not yet linked successfully. Advanced Source is being changed to complete typed JSON across adapters to avoid native-encoding normalization. Nested Details tree window paths and control budgets still require verification.

## Native Retained Acceptance Fixtures

The catalog fixture now covers all format families with exact PDF, GIF, IFC, and Semio subset overrides. Root resolved the pending spatial cases from their authored snapshot schema: SVG uses `/doc/root/name` (internally tagged XML, no `content` wrapper); STEP has a named `/header/fileName/name`; IFC4 uses a `kind:string,value` enum; IFC2x3 uses its complete optional EDM preamble. Semio collection-only subsets receive complete typed records instead of fabricated scalar fields. These source-checked fixtures still require the native catalog run to confirm exact application coordinates, codec round trips, retained publication, undo, and redo.

## Empty Values and Canonical Arguments

Host required-argument admission previously classified every null and empty string as missing. That prevented clearing a text field or assigning null even when the declared schema was `Any`. Added neutral fixture cases for null, empty strings, empty object keys, root JSON pointers, and JSON-encoded full-width integer controls. The checks now parse the **host-effective** argument object, so missing argument metadata cannot hide a value-encoding transport failure. `Any` now requires presence; string schemas explicitly admit an empty value with `min_length(0)`. Shared edit actions use the canonical `value` argument throughout and no longer retain unused `valueJson`, `source`, or `key` aliases. Strict duplicate-key checks also cover JSON-encoded field inputs. These later edits await the native rerun.

## Shipping Catalog Closure

A new neutral-fixture-backed gate failed on the actual default feature closure: only 7 of 36 format editor packages were included (nine subset editors). The Cargo feature graph and plugin runtime registration now use the complete 88-editor fleet for both native and component hosts. The separate full-app-catalog branch was removed. Native catalog acceptance now runs with default shipping features, and the optimized component check also builds those defaults. Compilation, linking, size, and browser runtime validation remain pending.
