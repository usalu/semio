# Remaining Build Contracts

The historical artifact-check run after native target inference reported eleven missing output contracts: four report builds, Demonstrator build, Stdio build-wasm-release, Hub build, WGPU wasm, OS Dev build, Print build, and Print build-viz.

Initial inspection of the report/print commands confirms dynamic document and output path selection, font provisioning inside compilation, and custom source watchers. Report outputs default to each document directory/dist; arbitrary second arguments and SEMIO_PRINT_OUTPUT_DIR can redirect writes. Per-document Nx targets need fixed output ownership and separate font preparation before caching these operations. The generic default report build currently duplicates the Zwischenbericht producer and needs an explicit aggregate/default selection boundary.

Demonstrator build/dev calls buildPlugins, buildEngineWasm and registry preparation internally, then starts Vite. E2E starts its own raw script server and can reuse an existing port. It needs the same explicit session/support/component/engine preparation and single server ownership model as the React playground work, including a concrete deterministic build deliverable. No output declaration has been added over this hidden pipeline.

These were initial inspection findings; the checkpoints below and linked compiler/report notes record later changes. Demonstrator remains pending.

## Print Font Prerequisite — 2026-09-08

The three canonical print fonts are tracked TTF assets. The previous helper downloaded from floating main URLs when files were absent and copied to an unowned global cache using only file size. It now validates authored TTF sources and atomically stages `@semio-tech/print:fonts` deliverables into the Print package's `dist/fonts`. No source assets are downloaded or rewritten. A minimal script owns this Nx target, and Print/Report build and long-test targets declare font preparation explicitly. Hidden font preparation was removed from Report compilation and Print long-test implementations.

The language-neutral catalog is schema validated, and native `@napi-rs/canvas` successfully loads each staged font. The test verifies byte identity, stale-file removal and preservation after invalid source bytes. The red test failed on the absent staging API; the green probe passed. Permanent Print quick tests now include the same coverage. The actual Nx producer and restore checks are pending.

Current Bericht implementation builds all three reports when no selection is given, superseding the earlier duplicate-default observation. Its per-document ownership and the Print compiler/tool provisioning refactor remain unfinished.

The real font target completed in 229 ms of Nx task-run duration (100 ms producer critical path); an identical invocation reported 1/1 cache hits. Deleting the owned font deliverable tree and rerunning Nx restored every byte from cache, and the native font loader opened all three restored TTFs. `@semio-tech/print:test-quick` passed in 4.7 seconds, including permanent font tests. The registry generator regenerated `.vscode/launch.json` from the updated seed in 33.4 seconds; the Print font command is present. These are task-run durations and exclude earlier shared-graph waiting.

## PDF ownership checkpoint — 2026-09-08

Print now exposes 87 catalog-owned PDF leaves and aggregate-only template/gallery targets. Pinned compiler, pinned minimal TeX support bundle and tracked font producers are separate Nx prerequisites. Actual Print PDF byte repeatability, Nx clean-output restoration and independent PDF.js text consumption passed for viz-api before the latest generic publisher extraction. The full 87-document collection remains running (34 leaves had published at this checkpoint). See `📓️print-compiler.md`; final-source warm restoration remains pending.

Report now exposes three catalog-owned PDF leaves, an aggregate verifier, Nx-coordinated watches and a separate four-fragment generator contract. Its configuration and root watch vectors pass, and existing fragment bytes passed the renderer check. Full Nx runtime/cache/editor qualifications are running. See `📓️report-documents.md`. The remaining non-PDF build owners listed above still need their hidden pipelines refactored.
