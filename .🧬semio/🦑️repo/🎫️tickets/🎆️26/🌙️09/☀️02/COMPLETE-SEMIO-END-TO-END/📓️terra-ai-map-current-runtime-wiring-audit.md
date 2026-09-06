# AI-Over-Map Current Runtime Wiring Audit

Date: 2026-09-06. Read-only source audit; no build, browser, model-provider, or database process was run. This supersedes the older UI-negative AI/Map notes: the Shell/worker preview lane has since been wired. It does **not** revisit the separately recorded durable-recovery admission issue.

## Current path

This GIS feature is deterministic native inference, not a request to an external LLM/model provider. `infer_gis_map_controlled` folds the server-materialized `GisMapSnapshot` into feature counts and bounds; it has no provider credential, request, or model-selection interface. Calling it an external-model journey would be inaccurate.

| Boundary | Current implementation | Evidence level | What is still absent |
| --- | --- | --- | --- |
| GIS editor action | `ProposeBoundsRegion` emits only `Effect::RequestInferenceProposal { GisMapBoundsRegion }`; it emits no mutation. | Native unit law only. | It requires an actually activated GIS editor instance to occur in a user session. |
| Browser Shell effect → UI/worker | `ShellHost` resolves exactly one hub-scoped open document, starts an epoch-owned inference port, posts `inference-open` then `inference-propose`, and mounts `InferencePortPanel`. | Source wiring plus React/codec tests; not a browser/HUB process journey. | The closed GIS actor must first be delivered and executed under the worker-containment boundary before a real click can emit this effect. |
| Browser broker → authenticated Hub routes | `backbone-worker` binds the port to a live private write lease, permits only the four scoped job calls, bounds JSON, and exposes a preview only from the Hub page. | Worker tests substitute `fetch`; they are not Hub HTTP tests. | No single browser process law proves actual authenticated lease acquisition, activated GIS effect, and all four requests together. |
| Hub job / proposal | The four routes authenticate the session, recheck Author membership and frozen base, execute the bounded native GIS service, and store an owner-private SQLite ledger proposal. | Hub native raw-HTTP tests build a trusted GIS test profile and SQLite ledger. | That fixture reaches only an `offered` proposal; it never applies a real Map edit. |
| Approval → durable Map group | Approval reconstructs a server-stamped parent command and records `approval-prepared`, but all constructed runtimes inject `UnavailableGisMapApprovalCommitterV1`. A child-bearing Map is refused before even consulting a committer. | Current native law deliberately expects HTTP 503 / `approval.commit-unavailable`. | A concrete per-document, fixed-three Store committer is absent. No approval can currently return `applied: true`. |
| Fixed-three Store / WAL | The Store has a public production-shaped `DurableOwnedThreeStoreMapAssemblyV1`; `ArtifactAuthority` exposes the existing-WAL journal sink; the Hub WAL verifier can validate a committed fixed-three decision. | Assembly tests use `DemoSnapshot` stores and `FakeJournalSink`; authority-sink tests are isolated. | No GIS/Hub document owner constructs the real parent/drawing/value Stores, factories, admissions, assembly, or host. |
| Native WGPU frontend | A nonblocking driver/runner translates the same effect to `DirectoryClient`. | Driver tests use fabricated receipts/pages. | It deliberately has no minted `DocumentExecutionTargetLeaseFieldsV1`; it fails `LeaseUnverified` before any request. |

## Exact seams

1. The actual user command is at [inference/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💡️inference/🦀️.rs:26). Its two laws at lines 41 and 52 establish only the no-document-mutation effect discipline.

2. The actual browser host handoff is [ShellHost/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3711): one matching `(pluginId, instanceId, scope)` entry is required; it mints the request ID and posts both worker messages at lines 3725–3733. The pane is mounted at line 7874 and intents are revalidated against its exact owner epoch/runtime key at lines 2017–2038.

3. The broker path is [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2443). It requires `executionTargetLease.live`, matching scope, and `grant.write`; its allowlisted Hub request at lines 2458–2466 cannot accept caller-selected URLs or a socket command. Job submission, event page, cancel, and approval are at lines 2527, 2582, 2608, and 2633. The tests beginning at line 4222 use `inferenceHarness` with a replacement `fetch`, so they do not prove the Hub route.

4. Browser plan opening can acquire verified component/descriptor bytes, but does not activate them at that point: [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:729) creates the private lease only after manifest/body/digest/descriptor checks. The source’s own target-lease test asserts that this region contains no `loadPluginModule` (line 4938). Therefore the UI handoff is ready **after** a closed actor is mounted; it is not evidence that a real GIS actor is currently mounted.

5. The native alternative cannot currently start: [WGPU Shell/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5189) documents that it has no native target lease and returns false at line 5199. `open_inference_port` terminates with `LeaseUnverified` at lines 5226–5229.

6. The Hub does real bounded deterministic inference in [runtime/🦀️.rs](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:267), materializes an authority-verified active checkpoint only at lines 558–571, and rechecks authorization/frozen identity before offering at lines 593–663. Its private preview is reconstructed and checked at lines 495–536. The four authenticated routes are registered in [bin.rs](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6333).

7. Durable acceptance is deliberately unavailable. Every current construction uses `UnavailableGisMapApprovalCommitterV1` in [bin.rs](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6591), including the native route fixture at line 7423. Further, [runtime/🦀️.rs](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:394) rejects any nonempty child list before calling the port. The current route law expressly asserts the 503 at [bin.rs](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7621).

8. The necessary typed building blocks exist but have no runtime mount:
   - `GisMapInference::create_region_group_work` creates parent, drawing, and value mutations at [inferences/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:93).
   - `DurableOwnedThreeStoreMapAssemblyV1` is production-shaped at [durable-group/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1045), but its current direct construction is the demo/fake test helper at line 3675.
   - `ArtifactAuthority::durable_group_journal_sink` routes through its retained existing-WAL actor at [artifact/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4699). It is not called by a GIS/Hub document actor.
   - A committed transaction decision can be typed/rechecked by the Hub verifier at [wal/🦀️.rs](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🧾️wal/🦀️.rs:198), but verification is read-side evidence, not publication.

## Smallest missing production slice

Implement one **server-owned per-document GIS Map approval committer** behind `GisMapApprovalCommitterV1`, registered during Hub document mounting rather than globally from a client request. Its owner must retain, for the exact `DocumentScope` and generation:

1. the materialized `GisMapSnapshot` plus the fixed `gismap-drawing` and `gismap-value` member identities;
2. real `ArtifactStore<GisMapSnapshot, GisMapMutation>`, drawing Store, and value Store; their exact admissions and three `ArtifactStoreOneItemPreparationFactory` Arcs;
3. the same document’s `ArtifactAuthority::durable_group_journal_sink(now_ms)`; and
4. the retained `DurableOwnedThreeStoreMapAssemblyV1` / mounted host until commit, absence-after-cancel, or terminal handoff.

The committer must ignore client command bytes as mutation authority. From the current Hub’s frozen, server-materialized base and approved `(job_id, proposal_hash)`, it must rederive `GisMapInference::create_region_group_work`, confirm that its parent canonical mutation equals the server-stamped command’s canonical diff/inverse, build all three typed admissions, then drive the assembly. Only its exact committed journal receipt may be converted to `CommittedInferenceWalWitnessV1` and fed to `ledger.reconcile_committed_approval`. `ArtifactHandle::submit`, an event-only caller, and a generic document receiver remain out of scope.

WGPU confirms the assembly constructor already accepts real typed Stores/factories/sink but is presently instantiated only by demo stores and a fake sink; there is no existing GIS per-document owner to reuse.

## Minimum executable acceptance

One process/native acceptance should create a trusted GIS profile, an Author and peer in one space, a Map with real drawing/value children, and a mounted per-document authority. It must drive:

1. the actual `ProposeBoundsRegion` command through a real activated GIS actor into the Shell effect handler;
2. a verified write target lease through the browser worker to the real Hub job route, then observe an owner-only offered preview;
3. the Author’s approval through the concrete committer, observing one committed decision event and `applied: true`;
4. exact changed parent/drawing/value snapshots together, no independently visible intermediate root, and a peer denied the proposal/approval; and
5. Hub/authority reopen, with the same one committed decision recognized and the three states restored without a second application.

The current sources separately prove parts of (1)–(3), but not their composition: browser tests fake fetch, Hub tests inject the unavailable committer, and Store tests use demo stores/fake journaling. Until this law exists, the honest visible result is “proposal offered, approval unavailable,” not a user-completed AI/Map edit.

