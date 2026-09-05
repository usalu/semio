# Flow public parent and SemioMembers reload audit

Status: source review only, 2026-09-04. No build was started or credited.

## Sixth viewer lifecycle law

The current `flow_viewer_member_factory_and_full_store_close_match_neutral_contract` is source-sound. It builds a real `Plugin<FlowApps>` with `viewer_with_members::<FlowViewer, SemioMembers>`, creates the registered viewer id, verifies the dynamically dispatched `FlowApps::FlowViewer` variant, checks the document capability is app-scoped read-only, and drives the actual `PluginApp::close_step` to `Complete` before requiring `close_terminal_is_empty`.

The earlier missing `package_id` is superseded: the fixture plugin now calls `.package_id("semio:flow-viewer-lifecycle")` before `try_build`. This matters because strict `PluginBuilder::try_build` rejects a builder without package identity. The real Flow package independently registers the same `FlowViewer`/`SemioMembers` route.

Source anchors:

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:124-155`
- `✏️s/🔌️plugins/🌊️flow/🦀️.rs:10-35`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23199-23707`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:27-41`

The registered `child-identity-check` now exact-selects this sixth law. It has not run in this audit, so this is not a runtime pass.

## What a real composed reload must prove

The necessary public source seams exist, but no current law composes all of them:

1. A Flow parent persists a durable `content` child reference, not its local scene cache. `FlowSnapshot` declares `content: ArtifactChild<SemioFlowSnapshot>` and its pack codec serializes that handle; the `FlowWorkingScene` cache is deliberately omitted. A fresh decoded parent consequently has no local working scene until its child is restored.
2. The parent can emit and hydrate a complete document pack through public `PluginApp::document_pack` and `PluginApp::load_document_pack`.
3. A live Semio child can emit its full initial-pack-plus-SPR envelope through `SpaceMember::envelope_pack_bytes`; this is distinct from the current-snapshot-only `document_pack_bytes`.
4. The public parent read path exposes `PluginApp::load_child_pack` and `child_packs`, but currently implements it as a direct call to `VcsArtifactApp::open_child`.

Source anchors:

- Flow content bridge and canonical child target: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:127-224`.
- Flow parent schema and wire codec: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:18-123`.
- Public pack/child dispatch and current direct delegation: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24630-24716`.
- Full-envelope persistence contract: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17593-17601`.

## Exact current RED: public child reopen is not retained

`VcsArtifactApp::open_child` first validates the declared parent child projection and captures a parent generation. It creates map/graph admission, then awaits `M::open(expected, owner, envelope_pack)`. It does revalidate the parent generation/projection before the final publication sequence; failed preparation is sent to the bounded `ChildMemberRetirement` abort lane. Those are valuable existing no-publication protections.

Do not call the current sequence atomic: `commit_child_member` awaits `member.set_owner` and `composition.graph_mut` before it inserts the child map and replaces the content root. The calls currently return no fallible result, but their await points mean this is an ordered publication protocol, not one indivisible transaction. A retained-open integration must preserve the existing admission/abort semantics and make no stronger atomicity claim without a shared linearization boundary.

But `MemberFactory::open` only accepts a borrowed `&[u8]` and returns a finished member. The open call has no retained request cursor, cancellation authority, close operation, or request-generation fence. A parse/decode failure is therefore merely a returned error at this boundary; it is not evidence that a nontrivial, paged Flow child source is retained then deterministically retired. `load_child_pack` preserves that limitation by calling `open_child` directly.

Source anchors:

- Factory contract and macro-generated raw open: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17960-17965,18285-18297`.
- Existing parent admission, generation recheck, abort and ordered commit: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19913-20040`.

## Queued authority correction — genesis is never a viewer operation

Split the future public composition API and acceptance journey by authority:

- `CreateInitialComposition` is author-authorized. It alone may mint the durable document/child relation, call `create_semio_member`, and publish the initial exact `OwnerRef`.
- `OpenExistingComposition` is the read-only viewer path. It receives an already-created persisted parent/child relation and may validate, retain, hydrate, and render it, but it must never use a surface id to mint a durable parent or child.

Consequently, the source half of the proposed round trip must use an author-capable editor solely to establish the persisted relation. The fresh viewer half must reopen that relation through `OpenExistingComposition`; it is not a second genesis. A surface-only unbound factory is safe only while it remains non-persisting. This correction preserves the required read-only capability boundary and prevents a viewer mount from becoming an implicit document-creation authority.

This is the first material blocker for a truthful source -> pack -> source public Flow parent/`SemioMembers` acceptance. Do not use `open_member_store`, a directly constructed `ArtifactStore`, a test-only owner wrapper, or the existing 13-law fixture as a substitute. Those bypass the retained requester and public `load_child_pack` lifecycle being sought.

## Smallest truthful acceptance law after retained open lands

Place one Flow crate law beside the existing real editor testkit, but build through `crate::plugin()` and the public `PluginApp` enum instead of `VcsArtifactApp::new` or direct `register_child`.

1. Create a real editor through `plugin()?.create_app(&create_flow_app().id)` and match its actual `FlowApps::FlowEditor` variant. Use this factory-created app's exact default `FlowSnapshot::content` reference; `FlowFixture::default` is already nonempty. Do not create a custom Flow child whose id is absent from that parent's declared projection.
2. Convert that exact default snapshot's cached scene with synchronous `flow_content_snapshot_from_working`, create its real `SemioMembers::Flow` child via `create_semio_member`, and use the factory-created editor variant's real `register_child("content", ...)` only for source genesis. `register_child` assigns the exact source `OwnerRef` during its commit, which is required before persistence. Capture `document_pack` and the resulting one `child_packs` entry only after that owner assignment. A standalone `create_semio_member` envelope has `owner=None` and will be rejected by restored `open_child`.
3. Close that source factory app under finite grants and require terminal empty. This retires the moved source member through the application's exact owner chain; no independent child store or cache may remain caller-held.
4. Create a second real editor through the same factory, call `load_document_pack` with the source parent bytes, and then feed the captured owned `ChildPackEntry` to public retained `load_child_pack`. This intentionally proves that reloading the parent has no local `FlowWorkingScene` cache to rely on and must restore the durable child separately.
5. Read `child_packs` from the fresh parent and require exactly one sorted `content` entry whose id, dialect and envelope bytes equal the captured child identity/history. Close both public parent apps under finite per-step grants and require terminal emptiness. The fresh parent must be the observed owner; a local scene cache is not an oracle.

The existing direct testkit helper is useful only as a source-data constructor: it currently calls `create_semio_member` and `register_child` on a direct `VcsArtifactApp`, so it does not exercise the required reopen path.

Relevant source:

- Direct-registration helper to avoid treating as acceptance: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2101-2125`.
- Real closed Semio enum and create/open entrypoints: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1120-1152`.

## Owner and byte-budget requirements

`SemioMembers::Flow` is not a generic fallback: its macro arm binds exactly `s.stdio.semio`, `v1`, `flow` to `ArtifactStore<SemioFlowSnapshot, SemioFlowMutation>`. The same closed table installs exact snapshot, owned-value, mutation and store-disposer factories. Its disposer drains displaced owners, leases, edits, metadata, causal/structural owners and final envelope one bounded unit at a time, checks nested byte/item grants, and requires an exact terminal-empty witness.

The new acceptance must retain this path. Use a nontrivial payload and finite grants (at minimum `1`, `64`, and `4096` bytes); assert every pending result stays within the current grant and that `Blocked` only occurs while a deliberately retained read exists. The current viewer fixture's empty default document and a 4 KiB grant cannot prove this Flow child close behavior.

Do not state an invented universal maximum total byte count: the installed Semio store disposer is incremental and caller-grant governed. The law should measure its own captured source and restored-child terminal journey, assert per-step bounds, and cap the fixture input by the schema/decoder's declared limits.

Source anchors:

- Exact owner-table installation: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1101-1117`.
- Bound-preserving phase disposer and terminal witness: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:790-1098`.
- Bounded Flow parent handle serialization: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:171-193`.

## Required hostile rows

The language-neutral corpus for the retained public operation needs rows for: wrong child id; wrong full dialect; undeclared slot; a valid Semio envelope with wrong `OwnerRef`; malformed/truncated envelope; changed parent child reference or generation while decoding; cancellation before and after typed decode; and a nonterminal source-read close. Every denial must retain/close the request and any opened member, leave child map/graph/root publication unchanged, and permit a subsequent valid operation. A success row must prove full-envelope, not snapshot-only, preservation.

## Gate

Extend the existing Flow source fixture/oracle first, then add exactly one fully-qualified Rust law to `child-identity-check` in `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts`; the script's `runExactCargoLaws` selector must list/preflight exactly that FQN. Run it only after the retained `MemberFactory`/parent operation is integrated. The current six-law target is useful for the viewer and current owner contracts, not proof of the new public reload behavior.
