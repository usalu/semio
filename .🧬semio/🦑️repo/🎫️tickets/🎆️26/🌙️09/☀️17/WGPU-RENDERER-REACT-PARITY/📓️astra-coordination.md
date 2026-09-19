# Wgpu React Parity: Current Coordination

## Scope and Acceptance

Continue the existing renderer parity ticket against the current React implementation. Acceptance requires matching shell and window geometry, window lifecycle and docking interactions, element rendering and input, scene rendering and actions, host integration, accessible names and keyboard interaction, and current native/browser build and runtime evidence. Earlier reports describe implemented work; they are not evidence that the current checkout passes.

## Repository Workflow

- Read `repo://goals` through the configured repo MCP stdio entry on 2026-09-19.
- Existing ticket `26/09/17/WGPU-RENDERER-REACT-PARITY` covers this task. MCP `ticket_reopen` returned `ticket is already open`; continue it without creating a duplicate.
- Preserve concurrent edits and avoid all modifying Git commands and worktrees.
- Generated evidence belongs under this ticket's `🗑️generated` directory. Retain Markdown audit and summary reports.

## Fleet

The current session permits four concurrent agents including coordination. Start with two GPT-5.6 Terra Extra High read-only audits and one GPT-5.6 Sol Extra High verification/execution worker. Replace completed audit slots with GPT-5.6 Sol Extra High implementation workers; use fresh Terra audit waves to review completed changes. Keep all available slots occupied with independent useful work.

## Work Order

1. Reconcile the previous fifteen implementation waves with current source and executable tests.
2. Repair current build/test failures and obtain fresh browser baselines.
3. Implement concrete surviving parity gaps in bounded, independently testable slices.
4. Compare React and wgpu window/chrome interaction journeys and visual geometry at the same viewport and state.
5. Re-audit, rerun affected gates, and close only when acceptance is substantiated.

## Initial State

The existing status report ends during Wave 15, with several integration/runtime checks unfinished. Both prior puzzle3d comparison ports require fresh health checks; port 6213 is not serving. Many unrelated changes are staged and unstaged in the shared workspace. They remain outside this task's edits.

## First Audit and Execution Wave

- Terra layout audit: retained Select still bypasses shared scrolling and foreground composition; overlay Image raster instances bypass foreground routing; React ContainerView overrides Overlay positioning. See `📓️astra-terra-layout-audit.md`.
- Terra window audit: reopening an empty dock fails; closed World3d state remains an input authority; Display templates lack targetable dock drag. See `📓️astra-terra-windows-audit.md`.
- Sol workers now own Select/overlay raster repairs, window lifecycle/Display drag repairs, and current build/test baselines. All three execution slots are occupied.
- Current React quick suite: worker reports 5/5 passing. Native suites are in progress; no current native pass claimed.

## Runtime Baseline

Started comparison listeners through `bun nx run @semio-tech/framework-os-dev:serve-puzzle3d-{react,wgpu}-dev --excludeTaskDependencies` at 6313 and 6213. These consume existing artifacts, so screenshots from this first pass are a baseline only, not verification of the current source.

In-app browser on 6213 reaches a painted puzzle3d shell, plan underlay, 3D geometry, and the introduction. The accessibility mirror is present but empty in this existing build. Console errors/warnings were empty in the inspected browser logs. A fresh renderer build is required before attributing missing accessibility or visual behavior to the current implementation.

The first ad-hoc Nx probe invocation was rejected by an unrelated project-graph cycle. The focused replacement selects only the `workspace` project so the probe runs once.

## Layout Contract Decision

The owned contract and independent generic renderer agree that Overlay is a positioning context with inset padding in normal flow; Absolute is out of flow. The React style projection and wgpu flow mapping currently contradict that contract in different ways. Sol baseline now owns a paired correction and a shared fixture with asymmetric inset, an inner fixed leaf, a following flow sibling, and an absolute sibling. ContainerView must stop overwriting other layout kinds' declared positioning. This follows the existing schema semantics, with no compatibility branch.

## Geometry Baseline for the Next Audit

At a 1280 × 720 browser viewport, the inspected React DOM reports Artifact `[3.1875,3.203125,76.203125,22.390625]`, Catalogue `[79.390625,3.203125,90.078125,22.390625]`, example trigger `[546,3.1875,192,22.390625]`, role group `[747.578125,3.1875,185.328125,22.390625]`, and the Top cap wrapper `[3.1875,31.984375,110.796875,28.765625]`. Body text is 12.8 px and chrome text is 11.2 px. The existing wgpu snapshot visibly has a sequential title cluster and smaller cap band. Current source still advances navbar `cursor.x` directly from leading tabs into the logo, and `Dock::render_stack` uses `theme.control_height` for the cap. A fresh geometry audit should verify the intended centered-cluster and cap metrics against these browser measurements after the current fixes land.

## Verification Queue

Native UI compilation is waiting for the shared Cargo build lock while other workspace jobs compile; it has not failed. Work continues on source and neutral fixtures. Parent owns fresh React activation and the browser-worker TypeScript check, then the final wgpu build/activation. Do not count the invalid first probe as a parity regression or a passing gate.

The initial `check-browser-worker` gate failed its artifact-freshness check: `🚀️boot.js is missing or stale; run the generate-browser-boot target`. It did not reach a source type-check failure. The owning `generate-browser-boot` and `generate-frame-worker` targets are running before the gate is repeated. This is direct evidence that the initial browser was consuming outdated generated glue as well as an older renderer binary.
