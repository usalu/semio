STATUS: PARTIAL - GREEN: driver, enum-field receivers, Run family (law passes opbin-20). WRITTEN, UNVERIFIED (waiting lock-37): mutation-diff KeyedMap + OrderedMap::retire_cold; hub foundation test port (SetCamera tuple variant, protocol-laws dev-dep) so the config/presence laws can run; hub space-app LIB port (initial_snapshot -> reference_birth, Arc<ViewModel> closure). NOT DONE: hub space-app TEST port (~30 call sites of the new artifact_app_laws harness: settle_registered_typed_operation(app,receiver,MountedOwnerPolicyV1,identity,cx), dispatch_typed(..,identity,..), missing imports of the *Setting leaves), hub camera -> OrderedMap, PropertyBag, Dag, Flow, pack-value cursor, derive switch.

## Table (family -> impl / law / check)
| family | wire | impl | law | check |
|---|---|---|---|---|
| shared driver `receive_to_completion` + `check_receiving_round_trip` (semio-os-dsl original_receiving) | - | written | - | GREEN opbin-1 rc 0 |
| hub SpaceConfigMutation (12 leaves + SpaceWindowCamera) | ordinal variants_binary | DslReceiving derives + original_ordinal_operation_receiving! | every_config_mutation_variant_is_received... | opbin-2 rc 1: blocked by foreign semio-s-space-core errors, 0 errors in hub files |
| hub SpacePresenceMutation (struct variants) | ordinal variants_binary | same | every_presence_mutation_variant_is_received... | UNVERIFIED |
| Dag / Flow / Run / Workflow | ordinal variants_binary | NOT STARTED, blocked (see below) | - | - |
| SpaceHistoryMutation (store/io/binary) | pack value encode (ToValue -> pack bytes) | NOT STARTED, needs a pack-value receiving cursor | - | - |
| InteractionConfigMutation | JSON bytes (same as wp1-window) | NOT STARTED, wait for hostcheck JSON-bytes binary receiving | - | - |

## Scope finding
- `impl OpBinary for` sites are ~230 across the tree (framework ~60 incl. fixtures, plugin workspace ~110 under ✏️s/🔌️plugins, hub 2), not ~20. Every `ArtifactApp::Mutation` needs one once the bound lands (hundreds of `type Mutation =`).
- Almost all hand-written sites use `dsl::variants_binary::{encode_op,decode_op}` (ordinal, or `tagged_value_binary`/`encode_tagged_op` with the protocol file): format u8 1, varint tag, record body. For these the receiving impl is `#[derive(DslReceiving)]` on the enum and EVERY leaf and nested record type plus `original_ordinal_operation_receiving!(T)` / `original_tagged_operation_receiving!(T, PROTOCOL)`. No per-family cursor code.
- Blockers inside the vocabulary (`DslOwnedReceivingField`/derive): scalars, String, Option, Vec, Box, PersistentList, nested derive records and enum variants (single unnamed or named fields) are covered. NOT covered: hand-crafted `BorrowedDslField`/`DslField` types (RunStatus, RunTrigger enum-with-record-shape, ...), `PropertyBag` (map of recursive PropertyValue, Dag nodes), `BTreeMap`, flow `Widget` (Dictionary/OrderedSet/OrderedMap/Tree roots), tuple Vec. Each needs a hand-written `DslOwnedReceivingField` in its owning crate.


## Vocabulary extension (coordinator-approved)
- 1a WRITTEN, UNVERIFIED: new derive `DslScalarReceiving` (dsl-record-derive root + emitters) gives a unit-variant enum an OwnedScalar/ScalarReceiving field receiver over FieldValue::Enum(ordinal). Opt-in (a new derive, so existing DslScalar users are not forced to impl RetireOwned). Hand-crafted scalar enums (RunStatus) can use it once they add the derive/`DslField` ordinal parity.
- 1b NOT STARTED: data-carrying enum as FIELD (RunTrigger-shaped kind-tag record; DagNodeKind). The derive's enum branch emits only DslOwnedReceivingVariants; the field form needs a record-shaped cursor selecting a variant by the `kind` text, reusing fields_cursor.
- 2 NOT STARTED: BTreeMap<K,V>/tuple-Vec receivers (FieldValue::Map; mirror VectorReceiving).
- 3 NOT STARTED: PropertyBag / flow Widget hand receivers (NativeIntrinsicReceiving exists for NativeReceive<DslValue> types and is the likely route for PropertyBag/Widget).
- 4 NOT STARTED: pack-value cursor (SpaceHistoryMutation); JSON-bytes cursor (Interaction) - hostcheck says wp1 needs no cursor under its ArtifactPersistedOperation trait.
- Space-core port: last_turn_work, `.map(|e| e.files)` for pack exports, `P: Clone` bound (claims line 15:20).


## Update (vocabulary landed)
- Unit-only enum field: already covered by `DslReceiving` on unit-only enums (borrowed + OwnedScalar + owned field). My DslScalarReceiving derive was redundant and is REMOVED.
- Data-carrying enum field (`#[dsl(statements)]` single-statement shape, ArtifactBody-like): `StatementReceiving<T: DslOwnedReceivingVariants>` (owned/wrapping) + derive emits `DslOwnedReceivingField` and borrowed `DslReceivingField` for DslReceiving enums. Laws: every_unit_enum_field_value..., every_statement_enum_field_variant... (module-companion receiving tests, opbin-12 GREEN). hostcheck builds ArtifactBody on it.
- RunTrigger-style enums with a hand `kind`-tag Record shape are NOT the statements shape: need a hand receiver mirroring their encoder (not started).
- BTreeMap: std BTreeMap nodes cannot be admitted against native allocation accounting up front (that is why PropertyBag is a sorted Vec). Root fix is to migrate such mutation fields to a sorted-vec map type with a Vec-style receiver, not a BTreeMap receiver. Not started.
- Space-index config leaves + SpaceArtifactRow/Dialect got DslReceiving; hub config typo `protocol::protocol::ColdMutation` and presence `decode_presence_native` fixed.
- Cargo.toml changes (locks done): hub foundation (+3 deps), dsl module-tests (+trace dev-dep).


## Findings 
- SpacePresence has a BTreeMap<String,SpaceWindowCamera>: no NativeReceive/NativeDefault/receiver for BTreeMap exists anywhere; decode_presence_native is a typed refusal until the map vocabulary lands (compile-only port; this is a REAL feature gap for hub presence decode).
- Cargo-manifest drift in hub crates (foundation, engine): derive-emitted `::semio_framework_*` paths need direct deps.
- Recommended derive-switch design: a one-turn ControlledFieldReceiving<T: DslField> fallback (from_value_controlled under the turn's work recipient) is the only way to cover hand-crafted DslField types without per-type code, but it gives up paged accounting for those fields and fails typed with WorkLimit for large ones. Needs a coordinator decision before the DslRecord/DslEnum switch.


## Run family (done)
Files: run snapshot (sub-record derives, RunTriggerFields mirror + RunTriggerMap + borrowed DslReceivingField for RunTrigger, DslReceiving on the two unit enums), 9 leaves, RunMutation aggregate (derive + `dsl::original_ordinal_operation_receiving!` + `dsl::artifact_persisted_binary_operation!`), law in run lib tests 🧪️tests/🔬️receiving (the companion package is broken by 8 stale sqlite snapshot errors, not mine).

## Plan for decision (b), OrderedMap migration
1. mutation-diff crate: `KeyedMap<V>` trait (lookup, pairs, put, take, discard) impl for BTreeMap and OrderedMap; make keyed_rows_apply/inverse/changing generic (macro unchanged). Needs Cargo dep on semio-framework-value-ordered (LOCK) and a public `OrderedMap::retire_cold` wrapper (the private retire_cold exists): OrderedMap's Drop asserts explicit retirement, so every cold clone/discard path must retire.
2. hub presence/config camera BTreeMap -> OrderedMap<SpaceWindowCamera> (3+5 usages + tests; tests must retire maps), then decode_presence_native via decode_presence_json (NativeReceive for OrderedMap exists).
3. census flag for std BTreeMap in persisted types.


## Hub test port notes
- The stale space-app tests need the current harness bundle (see 1102-tools/testkit/work/h.rs): MountedOwnerPolicyV1, EntityIdentity, cx per call. That is a mechanical but wide port (unit tests + compiled-dag/workflow window tests); not started beyond the lib fixes.
- The config/presence laws live in the foundation crate tests, which only need the foundation fixes above.
