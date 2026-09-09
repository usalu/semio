# Grok — GIS Map Inference OS/UI/Worker Path

Lane: OS/UI/worker GIS map inference proposal/approval. Ticket `2026/09/02/COMPLETE-SEMIO-END-TO-END`. 2026-09-09.

Repo MCP was not connected in this session. Work stayed in this already-open ticket. No git write. No Cargo / native Hub build. Ticket was not closed.

## User-facing path

On a hub-scoped GIS Map document (`s.gis.gismap`) the host chrome can request a bounds proposal without a second API. The existing worker posts `inference-open` then `inference-propose` to the hub routes already owned by hub inference.

While the job runs, `InferencePortPanel` shows bilingual EN/DE phase text, a real progress element, and Cancel (`inference-cancel`). Cancel never fabricates a terminal phase.

When the hub offers a typed preview, the panel shows region id and lon/lat extent, an accessible SVG overlay of the server ring (`role="img"`), Reject proposal (same cancel intent and hub cancel route), and Approve proposal (same approve intent and hub approval body `{jobId, proposalHash}`).

Reject and Approve are withheld until the offered page carries a preview whose `jobId` and `proposalHash` match the port. Blind hash approval remains impossible.

## What was implemented

- Neutral corpus: `rejectLifecycle`, `mapOverlay`, `controls`, `uiAffordances` on the GIS map inference port fixture
- Schema: `GisMapInferencePortV1` now requires those four fields
- Production: `projectGisMapInferencePreviewOverlayV1`, `gisMapInferencePortAffordancesV1`, EN/DE request/reject/overlay
- Host UI: SVG overlay, Reject via cancel route, Request, host request control on a live GIS Map session
- Worker / hub routes unchanged. No parallel API.
- Rust source twin: control enum gained Request/Reject/Overlay strings. Not compiled (Cargo owned elsewhere).

## Tests run

From repo root, `--skip-nx-cache`:

- `bun nx run @semio-tech/framework-os:gis-map-inference-port-check` — exit 0 — `ajv=1 hostileCorpora=7 transitions=38 strings=68 twinStrings=20 crossFixture=3`
- `bun nx run @semio-tech/framework-os:gis-map-inference-port-browser-check` — exit 0 — same oracle, then Tests 23 passed, 320 skipped
- `bun nx run @semio-tech/framework-renderer-react:scoped-presence-check` — exit 0 — scoped-presence-oracle checks=29 clean; Tests 7 passed (panel reject/overlay/request) plus 1 passed, 111 skipped

Independent oracle: AJV 2020 over the corpus (7 hostile mutations), handwritten reducer + overlay + affordance functions that import no production module, then the production reducer/overlay/affordances must agree. Reject walks `rejectLifecycle` (offered then cancel intent then server cancelled). Overlay path for the corpus ring is `M 0 100 L 100 100 L 100 0 L 0 0 L 0 100 Z`.

Cargo / semio-hub / WGPU driver laws were not run.

## Remaining native/runtime gaps

1. Hub `features.inference` is false until a trusted GIS catalog loads (`503 inference.unavailable`).
2. Worker still refuses without a live write execution-target lease (`inference.lease-unverified`).
3. WGPU has a turn driver but no user-facing panel; not built here.
4. Approval to durable Map commit is still a hub/store concern. This lane does not claim a live `applied: true`.
5. No mounted Chromium + hub process journey. Overlay/reject are host-panel + oracle proofs.
6. Host request strip requires the active session dialect `s.gis.gismap`. Otherwise use `proposeBoundsRegion`.
7. Directory-schema Rust control variants (Request/Reject/Overlay) are source-only; that crate was not compiled.

## Nonclaims

No WGPU map rendering, no external model provider, no two-user process journey, nothing persisted into the Map artifact by this port.

## Files changed

- OS schema JSON (`GisMapInferencePortV1` + overlay/control/affordance defs)
- OS fixture `gis-map-inference-port-v1`
- OS directory schema TypeScript + Rust control twin
- OS package barrel
- OS TypeScript oracle script
- ShellHost host-bootstrap panel + request control
- ShellHost mount
- scoped-presence React tests
- this report
