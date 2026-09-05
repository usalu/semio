# Flow Diff Strict-Owner Repair Blueprint

## Verdict

**RED — `FlowDiff` is not a valid carrier for a strict `FlowWorkingScene` owner.** This is a production design defect, not merely the temporary-child test failure being repaired in the current Flow gate. A generic erased local-owner disposer or an unbounded deferred queue would hide the defect rather than establish a bounded owner.

The current shared native session `33185` is a prerequisite for unrelated current Flow frontier work. This report is source-only: no Cargo/Nx command was started and no outcome from that session is attributed here.

## Current causal path

`FlowContentChild` is an `ArtifactChild<SemioFlowSnapshot>` whose local owner is an erased `Arc<dyn Any>` ([Flow alias](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:43), [generic storage](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2687)). Flow creates a new handle **with** a strict `Arc<FlowWorkingScene>` at [flow source](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:172), [flow source](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:189), and the normal leaf helper puts that handle in a durable `FlowDiff` at [diff helper](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:144).

The generic `ArtifactChild` retirement deliberately releases only `child_id` and `target`, not `local_owner` ([retirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:302)). That is correct for a generic erased cache, but it means a Flow strict scene can only be closed after Flow has first recovered the exact typed owner and handed it to `SceneRetirementFactory` ([factory](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/🦀️.rs:77)).

`FlowDiff` derives `Clone`, `ToValue`, and `FromValue` while containing both `content: Option<FlowContentChild>` and `artifact: Option<Box<FlowArtifact>>`; either branch can contain that erased strict owner ([schema](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:15)). Its current structural `absorb` overwrites a whole diff at [line 100](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:100) or overwrites `content` at [line 113](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:113). Both direct drops can reach the final `Arc<FlowWorkingScene>` without its typed cursor.

The exposed direct `FlowDiff::absorb` caller in this crate is the law at [lines 173-174](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:173), but that does not reduce the production defect: replication is allowed to invoke this trait on any persisted diff. `MutationDiff` itself requires `Clone`, a synchronous total structural `absorb`, and no progress, cancellation, ownership-return, or close channel ([contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:98), [contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:100)).

There are two additional production replacements outside that direct trait call:

- `FlowBuilderConstruction::mutate` and `ArtifactBuilder::absorb` assign a newly applied snapshot over the old one ([builder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:229), [builder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:237)).
- The ordinary retained preparation route constructs a post parent with an owner-bearing child at [editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:650); the preparation cursor can close an abandoned post through `SnapshotRetirementFactory` ([close path](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📬️preparation/🦀️.rs:224)), but a successful parent publication still leaves the durable cloneable snapshot carrying the owner.

`cache_flow_content` is another direct replacement of erased ownership ([Flow source](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:210)). It cannot remain a normal document/snapshot API under strict retirement.

## Option comparison

| Option | Verdict | Reason |
| --- | --- | --- |
| `FlowDiff` carries a domain wrapper with one fixed handback | Reject | `MutationDiff` requires `Clone` and total synchronous `absorb`. A second coalesced content diff arrives before the one handback is cursor-closed; it must overwrite, reject, or add another slot. The first violates strict ownership, the second violates totality, and a fixed number merely postpones the failure. Whole-artifact overwrite has the same problem. |
| Make generic `MutationDiff::absorb` a retained operation | Not this P0 | It is the architecturally valid general answer for resource-bearing diffs, but changes a trait with 51+ implementors and every generic coalescer. It must introduce cancellation, progress, retained old/new ownership, and an atomic publication contract globally. That is neither minimal nor Flow-contained. |
| Durable Flow parent/diff carries only identity references; the typed scene remains in a domain operation until child publication | **Accept** | It removes strict owners from all cloneable/serializable parent values. Flow owns one bounded operation and uses the existing typed child/member/store authority, rather than an erased disposer or a side queue. |

## Smallest honest Flow packet

1. **Remove local `FlowWorkingScene` ownership from every durable `FlowSnapshot`, `FlowArtifact`, and `FlowDiff`.** A parent `content` field is an exact stable `ArtifactChild` coordinate only. Delete/rework both the sparse `content` replacement and whole `artifact` replacement from `FlowDiff`; otherwise either field reintroduces the forbidden erased owner. `diff_set_snapshot`, `diff_replace_content`, `cache_flow_content`, and the builder assignments are not safe exceptions.

2. **Keep normal Flow content changes out of `MutationDiff<FlowSnapshot>`.** A widget/synapse mutation must become a typed `SemioFlowMutation` against the already-admitted `SemioMembers::Flow` child. Parent mutations may still alter genuine parent fields such as camera; they must not mint/repoint content as an ordinary leaf side effect. This aligns with the current parent/child separation already documented in [the hydration audit](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-flow-parent-document-pack-hydration-audit.md:62).

3. **Introduce one domain-owned `FlowContentPublicationOperation`, not a queue.** It owns at most one source `ChildContentView`/member lease, one typed child-mutation preparation, one post-child checkpoint receipt, and one parent/graph commit ticket. It must make bounded progress, inspect cancellation before and after every advancing stage, and retain the exact owner after a denial. The sequence is:

   `borrow admitted child → prepare typed Semio Flow mutation → child durable commit/checkpoint → revalidate document generation/authorisation → one parent/member/graph commit boundary → publish`.

   Before the final boundary, cancellation or any failure drives the typed child preparation and any uncommitted candidate through their domain retirement cursors. After that boundary, the member store owns the new child state. No cloneable parent value carries the live scene and no operation is silently abandoned.

4. **Do not label the current composition commit atomic.** `commit_child_member` still awaits owner and graph work before map/root publication; the existing composition transaction repair must first provide its promised all-or-nothing parent/member/graph boundary. The Flow operation depends on that packet. A serial lock alone does not make a cross-map publication atomic.

5. **Render and command from the actual typed child view.** `ChildContentView::typed_read` is the existing read seam, while `working_from_flow_content_snapshot` is the Flow conversion seam ([prior source map](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-flow-parent-document-pack-hydration-audit.md:37)). It may produce a bounded temporary for a render/command operation, but that operation, not `ArtifactChild::local_owner`, must retain and close it. Missing child materialization is an error, never `FlowWorkingScene::default()`.

This packet deliberately does not add generic local-owner retirement, permit content-derived member re-identification, or claim public restored-child loading. It depends on the separately staged request-owned `MemberFactory::Open`/selected-factory-to-typed-decoder bridge; the provider's selected dictionary/factory component evidence does not yet meet that public transport boundary.

## Required neutral and native proof

Extend the existing Flow fixture router at [fixture script](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/📜️script.ts), then register a separate exact law through [the Flow package script](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:27). Do not inflate the present ten-law `child-identity-check`; it has no real `FlowDiff`/child-publication assertion.

The language-neutral fixture should use plain parent/child coordinates and state transitions, not Rust owners. Required rows:

- success: stable parent `content` coordinate, one typed child mutation/checkpoint, then one graph/member/parent publication;
- cancellation at child preparation, checkpoint, and immediately before commit; each leaves the old child/parent visible and every candidate uncommitted;
- child checkpoint failure, graph failure, and root publication failure; retry publishes exactly one child state and one parent relation;
- two same-actor requests before the first terminal boundary; the second remains unstarted or is rejected explicitly, never reaches generic `FlowDiff::absorb`;
- stale generation/owner/scope, mismatched slot/dialect/child id, and a viewer request attempting creation; all deny before publication;
- grants of `1`, `7`, and `4096` bytes, with monotonic progress and terminal close assertions for every abandoned typed owner.

The native law must create a real `Plugin<FlowApps>` editor, use the actual `SemioMembers::Flow` member and `ChildContentView`, mutate a nontrivial Flow child, and prove: parent `content` identity is unchanged; the typed child checkpoint changes; render/command reads the changed child; and cancellation/failure leaves no parent, graph, map, or strict owner half-published. A second law must prove a viewer only opens that relation and cannot create it.

After the source oracle, add the new exact FQN to `child-identity-check` (or a new Flow-targeted gate if it is too broad), modify the launch seed, then regenerate launch metadata. The current `33185` result must be allowed to finish before the shared Cargo target is reused.

## Acceptance boundary

The present test-only `retire_child_local_owner` repair proves only that a specifically extracted temporary can be retired. It does not make generic `ArtifactChild` direct-drop safe, does not validate `FlowDiff::absorb`, and does not establish a durable Flow content mutation path. Flow remains RED for real content edits until this packet and its dependent composition/public-member-open packets have native evidence.
