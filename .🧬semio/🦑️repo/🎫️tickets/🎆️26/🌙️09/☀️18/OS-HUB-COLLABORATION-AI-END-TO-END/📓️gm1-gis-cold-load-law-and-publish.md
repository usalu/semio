# GM1 — gis cold-load law green, trusted catalog publish, hub on 7611

Slice GM1 (session 6, 2026-09-20). Owner: the last known blocker between this repo and a hub with a
PUBLISHED trusted catalog (outcomes 2 + 3).

## 0. HUB HANDOFF (top of report)

| field | value |
|---|---|
| **`/readyz` 200** | **YES.** `{"status":"ready", …,"artifactAuthority":{"ready":true},"features":{"openPlan":true,"openPlanExchange":true,…}}` — observed 20:20:22 on the PUBLISHED catalog, and again after the 20:3x restart. The trusted catalog is **published**: `trusted-stdio-gis-bootstrap` exited **0** at 20:20:12. |
| **`open_target_count() > 0`** | **YES**, at runtime: `features.openPlan` IS that predicate — `open_plan_ready = artifact_authority.catalog.open_target_count() > 0` (`🌎️hub/🏗️bootstrap/🦀️.rs:10053`) — and it reads `true`. |
| **`open-plan` answers for the gis Map target** | **YES.** `🗑️generated/gm1-live-open-plan.txt`: `open-plan status=200`, `surface={"surfaceId":"s.gis.gismap@1/*#editor","appId":"s.gis.gismap@1/*#editor","windowKindId":"gis2d-main","role":"editor","rendererTarget":"wasm"}`, `grant={read,write,observe}`, `browserActor.importInterfaces=17`. |
| **the gis blocker is GONE** | `genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface` is **GREEN** — locally (`🗑️generated/gm1-law-green.txt`, `2 passed; 0 failed … 47.94s`) and, twice, inside the bootstrap's own mandatory pre-publication proof. It had never passed before. |
| port | **7611** |
| hub pid | in `🗑️generated/gm1-hub-pid.txt` is the FIRST hold (dead). The live one is the `bun 🐍️ds1-hub-hold.ts` under `📜️gm1-hub-hold.sh`; find it with `lsof -nP -iTCP:7611 -sTCP:LISTEN` (that is the `os-hub` child; its bun parent is the holder). **Leave it running.** |
| data root | `.🧬semio/🌐hub/gm1-boot` (mode 700, never shared) |
| **restart command** (catalog already published — do NOT re-run the 44-min bootstrap) | `cd /Users/ueli/Documents/semio && nohup zsh ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️gm1-hub-hold.sh" 7611 > /dev/null 2>&1 & disown` |
| publish command (only if the catalog is lost) | `nohup zsh "…/📜️gm1-hub-boot.sh" 7611 > /dev/null 2>&1 & disown` — publishes AND boots; the whole bootstrap runs inside the fleet wasm mutex (rule 27); 44 min |
| **`OS_HUB_CREDENTIAL_SIGN_IN=true` is mandatory** | without it `/readyz` reports `publicSessionIssuance:false` and `POST /auth/sessions` answers **403**, so no browser can sign in. `📜️gm1-hub-hold.sh` sets it; `📜️gm1-hub-boot.sh`'s inline hold does not. |
| two humans, provisioned on this root | `user1@semio.dev` / `gm1-local-dev-pass-1` (`01a0c00d-cd33-7948-91cb-da23affa54ec`) and `user2@semio.dev` / `gm1-local-dev-pass-2` (`01a0c00d-d7f0-7a2b-b1b4-f4060b1e1c82`). Local dev fixtures on a loopback dev hub. `user1` signed in over `/auth/sessions` → **200**, token shape `session.v1.<32hex>.<64hex>`. |
| live log | `🗑️generated/gm1-hub-hold.txt` (hold), `gm1-hub-dev.txt` (publish run 2), `gm1-hub-dev-run1.txt` (run 1) |
| binary | `⚡️cache/cargo/target-gm1/debug/os-hub`, built by the bootstrap itself (TC1 §4: a hub built before the `DOCUMENT_BROWSER_ACTOR_INTERFACES` change refuses this catalog's actor record) |
| published generation | `94a9788a14060725869ab35b12b491b1d884d487bca3a1535ece346a2b757a71`, gis component `b5b2b6ff…`, descriptor `c7a5f1f7…` |
| second hub | copy `<gm1-boot>/trusted-catalog/` into the second data root, `chmod 700`, realpath, never share a root (DS1 §10.9) |

## 1. Inherited state (measured)

- TC1's run 2 generation is on disk and is the measurement subject of §2–§3:
  `.🧬semio/🌐hub/tc1-boot/trusted-catalog/generations/d1ac9205f3660c5cfaf22d9acc12fb6aaa0004eb12838839c7f1f3161818dae4/`
  with `packages/gis/component.wasm` (47 390 785 B, 17:37, sha256 `705a9efb…51ae3`) and
  `packages/gis/descriptor.semio` (477 852 B, sha256 `26fdc117…5722`) read out of its own
  `trusted-catalog.json`.
- Port 7611 free. No hub of TC1's or DS1's survives.
- Peers are live in `✏️s/🔌️plugins/🌍️gis/` (10 staged files, incl. the runner's `--features` fix TC1
  named) and in `🧰️framework/…/🏪️store/🦀️.rs` — the store crate was mid-refactor twice during this
  slice (`missing field stamped_edit_id` at 18:09, `Mutation: protocol::Mutation<ReplayProjection…>`
  at 18:31). Both cleared on their own; every cargo here is a retry loop, never a kill.
- `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs` was committed-clean when GM1
  started (only its `🟦️.ts` sibling carried the peer's uncommitted `--features` line). Nothing of a
  peer's was reverted or touched.

## 2. Reproducing the red law — and what it actually measured

Narrowest reproduction, no 40-minute chain: run the law's own cargo target directly against the
generation already on disk (preamble rule 25, private uplift dir):

```
CARGO_TARGET_DIR=…/⚡️cache/cargo/target-gm1 RUST_MIN_STACK=268435456 \
SEMIO_GIS_COMPONENT_WASM=<gen>/packages/gis/component.wasm \
SEMIO_GIS_COMPONENT_SHA256=705a9efb686cd69b7adf26c45e579df278944ba8a742a8e304a660d0f4051ae3 \
SEMIO_GIS_DESCRIPTOR_SHA256=26fdc11788475460186ea033e7ecb947824f144f38a3a400d0f7ab40159a5722 \
cargo test -p semio-s-plugin-gis --test component_cold_map_patch \
  --no-default-features --features component-receipt-acceptance -- --nocapture
```

One run is ~48 s of law time once the target is warm, not the 308 s TC1 measured on a cold tree.

**The first measurement killed the "the map never cold-loads" hypothesis outright.** With the
assertion replaced by a print (`🗑️generated/gm1-law-run1.txt`), the scene the law decodes is not the
default 152-feature demo map and not an empty document — it is a scene whose `map_fixture_json` is
**zero bytes long**, on BOTH renders:

```
GM1-DIAG turn=0 marker=cold-before revision=1 contains=false len=0 json=
GM1-DIAG turn=0 marker=patched-after revision=2 contains=false len=0 json=
```

That is a transport fact, not a document fact. `map_fixture_json` is a **scene LANE**: since
2026-09-19 (`TILEDMAP_SCENE_LANE_*`, `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:1937`)
`TiledMapScene::split_lanes` unconditionally moves the whole `{positions, routes, regions}`
descriptor OUT of the fixed-capacity `SurfaceProps.doc` and publishes it beside the surface as its
own `paged_text_carrier` child (`scene_surface`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:504-506`). The reason is in that lane's own
docstring: the gis demo map encodes to 59 667 bytes, nearly twice the 32 KiB `UiFixedBytes` ceiling,
so before lanes existed **the gis window refused admission and never published at all**.

The framework says this in as many words, in the doc comment of its own reader
(`🔌️plugin/🦀️.rs:7060`): *"`semio_framework_ui_scene::decode` alone returns the SPINE … so a test
that reads one off a bare decode reads an empty field."* `scene_from_patch` at
`🌉️component-cold-map-patch/🦀️.rs:159` did exactly that bare decode.

**Dates settle which side is stale.** The law was added `de617a7c17`, **2026-09-08 23:25**. The
tiled-map lane split landed `48a8c69cdb`, **2026-09-19 13:48** — eleven days later, as B3a's fix for
the surface that could not publish. Neither of the law's two callers could BUILD it in committed
state until TC1's `--features component-receipt-acceptance` fix yesterday evening (TC1 §4b), so this
was the law's first execution ever, and it was executing a reader that a later, deliberate, live-
proven product change had made unsatisfiable.

**Second measurement — the exact patch shape** (`🗑️generated/gm1-law-run3.txt`). The guest publishes
the surface ONCE in full and every later render as a delta:

```
patch revision=1 base=0 ops=4
  op[0] Upsert id=UiNodeId(2) key=c0    kind=text[{"positions":[{"id":"col]      ← the lane's packed leaf
  op[1] Upsert id=UiNodeId(1) key=framework.scene.tiledmap.mapFixture  Container ← the lane carrier root
  op[2] Upsert id=UiNodeId(0) key=gis2d.play.composite  surface/tiled-map@1      ← the spine
  op[3] SetRoot id=UiNodeId(0)
patch revision=2 base=1 ops=2
  op[0] SetComponent id=UiNodeId(2) kind=text[{"positions":[{"id":"pat]          ← the mutation, exactly
  op[1] SetComponent id=UiNodeId(0) kind=surface/tiled-map@1
```

So the gis component is **correct on both halves**: the cold pair lands in the document and the first
render publishes it, and the addressed `patchPositions` mutation republishes it. A reader that decodes
one patch in isolation sees an empty field on the first render and **no surface at all** from the
second render onward — which is what the 256-turn `no-tiled-map-scene` spin in run 2 was.

## 3. The fix — the law becomes a retained receiver, which is what a render host is

**No product code was changed, and none should be.** B3a2 §17.2 measured the live lane painting all
152 demo features through the tile proxy; putting `map_fixture_json` back inside the surface doc
would re-break exactly that. What was wrong is the law's *receiver*.

Landed in `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs`:

1. `RetainedSurface` (`:159-252`) — a revision plus an id-keyed node table folded from the published
   patches, the way every render host holds one. `apply` **asserts `patch.base_revision` equals the
   revision it already holds**, so a skipped or replayed publication is now a refusal the law makes,
   which the old one-patch reader could not even express.
2. `RetainedSurface::scene` — the ASSEMBLED scene: spine decoded out of `SurfaceProps.doc`, then
   every lane carrier child merged back in through the framework's own `SceneDoc::merge_lane`. It is
   the Rust twin of the React Interpreter's `sceneFromLanes` and of the framework's own
   `built_surface_scene`/`decode_fixture_scene_with_lanes`.
3. `RetainedSurface::carrier_text` — depth-first concatenation of every `Component::Text` leaf under
   one carrier, the exact inverse of `paged_text_carrier`, read through the retained id table.
4. `retain_patch`/`render_until_scene` take `&mut RetainedSurface`, and the test body threads ONE
   receiver across both renders (`:337-351`), because the second render is a delta against the first.

**Every assertion the law made is still made, unchanged**: the marker is asserted on EVERY decoded
scene inside the 256-turn loop (`:294`), `!before.contains("patched-after")`,
`!after.contains("cold-before")`, `after_revision > before_revision`, plus the cold-pair,
lifecycle, transport-lease and command-batch laws around them. The nine source-text markers
`proveGisComponentColdMapPatch` pins (`🟦️.ts:61-73`) are all still present. The law is strictly
stronger than before: it now additionally proves the lane carrier is published and carries the exact
payload, and that the revision chain has no gaps.

Measured result — `🗑️generated/gm1-law-green.txt`, 18:38:51, against the genuine wasm component
under Wasmtime:

```
test genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface ... ok
test genuine_gis_component_rejects_stale_cold_authority_before_loading ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 47.94s
```

and the content the assembled scene carries, read off the same run before the diagnostics were
removed:

```
turn=0 revision=1 contains=true len=82
  json={"positions":[{"id":"cold-before","lat":46.2,"lon":7.1}],"regions":[],"routes":[]}
```

i.e. the cold-loaded document, exactly, feature for feature.

## 4. Publish + boot run

### 4a. Run 1 (18:40:47 → 19:29:47) — the gis proof PASSED in the production chain, and exposed the next defect

Log `🗑️generated/gm1-hub-dev-run1.txt`. Both packages materialised (stdio then gis, `complete 8/8`
each), `verify-generation 8/8`, and then the thing TC1 never reached:

```
trusted-gis-cold-map native: genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface
trusted-gis-cold-map native: genuine_gis_component_rejects_stale_cold_authority_before_loading
trusted-gis-cold-map-receipt: {"package":"semio-s-plugin-gis","target":{"kind":"test","name":"component_cold_map_patch"},
  "laws":[both],"assertions":2,"artifactDir":"⚡️cache/hub/trusted-gis-cold-map/f5549e04…/exact-cargo-laws-MC8lB2/00"}
trusted-gis-cold-map-component: generation=f5549e04… component=32566bbc… descriptor=9787574a… exact=2
```

**TC1 §4c is closed**: the mandatory pre-publication proof passes against a freshly built, freshly
receipted gis component, inside `validateAndPublishTrustedStdioGisCandidate`, with its five hostile
mutations still pinning it.

The run then died one call later (`gm1-hub-dev-run1.txt:14846`):

```
error: document-open.invalid-fields
  at documentOpenObject            (📇️directory/🧬️schema/🟦️.ts:1314)
  at parseDocumentOpenPlanV1       (…:1379)
  at proveTrustedStdioGisCandidatePlan (🌎️hub/📦️packages/🦀️rust/📜️script.ts:10233)
```

### 4b. The FIFTH blocker, diagnosed in 60 seconds instead of 50 minutes, and fixed

The failed run leaves its candidate hub's data root on disk with the catalog already staged
(`<gm1-boot>/trusted-catalog/validation/gis-cXwJTC/candidate-data`), so the failing three calls can
be replayed directly. `🐍️gm1-open-plan-diagnose.ts` (new, permanent) boots that root with GM1's own
binary and prints raw statuses — capture `🗑️generated/gm1-open-plan-diagnose.txt`:

```
readyz artifactAuthority.ready=true openPlan=true openPlanExchange=true
readyz body={"schema":"semio.hub.readiness/v1","status":"ready",…,"artifactAuthority":{"ready":true},
             "features":{"openPlan":true,"openPlanExchange":true,"rebootstrap":true,"inference":true}}
create-space        status=202
announce-document   status=202   (outcome "accepted", event document.announced)
open-plan           status=404   body={"schema":"semio.hub.document-open-plan-error/v1","code":"not-found"}
```

**Two findings.**

1. **A hub carrying this catalog IS `artifactAuthority.ready`.** `/readyz` answers `status: "ready"`
   with every subsystem green. The materialisation is sound; only the plan probe was failing.
2. **The refusal is `not-found`, and it was invisible.** `proveTrustedStdioGisCandidatePlan` called
   `parseDocumentOpenPlanV1(await response.json(), …)` **before** reading `response.ok`. A refusal
   body is a two-field `semio.hub.document-open-plan-error/v1 { schema, code }`, which the strict
   parser rejects as `document-open.invalid-fields` — so every hub refusal, whatever its cause,
   surfaced as one generic parser error with the real `code` thrown away.

HT6's field-for-field diff of every Rust↔TS twin of the plan (15 plan keys, checkpoint 4, frontier 5,
scope 2, surface 5, grant 3, artifact 3, parentDialect 3, catalog 1, package 7+1, revalidation 2+2,
browserActor) found **no drift**, and this measurement is why: there was no plan to drift from.

The cause of the `not-found` is in the issuer (`🌎️hub/🏗️bootstrap/🦀️.rs:3097-3104`): a plan needs a
document descriptor **and** an ACTIVE ARTIFACT CHECKPOINT. `announce-document` registers only the
descriptor. The same probe, extended to create its document through the hub's own server-owned
creation transaction instead, gets a complete plan:

```
artifact-creation status=202 → phase ready, artifactId=artifact-4b2c946377d7a77e05b537d1f16dceb9
                               kind=s.gis.gismap schema=gis.map
created open-plan status=200
  receipt=open.v1.o7MJ0LTiQyFH6y6gA26beTVQDGtCWbyF6mNSbiahXYw
  surface={"surfaceId":"s.gis.gismap@1/*#editor","appId":"s.gis.gismap@1/*#editor",
           "windowKindId":"gis2d-main","role":"editor","rendererTarget":"wasm"}
  grant={"read":true,"write":true,"observe":true}
  browserActor.sha256=b96e6c1e… importInterfaces=17 (incl. wasi:clocks/wall-clock@0.2.0
           and wasi:random/insecure-seed@0.2.9 — TC1's two admitted names, live on the wire)
  checkpoint.baselineFrontier={headEditOrdinal:0,headEditId:"",lastCommitSeq:0,chainHash:32×0}
```

**`open-plan` answers for the gis Map target. Observed, at runtime, 19:35.** That is outcome 3's
open half, and it had never been seen before.

Fixed at the root in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, inside
`proveTrustedStdioGisCandidatePlan` (`:10213-10241`):

- the probe document is created through `POST /spaces/{space}/artifact-creations` with
  `sealSpaceArtifactCreateV1`, polled to `phase === "ready"`, and its `ready.artifactId` is the
  document the plan is asked for — the same server-owned transaction
  `createCheckpointPublicationProcessGenesis` (`:744`) already drives, and what a real client does
  before it opens anything. The bare `announce-document` is gone;
- the response status is read **before** the body is parsed, and a refusal now throws naming the HTTP
  status and the error body, so the next failure in this chain names its own `code`.

None of the eight source-text fences in `proveTrustedGisPublicationFixture` (`:12483-12512`) slices
into this function — they bound `publishTrustedBootstrapCurrent`, `proveTrustedStdioGisStalePlanRejected`,
`validateAndPublishTrustedStdioGisCandidate`, the materializer, the rotation and the two scripts —
and all five orderings they pin are unchanged.

### 4c. Run 2 (19:36:23 → 20:20:22) — PUBLISHED, and the hub is ready

Launched 19:36:23, acquired the fleet wasm mutex at 19:38:39 behind `pb3`, materialised both packages,
passed the gis cold-map proof a second time (generation `94a9788a…`, component `b5b2b6ff…`,
descriptor `c7a5f1f7…`, `exact=2`), issued the candidate plan, and:

```
=== bootstrap exit 0 at 2026-09-20T20:20:12+02:00 ===
=== 2. hold hub on port 7611 at 2026-09-20T20:20:12+02:00 ===
=== 3. readyz http=200 after 10s at 2026-09-20T20:20:22+02:00 ===
{"schema":"semio.hub.readiness/v1","status":"ready","runId":"c8390673…","mode":"development",
 "bindScope":"loopback","directory":{"ready":true},"storage":{"ready":true},
 "artifactCasBarrier":{"ready":true},"artifactPublication":{"ready":true},
 "artifactAuthority":{"ready":true},"adminAssets":{"ready":true},
 "features":{"openPlan":true,"openPlanExchange":true,"rebootstrap":true,"mcpWorkspace":false,"inference":true}}
```

**The trusted catalog is published and a hub serving it answers `/readyz` 200 with
`artifactAuthority.ready: true`.** 44 minutes end to end. That gate had never been reached.

### 4d. The sign-in wall, found by trying to use it

The first hold answered **403** to `POST /auth/sessions` for a correctly provisioned human, because
`/readyz` reported `authentication.publicSessionIssuance: false`. The switch is
`OS_HUB_CREDENTIAL_SIGN_IN` (`🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:145`), which neither
`📜️tc1-hub-boot.sh` nor `📜️ds1-hub-hold.ts` sets — so every hub any slice has held this week could
only ever be reached through the local-bootstrap pipe, never by a browser. **Outcome 3 is
unreachable on such a hub.** `📜️gm1-hub-hold.sh` (new) sets it, and the restarted hub reports
`publicSessionIssuance: true` with `artifactAuthority.ready` still true.

### 4e. Outcome 3's open half, on the LIVE published hub

`🐍️gm1-live-open-plan.ts` (new, permanent) — capture `🗑️generated/gm1-live-open-plan.txt`,
20:2x, against `http://127.0.0.1:7611`:

```
sign-in status=200 tokenShape=true                      ← session.v1.<32hex>.<64hex>
readyz status=ready artifactAuthority.ready=true features.openPlan=true
create-space status=202 spaceId=01a0c00f-4f3c-7834-a7e6-2ccf9de925db
creation-catalog status=200 generation=94a9788a… kinds=s.gis.gismap
artifact-creation ready documentId=artifact-2fb248125b8b2b4d56de25933d30ed21 kind=s.gis.gismap schema=gis.map
open-plan status=200
  surface={"surfaceId":"s.gis.gismap@1/*#editor","appId":"s.gis.gismap@1/*#editor",
           "windowKindId":"gis2d-main","role":"editor","rendererTarget":"wasm"}
  grant={"read":true,"write":true,"observe":true}  catalog=94a9788a…
  package={"pluginId":"gis","packageId":"semio:gis","version":"0.1.0","componentSha256":"b5b2b6ff…",…}
  browserActor.importInterfaces=17
  checkpoint={"checkpointId":"6c69fc21…","baselineFrontier":{headEditOrdinal:0,…}}
```

A signed-in human creates a space, creates a gis Map document and is issued a document-open plan for
it, by a hub serving a published trusted catalog. That is the whole open path outcome 3 stands on.

### 4f. H1b's runtime gate, on GM1's binary

`bun 🐍️h1b-hub-runtime-probe.ts --binary ⚡️cache/cargo/target-gm1/debug/os-hub --port 8853`
(`🗑️generated/gm1-h1b-runtime-probe.txt`): **44 PASS, 0 FAIL, exit 0**,
`h1b-hub-runtime: all checks passed` — liveness, readiness plus the closed-gate vocabulary, sqlite
persistence across a kill-and-reboot of the same data root, session mint/introspect/revoke/TTL
expiry, the auth rate limiter's `429`, and a real `/directory/socket/v1` presence upgrade. So the
binary this catalog is published and served with also passes outcome 2's own runtime gate.

## 5. Handing the hub to the collaboration worker

The hub this slice leaves running is on **7611**, data root `.🧬semio/🌐hub/gm1-boot`, pid in
`🗑️generated/gm1-hub-pid.txt`. Two signed-in `s` shells attach to it exactly the way C1c bound 6071
to 7501 (`📓️c1-collaboration-e2e.md`, E2E run 5 / S5.1), with 7501 replaced by 7611:

1. **Do not touch an existing `s` serve.** S2's cold `s` boot carries no `S_HUB_URL`, so it has no hub
   lane at all (`/_semio/hub/readyz` there answers the SPA fallback). Start a **second serve of the
   same staged output** — serve only, no activation — on a free port:

   ```
   cd /Users/ueli/Documents/semio && S_OS_PORT=6072 S_HUB_URL=http://127.0.0.1:7611 \
     nohup bun 📜️script.ts serve s react dev > 🗑️generated/<slice>-s-serve.txt 2>&1 & disown
   curl -sD- http://127.0.0.1:6072/_semio/hub/healthz   # must be 200 application/json, not text/html
   ```

2. **Provision the two humans against THIS data root**, with the binary this slice built:

   ```
   OS_HUB_DATA=$PWD/.🧬semio/🌐hub/gm1-boot \
     .🧬semio/🦑️repo/⚡️cache/cargo/target-gm1/debug/os-hub credential set \
     --email user1@semio.dev --display-name "User One"      # password on stdin
   ```

   and the same for `user2@semio.dev`. C1c's S5.1 proved both principals sign in through the shell's
   own form — no launcher, no relay, no `#semio-broker=` fragment.

3. **Two humans = two browsing contexts on ONE origin.** Since C1c §15, identity is a
   `sessionStorage` capability per browsing context, so the per-user `S_OS_PORT` pair is no longer
   needed: `bun 🐍️c1c-identity-probe.mjs http://127.0.0.1:6072 http://127.0.0.1:6072` with two
   playwright contexts is the shape that scored 19/19.

4. **The document both humans open must be created through the hub's own creation transaction**, not
   announced — §4b is the whole reason: `POST /spaces/{space}/artifact-creations` with
   `sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId, kindId: "s.gis.gismap", name })`,
   polled to `phase: "ready"`; its `ready.artifactId` is the document id. Only then does
   `POST /spaces/{space}/documents/{id}/open-plan` answer 200, and its `receipt` is what
   `POST …/socket-grants` exchanges for the document socket both shells ride.
   `🐍️gm1-open-plan-diagnose.ts` is a working, runnable example of steps 4's two halves.

5. **A second hub needs its own data root** — copy `<gm1-boot>/trusted-catalog/` into it, `chmod 700`,
   realpath it, never share a root (DS1 §10.9).

The one blocker C1c left that GM1 does not touch: after sign-in the `s` Home surface clears
`s.home.session-identity-required` but `s-home-main` is still never published, because `refreshUi`
talks to an actor that already stopped. C1c §run 5 names the fix (run `establishPrimaryWithShardRetry`
when the human component of the session key changes, `🏛️ShellHost/🟦️.tsx:3742/:3759`) and it is
unverified. That is a shell-host matter, not a hub one.

## 6. Files changed

Product code — three files, all three root fixes, none of them a weakened law:

| file | change |
|---|---|
| `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs` | `RetainedSurface` (`:159-252`) — the law becomes a retained receiver that reassembles the scene from spine + lane carriers and refuses a revision gap; `retain_patch`/`render_until_scene` thread it; the test body owns one across both renders. Every prior assertion is kept. |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `proveTrustedStdioGisCandidatePlan` (`:10213-10241`) — the probe document is created through the Hub's own server-owned artifact-creation transaction (which establishes the active checkpoint the open-plan issuer requires) instead of a bare `announce-document`; the response status is read before the body is parsed, so a refusal names its HTTP status and error body. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts` | `documentOpenObject` (`:1313-1318`) — the closed-object refusal now names `unexpected=[…] missing=[…] accepted=[…]`. It is the gate for the plan and every object nested in it, and a bare code could not distinguish an added field from a missing one, in which object — nor from a body that is not a plan at all. Prefix `document-open.invalid-fields` is unchanged, so the one law that asserts it (`🧪️space-artifact-creation-owner/🟦️.ts:1009`, a `toThrow` substring) still holds. |

Ticket-owned: `📜️gm1-hub-boot.sh`, `📜️gm1-hub-hold.sh`, `🐍️gm1-open-plan-diagnose.ts`,
`🐍️gm1-live-open-plan.ts`, this report, and the `🗑️generated/gm1-*` captures (`gm1-law-run1.txt`,
`gm1-law-run3.txt`, `gm1-law-green.txt`, `gm1-hub-dev-run1.txt`, `gm1-hub-dev.txt`,
`gm1-hub-hold.txt`, `gm1-hub-readyz.txt`, `gm1-open-plan-diagnose.txt`, `gm1-live-open-plan.txt`,
`gm1-h1b-runtime-probe.txt`).

**Nothing of a peer's was reverted.** The gis plugin's ten staged peer edits and the live
`🏪️store/🦀️.rs` refactor were left exactly as found; every cargo of this slice is a retry loop that
waited the refactor out (two compile breaks at 18:09 and 18:31), never a kill.

## 7. Honest gaps

1. **Two users on ONE document has still not been observed.** GM1 proves the open PATH end to end —
   a published catalog, a ready hub, a signed-in human, a created gis Map document, a 200 plan with
   its socket receipt — but no `s` shell was attached and no second human opened the same document.
   §5 is the recipe, and it is untested by me.
2. **The socket half is unexercised.** `POST …/socket-grants` and `…/socket/v1` were never called
   with the issued receipt. `browserActor.importInterfaces=17` is a plan field, not a browser that
   instantiated the 63 MB `closed-actor.mjs` (TC1's gap 3 stands).
3. **`OS_HUB_CREDENTIAL_SIGN_IN` is set only by `📜️gm1-hub-hold.sh`.** Every other hub launcher in
   this ticket (`📜️tc1-hub-boot.sh`, `📜️ds1-hub-boot.sh`, the inline hold inside
   `📜️gm1-hub-boot.sh`) still holds a hub no browser can sign in to. That is a real product/harness
   gap for outcome 3 and I fixed it only for my own hold.
4. **TC1 §5's N-plugin generalisation is still designed, not landed.** GM1 touched none of its eight
   rows; the catalog is still stdio+gis with one open target.
5. **The retained receiver in the gis law does not apply `SetLayout`/`SetStyle`/`SetActivity`/
   `SetAccessibility`/`SetBindings`/`SetMenu`/`SetRoot`.** They carry no surface doc and no lane
   payload, so they cannot change the assembled scene; a law that wanted to assert on layout would
   have to add them. `Upsert`, `SetComponent`, `SetChildren` and `Remove` are applied in full.
6. **Disk.** The `gm1-boot` data root is ~600 MB (two generations plus run 1's inert candidate
   validation dir `trusted-catalog/validation/gis-cXwJTC`). 40 GiB free at the time of writing.
   Deleting `validation/gis-cXwJTC` is safe; `generations/` and `current` are not.
