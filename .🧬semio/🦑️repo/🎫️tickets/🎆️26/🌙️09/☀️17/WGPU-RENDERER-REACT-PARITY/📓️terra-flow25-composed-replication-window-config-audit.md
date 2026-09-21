# Flow25 composed replication and window configuration audit

## Scope and evidence

This is a source-only audit. No production files were changed and no command was run.

The available Flow25 receipt is [`run.log`](🗑️generated/astra-runtime/flow-native25-full/run.log). Its two scoped failures are:

* `two_instances_converge_on_disjoint_edits`: replica A contains `inputNote` at `(40, 41)` while replica B contains the distinct `inputSlider` edit at `(300, 301)` ([receipt lines 155-174](🗑️generated/astra-runtime/flow-native25-full/run.log)).
* `flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows`: camera checks pass before the grid assertion fails ([receipt lines 176-198](🗑️generated/astra-runtime/flow-native25-full/run.log)). This receipt predates the current lane-count correction and the requested value diagnostic. It records only two `WindowConfig` lanes, so it is not evidence of the final post-correction grid values.

## 1. Disjoint composed-child edits cannot replicate

### Confirmed cause

`VcsArtifactApp` exposes one backbone attachment and one tick, but both delegate only to its parent `ArtifactStore`:

* [`plugin/🦀️.rs:31454-31475`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31454) attaches, ticks, and detaches `self.store` only.
* The Flow law attaches its two **parent** stores through `paired_registered_apps`, then `exchange_and_assert_convergence` calls only those parent ticks ([`plugin/🦀️.rs:7430-7444`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7430), [`plugin/🦀️.rs:7581-7597`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7581)).
* The Flow mutation itself is a real child-lane mutation. Local grouped publication rebuilds `child_content_root` and advances `child_content_generation` after the child member is changed ([`plugin/🦀️.rs:25515-25615`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25515)). The missing result is outbound/inbound child transport, not a Flow reducer failing to change a child.
* `ChildMemberRegistry` already gives the owner a bounded, exact lookup key: `OwnerRef.slot` plus `ArtifactRef.artifact_id`; every entry also retains the child's dialect ([`plugin/🦀️.rs:8445-8450`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8445), [`plugin/🦀️.rs:8522-8536`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8522)).
* `SpaceMember` deliberately has no attach, tick, or detach backbone operation ([`store/🦀️.rs:19592-19634`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19592)). It therefore cannot be fixed by adding a child loop around the existing parent API.

This is a framework composition defect that Flow exposes. It affects every app that persists semantic content in a child, not merely the two-instance fixture.

### Why the existing document binding cannot carry children

The browser `DocumentBackboneControlV1` is keyed only by `instanceId`, `bindingGeneration`, and `uri`; its exact-field check excludes a child route ([`binding/🟦️.ts:15-30`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts:15)). A raw `BackboneMessage` has only `Genesis`, `Mutations`, and `Ack` and carries no composition owner ([`store/🦀️.rs:18888-18905`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18888)). Sending a child message on the parent route would make the parent validate the child's genesis and event schema.

### Bounded production API and wire proposal

Make composition routing a framework-owned transport session below `VcsArtifactApp`, rather than a Flow-specific test helper.

1. Define a canonical `ComposedBackboneRouteV1` in the existing backbone schema family. It must carry the exact `OwnerRef` `{ parent, slot, childId }` and the child `ArtifactRef` including dialect. `OwnerRef` already provides the parent/slot/child identity ([`owner schema:8-12`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:8)); `ArtifactRef` is the durable id plus dialect ([`io schema:166-193`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:166)). A root route has the exact parent `ArtifactRef` and no child owner. Do not route by bare child ID.
2. Put `{ route, BackboneMessage }` in a composition envelope accepted by the external session. `BackboneMessage` remains the store-private payload. The session must reject a route whose parent, slot, child ID, or child dialect does not equal the live `ChildMemberEntry`; it must never fall through to the parent.
3. `VcsArtifactApp` owns one `ComposedBackboneSession`. On attach it creates a bounded `ChannelBackbone` pair for the parent and each currently live child, attaches the store-facing ends, and retains the actor-facing ends. `ChannelBackboneRemote` already supplies the required one-message `push` and `try_pop_front` primitives ([`store/🦀️.rs:19296-19354`](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19296)). The external backbone is owned solely by the session.
4. Add object-safe child transport forwarding to `SpaceMember`, then delegate it from the generated `space_members!` enum. The minimal member operations are attach a supplied virtual endpoint, tick reports, and detach/close the virtual endpoint. They are necessary because the exact child store is otherwise inaccessible through the heterogeneous member type. They must not expose Flow types.
5. On each session turn, move at most one local store message or one inbound composition envelope for one route, round-robin through the fixed `ChildMemberRegistry`. The session keeps its own bounded cursor. A route becomes active only after `commit_child_member` has inserted the member and published the associated root ([`plugin/🦀️.rs:24100-24111`](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24100)); it begins retirement before that member is removed or replaced. A same identity successor can retain its durable route, but a different child reference cannot inherit queued messages.
6. Inbound child reports must rebuild the affected immutable child root with the same `child_content_generation` and retirement discipline as local child publication. The existing `publish_child_content` path is the relevant owner boundary ([`plugin/🦀️.rs:24005-24014`](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24005)).

This keeps native and browser transport at the existing `Backbone` boundary and does not invent a Flow-only side channel. It requires a deliberate composition protocol update; attaching the same `Backbones` value to every child is impossible because it is move-owned and would also mix document genesis/event streams.

### Fail-first laws

Extend Flow's real two-replica test at [`unit/🦀️.rs:638-674`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:638):

* Attach a composed pair for replicas 71 and 72.
* Commit A's `inputNote` child edit and B's `inputSlider` child edit, then drive the actual composition session to quiescence.
* Assert both child content snapshots contain both edits and are equal. Assert parent document revisions did not acquire a child envelope.
* Deliver a valid child message under the parent route and under the wrong child dialect. Both must be refused without either parent or child revision changing.
* Remove a child while its route has queued input. The route must retire before the member, and the queued input must not be delivered to a different child added later.
* Replay one composed child envelope twice and prove idempotence through the actual store route.

## 2. Window grid settings remain unclassified until the new value receipt

### Confirmed source facts

* Every Flow direct window command starts from the exact captured window snapshot. `FlowDirectStoreWork` reads `context.window_config`, and the common emission clones it before changing only the requested field ([`flow editor:937-942`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:937), [`flow editor:724-753`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:724)). `setGridVisible` and `setGridFactor` are registered WindowConfig routes ([`flow editor:1388-1397`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1388)).
* The framework owns a distinct `ConfigStore` partition by `window_id` ([`window config:379-393`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:379)). It validates the operation's captured address against both window id and kind ([`window config:797-801`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:797)).
* A published WindowConfig page immediately refreshes the mounted operation authority ([`plugin/🦀️.rs:28216-28220`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28216)).
* Flow measures read the same per-window `ConfigView` and project `grid_visible`, `grid_snap_enabled`, and `grid_factor` directly ([`flow editor:2629-2637`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2629), [`grid measure:15-46`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🌐️grid/🦀️.rs:15)). `VcsArtifactApp::window_measures` independently captures configuration once for each window in the input roster ([`plugin/🦀️.rs:31573-31598`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31573)).

The old receipt proves a runtime failure, but cannot identify which of these boundaries is wrong after the lane fix. The code above does not establish a grid-specific cross-window write. Changing the Flow setters, treating grid state as global, or relaxing the test would be unsupported.

### Narrow next law and decision table

Keep the existing real runtime test and extend its diagnostic before any production edit. After each settled command, read the exact stored `FlowMainWindowConfig` for both `flow-main-left` and `flow-main-right`, then read their measures. Compare all three grid fields, including `grid_snap_enabled`:

| Stored partitions | Measures | Correct repair owner |
| --- | --- | --- |
| Wrong immediately after a settled grid command | Any | Retained WindowConfig publication/admission path. Preserve the whole-record snapshot mutation and repair the stale/omitted result path. |
| Correct | Wrong | `VcsArtifactApp::window_measures` capture/projection path or its supplied `ViewModel`; Flow's grid component is a direct projection and should remain unchanged. |
| Correct | Correct | The test's extraction/assertion is wrong; repair only that fixture law. |

The test currently asserts only visibility and factor ([`window ownership test:95-116`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:95)). The next permanent law should assert the stored and rendered values for left `(camera 12,-8,2; visible false; snap false; factor 10)` and right `(camera -21,5,.75; visible true; snap false; factor 20)`, after all four WindowConfig pages have settled. That is the smallest evidence needed to assign the repair without masking a production defect.

## Confidence

* **Composed replication gap: high.** The public attachment/tick implementation, fixture transport, child commit path, and absent child transport API all agree.
* **Grid root cause: not yet classified.** The failure is observed, but the only available receipt predates the corrected lane behavior and lacks stored-versus-projected values. The source confirms the relevant intended ownership boundaries, not which one violates them at runtime.

## Supersession: current composed transport and Flow26 receipt

The earlier composition analysis reflects a source snapshot before the framework's member-lane transport landed. It must not be used to claim that `BackboneMessage::Member` or child forwarding is absent.

The current source has an exact parent-backbone member lane: `BackboneMessage::Member { slot, child_id, envelopes }` ([`store/🦀️.rs:18977-19004`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18977)); the parent store sends it with `send_member_mutations` and queues inbound member traffic for its composing owner ([`store/🦀️.rs:17931-17952`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17931)). `VcsArtifactApp` announces member histories on attach, sends tail edits from `dispatch_emit_group`, and folds exact live member lanes after each parent `tick_backbone` ([`plugin/🦀️.rs:24428-24475`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24428), [`plugin/🦀️.rs:25683-25687`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25683), [`plugin/🦀️.rs:31524-31543`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31524)).

Flow26 still records two disjoint child replicas diverging, so the current implementation has not yet met the live convergence law. The remaining causal boundary should be assessed by a fresh Flow gate against the landed transport: specifically, whether the two-replica fixture drives enough actual ticks for outbound member tails and inbound fold, and whether either replica's live child route/refusal path rejects the delivered lane. No replacement schema or Flow-specific transport should be assigned from the superseded absence claim.

The Flow26 window configuration receipt now has four settled configuration pages plus one transient page, with camera and grid settings passing under that sequential schedule. That result does not repair the source-documented back-to-back same-window amend loss. The remaining same-byte reload failure is a stale allocative-observer assertion; see [`terra-flow26-same-byte-reload-transient-audit.md`](📓️terra-flow26-same-byte-reload-transient-audit.md).
