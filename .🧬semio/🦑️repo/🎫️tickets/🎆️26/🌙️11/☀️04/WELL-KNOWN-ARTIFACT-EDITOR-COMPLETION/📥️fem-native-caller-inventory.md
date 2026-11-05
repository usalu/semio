# FEM Native Caller Inventory

Read-only, no production edits/jobs. Fold cancellation marker verified: ordinary step checks cancelling before successful closed result handoff, routes rich close and returns Pending. Released control read remains guarded.

FEM roots:
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any

Within each root:
- 🚪️io/💾️binary/📸️snapshot/🦀️.rs receive_pack:2D73/3D84 returns Result<Self,ValueError>; outer receive_nested<RecordValue,Self> :77/:88 and inner RetainedRecordReceiving :79/:90 are old boundary signatures.
- 🧬️schema/📸️snapshot/native.rs receive_authoring:61 returns Result<Self,ValueError>; outer typed receiving :62, validation nested index :2D157/3D154. advance drops NativeReceive outcome.work at2D307/341 and3D301/335 through .result.map_err(primary). primary explicitly selects primary.or(quote_refusal).or(allocation_refusal), abandoning complete native failure custody.
- receive-record.rs typed row helpers :52; rows/variants outer+backing receives 2D72/75/93/96,3D69/72/90/93; support DOF vector2D174/3D175; element2D206/3D208; load2D245/3D247; root receive_exchange_record2D338/3D282. Helpers currently Result<T,ValueError>, nested fill callbacks similarly Result.
- identity-index.rs receive2D99/3D91 uses scoped_stage+nested receiver2D105/3D97 and vector2D117/3D109; lookup2D150/3D142 uses scoped_stage. String comparisons/sort perform repeated true domain work; cannot wrap entire operation as constant1.
- write-pack.rs:9 encode_exchange_pack returns Result<IoPayload,ValueError>, nested OriginalPackWriting :10; preserve original writer's cursor/progress/errors rather than fresh encode control.
- 🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs decode/encode_sqlite_snapshot_native :48–49 (3D; equivalent2D) use old Result signatures and generic store controlled-record helpers.

Actual native tests are any/🧪️tests/native-tests.rs, write-pack-tests.rs, loads/receiver-tests.rs. They require rich cause/work migration after production ports; no native pass inferred.

Current shared DSL receiving owner framework DSL/🧬️schema/🫴️receiving/🦀️.rs already concurrently uses receiving_turn meter and NativeSnapshotFailure: dynamic value :50, String :56, Option nested :72, Vec nested/backing :95/:97, Box :114. Preserve these newer changes; do not replay old Result-only assumptions.
