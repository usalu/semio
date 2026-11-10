# Lane dslrecv: generic owned-receiving cursors (value, set, map, block) for `DslReceiving` / `OpBinaryReceiving`

## Design (state: FIXED-verified)
The three missing cursor families are one generic cursor, because `FieldValue` already is a `NativeValueSource` (`dsl-record-ownership/🛬️source.rs`: records with ordinal keys, maps with name keys, lists, statements, blocks, `Value(DslValue)` delegation) and every native tree already has `NativeReceive<I: NativeValueSource>`.
- `🫴️receiving/📥️owned/🌳️intrinsic`: `NativeIntrinsicReceiving<T>` generalised to `NativeSourceReceiving<I,T,O>`; `O: NativeSourceOrigin<I>` says where the source sits in the moved field (`IntrinsicOrigin<T>`: field of a record, source `DslValue`, unchanged behaviour and phases; `WholeOrigin`: the moved `FieldValue` itself, no shape phase). `NativeIntrinsicReceiving<T>` is now a type alias, all phases/quotes/Pending/recover paths are the former ones (they delegate to `NativeReceivingOriginal`, whose `advance` loops actions while the grant admits the next exact quote).
- `🫴️receiving/📥️owned/🧊️field`: `NativeFieldReceiving<T>` = `NativeSourceReceiving<FieldValue,T,WholeOrigin>` covering Value-shaped fields (Dictionary, FlowUi), record blocks (Tree) directly, plus the named `SetReceiving` (`OrderedSet`) and `MapReceiving<V>` (`OrderedMap<V>`). `DslOwnedReceivingField` for `OrderedSet` and `OrderedMap<V>`.
- `🫴️receiving/🦀️.rs`: `receive_decoded` (borrowed `DslReceivingField` through the field's own `from_value_controlled`, one measured construction), impls for `OrderedSet`/`OrderedMap<V>`, `ReceivingOutcome`, and borrowed impls for the sets/maps. The macro `native_field_receiving!(Type,...)` (both traits for a type whose native receiver builds it from the field source) lives in `dsl/🧬️schema/🧊️field` (dsl-record).
- neural engine `📝️dsl/🦀️.rs`: `native_field_receiving!(Dictionary,Tree)`.
- flow-model `🧱️document-model`: `DslReceiving` derives on `WidgetLayout`, `FlowLayoutEntry`, `SynapseSpec`, `Widget`; `native_field_receiving!(FlowUi)`. The derive only gained the enum-level `DslReceivingField` (below).

## Tests (language agnostic + oracle)
`🗣️dsl/🧬️schema/🧪️tests/🧊️field-receiving/`: `🔣️.json` corpus, `🧬️schema/🔣️.json` JSON Schema, `🟦️.ts` (Ajv + SQLite `json_each` oracle for set distinct/ordered and map key order, canonical JSON for literals/blocks), `🦀️.rs` (4 laws over set/map/value/block: every denied axis charges nothing and keeps the quote, Pending is resumed with the identical grant, every cancel prefix `cancelAt` closes through paid turns to `terminal_is_empty`, outputs equal the std BTreeSet/BTreeMap and serde_json oracles).

## Additional changes found on the way
- The former intrinsic cursor stopped at admission phase `projectionStart` since 733 ("original projection must be driven"): the cursor now drives the staged projection (begin/drive/transfer/accept/finish, refusal handback, cancellation close). `StagedProjection` is a public type in `value/🛬️decode/🧳️owned/🧳️original/🧭️staged-projection` (dsl-record-model hit its 262144 B source budget). Debt: store `snapshot-clone/🧭️projection` has an identical private copy; reuse the public one.
- Admission output needs two `take_output` turns before the admission is terminal-empty (cursor phase 4 now loops via `taking()`).
- `FieldValue`/`RecordValue` had no `query_schema_key` (ordinal records could not be keyed, derived `NativeReceive` blocks parked forever): added in `dsl-record-ownership 🛬️source.rs`; `NativeSourceKeyReply(Complete)` re-exported from value-native-owned.
- Derive `DslReceiving` on tagged enums now also emits the borrowed `DslReceivingField` (one `Statements` entry), which flow-document's `add-widget`/`change-widget` needed.
- Stale pre-733 tests (intrinsic/work/bank) moved unwired to `dsl/🧬️schema/🧪️tests/🗄️owned-receiving-pre733` (grant-API, did not compile). Debt: port them to the effect API.

## Labels
- native flow-model lib W-dslrecv-20 rc0; wasm32-wasip2 W-dslrecv-21 rc0 (earlier W-dslrecv-13/14 also rc0).
- tests: `tst.sh W-dslrecv-<n> "<dsl-record Cargo.toml>" native -p semio-framework-dsl-record --test field_receiving` -> W-dslrecv-19 4/4 pass (set, map, value, block; each at every cancel prefix, denied turns charge nothing). TS oracle `bun test` in `🧪️tests/🧊️field-receiving` 4/4.
- Not run: flow-document compile (flowdoc owns it); a runtime test of Widget/Tree through the derived cursors.

STATUS: FIXED-verified W-dslrecv-20 (native), W-dslrecv-21 (wasm), W-dslrecv-19 (tests)
