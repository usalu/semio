# GIS Map Inference Preview and Scope Acceptance

## Outcome

The browser inference port no longer offers blind approval. The authenticated Hub events page projects one narrow typed bounds preview only from the owner-private retained proposal after hash, mutation kind, stamped region identity, finite coordinate, range, ordering, and closure checks. The browser strictly decodes that projection, retains it only for the exact offered job/hash, renders it in the host-owned panel, and refuses approval until that validated projection is present.

Inference UI and transport ownership now use `documentRuntimeKeyV1({kind:"hub", spaceId, documentId})` plus the exact retained scope and operation epoch. Equal document ids in different spaces therefore cannot overwrite, clear, or operate another scope's port. An effect whose plugin session does not resolve to exactly one Hub-scoped owner fails closed.

This is a preview and browser ownership result. It is not a durable Map-commit, WGPU-rendering, external-model-provider, or authenticated end-to-end process claim. Successful approval remains fail-closed until the Store-owned committer is integrated.

## Implemented Boundary

- Hub `read_gis_map_job_events` may add `preview` only when the page is fresh, uncancelled, `succeeded`, and `offered`, with the exact published proposal hash.
- The Hub projection decodes the retained canonical `GisMapMutation`, admits only the one server-stamped `CreateRegion`, and emits only `{schema, jobId, proposalHash, regionId, ring}`. Raw proposal/result, command/inverse, base pack, receipt, bearer, path, and provider data remain absent.
- Malformed bytes, a substituted hash or mutation, wrong region identity/kind, short/reordered/out-of-range/non-finite rings, or extra feature data fail closed. Stale, cancelled, failed, or approved pages carry no preview.
- The existing approval request stays `{jobId, proposalHash}`. Preview bytes are inspection data and never approval authority.
- Worker-to-Shell inference status now carries exact `DocumentScope`; the decoder validates the scope, safe operation epoch, closed status vocabulary, exact optional preview, progress bounds, and preview/job/hash relation.
- The worker requires the preview before emitting an approval request. The reducer clears preview on replacement by any non-offered page, approval, failure, lease loss, or close.
- Both strict Rust client DTOs carry the same optional typed preview without deriving `Eq` through floating-point geometry. The shared native port status and reducer now retain the preview only in offered/approving phases and require its exact job/hash before approval.
- Shell state is keyed by runtime key. Status admission requires current epoch, runtime key, space id, and document id. A ShellHost-owned pure gate proves an old status from scope A cannot overwrite scope B and closing A retains B's live inference owner. The host panel displays bilingual Region/Longitude/Latitude values and does not place the preview into plugin view state.

## Executed Evidence

### Hub source/schema oracle

`bun nx run os-hub:gis-map-proposal-source-check --skip-nx-cache` exited `0`:

```text
gis-map-proposal-oracle: ajv=1 hostile=7 node-sha256=2 independent-bounds=2 preview=1 lifecycle=9 visibility=7 errors=11 approval-rejections=11 cross-fixture=1; no external model provider, no WGPU rendering
gis-map-proposal-check: neutral source oracle passed with hostile=7; native laws and the two-user process journey remain unclaimed.
```

The neutral fixture cross-checks the canonical GIS mutation, SHA-256, independent coordinate fold, expected bounds, and exact preview. The in-source Rust law additionally rejects substituted hashes, non-`CreateRegion` mutation, wrong region identity/kind, malformed bytes, and short/reordered/out-of-range/non-finite rings. That new Rust law still requires execution by a root-owned Hub native gate.

### Browser schema, reducer, and worker

`bun nx run @semio-tech/framework-os:gis-map-inference-port-browser-check --skip-nx-cache` exited `0`:

```text
gis-map-inference-port-oracle: ajv=1 hostileCorpora=7 transitions=25 strings=60 twinStrings=18 crossFixture=3
Test Files 1 passed; Tests 9 passed, 260 skipped
```

The worker law proves a direct offered receipt with only a proposal hash cannot approve; after the exact polled preview is accepted, approval echoes only the Hub hash. Valid-looking cross-job and cross-hash previews terminate as transport failures and never send approval.

### Host rendering and scoped lifecycle

`bun nx run @semio-tech/framework-renderer-react:scoped-presence-check --skip-nx-cache` exited `0`:

```text
scoped-presence-oracle: checks=29 clean
Test Files 1 passed; Tests 6 passed
Test Files 1 passed; Tests 1 passed, 65 skipped
```

The React law first renders an offered hash without a preview and proves there is no Approve button or preview DOM. It then renders the exact German Region, Longitude and Latitude projection and proves the approval control dispatches only after that preview is present. A separate encoded-worker-response law gives scopes A and B the same document id, rejects stale-A and foreign-scope statuses, admits only B, proves closing A retains B, then proves closing B alone retires it. The same gate retains the two-space/same-document presence and worker-socket isolation laws.

### Full renderer typecheck

`@semio-tech/framework-renderer-react:typecheck --skip-nx-cache` remains red on concurrent/pre-existing repository diagnostics in tutorial snapshots, replication typed arrays, package reexports, PluginRuntime/UI contracts, Flow declarations, and existing worker execution-target helpers. The prior preview-slice ShellHost scope diagnostics at lines 3705/3707 are absent from the rerun; no preview/panel/scoped-inference diagnostic remains.

## Remaining Qualification Frontier

1. Execute the expanded Hub preview projection unit in the next root-owned exact native Hub group; this work does not contend for `space-public-boundary-sol-target`.
2. Mount the full actual Shell after the active Space WASI producer and required descriptor fleet are physically current, then invoke the real GIS action and observe the host preview in Chromium.
3. After the Store-owned fixed-three committer is available, run the authenticated SQLite Hub two-author journey: private preview, one approval, committed receipt, replayed Map parent/drawing/value triple, and peer denial.
