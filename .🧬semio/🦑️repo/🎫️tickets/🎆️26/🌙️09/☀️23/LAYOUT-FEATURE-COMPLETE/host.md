# Layout Host and Acceptance

Layout runs as plugin `semio:layout` in semio OS Dev. The component crate is `semio-hub-layout-component`. Its committed descriptor names plugin `layout` version `0.1.0`, editor `s.layout.layout@1/*#editor`, and the viewer. Cargo metadata supplies the Layout playground and browser ports. The WGPU composition uses the generated Specific OS catalog. Its prototype browser URL is `http://127.0.0.1:6179/?plugin=layout&example=demo`.

## Build and Browser Verification

Run from the repository root. The inferred preparation target pulls in the session catalog, browser support, fonts, WGPU Wasm and boot scripts, and the selected guest component. Activation publishes the staged runtime beneath `.cache/b/p/m/browser/prototype/runtime/wgpu/layout`. Serve runs the declared Vite host and depends on activation.

```text
repo run @semio-tech/framework-os-dev:activate-layout-wgpu-prototype --output=stream
repo run @semio-tech/framework-os-dev:serve-layout-wgpu-prototype --output=stream
```

Run the retained journey in ticket `26/09/16/LAYOUT-PLUGIN-END-TO-END/browser-journey.mjs` through `repo tools exec bun -- <journey-path>` once the server is listening. It exercises real WGPU accessibility and keyboard interaction, captures the canvas, stages a rect at (251, 337) on the 400 × 500 demo page, checks exact add/undo/redo, edits text, independently inspects SVG, PNG and PDF downloads, and verifies an archive reopen on a new owner after diverging from the saved original. Scope beyond that journey includes process restart and any newly implemented source import.

## Current Execution

The initial canonical activation reached registry admission and exited on the PDF editor owner. A second canonical run is active. The dedicated WASI build reached the Draw snapshot model and is awaiting the shared repair described below. The activation receipt is still pending.

The older ticket recorded a WGPU activation that reached the native descriptor producer and failed on five OS DSL compiler callers.

## Demo Route Regression

The retained browser journey previously requested `example=🎬️demo`. The Layout example source registers the stable ID `demo`; the host query passes IDs to the dialect registry without stripping a folder prefix. The retained neutral contract in `host-launch-cases.json` and the TypeScript/jsdom check in `host-launch-check.mjs` reproduced `actual: 🎬️demo, expected: demo`. The check passed after correcting the journey default URL. This restored the requested route to the shipped demo; runtime coverage remains the real browser journey.

## Managed Browser Preflight

Playwright 1.64.1 has Chromium 156.0.8078.4 installed in the repository tool cache. On native macOS arm64, `host-browser-preflight.mjs` served a secure loopback page and attempted adapter and device acquisition through actual browser APIs. Headless Shell timed out obtaining an adapter using both the journey flags and those flags plus Metal. Managed full Chromium in headless mode obtained both an adapter and a device with the original journey flags. The retained journey now selects Playwright channel `chromium`. The route and channel source check passed after that change. This preflight validates browser readiness; the Layout host still has to boot and complete the real journey.

Reproduce the isolated browser preflight with the repo-managed browser cache configured as `PLAYWRIGHT_BROWSERS_PATH` and `repo tools exec --light node -- <this-ticket>/host-browser-preflight.mjs '' chromium`.

## Cold Artifact Verification

The setup schema names the browser pin `chromium`. To provision it on a fresh checkout, use `repo tools setup toolchains chromium`. The normal activation target ensures its own build tool closure. The activation receipt expected at `.cache/b/p/m/browser/prototype/runtime/wgpu/layout/activation/🔣️receipt.json` must include `layout` and its staged artifact hash. The dev owner implements this cold-check operation. Its command source requires the original task invocation envelope:

```text
repo "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts" cold-boot-check layout wgpu prototype --catalog "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
```

The current WGPU source harness passes three structure cases with jsdom, ten rendered-color cases with colord, and four delayed publication cases with jsdom. These checks exercise the acceptance observer and prepare the actual browser run.

## Direct WASI Component Gate

Run the component check through the repository Cargo ingress while the full activation plans its graph.

```text
repo cargo --store=shared --configuration=prototype -- check --manifest-path "🌎️hub/🧩️compositions/📏️layout/🧩️component/📦️packages/🦀️rust/Cargo.toml" --target wasm32-wasip2 --lib
```

The target is installed. Session 82302 reached Source admission unit 804 of 937 and exited 1 before rustc. The ordinary `semio-s-artifact-stdio-pdf-tests-2` crate exceeded 4,000 active lines with 4,064. PDF declares this crate in normal dependencies and reexports its production 1.7 base IO, binary/deflate readers, editor page, and PDF/A editor window. The crate also has separately gated VT/H tests. Its name therefore does not indicate accidental dev-dependency selection. Root owns the production companion split. The activation command continues planning independently.

## Additional Live Interaction Gates

The existing journey covers 13 checks from boot through archive restoration. A gumball check also needs a physical WGPU pointer gesture after activating Transform on an actual selected frame. The renderer trace can correlate the pointer drag to the same frame’s streamed and committed translation. The Layout inspection controls and subsequent undo/redo must independently show the geometry change. The placed-link check requires mounting a Layout document with a ready source image, dragging its actual catalogue row, then verifying that the new frame refers to the persisted source and that output uses the linked image. The source-only and browser-GPU checks do not cover either interaction.

The PDF page-coordinate owner split let the second WASI component check pass Source admission and reach actual Rust compilation. That run adds an explicit package selector and uses one compiler job. The build still has to finish and the activation target has to stage the component.

The retained browser journey accepts `--physical-interactions` to add the live pointer gates. Run it after serving:

```text
repo tools exec bun -- ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/browser-journey.mjs" --physical-interactions --output-ticket ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE"
```

The gumball step finds the painted horizontal handle in the live viewport, sends a trusted pointer drag, requires a stream and commit addressed to the same selected frame, and verifies the inspector plus undo and redo. The placement step uses the registered `linked-asset` example, drags `layout-catalogue.link.chart-preview` through its WGPU transfer handle, verifies one frame was added, and confirms the exported SVG now paints the same PNG twice. The route checks and sharp-rasterized neutral handle coordinate cases pass. The live gestures still await the served Layout host.

## Current WASI Build

The second WASI check exited 101 while compiling `semio-plugin-tool-machine`. Rustc reported missing `NativeOwnedStorage` and `NativeSeedTemplate` re-exports in `semio_framework_value::native_decoding`. The current shared source exposes both names after a peer update. A fresh production build started in session 30199 using the same manifest, package, platform and one-job prototype selection with `build` in place of `check`. It will produce the actual guest Wasm if compilation succeeds. The canonical activation session is still in graph loading, and both Layout ports refused connections at the last probe.

The optional journey also performs physical resize, rotation, and Escape cancellation on the same frame. It reads all five geometry fields from the WGPU Inspector, checks the target and terminal gesture, and requires unchanged geometry after cancellation. Three JSON-driven geometry cases agree with the independent gl-matrix calculations. The route, rasterized handle, browser output paths, and all three browser scripts passed source checks. Resize, rotation, cancellation, movement, and the placed link still require the running host.

## Activation Registry Retry

The first canonical WGPU activation completed its cold owner checks and then returned BUILD_OWNER_REGISTRY_INCONSISTENT, service reference 828d9c98. Its first Original admission failure required the PDF 1.7 editor/document native owner. The loader followed that row once and still refused to admit it. The current registry has the exact existing document Cargo owner and the new page-coordinates Cargo owner. Canonical activation was reissued against the live registry while the renderer and guest continued through their own repository lanes.

## Shared Draw Dependency

The production guest advanced through the earlier re-export issue and into the Draw snapshot model. Its SQLite native encoder and decoder used the prior direct-return contract, while the active host trait receives a caller-owned output slot and returns the work disposition. The Draw snapshot host now forwards both the slot and original owner to the store’s native record helpers. Its relational image sample constructor uses the synchronous page reservation after the original checkpoint and exact-byte charge. The language-neutral Draw SQLite suite passed seven cases and 52 assertions against Bun SQLite, including retained source values, row ordering, cancellation, and independent edits. The shared guest rerun will provide Rust integration evidence.

The independent browser bridge and font packer are queued through their scoped targets with the original task envelope. The renderer’s raw Cargo prewarm has a real grant and is compiling wasm-bindgen.

The shared model’s retained gradient preparation had two obsolete allocation calls. Draw’s PathRaster owns one allocation marker per admitted retained step and passes it through both its physical allocations and nested gradient preparation. The neutral gradient suite passed ten cases with independent SVG rasterization. The neutral painted-path suite passed 28 cases with 972 assertions, including gradients, affine and crop behavior, cancellation, and ownership cleanup. Revised Rust fixture assertions match the slot against the actual capacity receipt on each gradient turn. The fresh production WASI build is queued behind admitted compiler work.

The extended journey starts with actual frame hover. It locates the restored frame’s pivot, clears selection during setup, then sends real browser pointer moves into and out of the frame. The renderer must log both native pointer moves with no pointer-down event during measurement. The selected-frame area must paint and then return to baseline while the Artifact panel confirms selection stays empty. The independent sharp raster checks confirm that the detector ignores changes outside the crop and recognizes a visible stroke and its removal.

## Retained Command Integration

The Layout mutation preparation now forwards original effects and their physical allocation slots to document validation and equality, frame and page override decisions, dictionary equality, and batch transform preparation. The batch path preserves sparse decision output, partial diagnostics, exact inverse events, and funded retirement. Edit Story now uses its production story-splice decision in the aggregate. The portable central check reported zero strict diagnostics across 403 source files and passed 46 portable, 45 golden, 500 generated, and nine refusal cases. The pinned Rust formatter parsed the changed decision and batch files without rewriting them. Native compilation and the real browser journey remain pending.

The first raw WGPU renderer build compiled hundreds of dependency units before native tool-machine and job receiving source changes invalidated the running snapshot. The current tool-machine manifest declares its native storage owner, and Job Receiving now uses the matching work-state variant. The unadmitted full guest and renderer retries were cancelled before rustc to remove their larger memory requests from the shared queue. A focused `wasm32-wasip2` check of the Layout mutation crate is queued under a supported 2,304 MB inherited capacity grant. The canonical activation and renderer staging requests remain active.

The focused mutation check entered the real Cargo admission and stopped on Layout diff at 275,949 of 262,144 bytes before rustc. After moving page cache and final-list code to their plan owner, a diff-only rerun reached 265,526 bytes at the new rejection frontier. The physical owner split reduced the observed bytes and still left admission over its configured limit. Pinned Rust syntax and browser fixture checks remain green. The live host is still waiting for native source admission and the graph that stages its browser products.

The concrete Layout diff owner later passed all 616 Source admission units after moving the generic retained collection and borrowed row view to the existing plan crate. The move removes 17,030 bytes from the concrete owner and keeps its typed delta integration tests in place. The diff-only Wasm check entered actual Rust compilation. A follow-up mutation check is queued under the same supported memory grant. These are source and compiler milestones. The staged browser still needs the full guest and renderer.

## Latest Compiler Evidence

The diff-only WASI Cargo job completed successfully after admitting all 616 source units. The focused Layout mutation check also passed source admission and compiled the actual Layout model, diff, and decisions owners. Its five Rust errors were missing references to the admitted demand inside the simple diff output. That owner has since restored the exact demand used for native receipts. The integrated fresh Layout guest is checking the actual semio-hub-layout-component package under a 2,304 MB inherited machine grant and is waiting for compiler image headroom.

The aggregate Layout mutation tests now drive the current original Job decoder and receiving effect with fixture-owned base, event and post snapshots. The fixture consumes the retained receiving receipt on success or refusal, drives the edit through capacity-backed retirement, and closes the decoder with the original Job caller. Pinned Rust syntax and whitespace checks passed. Native test execution remains pending.

The current raw WGPU compiler prewarm has entered Rust code generation using the actual wasm32-unknown-unknown renderer Cargo package. The canonical renderer target remains responsible for the wasm-bindgen wrapper, publication and receipts. Canonical activation, renderer staging, and the independent browser asset targets still wait in the repository graph. The served Layout product and its browser acceptance measurements remain unverified.

The real renderer Cargo build exposed a wasm32 arithmetic overflow in WGPU box-state identity. It packed two u32 fields into a usize and shifted by 32. The resolver now compares native ordered node identifiers, keeping both their arena index and generation and quoting the selected index type's actual allocation. All 14 neutral cases passed in Vitest against the existing lodash oracle, including two IDs with identical low 32 bits. The renderer was reissued against the warmed store. In parallel, the real Layout WASI component advanced from source sealing into Rust checking.

The ordered node IDs passed the actual wasm32 WGPU events compile. The next downstream check reported stale retained retirement and camera APIs in the renderer engine registry. Prepared canvas and Flow retirement now enter the original caller’s physical and work grants and preserve the Scene and Flow terminal witnesses. Tutorial graph and board coordinates now use their live viewport, graph and scene owners. The raw renderer retry is running. The integrated guest check stopped at semio-os-store-read during a concurrent rollout of ArtifactGenesisPack. A source audit found the current VCS owner, its explicit module export and current store callers agree.

The next real renderer compile advanced into the asset decode crate and stopped on newer mesh, PNG and InteractiveJob interfaces. Mesh backing and pages now use the real retained grant and post their receipts. Abort and lease retirement use their original demand and progress, and temporary authority contention yields. PNG passes the active physical allocation effect through decoding and retirement. The raw build was reissued with those current owners. The current guest check has passed the earlier VCS and Store-read failure before moving into OS plugin dependencies.

The integrated guest then compiled Draw, its Store/VCS dependencies, and the Layout model, diff and decisions. Rust reported a dangling doc comment in the Layout mutations library after native test relocation. The comment was removed and the live root parsed cleanly with the pinned formatter. A fresh 2,304 MiB component check is queued behind older work because current admitted renderer and graph memory prevent the normal full-link grant. The renderer continues code generation through its authentic wasm32 build. The product activation receipt and live port remain absent.
The next integrated guest request stopped before rustc at the Layout mutations crate’s physical source cap (270,079 of 262,144 bytes while reading the 24th file). Story output and inverse owners were already generic over the mutation interface and had a valid lower-crate link. Their two source files now live under that lower fields owner, transferring 24,642 bytes while keeping the runtime’s logical module exports. Both manifests still use the same crate dependency. All four touched Rust sources parse with the pinned formatter. The fresh WASI check is queued behind existing work.

The repository’s live source parser counted 23 files, 258,274 bytes and 2,047 active lines in the WASI mutations closure. The lower fields closure counted 15 files, 175,990 bytes and 2,008 active lines. These are measurements of checked-out source while the authoritative component check waits for admission.

The two renderer frame jobs now use the current preclaimed InteractiveJob return through the shared callback adapter. Their native fixture callers still exercise the same funded bodies. All seven portable frame outcome cases passed against Ajv and an independent Bun SQLite oracle. The running raw renderer then exposed a shared source timing skew in the DSL receiving traits. Cargo built the traits at 190 seconds and the derive at 830 seconds, across a shared update. The current borrowed and owned traits both declare RECEIVES_STATEMENT, and the borrowed trait declares receive_statement. A fresh raw renderer run is needed to compile the matching sources together.

The authoritative guest run admitted the smaller Layout mutation crate and entered rustc. It later saw a mixed-generation receiving API in the OS store; the live receiving owner had gained refusal after rustc had already checked that dependency. A fresh component check was issued, preserving the real component manifest and WASI preview2 target. The renderer itself detected a different concurrent generation change, refreshed its snapshot and retained its compiler grant before rebuilding.

The refreshed guest passed the exact semio-os-store-one-item-batch receiving seam and continued through the OS kernel. The independent renderer build has admitted and resumed. Browser boot and owner activation still wait for current facts and completed artifacts. The live browser journey and port 6179 remain pending.
