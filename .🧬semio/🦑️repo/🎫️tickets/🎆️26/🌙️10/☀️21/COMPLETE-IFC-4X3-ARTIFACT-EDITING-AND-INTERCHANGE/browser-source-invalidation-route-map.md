# Browser Source Revision Invalidation Route Map

Read-only source checkpoint2026-10-21. There is no single reviewed PluginSourceEvent document-change stream. Root's new revision lease/cache must cover existing mutation boundaries,not invent a producer from plugin availability events.

Paths below relative to renderer elements at C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements.

| Actual route | Invalidate before | Refresh only after |
|---|---|---|
| ShellHost/shell-actions/input-funnel/program-dispatch/TS:77 | plugin.handleAction; bind exact captured program/document owner | awaitOperationSettle at100; verify same owner/epoch,then read authenticated LocalInteraction identity |
| ShellHost/host-effects/operation-subscriptions/TS:51,78 | terminal completion callback before applying history/effects if earlier command invalidation not already outstanding | actual terminal publication + effects,then authenticated identity; cover both focused and spawned instances |
| ShellHost/identity-document-ports/hub-port-and-document-backbone/TS:147 receiveDocumentBackbone | valid mutations/retract message accepted for current exact open entry,including queued-before-port messages | entry.persistence.ingested(entry.port.receive(...)) resolves and exact entry still current; binding drains queued messages under same rule |
| ShellHost/backbone-worker/document-event-messages/TS:85 documentArchiveReplaced | before asynchronous readBack.archive / mergeAppDocumentArchive | actual readback/merge settles and held() still exact entry |
| ShellHost/shell-uri-documents/close-document/TS:25 onward | alongside entry.replacements.invalidate before port retirement | never refresh retired owner; localDocumentOwnerRegistry.retire/openDocumentSessions deletion close lifetime |

Current program-dispatch observer call is before awaitOperationSettle. It cannot alone certify the final source revision for a long retained operation. Do not move cache back to ready on that early response. Completion subscriptions already capture exact focused/spawned program ownership; use their publication path instead of a global UI refresh/hash heuristic. Extension/service/direct-browser-actor command paths also mutate through separate ingress; prefer a shared actor/document operation boundary or explicitly cover those callers (host-effects/dispatch-media-and-extension-effects and spawned-session/request-service-operation) before claiming complete coverage.

Remote backbone currently validates exact open entry then either buffers a bounded copied message or chains port.receive through persistence.ingested. That promise is the real ingestion boundary. A history patch or dirty-UI notification is downstream and cannot retire a previously cached hash before the mutation arrives. Keep cache unresolved while queued remote changes await a port. Archive readback uses held() entry identity; reuse that ownership check across every await.

PluginSourceEvent is defined in framework kernel/plugin-source/TS as snapshot plugins or built plugin with rebuiltAt. ShellHost registers-and-context-menu:376 routes it to install/reload. It is module availability/hot-reload,not document revision publication. Invalidate affected instance lifetimes on reload/uninstall independently,but do not confuse this stream with remote/local document changes.

Minimal lease policy: invalidation increments owner epoch and clears ready revision before dispatch; completion requests identity tagged with that epoch; install only if exact plugin handle,instance,document entry/lifetime,epoch and source lease remain current. Multiple pending mutation paths need a pending counter/token set or latest epoch readiness rule so one finishing request cannot reopen readiness while another mutation is outstanding. Close must retire tokens permanently; failed/cancelled mutation may restore readiness only by fresh authenticated read after settlement,not old cached hash reuse. Selection/hover-only events may be optimized later using proven effect semantics; conservatively invalidating is safer than an incomplete action allowlist.

No new hooks implemented here. The map is source evidence and implementation guidance; complete runtime/native event coverage remains an integration gate. Required tests include overlapping local/remote operations,queued remote beforebind,archive replacement during identity read,same-instance close/reopen,focused/spawned completion,extension mutation,failed/cancelled action and same-content undo.
