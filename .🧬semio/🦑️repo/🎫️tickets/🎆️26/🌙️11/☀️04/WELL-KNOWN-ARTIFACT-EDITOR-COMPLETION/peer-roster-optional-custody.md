# Peer Roster Optional Custody

The native GDB attribution identified an unreceipted 536-byte temporary allocation in app closure through lazy PeerPresenceRoot::empty_shared. Constructor opening had completed. The root agent owns the declaration's immutable EMPTY constant and both close-ladder implementations.

This agent changed PeerRoster storage to ManuallyDrop<Option<Arc<PeerPresenceRoot>>> and added inline ManuallyDrop<Option<PeerPresenceRootRetirement>> close custody. from_original retains Some(root), close custody starts None, and the convenience constructor explicitly allocates its owned empty Arc. There is no global shared-empty fallback.

The read helper borrows the live root or immutable EMPTY value. Field-separated interaction stamp reads use the same immutable static borrow without alias allocation. Render and command captures require Some(root) and fail closed after retirement.

Presence admission and validation reject absent root custody. Publication checks this before its typed Store commit, replaces the live Option root, and always queues the displaced root for funded retirement, including an empty root. Existing fixed retirement-slot reservations remain authoritative. No generated fixed byte quotation or production budget changed.

All normal/read/clone/test references were searched across framework, authored app and hub source. Tests that directly replace initial roots now retain the displaced empty owner in retirement custody. The held-root law drains the first empty displacement before asserting that the next captured root blocks retirement. Final close emptiness explicitly requires both root and inline close custody absent.

Edited callers reside in plugin stores/components and ingress, plugin-app implementation/render-surface/close-terminal, typed-command capture, field-separated interaction/render stamps, and presence/render contract fixture helpers. The root-owned declaration and close-ladder files were not edited by this agent.

Validation: all 13 edited Rust source files parse under the pinned AST parser. The audit agent's independent Ajv/SQLite handoff laws pass 4 tests and 62 assertions in 94 ms. Additional source pins are being added by that agent. Native physical validation remains queued under the root's controlled driver; no native/browser pass is claimed here.
