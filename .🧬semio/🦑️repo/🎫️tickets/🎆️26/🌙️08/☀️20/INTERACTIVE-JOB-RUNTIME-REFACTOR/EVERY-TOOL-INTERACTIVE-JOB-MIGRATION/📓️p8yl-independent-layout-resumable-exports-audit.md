# P8yl Independent Layout Export Audit

## Verdict

**REJECT** — the registered action route is materially improved, but public Wasm/UI-visible export calls still synchronously batch-drive the jobs and legal max-size terminal work remains unbounded.

## Evidence Accepted

- The app registers LayoutExportJobFactory through the compiler-bound ArtifactToolFactoryRegistry<EditorApp<LayoutPlayApp>>; the factory declares the four exact tool IDs, exact payload schema, document schema, concrete owner, and a resumable one-unit/2 ms contract.
- LayoutPlayApp::build_tool_job maps only ExportPng, ExportSvg, ExportPdf, and ExportPackage, checks the tool ID against the command, and copies the framework operation/authority material into LayoutExportRequest.
- Each four public command handler fails closed with layout-export-job-only; engagement redispatches the exact four public action names. Direct image, png, zip, and base64 Cargo rows are absent from Layout's Cargo manifest, and no legacy library API identity occurs in Layout Rust source.
- The job has cancellable, generation-checked Validate/Plan/Encode/Base64/Complete state, a bounded checkpoint envelope, deterministic replay-to-checkpoint verification, and explicit per-step base64 bounds.
- Read-only gates run here: rustfmt --edition 2021 --check on the export implementation and git diff --check -- Layout; both passed. No Cargo, native, Wasm, runtime-dispatch, or timing test was run because the shared disk budget reserves Cargo.

## Blocking Defects

1. **Public Wasm exports violate the governing rule.** LayoutSession::export_png, export_svg, export_pdf, and export_package directly call the synchronous helpers in ✏️editor/🌉️wasm/🦀️component.rs (233–258). Those helpers call run_layout_export_batch, which invokes run_to_completion (export component 1123–1145). export_media("layout:out") likewise calls the synchronous SVG helper (editor component 250–264). These are externally callable UI/Wasm routes with no progress, cancellation, checkpoint ownership, or 8 ms bound. Same-job batch adapters are allowed only for batch entry points, not reachable interactive APIs.

2. **The claimed max/cap boundedness is incomplete.** Validation only limits pages/stories/links and page frames (421–480). It does not bound parent pages/parent frames, spreads, paragraph styles, character styles, or other JSON-valued snapshot portions subsequently serialized by Package. Parent-frame lookup itself performs an unbounded parent_pages.iter().find, and planned parent frames escape MAX_LAYOUT_EXPORT_TOTAL_FRAMES. Therefore the 5,184 decoded-item contract cannot be proved.

3. **Several terminal units exceed the 8 ms ceiling at legal sizes.** begin_pdf builds all page object references in one allocation (548–557); PDF finalization clones and writes every xref offset in one step (569–584); completion flattens the entire output and copies it into the commit in one step (920–936). A legal multi-megabyte artifact makes the latter plainly non-bounded. The wrapper then decodes the complete commit and constructs the download effect in the same InteractiveJob::step. A post-hoc context.should_yield() cannot bound any of these indivisible units.

4. **Package input compatibility regressed/unproved.** export_package_zip ignores its supplied preflight_json argument (1171–1174), replacing it with an internal report. No differential test establishes that this changed output contract is intended.

5. **The schedule/cap tests are insufficient evidence.** The new test labelled 1/2/4/default only varies fuel_per_step in a single in-process batch driver (1187–1217); it does not exercise worker-pool counts 1/2/4/default or actual tool dispatch. It covers only dimension max+1, not every documented input/output/item cap. It was not executed in this audit.

## Required Repair

1. Replace the four Wasm/session and layout:out synchronous paths with operation submission/progress/completion handles; retain run_to_completion only under a clearly batch-only, non-UI boundary.
2. Schema-bound every traversed/serialized collection and value, including parent-page frames, styles, spreads, and arbitrary JSON, then make PDF header/xref and commit packaging cursor-resumable/chunked.
3. Preserve or deliberately replace preflight_json only with an explicit schema/contract change and parity test.
4. Add real factory-dispatch and worker-pool 1/2/4/default determinism/cancel/stale/max/max+1 tests, then run the serialized native/release/Wasm/timing gates when disk capacity permits.

## Residual Gates

All native/release/Wasm compilation, tool-host dispatch, downloader integration, runtime cancel/stale/replay behavior, 8 ms watchdog timing, adversarial max/max+1 envelopes, and actual 1/2/4/default worker schedules remain unrun.
