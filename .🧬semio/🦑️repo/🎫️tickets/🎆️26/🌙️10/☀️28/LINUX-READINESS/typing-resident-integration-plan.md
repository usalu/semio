# Typing Resident Integration Plan

Prospective Source plan only; do not edit behavior before actual52482 compiler evidence. Existing parent keyed state/BTreeMap and authored Original edit/net/resident implementation are incompatible representations, not a missing import. A canonical net Vec plus explicit resident storage can be integrated, but all consumers and ownership/accounting laws below belong to one coherent change.

## Exact Current Consumers

- Neutral typing owner `tool-machine/⌨️typing/🦀️.rs`: TypingState225–233 stores entries; Typing::resume252/254 reconstructs context and ToolTransaction from keyed entries; state322 snapshots keyed entries; TypingLedger::provisional366 exposes leaf references. Cold take/keep/send/commit/lapse/abort/all/retain_windows use BTreeMap get/insert/remove/keys/values/iter. Retained reader methods463–476 rely on sorted map cursor, slot identity and exact borrowed state.
- Neutral typing law test `tool-machine/🧪️tests/🔬️unit/⌨️typing-laws/🦀️.rs:140` serializes state.entries into the existing independent JSON oracle. Preserve semantic expected output/order rather than changing the fixture to mirror an implementation shortcut.
- Plugin follow `plugin/🛠️tool-machine/🔁️follow/🦀️.rs:52,67` reads retained typing state entries length and projects actual entry payload at index. This actual Original overlay path must switch to references into canonical net without cloning or constructing temporary keyed entries.
- Plugin typing commit `plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs:21,39–48` owns keyed entries, allocates a separate values Vec, reverses/pops entries, retires keys, then yields values. Canonical net should transfer the actual original Vec backing, updating phase/demand/retirement accounting instead of preserving obsolete allocation/key-copy machinery. Its test24 records actual payload pointers and transaction identity; expand this proof to exact net backing and capacity receipts.
- Shared neutral `tool-machine/🔗️read/📤️transfer/🦀️.rs` invokes original_transfer!(TypingLedger,TypingState,original_slot,original_slot_mut), then defines map_slots!(TypingLedger,TypingState). Vec resident supplies its own slot lookup, so keep transfer behavior but remove/adapt the TypingLedger map_slots instantiation to avoid incompatible get(window) calls or duplicate methods. Gesture map implementation remains independent.

## Coherent Storage Boundary

Mount typing/resident only after the ledger actually owns Vec<TypingWindow<M>> and writing state. Migrate all cold and retained accesses through the same sorted lookup/slot functions, preserving lexicographic window iteration, prefix-near-miss equality, stable slot keys across Vec relocation, live_windows counts, monotonic source revision and read/write exclusion. Vacant slot ownership must remain until the funded address close; removing an element cannot silently drop a borrowed Original or a key.

Canonical TypingState.net can remove positional string keys from persisted provisional state. The independent semantic oracle may still render ordinal key/value pairs for comparison if these keys are truly positional; that is a test projection, not a compatibility runtime state. Actual schema/derive/reader/export contracts must be updated together if they describe fields. RetireOwned derives will observe the new net vector; prove its allocation/release receipts instead of assuming old keyed-field costs still apply.

The orphan resident code is not automatically complete: insertion currently charges all shifted rows in one operation, and growth reserves/copies whole backing. A sufficiently large window set with a small retained grant may refuse forever unless the owned protocol bounds the set/grant or provides resumable paid shifts/growth. Existing code cannot establish arbitrary bounded-grant liveness solely by looping. Real corpus cases must cover growth, middle insertion, tiny grants, cancellation and full close before calling this storage generally bounded.

## Existing Original Safeguards to Preserve

Keep OriginalSourceBinding, actual decoder identity, ledger identity/revision validation, funded admission before mutation, retained read rejection, single active writer and cancellation-before-exchange behavior. Replacing the keyed state with net must move incoming/result/displaced original buffers through the same real retained slots. A clone, rebuilt Vec or fake receipt would invalidate the intended physical identity/heap witness proof. Close must retain all paid residues until terminal retirement and only then release writer ownership.

## Focused Acceptance Scope

After actual compiler red, repair and run existing neutral edit corpus/heap witness, parent typing law corpus, retained read/transfer tests, plugin original typing commit tests, and plugin follow overlay tests. Include exact backing identity through edit/reuse/split/empty/cancel and old-state reinstallation. Keep independent JSON oracle outputs and actual heap allocations reconciled with Original receipts; do not replace these tests with static compile-only checks. No Source, tests, compiler, Cargo, Native, process or lease operation was changed or executed during this audit.
