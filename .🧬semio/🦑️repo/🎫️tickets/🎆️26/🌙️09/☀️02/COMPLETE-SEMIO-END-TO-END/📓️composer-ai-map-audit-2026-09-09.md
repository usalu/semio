# Composer AI-over-Map End-to-End Audit

Read-only audit of the current working tree on 2026-09-09. Scope: proposal → inference → approval → GIS map UI → MCP, across hub inference, GIS gismap plugin, OS inference opening/port, and native proposal laws vs source-only UI.

## Executive verdict

**AI-over-map does not work end-to-end for a real user today.**

The vertical slice is **architecturally wired** (hub routes, deterministic GIS inference service, host-owned inference port, React `InferencePortPanel`, MCP wire bridge), but **runtime prerequisites and UI gaps** stop the journey before a user can reliably propose, review on the map, approve, and see a durable region commit in a mounted browser session.

**“AI” here is not an external model.** The only implemented inference is deterministic geographic bounds computation (`s.gis.gismap.inference`): scan lon/lat pairs, derive a bounding box, propose one `CreateRegion` with `kind: inference-bounds`. No LLM/provider/relay to OpenAI, Anthropic, or similar exists under `🌎️hub/💡️inference` or the GIS plugin.

Cargo/native law qualification is **owned elsewhere** (Home; receipt `🗑️generated/gis-map-proposal-native/exact-cargo-laws-N1AVY4/00` still compiling at audit time per `📓️root-current-acceptance-frontier.md`). This report does **not** recommend competing for Hub Cargo builds.

---

## What exists (by layer)

### Hub inference (`🌎️hub/💡️inference`)

| Piece | Path | Status |
|---|---|---|
| Closed wire schemas (request, events, approval, preview, reconcile) | `🧬️schema/🔣️.json`, `🧬️schema/🦀️.rs` | Complete |
| Frozen GIS Map binding identity | `📇️catalog/🦀️.rs` (`VerifiedGisMapArtifactBindingV1`) | Complete |
| Job runtime (submit, poll, cancel, approve) | `🏃️runtime/🦀️.rs` (`HubInferenceRuntimeV1`) | Complete in source |
| SQLite ledger, WAL witness, authorization | `🪶️sqlite/🦀️.rs`, `🧾️wal/🦀️.rs`, `🛂️authorization/🦀️.rs` | Complete |
| HTTP routes | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` (~8085–8090) | Registered |
| Production committer | `RetainedGisMapApprovalCommitterV1` wired at startup (~8381) when binding + artifact authority exist | Source-complete |
| Relay path schema | `🌐️relay/🔣️.json` | Declares `/_semio/hub/.../inference/gis-map/...` |

**Readiness gate:** `inference_ready = inference_runtime.is_some()` (~8397). Runtime exists only when `verified_gis_map_binding(catalog)` succeeds (~8348–8351). Without `configured_artifact_authority` loading a trusted catalog from `OS_HUB_DATA/trusted-catalog/current.json` with native codec providers, **`features.inference` is false** and all four routes fail closed with `503 inference.unavailable`.

**Test-support HTTP laws** (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, `feature = "test-support"`) prove submit → running → succeeded → offered proposal using real `infer_gis_map_controlled`, but the fixture deliberately mounts `UnavailableGisMapApprovalCommitterV1` (~1284), so HTTP approval always ends at `503 approval.commit-unavailable`. That is **intentional fail-closed evidence**, not production wiring.

**Library laws** with `RetainedGisMapApprovalCommitterV1` (`🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`, e.g. `gis_map_approval_committed_event_reaches_actor_frontier...`) show approval **can** reconcile to `Approved` with applied Map pack when the retained committer and checkpoint publisher succeed (~565–570).

### GIS Map plugin (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap`)

| Piece | Path | Status |
|---|---|---|
| Executable inference service | `🦀️.rs` (`gis_map_inference_service`, `infer_gis_map_controlled`) | Complete |
| Inference schema + `bounds_proposal` / `create_region_group_work` | `🏅️standards/.../🧬️schema/💡️inferences/🦀️.rs` | Complete |
| Shell action (no document mutation) | `✏️editor/🎮️commands/💡️inference/🦀️.rs` (`propose_bounds_region` → `Effect::RequestInferenceProposal`) | Complete |
| Action registration | `✏️editor/🦀️.rs` (~983): `proposeBoundsRegion`, category `inference`, `ActionKind::Shell` | Complete |
| Native controlled-inference + proposal laws (fixture) | `📇️native-codecs/🧪️tests/📇️native-codecs/🦀️.rs` | Source/oracle; native Cargo gate pending |
| Map canvas preview overlay for offered ring | *(none in editor modes/render)* | **Missing** |

Proposal semantics: job id must be 32 lowercase hex; region id `inference-{jobId}`; closed rectangular ring from inferred bounds; stale if counts drift; composition requires default `gismap-drawing` / `gismap-value` children without image.

### OS framework inference (`🧰️framework/.../💡️inference`)

| Piece | Path | Role |
|---|---|---|
| Dependency-aware cache (`InferenceCache`, `InferredField`) | `🦀️.rs` | Kernel optimization; **not** the hub job lifecycle |
| Worker port admission | `🚪️opening/🟦️.ts` (`InferencePortOpeningMailboxV1`) | Single opening, epoch, timeout |
| Port state machine | `📇️directory/🧬️schema/🦀️.rs` (`reduce_gis_map_inference_port_v1`) | Shared by worker + WGPU driver |
| Directory client hub calls | `📇️directory/🔌️client/🦀️.rs` | `submit_gis_map_inference_job`, `approve_gis_map_inference_job`, etc. |

### Shell / worker UI (`🧰️framework/.../📺️renderer`, `🧵️backbone-worker.ts`)

| Piece | Path | Status |
|---|---|---|
| React `InferencePortPanel` (en/de, progress, Cancel/Approve) | `🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx` (~199–242) | Complete |
| Panel mount + worker relay | `🏛️ShellHost/🟦️.tsx` (~9146–9149, `dispatchInferencePortIntent` ~2691) | Complete |
| Effect handler: open port + propose | `requestInferenceProposal` (~4367–4397) | Complete |
| Backbone worker driver | `backbone-worker.ts` (`openInferencePort`, `submitInferenceJob`, `driveInferencePort`, ~5345+) | Complete |
| Lease precondition | `inferenceLeaseVerified` (~5246): live write grant on execution-target lease | Enforced |
| WGPU native shell driver | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`GisMapInferenceDriverV1`, `ShellInferenceRunner`, `open_inference_port`) | Driver only |
| WGPU inference UI panel | *(no `InferencePortPanel` equivalent)* | **Missing** |
| GIS map draw of preview ring | *(no editor hook)* | **Missing** |

### MCP (`🧰️framework/.../🌉️mcp`)

| Piece | Path | Status |
|---|---|---|
| Four tools + path oracle | `💡️inference/🦀️.rs`, `📦️packages/🟦️typescript/💡️inference-bridge.ts` | Wire-shape conformance only |
| Explicit nonclaim | `inference-bridge.ts` header + tests | No live hub job, no map render |

MCP is relevant as an **alternate client** to the same hub routes once `features.inference: true`; it does not complete the map UI journey.

---

## Native proposal laws vs source-only UI

| Concern | Native / Rust laws | Source-only UI |
|---|---|---|
| Deterministic inference + budgets | `gis_map_inference_service`, `infer_gis_map_controlled`, native-codecs test | — |
| `bounds_proposal` identity/stale/bounds rejections | `📇️native-codecs/...` + `💡️inferences/🧪️tests` | — |
| Hub job lifecycle + visibility | `🏃️runtime/🧪️tests`, bin-unit HTTP (test-support) | — |
| Hub schema/fixture oracles | `📜️script.ts` gis-map-proposal-check, gis-inference-ledger-oracle | Bun/AJV |
| MCP ↔ hub paths | — | `inference-bridge-source-check` |
| Worker port FSM | Rust reducer tests + TS backbone tests | `gis-map-inference-port-check` (launch.json) |
| User-visible propose/approve | WGPU: **no panel** | React: `InferencePortPanel` **source-complete** |
| Preview on map canvas | — | **Not implemented** (panel shows lon/lat in `<dl>` only) |
| Mounted two-peer browser journey | — | **Not qualified** (`📓️root-current-acceptance-frontier.md`) |

**Gap pattern:** backend and worker logic are ahead of **map visualization** and **native shell chrome**. UI laws exist for the host panel (Vitest on `InferencePortPanel`, scoped-presence tests) but not for drawing the offered bounds on the GIS surface.

---

## What a user would do on the map (intended browser path)

Prerequisites that must all succeed:

1. Hub running with `native-artifact-execution`, trusted catalog loaded, GIS Map in catalog → `features.inference: true`.
2. User signed in; session authority current in ShellHost.
3. GIS Map document open in hub scope with **verified execution-target lease** (write grant). Without this, `inference-open` refuses `inference.lease-unverified` (`backbone-worker.ts` ~5246–5252).
4. Hub inference binding matches the open document’s frozen package/descriptor (otherwise jobs go stale or `inference.unavailable`).

User steps:

1. Open the GIS Map editor for the hub-backed document.
2. Open command palette / action UI and run **“Propose Bounds Region”** / **“Begrenzungsregion vorschlagen”** (`proposeBoundsRegion`).
3. Plugin dispatches `Gis2dCommand::ProposeBoundsRegion` → `Effect::RequestInferenceProposal { kind: GisMapBoundsRegion }` (no document write).
4. ShellHost `requestInferenceProposal` opens the single inference port via worker (`inference-open` → `inference-propose`).
5. Worker POSTs `semio.hub.inference-request/v1` to hub; hub runs retained `infer_gis_map_controlled`, offers preview (`semio.hub.gis-map-inference-preview/v1` with ring).
6. **`InferencePortPanel`** appears (top center overlay): phase text, progress bar, preview coordinates — **not** a highlighted region on the map.
7. User clicks **Approve** → worker POSTs approval with server-published `proposalHash`.
8. Hub `RetainedGisMapApprovalCommitterV1` attempts atomic parent+drawing+value publication; on success, normal document checkpoint fanout should add the region.

**WGPU-native path:** `RequestInferenceProposal` calls `open_inference_port()` which auto-intends `Propose` (`wgpu/🦀️.rs` ~4541), but there is **no UI** to cancel/approve except programmatic `approve_inference_proposal()` / `cancel_inference_proposal()` — not exposed to the user.

---

## End-to-end proof status

| Stage | Proved in tree? | Evidence type |
|---|---|---|
| Schema + fixtures | Yes | AJV oracles, Rust decode laws |
| GIS inference execution | Yes (native) | Unit + native-codecs fixture |
| Hub route submit → offer | Yes (test-support HTTP) | bin-unit with checkpoint gate |
| Hub HTTP approve → commit | **No** (fixture uses unavailable committer) | 503 by design in bin tests |
| Hub approve → commit (production committer) | Partial | Lib test with memory DB + retained committer |
| Worker port → hub | Yes | backbone-worker + OS tests |
| React panel → worker | Yes | ShellHost wiring + component tests |
| User sees proposal **on map** | **No** | No render hook |
| Two users (offer private, commit visible) | **No** | No qualified mounted journey |
| MCP live job | **No** | Bridge explicitly disclaims |
| Full dev launch without catalog | **No** | `inference.unavailable` |

**Conclusion:** The chain is **provably connected in fragments**, not **provably working as one user journey** on a running dev stack.

---

## Remaining blockers (non-Cargo)

Listed with paths; ordered by leverage for a working browser journey.

### 1. Hub inference unavailable without trusted GIS catalog (highest leverage)

- **Paths:** `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` (`configured_artifact_authority` ~444–459, `gis_map_binding` ~8348–8351, `inference_ready` ~8397)
- **Symptom:** `503 inference.unavailable`; worker terminates with `inference.unavailable` text (`📇️directory/🧬️schema/🟦️.ts`).
- **Fix lane:** Trusted-catalog / dev-bootstrap lane — ensure `OS_HUB_DATA/trusted-catalog/current.json` loads a profile whose open plan includes GIS Map editor with `s.gis.gismap.inference`, without touching Hub Cargo ownership.

### 2. Execution-target lease required before any proposal

- **Paths:** `🧰️framework/.../🧵️backbone-worker.ts` (`inferenceLeaseVerified` ~5246), `🚪️opening/🧬️schema/🔣️.json` (refused `inference.lease-unverified`)
- **Symptom:** Port opens refused; user sees terminal failure before submit.
- **Fix lane:** WGPU/browser execution-target lane — document must complete lease verification with write grant before `proposeBoundsRegion` is enabled (or surface clear UX when disabled).

### 3. No GIS map canvas preview for offered proposal

- **Paths:** GIS editor under `✏️s/.../gismap/.../✏️editor/` (no `inference`/`preview` in modes); preview only in `InferencePortPanel` (`🪪️host-bootstrap/🟦️.tsx` ~227–232)
- **Symptom:** User cannot spatially validate the AI proposal on the map; only numeric bounds in host chrome.
- **Fix lane:** GIS plugin UI — consume hub preview ring (or host-fed port status `preview`) and draw transient `inference-bounds` geometry before approval; must not write document until server-stamped approval.

### 4. WGPU native shell lacks inference port UI

- **Paths:** `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`inference_port_status`, `approve_inference_proposal` ~4583; no panel)
- **Symptom:** Native Shell users cannot approve/cancel without new UI or keybindings.
- **Fix lane:** Renderer/native Shell — mount shared port status chrome or wire actions to existing shell command surface.

### 5. No qualified mounted browser end-to-end test

- **Paths:** `📓️root-current-acceptance-frontier.md`; gap called out in `📓️fable-ai-map-proposal.md` nonclaims
- **Symptom:** No regression lock on propose → offer → approve → map refresh.
- **Fix lane:** Terra/WGPU — two-mounted-Shell or single-browser journey test with hub test-support profile (not new Cargo on hub binary).

### 6. HTTP integration tests do not exercise production committer

- **Paths:** `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` `gis_map_inference_fixture` (~1284 `UnavailableGisMapApprovalCommitterV1`); contrast production `🚀️bin.rs` ~8381 `RetainedGisMapApprovalCommitterV1`
- **Symptom:** Green HTTP tests still allow `approval.commit-unavailable` at the user-visible boundary.
- **Fix lane:** Test-support only — add one bin law with retained committer + test-support profile (`🗿️artifact-authority/.../🏗️test-support/🦀️.rs` already exists) to prove HTTP approve → applied without competing for release Cargo.

### 7. MCP is not an end-to-end substitute

- **Paths:** `🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.ts`, `🌉️mcp/💡️inference/🦀️.rs`
- **Symptom:** Tools exist; no binding → same `inference.unavailable` dead end.
- **Fix lane:** Same as (1); MCP follows hub readiness.

### 8. “AI” expectation mismatch (product)

- **Paths:** inference service metadata in `🗺️gismap/🦀️.rs`; no provider under hub/GIS
- **Symptom:** No LLM reasoning step; bounds inference only.
- **Fix lane:** Separate reasoning plugin ticket if external AI is required; out of scope for current gismap inference service.

---

## Highest-leverage next implementation lane

**Lane: “Browser GIS map proposal journey” (non-Cargo)**

1. **Unblock hub readiness in dev** — trusted catalog profile with GIS Map inference binding (owner: artifact-authority / hub bootstrap, not Hub compile).
2. **Gate the map action on lease + inference readiness** — disable or explain `proposeBoundsRegion` when lease or `features.inference` is false.
3. **Draw offered preview on the map** — transient overlay from port `preview.ring` / hub preview DTO.
4. **Qualify one mounted browser test** — palette action → panel offered → approve → document region count +1 (reuse `test_support::verified_gis_map_test_profile` server-side).

Defer WGPU panel parity until browser journey is green. Defer MCP live demos until (1) is true.

---

## Related ticket artifacts

- `📓️fable-ai-map-proposal.md` — slices A–D implementation log
- `📓️terra-ai-map-proposal-approval-current-p0.md` — original P0 packet (partially superseded by fable work)
- `📓️root-current-acceptance-frontier.md` — native proposal Cargo still active under Home; no mounted full journey

---

## Audit method

Read-only inspection of working tree sources, fixtures, tests, and ticket `🗑️generated` receipts. No files modified except this report. No tests executed in this audit session.
