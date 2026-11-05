# Shooting React Host Integration

Date: 2026-11-04

The current generated playground catalog pins shooting to `s.shooting.shooting@1/*#editor`, the React port 6019, and the mesh collection catalog. The current owner command is `repo run @semio-tech/framework-os-dev:serve-shooting-react-prototype --lang=en --output=stream`. The historical ticket shell scripts still use retired dev profiles and raw Bun/Nx, so they have not been used.

The browser export effect previously awaited every render in the shell effects pass, swallowed failures into the console, and offered no progress or cancellation. The new image export host module folds job events into stable active rows, runs each image in order, yields before expensive work, reports progress through Task Manager, and routes cancellation through the same event log. Cancellation during a pending image render ends the task immediately and suppresses the eventual download. Completed downloads remain available. Failures stop the batch and show a visible notice. English and German notices are explicit.

The shell starts the export asynchronously so its action/effect loop remains available for cancellation. Task Manager maps image export rows onto its existing media export lane and cancel control.

Validation is in progress. The owner serve command is waiting for machine capacity behind concurrent repository work; no server currently answers port 6019. Neutral fixture and independent oracle tests are being authored by the exploration agent and registered by the coordinator.

Changed source owners:
- `🖼️IconRenderHost/📤️export/🟦️.ts`
- `🏛️ShellHost/⚡️host-effects/📤️export-effects/🟦️.ts`
- `🏛️ShellHost/⚡️host-effects/🟦️.ts`
- `🧵️TaskManager/🟦️.tsx`
- `🏛️ShellHost/🎨️theme-driver-settings/🛍️settings-and-marketplace-hosts/🟦️.tsx`

All paths above are under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

## Asset Transform Preservation

The browser icon request now carries the same optional world instance transform as the scene: position, xyzw quaternion rotation, and scale. The browser applies it to the outer GLB frame before fitting and rendering, while the inner GLB axis conversion remains intact. This preserves gumball translations, rotations, and nonuniform or negative scales in preview and download.

Neutral point and bounds cases are in `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🖼️icon-transform/🔣️.json`. Three independently projects each point and transforms the box bounds in the Bun test `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️icon-transform/🟦️.ts`. The native agent consumes the same fixture.

## Current Verification

- TypeScript parsed all five edited export host files without syntax diagnostics.
- First owner serve attempt: admitted after approximately eight minutes of machine-capacity queueing, then refused before graph admission with `Rust cache envelope disagrees with repository total budget` from repository storage budget enforcement.
- Second exact owner serve attempt: admitted promptly, then refused at the same budget check.
- Browser proof has not run because the server could not start. The retained portable `browser-export-probe.mjs` will verify readiness and download both example formats, decode PNG/SVG through Sharp, and assert dimensions and visible pixels.
- No cache receipt, storage partition, or infrastructure policy has been altered by this agent.

## Export Job Contract Tests

The neutral fixture and XState oracle suite are now authored at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧪️export/🟦️.ts` and its colocated JSON fixture. The suite covers successful, cancelled and failed event folds, invalid transition preservation, stable snapshots, ordered image downloads and progress, render and delivery failures, cancellation before work, cancellation during a pending render, late download suppression, cancellation before the admitted render microtask starts, throwing subscribers and notices, and explicit English/German labels.

The retained browser probe now asserts the base example's exact 256×256 PNG/SVG dimensions, nonblank decoded pixel variation, SVG geometry, Catalogue mutation from two to three shots, undo back to two, and Viewer canvas boot after export.

A third exact owner serve invocation was started after the storage allocator owner updated its policy; it is awaiting machine capacity. No browser success has been claimed.

## Continued Acceptance Work

The third serve attempt passed capacity admission after roughly thirteen and a half minutes and progressed beyond the earlier storage-budget refusal into graph input-closure verification. The owner command is still running; no browser readiness has been claimed.

The browser probe now downloads the real current document through the app's Save Download command after create and undo, parses that authoritative snapshot, and checks shot counts plus original ids. It also verifies all viewer surface fault fields and checks the current export failure notice case-insensitively after viewer startup. Pixel evidence compares RGB only among fully opaque pixels, excluding alpha-edge variation, and requires a visible nonwhite mesh region.

The cancelled-render path now omits its internal cancellation exception from the localized terminal notice. The existing pending-render test runs in German and checks the exact German notice and cancellation outcome.

## Executed TypeScript Acceptance

The registered Bun suite completed with **30 passing tests across 8 files, exit 0**, as verified by the TypeScript agent in `typescript-verification.md`. This includes all 11 image-export lifecycle tests (including German pending cancellation and cancellation before the render microtask) and all 4 Three-backed transform tests. No TypeScript source repairs were required after executing the suite.

The declared serve command remains active in global graph verification; browser acceptance has not run yet.

## Repeated Global Graph Invalidation

The same third serve process completed all 2,024 input-closure checks, prepared all 1,037 stale projects, then completed another full 2,024-closure verification. It subsequently reported:

> graph: refreshing 1037 projects (generation 885)
>
> graph: 639 stale facts (typescriptRealpath 534, file 61, analysis 30, readdir 12, digest 1, exists 1) leave 1037 projects stale

The listed observations included newly added unrelated test files, changed procedural/stdio Cargo manifests, repository storage/source-access changes, shell changes, and earlier shooting styling request changes. The process returned to cargo-owner discovery. No shooting serve target or browser startup had executed at this point.

The process remains under normal repository controls; no graph receipt, source generation, cache partition, or peer task was altered.

## Sequenced Serve Cancellation

At the coordinator's instruction, only the third pending serve process was cancelled through normal SIGINT to release its accumulated graph memory and allow the existing native acceptance work to proceed. It exited **130** and reported:

> Cancelled (SIGINT); leases released.

The last phase was still global project inference (655 of 1,029 after the repeated graph invalidation). No application target or server had executed. Peer jobs, dependency providers, graph receipts and cache state were untouched. The plan remains to complete native acceptance, then restart the same declared serve route under fresh normal provenance and run the retained browser probe.

## Repository Browser Preflight

The raw Playwright import initially checked its user-global browser location and reported no executable. The repository's first-party `loadPlaywrightV1` adapter sets the declared shared browser home before loading Playwright. Under that adapter the matching Chromium 1248 executable is present.

Executed the owned preflight:

`repo tools verify chromium --lang=en`

Result: **exit 0**, pin **1248**, found **1248**, status **ok**. Its owner hook verifies executable access and host requirements for both Chromium and the matching headless shell. No browser installer, dependency synchronization, provider edit, or cache policy/partition change was needed.

The retained acceptance probe now imports the repository Playwright loader and shared `browserLaunchOptionsV1` policy instead of choosing its own environment or platform flags. The provisioning owner, if ever needed, is the tool table's `chromium` row delegating to the controlled native-dependencies `sync browsers` route, not a raw installer. Serve remains stopped until native acceptance completes.

## Current-Guest Behavioral Check

The retained browser probe now requires Add Shot's real document readback to persist exactly one additional saved-camera row. The new shot must carry a nonempty `cameraId` referencing that new row, absent from the initial document. It records the camera identity and counts, then still requires full undo readback to equal the complete initial document. This catches the older pre-capture guest behavior. The updated probe passed syntax validation; actual browser execution remains pending.

The separate activation-freshness owner corrected the warm-marker shortcut and reports 57 focused tests passing in `activation-freshness.md`. Activation now checks current module/support and owner-source identities before receipt reuse. Its combined artifact identity and owner-source aggregate do not replace the separate current producer/source-to-guest evidence still required for runtime acceptance.

## Current Batch Host-Yield Contract

Bounded current-source check while25982queues: actual applyIconRenderExportEffect supplies yieldToHost as setTimeout(resolve,0), a host task boundary. runIconExportBatch awaits it before every item, checks cancellation before render and again before download, and records progress after delivery. Thus the current production adapter avoids an all-microtask multi-image loop; no concrete missing inter-image host yield was found and no source edit was warranted. Pending-render cancellation/late-delivery handling remains covered by the actual prior focused suite. This source check does not prove real rendered Task Manager interactions or application browser acceptance.
