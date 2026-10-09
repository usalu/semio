# Owned History Fold: Current Source and Native Receipt

The current source replaces coroutine-local deferred destruction with awaited admission of the actual typed retirement frame. The original value remains inline in an admission future until real item and capacity credit funds that frame. Once admitted, the local mutates the original inside `ControlledRetirement<T>`. Scope exit queues the same already-admitted frame; `take` transfers the original and queues the empty frame for separately granted physical release. Unsupported local admission returns `(ValueError, original)` without registration or heap movement. Cancellation of a waiting admission sees its pending exact frame demand before destroying the coroutine.

The admitted frame constructor remains first-party. Its new `original`, `original_mut`, and `take_original` methods expose only the original typed owner before cursor construction; the final method refuses once a retirement frontier has been constructed. Frame births consume actual current item/capacity credit and their receipts are included in the current poll. No capacity is fabricated from history extent or inferred from logical copy credit.

Fold scratch indexes and the supersession output now use `HistoryFoldIndex<K,V>`, an owned AVL arena. Original Vec node slots remain allocated when a node is removed or popped; displaced keys remain in the same owner until controlled retirement. Node retirement declares every field and exact deferred scaffold. Sorted live entries determine semantic equality independently of arena shape or dead slots. Native BTreeMap/BTreeSet have not been declared physically accountable.

## Exact direct Store consumer contract

`protocol::HistoryFold.supersessions` is now `protocol::HistoryFoldIndex<protocol::MutationId, protocol::EffectiveSupersession>`. Store sites carrying that field need direct type migration; do not convert it back into an opaque native BTreeMap or add compatibility adapters. The index exposes borrowed `get`, `get_mut`, `contains_key`, `iter`, `keys`, and `values`, plus owned `insert`, `remove`, and `pop_first`. `get_or_insert` returns a mutable value. Mutating removal retains the original node slot and key custody. Sorted iteration yields `(&K,&V)` and supports reverse iteration. `FromIterator<(K,V)>` is a cold construction path. The direct `first_entry_after(&key)` query now borrows the next strict successor by AVL navigation without allocating a range cursor. Borrowed `IntoIterator` traverses live sorted entries without allocation.

Explicit receiving locations currently include Store supersession fields, `fold_envelope_history(...).supersessions`, `report_replay`, `replay_window`, `same_effective_inputs`, effective-operation and prefix-digest paths, superseded-position comparison, and history hydration. Root assigned this worker the narrow required Store receiving migration. The Store alias now names the same original HistoryFoldIndex, and derived replay copies use first_entry_after. Only supersession-specific constructors were changed from native-tree construction to direct owned index construction. ArtifactStoreSupersessionRetirement now retains ControlledRetirement<EffectiveSupersessions> inline, rather than flattening the original node/key/payload custody into byte buffers. Its grant, demand, and terminal methods delegate directly to the canonical owned cursor; broader Store retirement/factory bodies remain with their active owner.

## Exact source manifest

- Framework Value controlled retirement: `🌱️value/♻️retirement/🎮️controlled/🦀️.rs`: typed admitted constructor visibility and original borrow/mutation/transfer APIs.
- Replication fold: `📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs`: awaited local admission, preborn frame custody, genuine birth receipts, canonical owned scratch indexes and all direct coroutine consumers.
- Replication transition: `📡️replication/🔗️causal/🔀️transition/🦀️.rs`: supersession output owns `HistoryFoldIndex`.
- Fold index: `🔁️fold/🗂️index/🦀️.rs`, `🧫️fixtures/🔣️.json`, `🧪️tests/🦀️.rs`: independent semantic index law checks each action against serde_json, every AVL balance/count invariant, removed node custody, and semantic equality across reverse insertion.
- Fold physical law: `🔁️fold/📏️retirement/🧪️tests/🦀️.rs`, `🧫️fixtures/🔣️.json`: same-poll new-local admission under zero/subexact capacity preserves original pointer and heap custody; unsupported admission returns original; existing independent retirement heap laws remain active.
- Replication retirement: `📡️replication/🧬️retirement/🦀️.rs`: MapPresence, MapEntryDelta/Operation, InputReplacement, MutationOrigin, ConflictId, and ConflictKind declare exact birth/support using deferred original fields.
- Transition unit law: `🔀️transition/🧪️tests/🔬️unit/🦀️.rs`: expected supersession fixture uses the current owned output type.

## Actual native receipts

The registered command is `bun nx run @semio-tech/framework-replication-rs:test --args="quick --lib bounded_history_ --no-fail-fast -- --nocapture" --excludeTaskDependencies --skip-nx-cache`, launch row `⏳️artifact-io-history-fold-retirement🧪️` in seed and live launch files. It uses repository Cargo preparation and cached native owners with no target-directory override.

- Prior root current4: two physical laws passed; three semantic decoding/fold laws failed, including a same-poll unreserved frame birth and unsupported native BTree scratch custody.
- Current5, session 37175: native RED before Replication on four current shared Value syntax errors. Exact diagnostics are in generated/owned-closure/history-fold-retirement-current5.log, lines 93–128. Those receiving owners were notified.
- Current6, session 63919: native reached Replication, RED on duplicate counter initializer fields authored in the rewrite; fixed directly.
- Current7, session 14669: actual native GREEN, exit 0, eight tests passed (280 skipped by focused filter). Both prior physical heap laws, all three prior semantic fold/transition/envelope laws, same-poll admission pointer/heap law, unsupported original-refusal law, and owned AVL arena differential law executed. The generated log includes their actual `[DEBUG]` runtime receipts. Nx duration 7.5 s; test execution 0.704 s.

The narrower laws establish retirement frame admission and closure. They do not yet prove that every normal fold payload allocation or index insertion is charged to the poll's capacity budget. Existing physical laws observe cancellation/retirement, and that scope is preserved explicitly.

- Current8, session88631: actual native GREEN9/9, including mixed preborn/waiting-local cancellation with exact heap receipts.
- Current9, session4833: actual native GREEN10/10, 280 skipped by focused filter, adding all ten manual protocol variants' actual native frame/scaffold/backing admission and release laws. DEBUG records each variant. Test execution1.151s, Nx6.4s.

## Store receiving source and current validation

New direct borrowed iterator/successor APIs are in the original fold index owner. Store root changes are restricted to EffectiveSupersessions' owned type, supersession constructor sites, derived history replay's strict successor query, and the original-custody supersession retirement body. A new neutral receiving fixture and native law under Store `📏️supersession-index` compare ordering and successor results against independently parsed JSON, preserve original payload pointers through direct fold→Store type assignment, and verify heap birth/release receipts under independent undergrants through exact terminal drop.

Registered seed/live row `🏪️artifact-io-owned-supersession-receiving-native🧪️` executes `bun nx run @semio-tech/framework-os-kernel:test --args="quick --lib owned_history_fold_supersessions_ -- --nocapture" --excludeTaskDependencies --skip-nx-cache`. Actual current1 session62075 is RED before Kernel compilation on current shared Value ordered-map insertion calling lookup.advance with the old comparison-only arity and two-element return. Current advance needs an independent retirement grant and returns three elements. Exact generated receipt lines460–505; current Value owner notified through root. No receiving runtime pass is claimed.

## Fresh whole PackJSON receiver receipt

The previously registered complete PackJSON native row was run after the Object last-member/custody APIs. Actual session66385/current1 is RED before PackJSON on five contemporaneous Value ordered-map CloneCursor `self.lookup` references (close_step309 and next_capacity331), where the clone cursor has no lookup field. Exact generated receipt `pack-json-owned-members-current1.log` lines488–528. This is not a PackJSON runtime pass. The owning foundation was notified through root; identical native retries are held until that source changes.

Current Store receiving2 session81101 reached current Value and Replication compilation, then RED before Store on three current UI Scene math errors: removed ungranted owned_retirement1078 and removed next_close_byte_demand/two-argument close_step1080. Exact generated receiving2 log lines4668–4722. That owning foundation remains with General/root; no receiving runtime pass is claimed.

Fresh whole PackJSON2 session57045 is actual native FULL GREEN66/66, zero skipped, exit0, after concrete shared Value source repair. This includes the new owned_json_member_transfer_keeps_independent_key_value_and_backing_custody law with DEBUG original key/value/backing receipt. Native test duration20.464s; Nx25.7s. Relative test artifact routing in the preceding Kernel command was identified from its receipt; that failed command's own nextest receipt was moved to the real ticket generated folder. Subsequent native commands must use the absolute ticket SEMIO_TEST_ARTIFACT_DIR, as the registered launch row already does.

## Multiple Waiting Original Frame Items

The public admission contract now also publishes `next_local_admission_item_demand()`. A coroutine may hold two registered but not yet awaited admissions; destroying it would birth both actual typed frames. Pending item count is tracked alongside exact frame capacity. Work and cancellation refuse unchanged when the caller has not funded all pending frame items before an atomic coroutine destruction. No larger item grant is fabricated. The funded destruction receipt reports every actual frame birth, and normal sequential awaited runners still require one pending frame item.

Actual current10 session90591 is native GREEN11/11 (280 skipped), exit0, including a new two-waiting-original law: zero/subexact item grants preserve both originals and produce zero heap movement; an explicit two-item grant births both real frames, releases the future allocation, and reports the exact heap receipt. Native execution3.168s. This current run also recompiles the direct borrowed successor and iterator changes used by Store.
