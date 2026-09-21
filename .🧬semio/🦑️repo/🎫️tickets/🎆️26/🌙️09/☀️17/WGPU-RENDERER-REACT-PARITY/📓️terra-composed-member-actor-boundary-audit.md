# Composed member actor boundary audit

## Scope and evidence

This is a source-only audit. No command or production edit was made by this audit.

The immediate native compile failure came from the newly added `BackboneMessage::Member` not being covered by the document-actor decoder and the two actor relay matches. The current source now handles it deliberately: root-only decoding refuses it at [`sync/🦀️.rs:589-594`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:589), and both native and wasm `relay_one_backbone` paths return `VcsError::Backbone("a composed member requires its exact member transport lane")` ([`sync/🦀️.rs:1964-1977`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1964), [`sync/🦀️.rs:3534-3545`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3534)).

## Confirmed current ownership boundary

`BackboneMessage::Member { slot, child_id, envelopes }` is an exact child lane, not a root mutation ([`store/🦀️.rs:18992-19002`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18992)). The parent `ArtifactStore` keeps received member messages opaque in `member_inbox`; only its composing `VcsArtifactApp` can resolve `(slot, child_id)` to a live member and ingest the payload ([`store/🦀️.rs:18569-18574`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18569), [`plugin/🦀️.rs:24463-24482`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24463)). Local forwarding is complete up to that parent backbone: attachment announces member histories, local child edits send tail batches, and a parent tick folds inbound child traffic ([`plugin/🦀️.rs:24428-24462`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24428), [`plugin/🦀️.rs:25683-25687`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25683), [`plugin/🦀️.rs:31524-31543`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31524)).

The document actor cannot perform that fold. Its `DocumentBackbone` admission decodes only root mutation envelopes, checks that every envelope names `self.document_id`, retains entries by root mutation id, persists them in the root document, and sends root `ClientFrame::Commands` ([`sync/🦀️.rs:1911-1933`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1911), [`sync/🦀️.rs:2698-2725`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2698)). It owns neither `ChildMemberRegistry` nor the child dialect/owner reference. Flattening a `Member.envelopes` byte batch into that path would falsely make the child mutation root persistence, root causal identity, and root Hub disposition.

The strict refusal therefore preserves authority and avoids silent loss. It is not composed replication support: Flow25/26's two-replica child convergence remains blocked at this actor/hub boundary.

## Missing external transport, precisely

There is no wire form for a routed member lane. The public Hub protocol carries only root `ClientFrame::Commands { batch_id, envelopes }` and `ServerFrame::Commands { envelopes, origin, frontier }` ([`wire/🦀️.rs:45-56`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:45), [`wire/🦀️.rs:517-538`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:517)). Those frames have neither `slot` nor `child_id`; they cannot recreate `BackboneMessage::Member` at a peer. The existing inbound delivery path similarly fabricates only `BackboneMessage::Mutations` before emitting its raw document-backbone effect ([`sync/🦀️.rs:2754-2772`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2754)).

At the UI boundary, Shell currently admits only root `Mutations` effects and sends them to `ArtifactActorMsg::DocumentBackbone`; it ignores acknowledgements and refuses genesis ([`Shell/🦀️.rs:1247-1261`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1247)). Inbound raw document backbone effects do already reach `plugin_receive_document_backbone`, which pushes them into the exact live binding and ticks the composed app ([`Shell/🦀️.rs:9333-9352`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9333), [`plugin/🦀️.rs:37544-37565`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37544)). That existing inbound seam can carry the complete `Member` message unchanged once the actor/hub becomes able to fan it out.

## Minimal complete production seam

Add a distinct composed-member transport branch; do not widen root `DocumentBackbone` decoding or root retention.

1. Introduce `ArtifactActorMsg::ComposedMemberBackbone { message }` and a corresponding raw actor event. Its admission decodes exactly one `BackboneMessage::Member`, checks the fixed hot-message byte limit, nonempty bounded `slot`/`child_id`, and nonempty canonically decodable child envelope batch. It does **not** apply `self.document_id` scope, call `persist_operations`, enter `DocumentBackboneRetentionV1`, or use `pending_batches`.
2. Extend the Hub wire with a dedicated client/server member-frame pair. The client carries the outer parent document session plus exact `slot`, `child_id`, and opaque canonical child envelopes; the server fan-outs the same lane with an origin and its own member-lane disposition/frontier. A root `Commands` frame cannot substitute because it has no route. The actor owns only bounded raw message retention and server acknowledgement for this frame; it never interprets the child semantics.
3. On remote member delivery, the actor reconstructs the exact `BackboneMessage::Member` and pushes it to `ChannelBackboneRemote`; it emits the existing raw inbound backbone event. The plugin binding then calls `tick_backbone`; `VcsArtifactApp::fold_member_inbound` is the sole route validator and child ingester.
4. Update all Shell egress sites through `route_document_backbone_effects` to route `Member` to the new actor command. The root `Mutations` arm keeps its current root persistence and Hub acknowledgement behavior. Genesis remains cold-only; Ack remains local-only.
5. Start member history transfer only after the recursive document archive and its child entries are admitted at the peer. A received member lane whose child is absent remains a composing-app fault, as current `fold_member_inbound` requires; it must never be buffered indefinitely in the document actor or redirected to another child.

This is a framework transport change, shared by every composed artifact. The flow app and the store's local `Member` implementation need no Flow-specific replica path.

## Fail-first behavioral law

Extend the existing native `ArtifactHost`/mock-Hub document-backbone law around [`sync tests:1701-1713`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs:1701) with two real composed parents, each owning the same `content` child.

* Dispatch a child edit on replica A and drive the actual shell effect → actor → hub → actor → plugin binding → `tick_backbone` path.
* Assert B's **child** changes and both composed roots converge; assert A/B parent root envelopes, root persistence revisions, and root `pending_batches` contain no child mutation.
* Send an otherwise valid member payload under a wrong slot or removed child ID. The actor forwards it without semantic mutation, the composing app refuses the live-route mismatch, and neither root nor a successor child changes.
* Deliver the same member envelope twice and require child-store idempotence through its own ingest route.
* Close the parent binding before a member fanout arrives. The stale raw frame must be refused at the exact binding and must not reach a same-URI successor.

The current strict-error arms remain a valid pre-implementation law: any unsupported external composed egress fails explicitly, never persists or emits child bytes as a root `Commands` batch.

## Confidence

High for the current boundary and missing routing evidence. The specific Hub member-frame shape requires the Hub protocol owner to choose its acknowledgement representation, but an exact routed frame is required; no existing root command frame can preserve child ownership.
