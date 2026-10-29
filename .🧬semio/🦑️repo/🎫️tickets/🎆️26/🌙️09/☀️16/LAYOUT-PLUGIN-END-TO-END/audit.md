# Layout End-to-End Static Audit

2026-10-28. Read-only source audit; no source changed. Read root, s, and layout AGENTS plus build skill. Ticket lifecycle handled by coordinator.

## Current Integration

- Native composition: `🌎️hub/🧩️compositions/📏️layout/🦀️.rs` builds closed editor/viewer fleet, binds drawing codec, registers document declaration, mutation rosters, text fonts, write capability and artifact activation. Owner test: `repo run @semio-tech/layout-plugin:test`.
- Browser component is a distinct crate: current s generated catalog selects `🌎️hub/🧩️compositions/📏️layout/🧩️component/📦️packages/🦀️rust`, package `semio-hub-layout-component`, WASM `semio_hub_layout_component.wasm`.
- Explicit current catalog: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`. It contains layout in plugins, sources, playgrounds and Rust catalog. Layout app is `s.layout.layout@1/*#editor`, React port 6079, wgpu 6179, examples demo and demo-session.
- Older framework registry `dist/sessions/layout` instead names parent crate `semio-hub-layout` and old WASM `semio_hub_layout.wasm`. Its session bridge URL remains `/🔌️plugin-modules/📏️layout/🌉️bridge.js`. Do not use this stale session as evidence of current component readiness.
- Every exports-map file in layout artifact TypeScript package currently exists.

## Launch Contract

OS dev script owns `prepare`, `activate`, `serve` routes. `♻️activation/🏃️execution/🟦️.ts` accepts `activate layout react prototype` or release and computes `@semio-tech/framework-os-dev:activate-layout-react-prototype`. These variant targets are absent from authored project JSON; graph admission must establish whether they remain generated. Older dev profile targets are obsolete.

`♻️activation/🌐️serve/🟦️.ts` accepts `serve layout react prototype [server options]`, requires matching activation receipt and catalog variant, and fixes the React port to 6079. Script entry accepts explicit `--catalog <path>` before route dispatch. Direct owner entry is `repo 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts --catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated serve layout react prototype` after staging. Actual graph target admission and browser launch are coordinator work; not verified here.

## Applicable Verification

- `@semio-tech/layout-layout-rs`: build, check, test, test-media-adoption-source, verify-layout-window-ownership, verify-layout-document-contract, verify-layout-frame-selection, test-snapshot-sqlite[-source|-native], verify-physical-codecs.
- Rust package test router includes artifact, editor commands, two content mutation and two frame mutation test packages.
- Owned verify routes also cover drawing export, drawing segments, text flow, PDF streams, diff algebra and canonical JSON; several have no matching authored manifest target and require owner router dispatch.
- `@semio-tech/layout-js:test` only runs two example tests (demo-session and demo), not full editor, transport or export integration.
- Composition surface tests only prove viewer never mutates and editor/viewer dialect identity. They do not prove browser load, pointer commands, export download or reload persistence.
- LayoutPlayApp implements action argument bridge, document publication factory, window config/transient owners, reserved importer, retained command/export jobs and text font installation. Reserved importer currently accepts only fields:in; format imports advertised by app must travel through separate I/O converter routes.

## Blockers and Limits

1. Current versus stale catalog component selection must be kept explicit through staging and serve; stale parent WASM is not an acceptable browser proof.
2. Graph admission, component build and browser workflow remain unverified in this audit. Even `repo --service=off run @semio-tech/layout-js:test --dry-run` queued on global capacity (five running, nine ahead, negative memory spare above 4096 MB floor); no passing test result is claimed.
3. Export existence and action bridge code do not establish shell end-to-end operation. Coordinator should prove open demo, edit page/frame, undo/redo, export SVG/PNG/PDF and document reopen through the selected component.
4. No static missing exports-map files or absent layout catalog membership was found. Avoid adding compatibility aliases for the stale parent-component path.
