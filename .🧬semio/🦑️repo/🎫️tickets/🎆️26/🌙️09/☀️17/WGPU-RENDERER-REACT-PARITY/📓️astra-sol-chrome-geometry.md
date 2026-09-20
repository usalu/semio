# Astra Sol — Shell Chrome Geometry and Dock Cap Parity

## Result

The WGPU shell now derives navbar and footer positions from measured occupied bands, using the same widest-free-band and centered-clamp law as React. `top-middle` is part of the centered cluster. The footer reserves the live React sequence `presence → HubConnectionIndicator → bottom-right`, while the mobile device gate keeps only the hub badge. Dock caps now use `controlHeight + 2 × padding`, independently of navbar height, and the selected tab of the globally active stack receives the selected fill.

Production source is stable for root-owned native/Wasm integration.

## Shared contract first

The neutral fixture at `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔝️navbar-centered-band/🔣️.json` now declares:

- `top-middle` belongs to the centered navbar cluster;
- footer trailing order is presence, hub connection, bottom-right;
- mobile omits desktop navbar controls and retains the hub connection badge;
- Dock cap depth is derived from control height plus both padding edges;
- a customized theme proves navbar height `52` and Dock cap depth `40` are independent (`C=24`, `P=8`, `C+2P=40`).

The existing TypeScript React-helper oracle consumes the same fixture and validates both the default accidental equality (`28.8`) and the customized independent case. Rust consumes the same JSON for free-band, centered-placement, responsive contract, and token-formula laws.

## Shell navbar

`Shell/🎯️targets/🧊️wgpu/🦀️.rs` now measures exact retained-chip widths before painting:

1. The occupied leading band contains `top-left`.
2. The occupied trailing band contains `top-right`, the inter-item gap, and fullscreen.
3. The centered desired width contains logo/title/role badge, example group, mode group, role group, and `top-middle`, with React's double gap between sibling groups and no gap between items inside one button/tab group.
4. The centered span is clamped to the widest free band through the shared `navbarFreeBandV1`/`navbarCenteredLeftV1` Rust twin.
5. Physical centered control rectangles are clipped to that span before paint and hit registration.

The mobile gate omits `top-left`, `top-middle`, `top-right`, example, mode, and role-switch groups. It keeps logo/title/role badge, mobile-panel toggle, and fullscreen, reserving the trailing controls before clipping title content.

## Shell footer

The old direct sync-status pill was removed from footer painting. `s-sync-status` remains the real bottom-left panel tab, avoiding the previous duplicated and semantically stale sync indicator.

Desktop geometry is planned before paint as:

`bottom-left | widest free band containing centered bottom-middle | presence · hub · bottom-right`

Mobile omits all three panel-tab bands and presence, retaining the hub badge at the trailing edge. Bottom-middle rectangles are clipped to the computed centered span. Ambient presence and hub status do not register invented button actions.

### Honest native hub projection

The badge uses the existing native host state:

- no hub environment → offline;
- configured environment with an in-flight identity bootstrap → connecting;
- configured environment with no identity after bootstrap → signed out;
- cached identity degraded offline → offline;
- current attached document `Live`, `Connecting`, `Backoff`, or `Detached` → live, connecting, reconnecting, or offline;
- live peer count comes from that document's `RemoteState::Live`.

The labels and icons match React's localized hub vocabulary in English and German. The badge is noninteractive because WGPU has no native `openHubWorkspace`/`framework.hub.signIn` action seam; no fake action is exposed.

Exact residual: React aggregates statuses across every attached document and reports the busiest live document's peer count. WGPU currently owns only the active native document's `sync_status`, so its badge is an honest current-document projection. Adding an all-open-document status registry and a native hub-workspace action is a separate host-state packet.

### Remaining host/auth parity matrix

| Surface | Status source | Signed-out status | Sign-in action | Remaining gap |
|---|---|---:|---:|---|
| React ShellHost | all open document statuses + hub session | yes | `framework.hub.signIn` through `openHubWorkspace` | none in this bounded comparison |
| native WGPU | current attached document + native identity/bootstrap | yes | absent | needs an all-document status registry and native hub-workspace action seam |
| browser WGPU | identity/offline flags, no WGPU document-status registry | state currently resolves offline | absent | needs browser WGPU transport status publication and the same hub-workspace action seam |

This host/auth gap is open and prevents full HubConnectionIndicator parity. This packet completes the bounded sequence geometry without inventing a status source or an action the host cannot execute.

## Dock cap geometry

`Dock/🎯️targets/🧊️wgpu/🦀️.rs` has one `dock_cap_depth(theme) = C + 2P` authority. It drives:

- stack tab-bar/drop rectangles;
- top and bottom cap layout;
- silhouette edge depths;
- safe body clearances;
- glass group rectangles;
- tab select hit rectangles.

The tab's icon, text, and close/focus/drag controls occupy the inset `C`-high control row at `cap.y + P`. Action hit rectangles therefore remain ordinary control height while selection and cap/drop geometry use the padded outer cap. The globally selected tab paints `theme.selected` only when its stack is globally active; other stacks retain their ordinary cap surface and border.

## Laws and validation

Added or updated Rust laws in the existing Shell navbar/footer parity and Dock WGPU unit modules cover:

- shared neutral free-band and placement vectors;
- the customized navbar-height fixture;
- `top-middle` containment in the centered physical hit band;
- no overlap between centered and edge navbar hit rectangles;
- desktop footer sequence and mobile hub-only gate;
- exactly one `s-sync-status` tab and no invented status actions;
- cap, silhouette, safe body, drop, select-hit, and action-hit dimensions from `C+2P`;
- full-cap selected fill only for the globally active selected tab.

Validation performed:

- `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-react:test' --skip-nx-cache -- exhaustive <navbar-centered-band test>`: **1 file, 20 tests passed**.
- `rustfmt --edition 2024 --emit stdout` parsed the two changed Rust production files and their two Rust test modules without syntax errors.
- Native/Cargo/Wasm builds were intentionally not started here; the root agent owns the shared build queue and was sent a source-stable checkpoint.

Two initial focused Nx attempts used targets/configurations that excluded the neutral suite and returned “No test files found”; the exhaustive renderer-react target above is the applicable passing oracle.
