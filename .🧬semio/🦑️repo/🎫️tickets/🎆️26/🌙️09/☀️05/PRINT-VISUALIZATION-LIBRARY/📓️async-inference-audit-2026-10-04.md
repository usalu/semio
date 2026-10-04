# Asynchronous Chart Inference Audit

The canonical TypeScript `inferVizChart` now returns a promise and performs chart validation, numerical layout, TikZ emission, scene construction and result admission inside an owned worker. The synchronous pure planners remain the numerical probe surface; no second mathematical implementation or compatibility API was introduced.

The caller owns the worker, signal listener and message/error listeners. Cancellation immediately settles an incomplete result with empty TikZ and no plan or scene, terminates the worker and removes the listeners. Already aborted requests create no worker and report no progress. Progress callbacks remain on the caller's event loop, and aborting at completed progress prevents publication. Worker progress is monotonic with a fixed total; duplicate completed counts are suppressed.

Worker dispatch uses the native Web Worker interface in Bun/browser hosts and the system `node:worker_threads` interface when Web Worker is absent. The worker URL remains inside the inference taxonomy. Public API types use only owned chart/scene types and platform `AbortSignal`.

The existing print paint module now owns a pure resolution leaf shared by browser inference and the LaTeX stylesheet generator. The pure leaf statically imports the canonical styling JSON and first-party color mixing. Its generator parent retains filesystem output. Existing token-derived palettes remain a single implementation. The closed output schema exposed an existing font bug: CSS font stack strings were cast to arrays. Theme inference now derives ordered family arrays from those authored strings.

## Test-Driven Evidence

The new language-neutral feature scenarios were authored before implementation. The original synchronous inference failed the independently scheduled timer check: the timer did not run before the 20000-row inference completed. At that stage the registered Nx mutation check reported 10 passed and 1 failed. No claim of cancellation success relied on aborting synchronously inside a progress callback.

After implementing owned workers and correcting the font representation, registered Nx exhaustive inference checks passed 402/402 with zero mismatches and errors. This included the independent timer cancellation, early cancellation, AJV result contract and D3 numerical placement checks. Registered Nx strict inference build also passed, checking both the public barrel and the new worker leaf; the barrel exported 248 symbols.

Additional checks exercise independent timer cancellation after layout begins, strictly increasing completion progress, deterministic repeated inference, cancellation at completed progress and an observer throwing during progress. They are registered in the existing differential harness and described by language-neutral feature scenarios.

## Final Runtime Verification

`bun x nx run '@semio-tech/print-viz-inference:test-exhaustive'` passed **408/408**, with 16 mutation checks and 49 render checks; there were zero mismatches and zero errors. This replay includes the concurrently authored physical text-size contract. The real Chromium worker check compiles first-party browser bundles in memory, serves their worker entry from loopback and runs both successful inference and cancellation from an independent browser timer. Its successful point coordinates agree with `d3-scale`.

The final registered strict build, `bun x nx run '@semio-tech/print-viz-inference:build'`, also passed after the completed lifecycle changes and exported 248 symbols. It ran for 35.9 seconds and checked both the public surface and worker entry.

Actual Node **v24.14.1** execution of the bundled canonical entry and bundled inference worker also succeeded. The runtime output was `[DEBUG] {"valid":true,"points":[[0,10],[80,30]],"started":true,"fired":true,"cancelled":true}`. This verifies the system `worker_threads` path and independently scheduled cancellation after layout starts. Node was tested using built entries; raw TypeScript sources containing JSON imports are consumed through Bun or a bundler. The generated Node entries and temporary output log are inside the ticket's `🗑️generated/node-worker-runtime` directory for ticket cleanup.

The browser constructor uses the literal `new Worker(new URL("./🧵️worker/🟦️.ts", import.meta.url), { type: "module" })` pattern so browser bundlers can discover the worker entry. The TypeScript strict build checks the worker as an explicit entry in the existing `📜️script.ts` command.

Nx's Windows isolated plugin startup failed once before dispatch. Subsequent verification disabled plugin isolation and reused the ticket's previously validated source-independent Nx graph. No Git state was changed.

## Owned Files

- `print/🧬️schema/💡️inferences/🟦️.ts`
- `print/🧬️schema/💡️inferences/🧵️worker/🟦️.ts`
- `print/🧬️schema/💡️inferences/🎨theme/🟦️.ts`
- `print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts`
- `print/🔨️modules/🎨print-design-token-paints/🟦️.ts`
- `print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts`
- `print/🧪️tests/🧬️chart-mutations/🟦️.ts`
- `print/🧪️tests/🧬️chart-mutations/🥒️.feature`
- `print/README.md`

## Current Font And Legend Worker Replay

Actual replay completed at 2026-10-04T03:08:14Z on Windows x64 against the frozen final renderer, pure font metrics/catalog owner, worker and closed text-font result schema. This section supersedes the earlier host evidence for those current sources. No production source was changed during this replay.

The registered command `bun x nx run '@semio-tech/print-viz-inference:test' --args='quick mutation'` exited 0: **16/16 passed**, zero failures and errors, 11.7 seconds, 0/1 cache hits. It used `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, `NX_FORCE_REUSE_CACHED_GRAPH=true` and `NX_WORKSPACE_DATA_DIRECTORY=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY/🗑️generated/nx-font-metadata-final`. Actual terminal session 82893 completed successfully. This registered suite includes Bun worker cancellation during layout, early cancellation, strictly increasing completion progress, deterministic publication, completion-time abort, observer failure, browser worker/timer execution, owned schema/AJV admission and the independent D3 placement oracle.

The retained authored audit input is `📜️async-runtime/📜️script.ts`, invoked from repository root with `bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY/📜️async-runtime/📜️script.ts'`. Actual final terminal session 4366 exited 0. The helper emits every bundle, JSON record and terminal log into `🗑️generated/async-runtime-final` or the generated root. The unrelated publisher helper and the old generated native source-mutation helper were not executed. The only retained files added/updated in this final replay are the authored helper and this audit.

| Host | Actual version | Runtime result |
| --- | --- | --- |
| Bun | 1.4.2 | PASS |
| Chromium | 151.0.7922.34 | PASS |
| Node | v24.14.1 | PASS |

All three hosts executed the same canonical inference API. Bun consumed the TypeScript entry directly; Chromium loaded first-party browser bundles and their literal worker URL over loopback; Node loaded first-party built entries and exercised `node:worker_threads`. Browser and Node bundles were freshly generated with Bun, including the current pure font owner. No external runtime package was included by the worker implementation.

Each successful publication reported completed counts `[0,1,2,3,6,7,8,9]` with fixed total 9, strictly increasing counts, completed total, and deterministic repeated full results. For the 50000-row workload, each host reported layout progress **129/50007** before an independent same-thread `setTimeout(...,0)` callback fired and aborted the signal. All returned `complete:false`, empty TikZ, no plan, no scene and `print.chart.cancelled`. Already aborted requests reported zero progress and the same atomic incomplete shape. Node exited normally after worker teardown.

The mixed axis/legend figure published `Palette → Share Tech Mono`, `A → Anta` and `B → Anta` in both render-plan text items and drawing-scene text nodes. The emitted TikZ contained `\\SemioMono` and `\\SemioSans`. The worker's native structured-clone transport and final JSON projection retained all three font values. Read-only source inspection confirmed the text variant alone admits optional `font:string` with `minLength:1` on both render items and scene nodes, the worker calls `validateVizChartInference` before publication, and `renderVizScenePlan` forwards the actual font rather than substituting another family.

For every host, the owned validator and independent AJV 2020 validator accepted successful, early-cancelled and timer-cancelled results. Ten additional font-admission variants per host (30 total) agreed between both validators: omitted and arbitrary nonempty custom family names accepted on plan/scene text, empty and numeric font values rejected on text, and font properties rejected on nontext plan/scene nodes. Thus the published canonical scene passes the complete closed output admission, including typography.

Two audit-input defects were corrected before the final replay: the first hand-authored probe used a string title instead of the required localized map; the first Node probe inherited `--input-type=module` into its file worker. The final helper uses localized en/de title maps and `node -e` with an async invocation. These failures were confined to the audit launcher/fixture; no production workaround or compatibility API was introduced. Final host evidence is from the corrected terminal run.

### Exact Source And Bundle Digests

SHA-256 values below were measured by the successful helper and retained here before generated artifact cleanup. Source paths are relative to the repository's `🧰️framework/🛍️products/📓️print` owner.

| Source | SHA-256 |
| --- | --- |
| `🧬️schema/💡️inferences/🟦️.ts` | `edb3dfe3639b08a14ef87d750e3aee277a6dd08abb647f91708a177224620dd9` |
| `🧬️schema/💡️inferences/🧵️worker/🟦️.ts` | `66d0167f473285d75e45aa214ccb1c3086521e8c62a089f6c5cf93bdad75ea6e` |
| `🧬️schema/💡️inferences/✅️validation/🟦️.ts` | `75bbdfac96b518a69a0e7c096c0afc96718760ab44c17853d24f4a86fdeaa86b` |
| `🧬️schema/💡️inferences/🔣️.json` | `9ccc58433dddde5eb7b4300e5541844f7baef318815cf65a8ce052872ed9b576` |
| `🧬️schema/💡️inferences/🖼️render/🟦️.ts` | `6cea8a832d1c50dc04229183394844509d63833a1c752c7f53e94b340c013fa6` |
| `🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` | `c2de06b5dbe7b1d3cc6e7434f501f7d8739639c3cffcbfe5ef079f825e5494a5` |
| `🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json` | `68829670a0124ebf1053816bef7a05bf65684f71f33f28b31e6f4690affc12eb` |
| `🔨️modules/🔤print-font-catalog/🔣️.json` | `b63eb4a1255760f5b19a02298da33c34bf14c21a4135db78eb344b02839ab451` |

| Build output | Bytes | SHA-256 |
| --- | ---: | --- |
| browser/main.mjs | 1549407 | `fbec4e2c3091f71cdf332d53e83bb1271eef79301a3f02722f548b93683981f2` |
| browser/🧵️worker/🟦️.ts (bundled JavaScript) | 2830626 | `d1d5957b44fc7a6f68d4d5fc90ecd5d183598c5284f218f32ce7bb7a0e6d4967` |
| node/main.mjs | 1549407 | `fbec4e2c3091f71cdf332d53e83bb1271eef79301a3f02722f548b93683981f2` |
| node/🧵️worker/🟦️.ts (bundled JavaScript) | 2830626 | `d1d5957b44fc7a6f68d4d5fc90ecd5d183598c5284f218f32ce7bb7a0e6d4967` |


## Current Axis And Legend Closure Replay At 10:57 UTC

This section supersedes the earlier three-host evidence for the current axis and legend typography. The retained authored helper was executed after the root axis and native legend TypeScript owners released their production sources. Its corrected current execution exited 0 in 7.46 seconds at 2026-10-04T10:57:19.653Z, using the actual canonical entry in Bun and freshly built canonical entry/worker bundles in Chromium and Node. The terminal log is ticket-generated `async-runtime-current-terminal.log`.

The helper now fingerprints the complete first-party runtime import closure with Bun's import scanner, seeding the separate worker and closed result schema explicitly. Both initial and final fingerprints are identical over 29 owned files: `da16450fca82704810ea29f53c61267fced7018fc5a7b33f519555c70f0e3d77`. Thus no first-party runtime source changed during the complete three-host replay. No production source was edited by this replay.

| Host | Version | Current Runtime Result |
| --- | --- | --- |
| Bun | 1.4.2 | PASS |
| Chromium | 151.0.7922.34 | PASS |
| Node | v24.14.1 | PASS |

All hosts published deterministic full results with strictly increasing progress [0,1,2,3,6,7,8,9] and fixed total 9. Each independently scheduled timer fired after layout progress 129/50007 and returned the atomic cancelled shape: complete false, empty TikZ, no plan or scene, owned cancellation diagnostic. Already aborted requests produced zero progress and the same atomic incomplete shape. Node exited normally after worker teardown.

The current canonical output contains nine font-bearing texts in both plan and scene: ticks 0, 0.5, 1, 1.5 and 2 plus legend labels A/B use owned Anta; titles Axis/Palette use owned Share Tech Mono. Their structured-clone values exactly agree across plan and scene, and emitted TikZ includes SemioSans and SemioMono selectors. Owned and independent AJV 2020 validators accept complete, early-cancelled and timer-cancelled results in each host. All 30 further font-admission variants agree with the closed result schema.

The first current audit execution (session 76748) exposed two defects in the audit input rather than production: its old font assertion expected only three texts although current axes now correctly carry five tick fonts and a title font, and a global text replacement accidentally inserted the source fingerprint function into browser and Node launcher code. The corrected input checks all nine current owned font values and keeps fingerprinting in the outer audit process. The final green is from that corrected input. No production workaround was introduced.

### Current Complete Owned Closure

Paths below are relative to the repository root. Hashes are actual SHA-256 values read before and after the successful run.

| Source | SHA-256 |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🟦️.ts` | `400624f3790f23cd71a69e13a2746eacc13007e0a661a7025e0eb85269e9dc79` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` | `09e8b5afb79a554b01429a7ab71d7a84b61195b9ca1738f1c0bdac1dde66aaca` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` | `2617c03029c26a0cb97b3cf7fab1fa159b03c3f821d163ba55a55902c76170f3` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` | `b3f26600887b2de5b5fd755d989294ba84ab760e70b9e636d35260f0a30f46dd` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json` | `68829670a0124ebf1053816bef7a05bf65684f71f33f28b31e6f4690affc12eb` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` | `c2de06b5dbe7b1d3cc6e7434f501f7d8739639c3cffcbfe5ef079f825e5494a5` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🔣️.json` | `b63eb4a1255760f5b19a02298da33c34bf14c21a4135db78eb344b02839ab451` |
| `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` | `8bb0500e563bfdb3a6245800cb17c5f588c7c44f63ea83cc2b64feb9f6fc9669` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✅️validation/🟦️.ts` | `75bbdfac96b518a69a0e7c096c0afc96718760ab44c17853d24f4a86fdeaa86b` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✒️mark/🟦️.ts` | `b50edba869c3f21d35d58a854fc9601dc3972229229307341255f5242a7f748a` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌊flow/🟦️.ts` | `a0f20875427dbe943a80f74aa439652efbb56f62a81ea296dea60cc4466c4b01` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌍geo/🟦️.ts` | `9d4d9deb711813fa49f0446ad1d8fdc84bf0b40ffbf5c133c3990ebf37abaf0c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌳hierarchy/🟦️.ts` | `4c84cea6ee7f39110c071f32be8d883b9103c8cb54cab975c42968e9f99a08f8` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🟦️.ts` | `a83969e069d4f49c3d96d441a884981a77245ffdeda26be909f30dd6e7829f21` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📍spatial/🟦️.ts` | `58adbb55e17d190f0d2092c320c3d102d776f3b9a8b1dfc5fad778359b642edf` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📐scale/🟦️.ts` | `25837eba287f63342521ae4123f79c609641e9edc925f37091e78b60f74a1477` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📚️catalogue/🟦️.ts` | `df531636e322fb46a7a85eab4a6e8df8ff8899bd000456b0878097779bd26b13` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔢format/🟦️.ts` | `6c6e25d5d6899030907782fc7245dbf7386fbf2997276bdaeb5dac1b54101d0c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔣️.json` | `9ccc58433dddde5eb7b4300e5541844f7baef318815cf65a8ce052872ed9b576` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🕸️network/🟦️.ts` | `db0f836e4794eac93fb4465bd2c0001337889cf5348f2f369da0b14bdcf27668` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts` | `3ae49ceb6604bd265879155615aeca1bfb54fa92a5e60bee643e90440266ecad` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts` | `edb3dfe3639b08a14ef87d750e3aee277a6dd08abb647f91708a177224620dd9` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🥧shape/🟦️.ts` | `d854c5267131fd6b41fffbf300097d87549aa02a250c172f5a8e75bed9363d1c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧭coordinate/🟦️.ts` | `a71d09747c422927db8b9d529a34a406a0599449887cb9c3409774562bd9e449` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧮transform/🟦️.ts` | `631cd829b2b0f1cb4c0f00089d972008cb407d1153a6f5a283f9bab1864c41d9` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧵️worker/🟦️.ts` | `66d0167f473285d75e45aa214ccb1c3086521e8c62a089f6c5cf93bdad75ea6e` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🎨️color/🔣️.json` | `def3436d7f6ee7265d9e21dc44bcd9ddd22b18aa36485d2171133afda608b0b0` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts` | `93a24de87649f8b36caa6854cb38ec27fb8c034afe88b31a42348d30d6bf1f05` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json` | `12708b66ed5b56e63703310f26cd3adbef436b047b022742901b17c5433b1096` |

| Current Build Output | Bytes | SHA-256 |
| --- | ---: | --- |
| `browser/main.mjs` | 1552141 | `bfcc17868404bfe6525d3ad901fde02f42b8dc43b8d1c9619e84ce3a4fb41e91` |
| `browser/🧵️worker/🟦️.ts` | 2833936 | `5cceaaa2dbebe1bf350e9e48a10f7724fe79aa8580090047fc4d65f0170a69f9` |
| `node/main.mjs` | 1552141 | `bfcc17868404bfe6525d3ad901fde02f42b8dc43b8d1c9619e84ce3a4fb41e91` |
| `node/🧵️worker/🟦️.ts` | 2833936 | `5cceaaa2dbebe1bf350e9e48a10f7724fe79aa8580090047fc4d65f0170a69f9` |

## Final Released Stroke And Closed Theme Replay At 11:30 UTC

This final section supersedes the 10:57 UTC replay and its da16450f closure because root subsequently completed the schema-owned axis paint/width/opacity roles and the required closed theme chrome contract. The unchanged retained helper ran only after root released those production sources and reported its registered 174/174 differential gate green. The actual helper command exited 0 in 7.05 seconds at 2026-10-04T11:30:34.934Z. No implementation/runtime source was rewritten in this replay.

All three actual hosts passed: Bun 1.4.2, Chromium 151.0.7922.34 and Node v24.14.1. Each published deterministic results with progress completed counts [0,1,2,3,6,7,8,9] and fixed total 9; all counts strictly increase from zero to the completed total. For the 50000-row workload, layout progress 129/50007 preceded an independent timer firing and successful atomic cancellation. Already aborted requests emitted zero progress. Both cancellation paths returned complete false, empty TikZ, no plan, no scene and the owned cancellation diagnostic. Node exited normally after worker teardown.

The nine font-bearing plan and scene texts agree in every host: axis ticks 0/0.5/1/1.5/2 and legend labels A/B use Anta; titles Axis/Palette use Share Tech Mono. The TikZ output retains SemioSans/SemioMono selectors. Owned validation and independent AJV 2020 accept each complete, early-cancelled and timer-cancelled result, including the newly required theme chrome field. All ten private font-admission variants per host pass (30 in total).

Initial and final fingerprints of the complete 29-file first-party runtime closure are identical: `929794dfc1adea16993acd23d92ba688a10320ee355e5ecc3f99fb48aa7abada`. This is the final current closure fingerprint; no runtime source changed during its three-host proof. Both browser/Node main bundles are 1552818 bytes with SHA-256 `27f2a1630aea0649cb9523f4183ba94aa040e57e148beb9e46a03969c053dbc6`; both worker bundles are 2839975 bytes with SHA-256 `22c6e6ad4494688a197f4f07d122037c68c9b195bf4cfb4f096722a8e62ee0ec`.

### Final Released Complete Owned Closure

| Source From Repository Root | SHA-256 |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🟦️.ts` | `400624f3790f23cd71a69e13a2746eacc13007e0a661a7025e0eb85269e9dc79` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` | `09e8b5afb79a554b01429a7ab71d7a84b61195b9ca1738f1c0bdac1dde66aaca` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` | `2617c03029c26a0cb97b3cf7fab1fa159b03c3f821d163ba55a55902c76170f3` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` | `b3f26600887b2de5b5fd755d989294ba84ab760e70b9e636d35260f0a30f46dd` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json` | `68829670a0124ebf1053816bef7a05bf65684f71f33f28b31e6f4690affc12eb` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` | `c2de06b5dbe7b1d3cc6e7434f501f7d8739639c3cffcbfe5ef079f825e5494a5` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🔣️.json` | `b63eb4a1255760f5b19a02298da33c34bf14c21a4135db78eb344b02839ab451` |
| `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` | `8bb0500e563bfdb3a6245800cb17c5f588c7c44f63ea83cc2b64feb9f6fc9669` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✅️validation/🟦️.ts` | `75bbdfac96b518a69a0e7c096c0afc96718760ab44c17853d24f4a86fdeaa86b` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✒️mark/🟦️.ts` | `b50edba869c3f21d35d58a854fc9601dc3972229229307341255f5242a7f748a` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌊flow/🟦️.ts` | `a0f20875427dbe943a80f74aa439652efbb56f62a81ea296dea60cc4466c4b01` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌍geo/🟦️.ts` | `9d4d9deb711813fa49f0446ad1d8fdc84bf0b40ffbf5c133c3990ebf37abaf0c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌳hierarchy/🟦️.ts` | `4c84cea6ee7f39110c071f32be8d883b9103c8cb54cab975c42968e9f99a08f8` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🟦️.ts` | `7a65e22068cb4d9fae540d3584d9e280056f3c74c2a60c9ce9ddaa1ef8232d64` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📍spatial/🟦️.ts` | `58adbb55e17d190f0d2092c320c3d102d776f3b9a8b1dfc5fad778359b642edf` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📐scale/🟦️.ts` | `25837eba287f63342521ae4123f79c609641e9edc925f37091e78b60f74a1477` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📚️catalogue/🟦️.ts` | `df531636e322fb46a7a85eab4a6e8df8ff8899bd000456b0878097779bd26b13` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔢format/🟦️.ts` | `6c6e25d5d6899030907782fc7245dbf7386fbf2997276bdaeb5dac1b54101d0c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔣️.json` | `a1232c89dcf15c63c22962af544e2521196529dc6a03b151141fdda6a255d7d3` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🕸️network/🟦️.ts` | `db0f836e4794eac93fb4465bd2c0001337889cf5348f2f369da0b14bdcf27668` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts` | `c3f1a0091dfab59ff0088c61d2a5f1c85382492195f2f1364cba9e1d14297a0b` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts` | `edb3dfe3639b08a14ef87d750e3aee277a6dd08abb647f91708a177224620dd9` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🥧shape/🟦️.ts` | `d854c5267131fd6b41fffbf300097d87549aa02a250c172f5a8e75bed9363d1c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧭coordinate/🟦️.ts` | `a71d09747c422927db8b9d529a34a406a0599449887cb9c3409774562bd9e449` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧮transform/🟦️.ts` | `631cd829b2b0f1cb4c0f00089d972008cb407d1153a6f5a283f9bab1864c41d9` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧵️worker/🟦️.ts` | `66d0167f473285d75e45aa214ccb1c3086521e8c62a089f6c5cf93bdad75ea6e` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🎨️color/🔣️.json` | `def3436d7f6ee7265d9e21dc44bcd9ddd22b18aa36485d2171133afda608b0b0` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts` | `f16f91f8b5893e9ef05875cb67752a8fce9c3682057e533d6d4c05c17b1c7cb8` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json` | `43ae8c008bab069bd82fd56b043e14c222149af07443fc2fff285f5b10c83359` |

## Final Foreground And Shared Text Emitter Release Replay

Actual retained helper replay: `bun <ticket>/📜️async-runtime/📜️script.ts`, terminal session1613, exit0. Terminal output retained at generated/async-runtime-current-terminal.log. Evidence timestamp: 
04/10/2026 11:54:10
. Initial/final source closure hashes are equal, covering 
29
 first-party runtime files. No production or helper edits.

Initial SHA256: `
6b46d001d7ffd2b8f4991925e980222d6943a8736e3e8f1717a702c632c38694
`. Final SHA256: `
6b46d001d7ffd2b8f4991925e980222d6943a8736e3e8f1717a702c632c38694
`.

| Host | Version | Pass | Progress records | Result admissions | Font admissions |
|---|---|---|---|---|---|
| Bun | 1.4.2 | True | 8 | 3 | 10 |
| Chromium | 151.0.7922.34 | True | 8 | 3 | 10 |
| Node | v24.14.1 | True | 8 | 3 | 10 |

All three hosts actually verified complete deterministic result publication, strictly increasing bounded progress ending at total, pre-abort with zero progress and absent partial outputs, timer-triggered mid-worker abort and atomic cancellation diagnostic, nine font-bearing plan/scene text entries with matching families, and native Sans/Mono TikZ selectors. Owned validator and independent AJV admitted valid/pre-abort/cancelled outputs. Both plan and scene admitted absent/custom text font while rejecting empty/numeric font and font on non-text objects. Raw per-host progress/font/check values and individual source hashes are preserved in generated/async-runtime-final/evidence.json. This replay supersedes earlier closure hashes for current root foreground/shared-text changes.

### Current Release Source Hashes

| Source | SHA256 |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🟦️.ts` | `400624f3790f23cd71a69e13a2746eacc13007e0a661a7025e0eb85269e9dc79` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` | `09e8b5afb79a554b01429a7ab71d7a84b61195b9ca1738f1c0bdac1dde66aaca` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` | `2617c03029c26a0cb97b3cf7fab1fa159b03c3f821d163ba55a55902c76170f3` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` | `b3f26600887b2de5b5fd755d989294ba84ab760e70b9e636d35260f0a30f46dd` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json` | `68829670a0124ebf1053816bef7a05bf65684f71f33f28b31e6f4690affc12eb` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` | `c2de06b5dbe7b1d3cc6e7434f501f7d8739639c3cffcbfe5ef079f825e5494a5` |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🔣️.json` | `b63eb4a1255760f5b19a02298da33c34bf14c21a4135db78eb344b02839ab451` |
| `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` | `8bb0500e563bfdb3a6245800cb17c5f588c7c44f63ea83cc2b64feb9f6fc9669` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✅️validation/🟦️.ts` | `75bbdfac96b518a69a0e7c096c0afc96718760ab44c17853d24f4a86fdeaa86b` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✒️mark/🟦️.ts` | `b50edba869c3f21d35d58a854fc9601dc3972229229307341255f5242a7f748a` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌊flow/🟦️.ts` | `a0f20875427dbe943a80f74aa439652efbb56f62a81ea296dea60cc4466c4b01` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌍geo/🟦️.ts` | `9d4d9deb711813fa49f0446ad1d8fdc84bf0b40ffbf5c133c3990ebf37abaf0c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌳hierarchy/🟦️.ts` | `4c84cea6ee7f39110c071f32be8d883b9103c8cb54cab975c42968e9f99a08f8` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🟦️.ts` | `7a65e22068cb4d9fae540d3584d9e280056f3c74c2a60c9ce9ddaa1ef8232d64` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📍spatial/🟦️.ts` | `58adbb55e17d190f0d2092c320c3d102d776f3b9a8b1dfc5fad778359b642edf` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📐scale/🟦️.ts` | `25837eba287f63342521ae4123f79c609641e9edc925f37091e78b60f74a1477` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📚️catalogue/🟦️.ts` | `df531636e322fb46a7a85eab4a6e8df8ff8899bd000456b0878097779bd26b13` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔢format/🟦️.ts` | `6c6e25d5d6899030907782fc7245dbf7386fbf2997276bdaeb5dac1b54101d0c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔣️.json` | `a1232c89dcf15c63c22962af544e2521196529dc6a03b151141fdda6a255d7d3` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🕸️network/🟦️.ts` | `db0f836e4794eac93fb4465bd2c0001337889cf5348f2f369da0b14bdcf27668` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts` | `83cbca27822db8ecb72cf041a18d999be560122e8d19164934d8484b5b51dca4` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts` | `edb3dfe3639b08a14ef87d750e3aee277a6dd08abb647f91708a177224620dd9` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🥧shape/🟦️.ts` | `d854c5267131fd6b41fffbf300097d87549aa02a250c172f5a8e75bed9363d1c` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧭coordinate/🟦️.ts` | `a71d09747c422927db8b9d529a34a406a0599449887cb9c3409774562bd9e449` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧮transform/🟦️.ts` | `631cd829b2b0f1cb4c0f00089d972008cb407d1153a6f5a283f9bab1864c41d9` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧵️worker/🟦️.ts` | `66d0167f473285d75e45aa214ccb1c3086521e8c62a089f6c5cf93bdad75ea6e` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🎨️color/🔣️.json` | `def3436d7f6ee7265d9e21dc44bcd9ddd22b18aa36485d2171133afda608b0b0` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts` | `ab3977af7bc621d2f1f379d646dd6e115556a95e5b507fe51910ae050164b405` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json` | `0ff388db2cd4e8675b74e0397d98651033c211b2e918d9b1e0a0d1bfe799a610` |

### Exact Current Host Check Evidence

```json
[
  {
    "host": "Bun",
    "version": "1.4.2",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  },
  {
    "host": "Chromium",
    "version": "151.0.7922.34",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  },
  {
    "host": "Node",
    "version": "v24.14.1",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  }
]
```

## Metadata Source Supersession After Session1613

Fresh source comparison against session1613 stable29-file snapshot6b46d001… found following changed bytes. Prior run remains truthful at its timestamp but is not a final-current-source frozen closure. Native owner confirms main-schema deltas are family x-semio-family-options descriptors/controls, not ChartSpecification wire definitions. A fresh actual three-host replay is required after metadata release; no new replay started before freeze.

- `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\🔣️.json` — previous `0ff388db2cd4e8675b74e0397d98651033c211b2e918d9b1e0a0d1bfe799a610`, current `be21884678521e843d197f59c53badc0e56809f52c31d52a8f583e838785b8dd`.

## Current Family Metadata Release Three-Host Replay

Actual retained harness replay completed terminal0,2.8465s at 
04/10/2026 14:08:36
. Command: bun ticket/📜️async-runtime/📜️script.ts. Terminal retained generated/async-runtime-final-metadata-terminal.log. Fresh initial/final29-source closure hash is identical: `
a422e2dd0e52458648113b58f0cce81ed46a1dbabe96018132da214c9990b1fb
`. This candidate current metadata release supersedes session1613 for main-schema bytes; any later input/schema edit requires invalidation. Final catalogue completion is not claimed.

All Bun/Chromium/Node host records pass deterministic result, increasing8progress steps, preabort0progress/atomic absence, actual timermidworkerabort+atomic cancellation,9font-bearing texttransport, native fontselectors,3owned+AJVresultadmissions and10fontadmissions.

### Exact Host Evidence

```json
[
  {
    "host": "Bun",
    "version": "1.4.2",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  },
  {
    "host": "Chromium",
    "version": "151.0.7922.34",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  },
  {
    "host": "Node",
    "version": "v24.14.1",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  }
]
```

### Exact Current29Source Hashes

| Source | SHA256 |
|---|---|
| `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🌗️mixing\🟦️.ts` | 400624f3790f23cd71a69e13a2746eacc13007e0a661a7025e0eb85269e9dc79 |
| `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🔣️.json` | 09e8b5afb79a554b01429a7ab71d7a84b61195b9ca1738f1c0bdac1dde66aaca |
| `C:\git\semio\🧰️framework\🔨️modules\🧬️schema\✅️validator\🟦️.ts` | 2617c03029c26a0cb97b3cf7fab1fa159b03c3f821d163ba55a55902c76170f3 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🎨print-design-token-paints\🧮️resolution\🟦️.ts` | b3f26600887b2de5b5fd755d989294ba84ab760e70b9e636d35260f0a30f46dd |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\📏️metrics\🔣️.json` | 68829670a0124ebf1053816bef7a05bf65684f71f33f28b31e6f4690affc12eb |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\📏️metrics\🟦️.ts` | c2de06b5dbe7b1d3cc6e7434f501f7d8739639c3cffcbfe5ef079f825e5494a5 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\🔣️.json` | b63eb4a1255760f5b19a02298da33c34bf14c21a4135db78eb344b02839ab451 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🖼️assets\🔣️viz-catalog.json` | 8bb0500e563bfdb3a6245800cb17c5f588c7c44f63ea83cc2b64feb9f6fc9669 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\✅️validation\🟦️.ts` | 75bbdfac96b518a69a0e7c096c0afc96718760ab44c17853d24f4a86fdeaa86b |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\✒️mark\🟦️.ts` | b50edba869c3f21d35d58a854fc9601dc3972229229307341255f5242a7f748a |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌊flow\🟦️.ts` | a0f20875427dbe943a80f74aa439652efbb56f62a81ea296dea60cc4466c4b01 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌍geo\🟦️.ts` | 9d4d9deb711813fa49f0446ad1d8fdc84bf0b40ffbf5c133c3990ebf37abaf0c |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌳hierarchy\🟦️.ts` | 4c84cea6ee7f39110c071f32be8d883b9103c8cb54cab975c42968e9f99a08f8 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🎨theme\🟦️.ts` | 7a65e22068cb4d9fae540d3584d9e280056f3c74c2a60c9ce9ddaa1ef8232d64 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📍spatial\🟦️.ts` | 58adbb55e17d190f0d2092c320c3d102d776f3b9a8b1dfc5fad778359b642edf |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📐scale\🟦️.ts` | 25837eba287f63342521ae4123f79c609641e9edc925f37091e78b60f74a1477 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📚️catalogue\🟦️.ts` | df531636e322fb46a7a85eab4a6e8df8ff8899bd000456b0878097779bd26b13 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🔢format\🟦️.ts` | 6c6e25d5d6899030907782fc7245dbf7386fbf2997276bdaeb5dac1b54101d0c |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🔣️.json` | a1232c89dcf15c63c22962af544e2521196529dc6a03b151141fdda6a255d7d3 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🕸️network\🟦️.ts` | db0f836e4794eac93fb4465bd2c0001337889cf5348f2f369da0b14bdcf27668 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🖼️render\🟦️.ts` | 83cbca27822db8ecb72cf041a18d999be560122e8d19164934d8484b5b51dca4 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🟦️.ts` | edb3dfe3639b08a14ef87d750e3aee277a6dd08abb647f91708a177224620dd9 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🥧shape\🟦️.ts` | d854c5267131fd6b41fffbf300097d87549aa02a250c172f5a8e75bed9363d1c |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧭coordinate\🟦️.ts` | a71d09747c422927db8b9d529a34a406a0599449887cb9c3409774562bd9e449 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧮transform\🟦️.ts` | 631cd829b2b0f1cb4c0f00089d972008cb407d1153a6f5a283f9bab1864c41d9 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧵️worker\🟦️.ts` | 66d0167f473285d75e45aa214ccb1c3086521e8c62a089f6c5cf93bdad75ea6e |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\📸️snapshot\📊️chart\🎨️color\🔣️.json` | def3436d7f6ee7265d9e21dc44bcd9ddd22b18aa36485d2171133afda608b0b0 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\📸️snapshot\📊️chart\🟦️.ts` | ab3977af7bc621d2f1f379d646dd6e115556a95e5b507fe51910ae050164b405 |
| `C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\🔣️.json` | be21884678521e843d197f59c53badc0e56809f52c31d52a8f583e838785b8dd |
