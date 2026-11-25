# IFC 4x3 / 2x3 Port, Round 6

## State
- Last full 4x3 run (ifc54): 69 passed, 11 failed. Since then, fixed and individually confirmed: enum (full53), guided list/measure/mixed/required-type-input. value-select now passes its assertions but fails in retirement (below). Full suite not rerun after the latest edits.
- 2x3 ports and scalar/enum/set commit law: not started.
- Rules crate (`semio-s-artifact-stdio-ifc4x3-rules`) test target: compile fixes applied, never built (see below).

## Fixes this round
- ISO 10303-11 scoping (`machine-population.rs`): an identifier absent from the entity's attributes falls through to schema-scope resolution (type, enumeration item, constant); only a name missing from every scope is `express.unbound`. End-to-end proof: enum `full53` (IFCDERIVEDUNIT, qualified item `IfcDerivedUnitEnum.USERDEFINED`) passes. Law `local_identifiers_resolve_attributes_then_schema_scope_and_unbound_names_stay_refused` added in `machine-local-tests.rs` (ACCELERATIONUNIT pass, USERDEFINED without user type fail, with user type pass). It covers the qualified item only; shadowing and unbound-name laws are open (IFC has no attribute that shadows a type name; needs a synthesized entity). ifcopenshell is not installed offline.
- `type_input.rs` phase 14 to 11 transition now resets `state.offset` (a stale 9 from the type-name scan sliced a 5-byte binary).
- value-select fixture: the four `domain: null` cases carry explicit `ready: false` (schema updated); the test asserts `domain` when set, else the case's `ready`.

## Rules crate test target
- `//!` first lines of four `include!`d fixtures changed to `///` (E0753).
- `SetEntityArg` import path corrected.
- `machine-root-fixtures.rs`: `..super::super::super::PROGRAM` no longer resolves; the constant it named was replaced by the runtime-decoded `ifc4x3_express_program()`. No empty base is used. History (`git log -p`) shows earlier fixtures built programs from `PROGRAM.functions[..]` plus probe functions. The probes call the canonical `IFCVECTORSUM`, `IFCVECTORDIFFERENCE`, `IFCSCALARTIMESVECTOR` and read derived `DIM`/`DIRECTIONRATIOS`, so an empty base cannot be shown to be sufficient. `probes()` now builds each probe program as the canonical program with the probe functions appended (same pattern as the existing `memos()`); `PROBES` callers updated.
- Not yet built: more errors may follow.

## Open
- value-select stall: with fuel 0 and yield true the staged owner never retires. Debug at round 5000 showed demand copy 24 / release 32 / depth 4, `birth=false`, `control=false`, grant not exhausted, `retirement_should_yield()` true, `fuel_remaining()` 0. The fixture's fuel stock is 1<<30; the test needs about 1M render turns at 1000 fuel each (146 to 254 s), which spends the whole stock before retirement. This is the same cost as the 71 s set-select test: the product does too much work per turn. Needs a profile of per-step work in the type-guidance state machine (a single-thread sample of one render, with the test alone), not a larger stock.
- guided_controller: "retained source original authority is closing or transferred".
- denied_axes: action swap expected refused, accepted.
- required_type teardown: "test arena retirement refused" in some run orders.
- Editor: `domain_insertion...` (`retained_text_bytes() >= retained_bytes`), `actual_publication_wire...` (`ifc-domain-argument-arity`, RemoveValue).
- staging-native: text-birth-axis stock, expired-deadline "original Source clock declined new construction", render_owner "admission birth changed its original Work or Source", identity_drafts unsettled frame.
- Commit law (scalar, enum, set leaf); 2x3 test-target ports (11 base, 24 base-details errors).
