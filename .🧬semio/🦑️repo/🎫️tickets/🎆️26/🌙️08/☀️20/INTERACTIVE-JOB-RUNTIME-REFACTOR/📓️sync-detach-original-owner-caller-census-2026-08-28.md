# Sync Detach Original-Owner Caller Census

## Scope And Decision

Read-only caller/ownership handoff to Dag, aligned with the complete [post-R11 original-parent proposal](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️runtime-opening-original-parent-funding-proposal-r1-2026-08-28.md). No competing API, production/test edit, source oracle, Cargo, native test, or new source hold. Only this report was created. Current Plugin codec source-oracle staging remains ticket-only and unexecuted.

The proposal keeps one original resident root, the original app/Store field association, one typed backbone entry in the same FIFO, and the same inline Release. Its private bound-child release, charged unlink metadata and pointerless ClearedAwaitingBinding residue are proposed, not current APIs. Resident R11's actual25/25 is not proof of this parent integration or4096 fit. The separate SyncSession parent and channel retirement remain unidentified/unimplemented; this census does not grant them RuntimeAppCell authority.

The complete [Sync84 source review](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️coordinator-store-sync84-source-review-2026-08-28.md) remains source-only. Its14 trait qualifiers/65 await removals/one test import did not repair production detach. No repeated OS compilation was attempted.

## Exact SyncSession Roots

[SyncSession](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:866) owns its Store by value, optional command sender, optional Tokio broadcast event receiver, and status. It has no retained request, prepared Store receiver, actor-runner ticket or detach progress field.

[attach](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:888) consumes ArtifactChannels plus the event receiver. The channel backbone moves into the Store's async attach; only after success are cmd_tx/events installed. ArtifactChannels additionally owns a native ArtifactActorRunnerTicket ([definition](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:997)); this field is not retained by SyncSession. Its normal leftover destruction returns the ticket through return_once, not a session-owned completion witness. The actor's existing host/runner owners remain separate.

[detach](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:896) currently does exactly:

1. If cmd_tx exists, call send(Detach) and discard the Result.
2. Await Store::detach_backbone, whose actual current return is synchronous Result<Option<Backbones>,VcsError>.
3. Clear cmd_tx and events.

The await mismatch is the known compiler blocker. Removing await alone would discard both an exact returned Backbones owner and a refusal; changing async void alone cannot establish retained funding. Before any future request publication, the original Store receiver and original channel/request ownership must already be installed under the selected parent plan. No current field provides that joined transaction.

Repository Rust lexical census (rg, excluding ticket/build/node_modules paths) found exactly two authored SyncSession::new calls, [3900](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3900) and [3910](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3910), both cfg tests for receive/materialization. Neither calls attach or detach. No other SyncSession constructor/type use establishing a live RuntimeAppCell-owned session was found. Hub SyncSessionRecord and Neo4j labels are unrelated types. This is a source census, not macro expansion or runtime reachability proof.

## Mailbox Request Custody And Publication Boundary

[ArtifactMailboxSender::send](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:223) accepts the whole ArtifactActorMsg. Full, Bytes, Closed and Stale each return that exact message in ArtifactMailboxSendError; [into_message](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:170) recovers it without cloning. Detach is a unit variant, but it still represents exactly one requested control action and consumes one logical slot. Its current semantic byte count is1 (counter initialized at397, no payload addition at473), not the physical enum/slot/Arc/mutex Layout.

The exact sender owns an Arc to ArtifactMailboxAuthority plus mailbox generation. Each new pair starts mailbox generation1 ([366](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:366)); this is not the host's checked document generation or the runner's generation. Equal numeric generations, URI/document ID, Plugin instance ID, or NativeCloseKey cannot identify the same mailbox authority.

The current implementation has no reservation or in-place send-from-source operation. Under a blocking, poison-recovering mutex it validates generation/closed/64-slot/1MiB semantic-byte capacity, then installs the message and updates queue metadata. After releasing the mutex it invokes the extracted Waker and wake callback, then returns Ok. A callback panic therefore occurs after publication: the message may already be in the queue even though the caller never receives Ok. A retry inferred solely from an absent return could duplicate Detach. This is a direct source finding, not an executed panic test.

Required integration with Dag's selected plan is the original source-owned request and exact mailbox reservation retained through precommit refusal, followed by a recoverable committed state before wake-tail work. This restates the reviewed shared-commit obligation; it does not introduce another mailbox, reserve API, callback authority or root. Existing send callers must respect any eventual reservation in the same64-slot authority. The current ignored send Result cannot be carried forward as a compatibility path.

Actor processing is not an immediate receiver-retirement receipt. [drive_one](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1413) handles Detach by setting closing and closing command ingress, then later drains mailbox owners and one outbound backbone message per turn before Terminal. The native runner preserves a pending ActorTurnOwner and eventually moves it into terminal_turn ([2288](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2288)). request_close, terminal state and external-ticket return are separate operations; current close_one_terminal_owner still drops whole terminal owners ([2378](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2378)). No bounded payload/scheduler-tail proof is inferred.

## Exact Store Receiver And Channel Descendants

[Store::detach_backbone](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:15687) first clears envelope.backbone, calls bump, then returns backbone.take(). On saturation the descriptor has already changed; bump uses unchecked generation increment and cursor/revision work ([16348](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:16348)). The original descriptor owns its URI String. Both descriptor and original Backbones must reach the same prepared typed shell; preserving only the channel pointer would lose a separate owner.

[replace_backbone_retained](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:14177) checks logical capacity, moves the old backbone, then allocates its retirement Box. Existing [FIFO/reservation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:1489) capacity1024/eight reservations and checked (slot,generation,remaining) identities are not shell or changed-entry physical funding. Its current Complete branch pops/drops the erased Box before any same-root bound-release receipt. Dag's selected single ResidentRecord shell must replace that backbone-specific Box, and the original FIFO entry must remain through record Clear, admission release/Clear and the binding's final granted pop. No second queue or independent permit is needed or proposed here.

[ChannelBackbone](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:16826) owns URI plus optional inbound/outbound Arc<Mutex<VecDeque<BackboneMessage>>>. Its actor-side ChannelBackboneRemote holds aliases to those same queues and another URI. Thus Store-side retirement depends on the actual actor-side alias lifetime, not merely successful Detach enqueue. [ArtifactStoreBackboneRetirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:16897) currently retains backbone/queue/message/bytes and refuses unique-queue extraction while a peer alias exists. Its current logical truncation and zero-byte shell/backing drops are not full physical accounting proof. The eventual private bound-child release cannot replace this real descendant close.

## Event Receiver Cannot Be Cleared As Completion

ArtifactEvent includes owned mutation vectors, pack/spr vectors, presence vectors and preview payloads ([definition](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:506)). SyncSession's events=None destroys its exact broadcast Receiver.

Pinned local Tokio1.52.3 [Receiver::Drop](/Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/sync/broadcast.rs:1548) locks the tail and walks unread slots. RecvGuard::Drop at1713 assigns slot.val=None for the last receiver; clone_value at1705 clones T. Consequently a drain of try_recv is not an allocation-free typed retirement substitute, and keeping a sender does not prevent the last-reader payload path.

The exact receiver must remain structurally retained until its actual reviewed event ownership protocol can retire it. No typed event-close receiver or parent association was identified here. Clearing the Option, returning a raw receiver, dropping a catch result, or claiming an actor close flag does not establish that boundary.

## Authored Forwarders Requiring The Same Outcome

These are current source boundaries to coordinate, not authorized edits or a claim all are executable today.

| Boundary | Current behavior and required ownership join |
| --- | --- |
| [Store::detach_backbone](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:15687) | Raw Result<Option<Backbones>> after descriptor/bump mutation; original Store field/FIFO preadmission precedes any take. |
| [SpaceHost::detach_backbone](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:18214) | Async wrapper forwards the exact raw Result from meta. Its meta Store needs its own original association; document/config/draft/interaction selectors cannot be silently extended to Space. |
| [SyncSession::attach/detach](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:888) | Consumed channel/event inputs, request-before-Store, void detach. Retain actual source receiver/request/channels and propagate progress/fault through the selected original parent. No live session parent established. |
| [PluginApp trait](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11676), [VcsArtifactApp impl](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:24445) | Trait detach is async void; impl discards Store return then clears cache. Must not claim success or invalidate/cache-drop on a refused precommit detach. Actual document Store belongs to Dag's selected wrapper parent. |
| [plugin_detach_backbone](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:29863) | Numeric instance lookup, resolve_ready(void), unconditional Ok. Must use the original retained instance association and preserve pending/refused/committed outcome; no whole-future/local-owner fallback. |
| [OsWorkflowStore wrapper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:785) | Void sync forwarding discards inner result. Its native attach also still names Box<dyn Backbone>; this is existing source inconsistency, not a new compiler result or authorization to widen edits. |
| [ShellSyncChannel](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:1032), [detach_sync_backbone_internal](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:2977) | Separate shell-owned channel, not SyncSession. Takes the whole channel before sends, ignores send/Plugin results, calls host close, then clears status/presence. Its original channel/event/strings and instance association must survive refusal. |
| [ProgramBridge](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:529), [wasm_program_exchange](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:285) | Current native branch returns an explicit unavailable detach error, not a call to plugin_detach_backbone. Do not present the Shell→Plugin path as end-to-end connected. |
| [ArtifactHost::send/close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1117) | send async void discards exact message refusal. Native close uses the runner/quarantine path; wasm close sends Detach and ignores refusal. Both are distinct from Store detach. |
| [BackboneWorkerHost Close/Send](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️component.rs:64) | Browser worker has its own map/sender/event task; current host.send is called without await and direct Send discards mailbox errors. Uncompiled browser source finding, not a tested native regression. |

Shell's three direct detach helper calls are3442 (manual attach replacement),3488 (open_document replacement),3553 (explicit detach action). Each currently proceeds after the void helper. Shell attach/open at3451/3494 reaches the explicit ProgramBridge error before installing sync_channel; therefore no working current shell attachment is inferred. Authored Store/Space/Plugin detach fixtures at Store23795/24515 and Plugin35227 also need coherent outcome/cleanup migration when their production surface changes; they are not additional selected or executed tests here.

The original two Store refusal laws remain the unchanged desired gates: backbone_detach_refusal_preserves_descriptor_and_payload_at_full_destination and backbone_detach_refusal_preserves_descriptor_and_payload_at_generation_overflow. OS6's previous compiler RED executed neither; Sync84 source acceptance does not change that.

## Handoff To Dag

The same-parent Store/FIFO plan can name the concrete Store source and typed shell above. It must not issue a RuntimeAppCell field receiver for an unrelated SyncSession or ShellSyncChannel. For those separate aggregates the unresolved inputs are the actual original registered parent, original mailbox reservation/request association, event-receiver retirement owner, and exact actor completion/alias handback. The host/runtime generation, mailbox generation and Plugin instance identity are separate scopes.

No source change is requested from Dag during this review. His original-parent proposal remains the single design authority; this report supplies exact current caller obligations and the unsent/accepted request distinction. Native remains idle pending a separately explicit lease, with no OS retry while detach is knowingly unjoined.

## Read-Only Source Observations

No immutable repository/source hold is claimed. Paths above were read with sed/rg; SHA256 observations were then obtained with shasum. Earlier broad reads encountered one nonexistent guessed extensions directory and output truncation; the final caller census reran from the actual repository and the complete selected171-line post-R11 report was read separately. No absent path was treated as evidence loss.

| Source | SHA256 |
| --- | --- |
| Dag post-R11 proposal | 930a3cf899e01a4e776e0d2adfe44fb4b7219c37da35fea07f8784c6526b9004 |
| Coordinator Sync84 review | af8794fca5347e14c06ac35f2cf9f921df2ff184e1ce5b81b19ad70b1aef3f45 |
| Store Sync | 62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6 |
| Store main | 7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4 |
| Plugin main | 2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca |
| OS host | 53e1044cceec42907c6e741230279aa439b806d6bb232c4ef592580c3cd90211 |
| Renderer Shell | 527aae4b56941a85798cca575f5556f7f4799f5fdb62d6c5bbbd73f17d548ae7 |
| ProgramBridge | 3c472aa946e552b9c19d7fdfec697475e45b9b3a5aaca7695b52066abd0a7edf |
| Browser backbone worker | fe96e3431622fea00dd7c68378a24d5fee1c98ccee588386af9d511a08f214a9 |
| Pinned Tokio broadcast.rs | 75e937843099f1b5b2f842b2ee308d93a982869f959c4e62eef2401094d6c579 |

