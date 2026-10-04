# Final Ownership And Command Audit — 2026-10-04

Read-only audit of the shared live tree at C:/git/semio. Only this retained Markdown was written. No Git mutation, goal/ticket lifecycle action, new permanent script, production edit, build or test execution occurred. Source inspection is not runtime certification.

## Concrete Findings To Coordinate

1. The new independent font-metrics check provider is currently unregistered: `rg -n printFontMetricsChecks 🧰️framework/🛍️products/📓️print -g '*.ts'` returns only its definition in `🔨️modules/🔤print-font-catalog/📏️metrics/🧪️tests/🟦️.ts:19`. The native compiler runner consumes the neutral vectors and measures native advances, but its current imports do not execute this provider's AJV, digest, Canvas and shaping checks. Wire the provider into the existing inference probes or existing metadata test owner before claiming independent Canvas verification. This is a concrete current-source execution gap, potentially already being resolved concurrently.
2. `.vscode/launch.json` contains grouped Rust build/test, font metrics generation, combined native grammar and families-only commands beside the existing print routes. `.vscode/🧩️launch.seed.jsonc` still has the older native grammar entry at the start of the configurations and does not contain the new metrics/Rust/families-only routes. Existing repository launch-composition tests explicitly consume both files. Keep the authored seed consistent with the grouped current registrations to preserve regeneration. No failure of launch regeneration was executed here.

## Canonical Ownership

Inspected product root is `🧰️framework/🛍️products/📓️print` (all subsequent product-relative paths resolve there). Root `AGENTS.md` and `🧰️framework/🛍️products/AGENTS.md` were read; no closer product AGENTS file exists in the enumerated framework tree.

`🧬️schema/🧬️mutations/🟦️.ts` owns semantic change events, guarded diff production and inverse changes. It imports the canonical snapshot/diff facets. `🧬️schema/💡️inferences/🟦️.ts` owns async worker creation, cancellation, progress and atomic result publication. Abort removes listeners and terminates the worker; pre-aborted input returns a cancellation diagnostic. The worker plans, emits TikZ, obtains numerical scene output when applicable, validates the result, and transfers an owned JSON result. Catalogue presets explicitly return complete TikZ with a scene-unavailable diagnostic rather than fabricating a portable numerical scene.

`🧬️schema/💡️inferences/📦️packages/🟦️typescript/🟦️.ts` reexports the canonical snapshot, diff, mutation and inference facets plus the colocated numerical helper implementations. No second implementation owner or retired module facade was found. `🦀️.rs` publishes native snapshot/diff/mutation/inference facets and registers the native service through the existing OS inference registry. `📦️packages/🦀️rust/Cargo.toml` points to `../../🦀️.rs`, uses only existing first-party runtime crates and has serde_json solely as a dev dependency.

`rg -n viz-kernel` across product TS/JSON/Rust/Markdown finds only the historical oracle label `viz-kernel-twin-parity` in `🔮️oracles/🔣️.json:1115`. No production source import, export, or package route targets the retired `🔨️modules/📊️viz-kernel` path.

## Closed Contracts And Dependencies

`🧬️schema/💡️inferences/✅️validation/🟦️.ts` validates authored ChartSpecification using `🧬️schema/🔣️.json` and first-party `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts`. Result validation consumes `🧬️schema/💡️inferences/🔣️.json`. The root result object closes extra properties; its success/failure alternatives distinguish nonempty successful TikZ from empty failed TikZ with nonempty diagnostics. The nested plan, scene and diagnostic definitions visibly close structured objects. Legitimate data and option maps retain schema-governed map entries. Worker publication invokes the result validator before completing.

Inference package `package.json` has D3 references only in devDependencies and no runtime dependency collection. Main print package retains existing canvas/pdfjs/sharp dependency boundaries for print/testing infrastructure; none appear in the inspected production worker static closure. Numerical production types in inspected inference owners use repository-owned schema, scene and geometry types.

An inline read-only PowerShell traversal of literal relative import/export references from the inference worker plus the pure font metrics entry found 25 TS files with no missing relative imports and no bare external imports. It intentionally follows type-only imports too, therefore it is a conservative superset of static runtime imports. Worker adapters retain the deliberate guarded variable dynamic import of system `node:worker_threads`; browser workers choose the global endpoint first. This is a source import audit, not a browser bundle/runtime test.

The closure consists of canonical inference worker/root, validation, catalogue, render, mark, scale, format, coordinate, transform, shape, theme, hierarchy, network, flow, geo and spatial leaves; canonical snapshot/chart leaves; first-party schema validator, 2d/text, styling mixing and pure print paints resolution; and pure font metrics. Exact additional framework paths inspected are `🧰️framework/🔨️modules/◻️2d/🟦️.ts`, its `📝️text/🟦️.ts`, `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🟦️.ts`, and the schema validator above.

`🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` imports only its source-bound `🔣️.json` and exports first-party metric contracts and plain-text shaping/measurement. It has no filesystem, native drawing or third-party import. The filesystem font-catalog parent imports this pure leaf for digest admission; the pure leaf does not import that parent. At the inspected revision, the numerical renderer has not yet imported measurePrintSans; final integration must preserve this direction. Neutral font vectors and independent Canvas/AJV test provider exist beside the pure leaf, with native compiler measurements imported in `🧪️tests/🧬️native-chart-grammar/🟦️.ts`.

## Command Registration

Inspected `📦️packages/🟦️typescript/📋️project.json`, `package.json`, `📜️script.ts`; inference equivalents; native `📋️project.json`, `Cargo.toml`; font-catalog `📜️script.ts`; `.vscode/launch.json`; and `.vscode/🧩️launch.seed.jsonc`.

Relevant Nx targets invoke existing `📜️script.ts` routers. The main print native-grammar target forwards all arguments, permitting the grouped `--families-only` launch route. Its router dispatches to the existing containment test owner or full native compiler owner. Metrics generation routes from package script through Nx to the existing font-catalog script's `metrics` command. Native build/test invoke the native owner script. Inference quick/long/exhaustive/test/build targets invoke the colocated script; the launch generic project selectors include `@semio-tech/print-viz-inference`. The inference build checks both barrel/worker and native runner/helper strict types before counting exports. No additional standalone renderer or permanent script was introduced by this audit.

## Evidence, Revisions And Limits

Read ticket `📓️source-inventory-2026-10-04.md`, `📓️inference-relocation-2026-10-03.md`, `📓️axis-customization-and-defaults-audit-2026-10-04.md`, `📓️verification-matrix-2026-10-04.md`, and `📓️native-final-verification-2026-10-04.md`. Their prior runtime results are evidence reported by execution owners, not tests rerun by this audit. Current combined native/family and catalogue gates remain the root coordinator's completion requirements after source edits.

SHA256 at inspection:

- `🧬️schema/💡️inferences/🖼️render/🟦️.ts`: `FB2C50522D2B5BBD5A195411194AF66FFB50C354583313884FD76FA347385AA2`.
- `🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts`: `67AD55E644164012E75B2D63164EF38CF967A03CC6AD04ECA71EE76514A26B0A`.
- `.vscode/launch.json`: `2E620C94DEC1C4B3BF286106B6920759919F628099CB2BB859EC3F8BA63296A9`.

No runtime passing claim is made here. Shared numerical legend/native compiler sources are actively changing, so these hashes and the two findings describe the observed revision only. Final root verification must include the integrated metrics provider and revised numerical renderer.

## Concurrent Legend Metadata Follow-Up

A subsequent read confirmed frozen typed `VIZ_LEGEND_DEFAULTS` at `🧬️schema/📸️snapshot/📊️chart/🟦️.ts:12`, sourced directly from `ChartGuide[x-semio-guide-defaults].legend` in the canonical input schema. This adds repository-owned metadata rather than new primitive admission or an external runtime type. Renderer consumption is still being integrated by its owner; this follow-up does not certify its runtime geometry.

The launch seed changed concurrently after the initial snapshot: it now contains Rust build/test entries at lines5/6 with their earlier presentation positions. Metrics generation and families-only still have no match in that seed at this follow-up. Thus finding2 now specifically concerns those absent routes and seed grouping, while Rust route presence has been resolved concurrently.

## Coordinator Launch Seed Resolution

The seed now contains all five task-owned Rust build/test, native grammar, family-only and font-metrics routes with the same command/name/presentation as the grouped actual launch file. A fresh independent JSONC-parser read verified exact equality of those five configurations between both files; unrelated seed configurations were compared before/after and preserved. The native font provider registration remains owned by the active numerical legend lane until its final combined replay.

## Second Closure Pass — Resolved Findings And Publisher Cancellation

Reread the live inference probes, numerical renderer, launch seed and generic publisher after the root coordinator's follow-up.

Both initial integration findings are resolved in current source. `🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts:2` imports `printFontMetricsChecks`; its check composition at line1601 includes those checks under module fonts. Root reports actual execution of all 33 font checks in the latest quick baseline (591 checks, 569 pass, 22 legend failures); this audit independently confirmed wiring but did not rerun that inference target. `.vscode/🧩️launch.seed.jsonc` now contains all five grouped routes for metrics generation, native Rust build/test, combined native grammar and families-only; the earlier seed findings are superseded. Root's independent JSONC comparison is reported evidence, not rerun here.

The active numerical renderer imports `measurePrintSans` directly from `🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` and imports `VIZ_LEGEND_DEFAULTS` from the canonical snapshot/chart facade. It does not import the native font-catalog parent, Canvas test provider, filesystem or third-party font code. This preserves the pure browser-worker import direction.

### Demonstrated Publisher Defect

Inspected generic owner `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts`. Its new identity helper reads pairs of 64 KiB chunks asynchronously, holds the existing exclusive publication lease, and closes both file handles through nested finally blocks. Directory inventory rejects unlisted entries and symbolic links, compares owner/version/sorted file inventory, and checks destination file size/mode before byte comparison. No additional concrete resource or inventory failure was demonstrated in this pass.

An actual focused runtime probe invoked production `stageArtifacts` twice with the same owned empty artifact, then instrumented the real Node FileHandle prototype read method to abort the supplied signal when its awaited EOF read resolved. Before a post-await cancellation check, stageArtifacts fulfilled despite the abort. Actual console output was:

```json
{"fired":true,"aborted":true,"outcome":"fulfilled"}
```

Probe command: `bun <ticket>/🗑️generated/publisher-eof-cancel-probe.ts`; exit code 0. The probe used the production publisher and real resource leases/file handles, not a copy of its implementation. It restored FileHandle.read in finally and removed its own data directory. Its authored input is retained under ticket/generated for root cleanup. The observed defect requires checking cancellation after awaited reads and before accepting byte identity. Root was immediately informed and owns the fix/replay.

Cancellation can also arrive during awaited file-handle closes before the identity result is returned to stageArtifacts. A final signal check at the caller immediately after awaited identity comparison covers the publication reuse decision as a whole; this latter timing was source-inferred, not separately probed.

No native/catalogue build was run or restarted. The actual quick baseline's 22 legend failures remain owner work, and this audit does not certify numerical legend geometry. The publisher cancellation finding is the sole new demonstrated failure in this pass.


## Third Closure Pass — Font Propagation And Cancellation Resolution

The proven EOF cancellation defect is resolved in the inspected publisher. `haveIdenticalBytes` now checks signal immediately after both awaited reads; `stageArtifacts` checks it again after awaited identity comparison and handle closure, before returning on identical artifacts. Replayed the same actual FileHandle EOF-abort probe against current production code:

```json
{"fired":true,"aborted":true,"outcome":"rejected: Error: audit EOF abort"}
```

Command: `bun <ticket>/📜️publisher-audit/📜️script.ts`; exit code 0. The authored probe was moved from generated into this retained ticket input folder. Its generated data stays under ticket/🗑️generated and removes itself after the run. The before/after runtime evidence establishes cancellation rejection at the previously demonstrated failing boundary. No remaining concrete cancellation, handle leak, inventory or symlink failure was demonstrated in this focused audit. Root reports the registered EOF fixture/schema/test and independent Python unchanged-byte oracle passing; those additional targets were not rerun here.

Current font propagation is explicit: numerical render text alone carries optional repository-owned `font?: string`, text scene conversion retains it, and the 2D Canvas text branch uses `entry.node.font ?? sans-serif` in its font declaration. TikZ text resolves tracked Anta/Share Tech Mono/Noto Emoji families back to existing native selectors and otherwise emits the explicitly authored family using fontspec. Pure `📏️metrics/🟦️.ts` now imports its metrics JSON and the tracked font-catalog JSON only; `printFontFamily`/`printFontTexSelector` retain first-party string interfaces and do not import native registration or Canvas.

The closed inference JSON schema admits nonempty string font only in the text plan and text scene alternatives (inspected occurrences around lines 602 and 1099); non-text alternatives remain closed. The existing first-party 2D text schema admits the same optional nonempty font. The neutral result fixture contains tracked/custom-family success cases, empty/numeric font rejection, and rejection of font on rectangle plan/scene variants. Their existing check provider compares owned validation to independent AJV. This is admission/source inspection, not a fresh contract target run.

Main print package now sets `nx.includedScripts` to an empty array while retaining its package script `generate-font-metrics: nx run @semio-tech/print:generate-font-metrics`. The authored project target remains `nx:run-commands` with cwd `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript` and exact command `bun ../../🔨️modules/🔤print-font-catalog/📜️script.ts metrics`. Thus the visible command route delegates to the owned generator rather than inferring its wrapper as the Nx implementation. Root's fresh selected-target check and strict 250-export result are reported execution evidence, not rerun by this audit.

Latest SHA256:

- Generic publication owner: `4A90B596BE3674C603C0B2A091F9F937284BFF9173E89F4F070573895DC29907`.
- Numerical renderer: `6CEA8A832D1C50DC04229183394844509D63833A1C752C7F53E94B340C013FA6`.
- Closed inference schema: `9CCC58433DDDDE5EB7B4300E5541844F7BAEF318815CF65A8CE052872ED9B576`.

All earlier findings in this report are now explicitly superseded by the resolved follow-ups. No production source was changed in any audit pass. Final native and catalogue runtime evidence remains with the active execution owners; this pass did not start either build.
