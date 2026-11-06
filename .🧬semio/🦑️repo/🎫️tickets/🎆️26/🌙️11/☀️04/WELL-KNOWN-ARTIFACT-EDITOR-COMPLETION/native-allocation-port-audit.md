# Native Allocation Port Audit

Read-only bounded scan of framework and product Rust production call sites, excluding value native allocation/observer/child/work and Job/Source recipients owned by other agents. Locations reflect concurrent workspace state. No compilation performed.

## Priority and semantics

1. SQLite snapshot model is the central shared bridge: model lines 276 and 293 emit a single NativeEncodeAllocation and advance forwarded_bytes/owned at admission. Artifact lines 88 and 401 and transfer lines 20, 54, 70 reduce native requests to bytes. Implement pending request identity, actual settlement and cancellation across the stage interface first; downstream snapshot bridges inherit the limitation. Other SQLite files listed below use the same one-phase bytes callback.
2. Plugin foreign operation chain is end-to-end one-phase: guest authority calls operation_allocation, snapshot reservation advances a Cell, interpreter imports and Wasmtime operation requests carry four numbers, and EntityIdentity receiving and post-turn codec reservations advance owned/reserved on admission. The wire contract, schema and both runtime bindings must distinguish quote/post/cancel and correlate request; preserving refusal code and settlement errors requires explicit matching. The literal operation-allocation dispatch in interpreter host-replies and imports, Wasmtime func_wrap_concurrent and OperationRequest::Allocation are coupled string/control bindings. Check corresponding WIT and generated bindings as well.
3. Plugin shard identity recipient enforces policy per request; its quote, post, cancel require explicit pending state and physical actual accounting. The snapshot plugin authority similarly uses one shared owned counter across native and SQLite receipts; avoid counting requested bytes as retained.
4. History and store member receiving recipients reserve work grant on admission; track pending versus actual and cancellation, without assuming conservation. Native forwarded controls accepting an already event-aware port are pass-throughs and not necessarily offenders. Source JSON relay is a genuine Source recipient delegated separately.

## Location roster

### SQLite snapshot producers and bridges

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🦀️.rs:88`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🦀️.rs:401`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🦀️.rs:20`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🦀️.rs:54`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🦀️.rs:70`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/🦀️.rs:157`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/🦀️.rs:185`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/🦀️.rs:248`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/🦀️.rs:276`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/🦀️.rs:293`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:79`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/💰️backing/🦀️.rs:32`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs:10`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🪶️sqlite/🦀️.rs:15`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:206`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:349`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs:11`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs:11`
- `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:12`

### Plugin and foreign operation boundary

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs:29`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs:38`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs:52`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs:54`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs:81`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:84`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📣️host-replies-wasi-and-channel-faults/🦀️.rs:41`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📣️host-replies-wasi-and-channel-faults/🦀️.rs:46`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:958`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🐎️wasmtime/🦀️.rs:3`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🐎️wasmtime/🦀️.rs:26`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🐎️wasmtime/🦀️.rs:51`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📨️slot/🦀️.rs:9`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🎛️guest-runtimes-and-test-doubles/🦀️.rs:149`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🦀️.rs:4`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🦀️.rs:17`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🦀️.rs:21`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🦀️.rs:30`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🦀️.rs:93`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🦀️.rs:94`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🧬️codec/📥️receiving/🦀️.rs:5`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🧬️codec/📥️receiving/🦀️.rs:12`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🧬️codec/📥️receiving/🦀️.rs:29`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔤️encoding/🦀️.rs:17`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🦀️.rs:13`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🦀️.rs:8`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🦀️.rs:9`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🦀️.rs:10`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🦀️.rs:11`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs:8`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs:52`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs:69`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs:72`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs:73`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs:99`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs:100`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:34`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:38`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:50`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:54`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:67`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:72`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:90`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs:94`

### Other native receiving/control

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs:2`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs:16`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs:88`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs:130`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs:141`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs:7`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs:50`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:23`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:199`

