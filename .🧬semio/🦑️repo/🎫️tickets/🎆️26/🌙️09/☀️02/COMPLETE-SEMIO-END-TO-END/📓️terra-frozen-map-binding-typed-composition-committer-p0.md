# Frozen GIS Map Binding and Typed Composition Committer — P0

## Verdict

**RED: neither half of the requested first executable slice exists.** The hub can derive a private `InferenceIdentityV1` from a verified catalog, but it throws away the concrete selection and has no Map execution binding. Separately, the GIS `CreateRegion` proposal is typed and deterministic, but it is only a parent `regions` delta. It does not construct a `ChildEmit`, and it cannot truthfully be called an atomic parent/child commit.

This is a current-source audit on 2026-09-05. No build, native, browser, WGPU, or process test was run for this report. It intentionally excludes the active directory-event, presence, browser/Home bootstrap, Flow, and Stdio work.

## Decisive source evidence

| Concern | Current source | Proven fact | Decisive gap |
|---|---|---|---|
| Trusted catalog selection | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:271-326` | `VerifiedTrustedCatalog` retains a verified package/component/artifact/dialect/surface/grant selection and catalog generation; `resolve_document_open` requires exact descriptor owner/hash, kind/schema/pack hash and editor/viewer role. | It has no Map-inference binding object and no causal-frontier/base-pack assertion. |
| Startup lifetime | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:361-400, 5833-5870` | Startup loads `ConfiguredArtifactAuthority { catalog, authority }`. | `HubState` keeps the catalog only as `Option<Arc<dyn DocumentOpenCatalogAuthorityV1>>`; the concrete verified selection/factory is erased before any inference can use it. |
| Inference identity | `🌎️hub/💡️inference/📇️catalog/🦀️.rs:47-76`, `🧬️schema/🦀️.rs:49-110` | It checks descriptor/package/service facts, descriptor digest and current `ArtifactFrontier`; identity contains scope, user/session/generation, descriptor digest, catalog generation, a SHA-256 package hash, frontier, chain hash and input hash. | `package_hash` is only the component SHA-256. The identity omits package id/version, component BLAKE3, descriptor-byte SHA-256, exact selected artifact/pack hash, full parent dialect, surface and grant. `catalog.resolve(...)` checks codec presence but returns no retained binding. |
| Native executable | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:197-291` | `gis_map_inference_service` has fixed metadata and `infer_gis_map_controlled` supplies bounded checkpoint callbacks. | The process-global inference registry and wire entry accept caller-provided canonical payload. They are not an authority to select or open a document. |
| Typed proposal | `…/🧬️schema/💡️inferences/🦀️.rs:38-52` | `GisMapInference::bounds_proposal` makes exactly one `GisMapMutation::CreateRegion`, deterministically named `inference-<job-id>`, and refuses stale counts, duplicate ID, bad bounds and too many regions. | It has no approval, actor, commit, group receipt or document lock. |
| Map composition | `…/🧬️schema/📸️snapshot/🦀️.rs:25-68`, `…/🦀️.rs:77-105,166-174` | A Map snapshot contains `drawing`, optional `image`, and `value` `ArtifactChild`s. Drawing/value are re-derived from feature data. | Those two child handles are re-minted from a `DefaultHasher` content key; the target IDs are fixed strings. This is not a stable child-member identity or a persisted child-content publication contract. |
| Proposed mutation | `…/🧬️mutations/🌐create-region/{🦀️.rs,🔺️diff/🦀️.rs,↩️inverse/🦀️.rs` | `CreateRegion` has a typed parent diff and an exact inverse `DeleteRegion`. | Its diff changes only `GisMapDiff.regions`; it neither changes child handles nor supplies typed drawing/value child mutations. Repository search finds no GIS `ChildEmit`. |
| Existing group primitive | `🧰️framework/…/🏪️store/🦀️.rs:19361-19565`; `🔌️plugin/🦀️.rs:21166-21388,22592-22723` | `CompositionCoordinator::dispatch_group` previews all members, applies child edits then parent, stamps one group ID, and returns a `GroupReceipt`; the mounted typed-operation path checks cancellation/freshness and can call `dispatch_emit_group`. | `dispatch_group` is in-memory two-phase dispatch plus compensating undo, not a durable atomic visibility transaction. The GIS app has no owned child store/emit adapter to call it. |
| Visibility primitive | `🧰️framework/…/🌿️vcs/🦀️.rs:206-244`; `🏪️store/🦀️.rs:2164-2180,2401` | `ArtifactGroupVisibilityOwner` can commit/abort one prepared history/cursor visibility decision; prepared history/cursor reads are hidden until its decision. | It is not wired around the GIS parent, live child stores, durable append, or hub fanout. |
| Ledger boundary | `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:219-326` | `prepare_approval` canonical-decodes bounded command bytes and `reconcile_committed_approval` requires an exact committed WAL witness. | It carries opaque diff/inverse bytes. It has no typed Map receiver and its SQLite transaction cannot make the composition commit atomic. |

The existing GIS native-codec test at `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/🦀️.rs:31-109` proves a two-codec receipt and a local controlled inference/proposal/inverse trace. Its own final debug string explicitly says it has **no hub approval authority**.

## First correction: define the honest P0 boundary

The smallest executable P0 is **not** a broad MAP endpoint or renderer. It is a private server-owned operation that can produce one immutable, approval-ready typed group plan only when all facts below remain true:

1. a retained verified Map selection and executable service are the same trusted component;
2. the exact Map parent and every affected existing child have been opened through the selected retained member opener;
3. the proposal’s base `ArtifactFrontier`, canonical parent pack hash, parent dialect, and child coordinates still match under one document gate; and
4. a prepared visibility transaction can either expose *all* parent/child roots with one durable group receipt or abort/retire every owner.

The operation may stop at `PreparedForApproval`; it must not publish a generic command, call `ArtifactHandle::submit`, or send UI/socket data. That keeps it independent of active transport work and makes the first native acceptance non-vacuous.

### Required bounded types

Add these private hub/composition types rather than extending `InferenceIdentityV1` with loosely related strings:

* `VerifiedGisMapArtifactBindingV1`: constructed once from `Arc<VerifiedTrustedCatalog>`, the selected `VerifiedDocumentOpenSelectionV1`, and the exact GIS native provider receipt/service. It freezes: catalog generation; plugin/package/version; component SHA-256 and BLAKE3; descriptor-byte SHA-256; artifact kind/schema/pack hash; full `ArtifactDialect`; selected surface fields and grant; service schema/version, algorithm/policy; and the non-capturing `gis_map_inference_service` executable identity. Construction rejects duplicate matching selections, any receipt/service metadata mismatch, or a non-editor write grant. It owns neither a client payload nor a global-registry lookup.
* `GisMapProposalBaseV1`: a retained selected Map opener result plus immutable scope, descriptor digest, `ArtifactFrontier`, canonical pack SHA-256, parent revision/generation, and exact child `(slot, ArtifactRef, child root generation)` coordinates. Its fields are captured before inference and revalidated before preparing and committing.
* `GisMapCreateRegionGroupPlanV1`: private, non-clone, bounded owner for one `CreateRegion`, its typed inverse, **explicit typed child work**, frozen binding digest, base facts, server-stamped actor, job/proposal/mutation IDs, and all retirement owners. It has `Pending → Offered → Approved → Prepared → Committed|Aborted|Stale|Cancelled → Retiring → Empty` states. No client command bytes are accepted here.

`HubState` should retain `Option<Arc<VerifiedTrustedCatalog>>` in addition to the intentionally erased `openable_catalog`. Construct the Map binding immediately after `configured_artifact_authority`, before readiness is published. Do not reconstruct it later from a public `DocumentOpenPlanV1` or a client receipt.

## The parent/child issue must be solved inside this packet

`CreateRegion` changes the data from which `drawing` and `value` are declared derived. A parent-only Map mutation therefore cannot stand in for an atomic Map composition result. Conversely, issuing a `ChildEmit` from freshly re-hashed handles would violate the existing child member/open contract and leak direct-drop local owners.

The P0 must first select one domain-authoritative model and test it:

* **Chosen model:** stable admitted drawing/value child member identities; typed child checkpoint/content changes accompany a parent `CreateRegion`. Parent child references stay constant, and the grouped receipt moves parent and each touched child together. `image` is untouched and must not be synthesized.
* **Not allowed:** content-derived child IDs, whole-snapshot/raw JSON replacement, a generic JSON patch, a copied child pack inside the Map parent, or a parent-only commit advertised as a group.

This requires a small GIS-only `create_region_group_work` builder near `GisMapInference::bounds_proposal` that takes the opened typed Map parent and admitted typed drawing/value members. It derives the new drawing/value content from the proposed Map result using the existing domain converters in `…/🧬️schema/🦀️.rs:463+` and `gis_map_value_from_descriptor_json` in `…/🗺️gismap/🦀️.rs:147+`, then creates **typed** child mutations/inverses for the existing child stores. Its only output is `GisMapCreateRegionGroupPlanV1`.

There is currently no usable typed Semio drawing/value child mutation bridge in GIS. That is the immediate domain RED. Do not hide it behind a `ChildEmit` with unverified bytes. The plan can be mounted only after the public selected `MemberFactory::Open` program supplies those typed stores and their bounded retirement factories.

## Commit operation and ordering

After explicit approval, a private `GisMapProposalCommitOperation` owns the document-scope async gate and advances one bounded unit at a time:

1. `Acquire`: claim the per-scope gate; re-run `check_live_inference_author`; require Author exactly, not Admin by implication.
2. `Revalidate`: compare all frozen binding fields, descriptor digest, exact scope/document, current parent/child refs and generations, `ArtifactFrontier` (`head_ordinal`, head edit ID, last commit sequence, chain hash), canonical base pack hash, job/proposal hash, and ledger identity. Any mismatch enters `Stale` before an apply.
3. `Prepare`: preview parent `CreateRegion` and typed drawing/value child effects. Reserve each history/cursor/root under one `ArtifactGroupVisibilityOwner`; capture all displacement/abort owners before the first await. A cancellation/deadline checkpoint runs before and after every open, preview, reservation, and durable write.
4. `Durable`: append exactly one canonical parent-plus-child composition event/receipt to the event/WAL authority. The append returns a committed proof naming the same scope, generation, job, proposal hash, mutation ID, command hash, group ID and every member edit. This cannot use `db_artifact::diff_entries` or ordinary `submit_commands`, both of which deliberately do not interpret the GIS schema.
5. `Publish`: only after durable success, flip `ArtifactGroupVisibilityOwner::commit`, adopt every staged history/cursor/root, expose the group to document fanout, and revalidate the operation's retained root generation. If durable append fails or cancellation happens before it, abort the visibility owner and drain every prepared parent/child owner. If an error occurs after append, keep the plan in `CommittedUnreconciled` for witness-only recovery; do not rerun inference or compensate a durable commit.
6. `Reconcile/retire`: call the existing ledger `reconcile_committed_approval` only with the receipt-derived committed witness. Then emit one normal document event attributed to `user:<id>#session:<id>` plus job/proposal/group IDs. The job owner receives progress/terminal status; other users receive no proposal contents, only this committed document event. Drain the opener, group-plan and displaced-root owners within fixed grants.

The present `CompositionCoordinator::dispatch_group` cannot be substituted for steps 3–5: it applies child edits then parent edits and relies on best-effort compensating `undo` after late failure. It is useful for the typed preview/operation shape, but it is not durable all-or-nothing publication.

## Schema-first fixture and laws

Create one neutral fixture family, `gis-map-frozen-binding-create-region-group-v1`, owned by the hub/GIS boundary. It contains no live secrets and fixes:

* the complete frozen binding projection, base scope/frontier/pack hash and all Map child coordinates;
* canonical typed `CreateRegion`, inverse, drawing work, value work, parent-plus-child group ID and committed receipt digest;
* bounded limits: one Map parent, exactly drawing + value touched children, zero image changes, one proposal, one approval, one group, 64 KiB typed work total, and a finite progress/close trace;
* two identities in the same scope, one peer/cross-space viewer, and server-stamped actor/event records.

An independent Bun/AJV fixture script must frame/hash the binding and typed proposal fields and recompute the region ring without importing Rust codecs. It must reject at least: component SHA-256 or BLAKE3 substitution; descriptor hash; package/version; artifact/pack schema; every parent-dialect member; surface/grant; service version; scope/document; descriptor/frontier/chain/base-pack; parent or child coordinate/generation; proposal/inverse/mutation/group ID; duplicate region; a child omission; an image mutation; stale session/auth generation; wrong approver; cancel before/after prepare; and stale/replayed approval.

Register native laws in ticket-owned GIS/hub targets only after the prior public opener and atomic visibility primitive are green:

1. **Binding law:** a genuine verified GIS Map catalog selection and native receipt make one binding; every pinned neutral substitution fails before `infer` is called.
2. **Typed planning law:** real retained Map + real admitted drawing/value stores yield exactly one `CreateRegion`, inverse and two typed child effects; source pack is not client-provided, IDs are stable, and all owners close to terminal through 1/64/4096-byte grants.
3. **Atomic success law:** one committed durable group proof exposes parent and both child roots together, creates one causal group record and one actor-attributed document event.
4. **Atomic failure law:** inject late child, parent, append and commit failures plus cancellation at every stage. No root becomes visible, no event/fanout/ledger reconciliation occurs, and all prepared owners become terminal-empty. A post-append fault recovers from the exact witness once and never calls inference twice.
5. **Multi-user law:** A (Author) creates/approves; B (same scope) sees one ordinary group event but neither proposal nor job bytes. Viewer, Admin-without-Author capability, cross-space user, stale session/generation and duplicate approval create nothing.

React/WGPU have no current inference action or renderer state to adapt. Their P0 parity is deliberately a **shared read-only result DTO** after native law 3: both map surfaces must render the same canonical `offered|applied|stale|failed` status with EN/DE accessible labels, and neither surface may execute or mutate. Browser execution-target lease verification and WGPU map rendering remain separate REDs; passing this status parity is not a rendering claim.

## Minimal ownership split

1. **Hub selection/binding slice:** `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`, `🌎️hub/💡️inference/{🧬️schema,📇️catalog}/🦀️.rs`, and `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`. Owns binding schema, startup retention and selected service/receipt validation. It creates no routes.
2. **GIS group-work slice:** the Map inference/mutation/schema leaves above. Owns stable-child semantics and typed drawing/value work, including its independent fixture. It does not touch hub routing, UI, or generic framework APIs.
3. **Framework durable group slice:** `ArtifactGroupVisibilityOwner`, prepared history/cursor/root staging and the event/WAL bridge. Owns atomic publish/abort/recovery receipt. It must be green before a hub committer claims an atomic commit.
4. **Hub committer slice:** new private inference runtime/operation beside `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs`. Owns authorization fences, document gate, ledger witness reconciliation and fanout after receipt. It creates no public MAP route or renderer in this packet.

## Nonclaims

This does not claim a MAP HTTP/socket route, a real model/MCP provider, generic artifact execution, browser/WGPU installation or rendering, a worker/restart scheduler, full GIS child genesis, or a completed two-user UI journey. The existing local native codec and ledger tests remain component evidence only. Until the Map child model and durable group visibility are implemented and exercised, a parent-only `CreateRegion` must remain explicitly non-atomic.
