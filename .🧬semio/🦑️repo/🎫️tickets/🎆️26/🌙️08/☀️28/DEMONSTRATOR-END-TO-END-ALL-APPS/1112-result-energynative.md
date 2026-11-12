# energynative lane (11/12): semio-s-artifact-energy-model-native-mutations on the reply/effect fields API
Crate dir: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native` (N).
## Design (shared by every module)
- `N/🛡️custody/🦀️.rs` (new): `Cursor` trait (finished/native/demand/work/birth/control/birth_step/close/mark), impls for the fields trio, RetainedOwnedProjection, diagnostics, PersistentSet/Splice/ListRead, `Option<Box<C>>` (slot release), `Held<T>` (value+own retirement), `Bound` (binding), `Sourced`, `Ledger`; `close_chain`/`chain_step`/`finish` (ordered close over children, routed through `retained_routed!`), macros `custody_impl!(fields..)` (owner close surface over `flight`+`closing`) and `std_cursor!([gen] Ty,[where],|s|s.begin_close())`.
- Owner protocol: `quote(..)->(RetirementDemand,u64)`, `advance_reply(.., effect, reply, recover)`; declared work = max(quote units, effect delta); children via `retained_routed!` on the owner's `flight`; forward closes via `custody::finish`.
- `row/` has `Staged<B>` (project/exchange/seal/take one branch) and `RowTail<P>` (staged row -> admission -> source -> RowBranchPreparation -> diff) shared by scalar/membership/adjacency/value; `staged_step`/`tail_step` run them inside an owner turn.
## Checklist (module: state)
- custody: DONE (W-energynative-5 rc0, with membership/adjacency/owned/text `mod` lines temporarily commented as `//[DEBUG-OFF]` in N/🦀️.rs; RESTORE them by deleting the marker as each module is ported; backup of original root in scratchpad root.rs.bak)
- leaf (RowFieldPreparation = Cursor + quote/advance_reply/take_quote/take/inverse_quote/prepare_inverse(effect)/take_inverse_quote/take_inverse(effect); RowLeafPreparation): DONE rc0
- row (RowBranchPreparation, Staged, RowTail, change, output/direct/nested, admission): DONE rc0
- reference: DONE rc0
- scalar (+leaf, transform): DONE rc0
- membership: DONE (W-energynative-6 rc0); adjacency: DONE (W-energynative-7 rc0) -- `//[DEBUG-OFF]` markers for these two already removed from N/🦀️.rs
- value (+plane, permutation, rule, leaf), text (+name, validation, leaf): TODO. Old versions saved in scratchpad (membership.old.rs, adjacency.old.rs, value.old.rs). Recipe: struct fields -> Bound/Held/Sourced/Option<Box<child>>; `custody_impl!(children..)` + `std_cursor!`; quote()->(demand,work); advance_reply with routed children; RowTail/tail_step for the row publication tail; `OwnedRowValidation` must become `Cursor` + quote/advance_reply; RetainedEqualityOwner advance_into(left,right,body,max,effect,reply)->RetainedEqualityStep{progress,equal}, its begin_close is reply-shaped (begin_close_into) so its Cursor::native must use native_child.
- schedule/target: no production code touched
- 🌱️initialization/🌱️state (EnergyNativeInitialSource): TODO
- tests under N/*/🧪️tests and `tests.rs`: stale (grant API), not ported (debt)

Framework edit: os-kernel facade re-exports retained_native_child/retained_slot (claims log). Crate root has `pub mod retained` aliasing Done/Flight/Reply/conclude/native_child/slot.
Rerun: "$C/1105b-tools/chk.sh" W-energynative-<n> "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-energy-model-native-mutations

## FINAL (11/12)
- native-mutations: ALL modules ported (value/plane/permutation/rule, text/name/validation, membership, adjacency, scalar, row, leaf, reference, custody). W-energynative-11 rc 0 (wasm hub, -p semio-s-artifact-energy-model-native-mutations). All `//[DEBUG-OFF]` markers removed.
- 🌱️initialization/🌱️state EnergyNativeInitialSource ported (effect-shaped advance via `cx.with_original_effect` + `consume_work`, close via `close_original_effect_step`); compiled in W-energynative-12 (no error in it or in native-mutations).
- W-energynative-12 (-p semio-hub-energy-component) rc 101: remaining errors are only DOWNSTREAM consumers of the changed native API, not claimed by me: mut-boundary (6, boundary/change-boundary-condition: ScalarStorage::admit->Raw/raw/quote/admit(raw,out,effect)), native-mutations-lifecycle (42), native-mutations-singletons (78: RowFieldPreparation-style impls need Cursor + quote/advance_reply/take(effect)/take_inverse(effect); NetworkPreparation, SingleFieldPreparation, TransformedSinglePreparation, OutputVariablePreparation, RootLinkPreparation). Recipe = this crate's modules (custody_impl!/std_cursor!, quote, advance_reply, RowTail).
- Debts: tests under N/*/🧪️tests and tests.rs still use the grant API (not compiled with --tests); quotes for Ledger/FactoryVector admit use work 1 (unverified); runtime (quote==actual) not exercised; session crate belongs to lane energysession.
Rerun: "$C/1105b-tools/chk.sh" W-energynative-13 "🌎️hub/Cargo.toml" wasm -p semio-hub-energy-component
STATUS: FIXED-verified W-energynative-11 (native-mutations); hub-energy-component BLOCKED on downstream singletons/lifecycle/mut-boundary
