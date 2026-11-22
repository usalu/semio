# WASM Producers Share the Retained Source Mechanism

## Finding

The descriptor job does not compile from a private copy. It compiles from the live tree, but captures every admitted Rust body in `CargoSourceWitnessV1` before Cargo starts, defers a drift event when that body is held, and afterwards accepts the artifact only if rustc's own consumed-byte checksums (dep-info `# checksum:blake3=`) equal the admitted bodies. The WASM path had only the first half of the old behavior: any edit of any admitted file cancelled the compile (exit 75 after 6 restarts).

## Design

1. `RetainedCargoSourcesV1` (new, `native-build/📥️source/📎️retained/🟦️.ts`) wraps the existing witness: `admit` captures the admitted roster, `defer` is the drift callback, `prove` verifies consumed bytes after the compile and returns `CargoSourceProvenanceV1`.
2. The two pieces both owners need are now one implementation: `captureAdmittedCargoSourcesV1` and `deferCargoSourceDriftV1`. The descriptor path (`native-build/🟦️.ts`) calls them instead of its former inline copies.
3. `buildWasmWebV1` admits, retains, passes `retained` to `runWasmCargoCommandV1` (an edit to a retained file is deferred, an unretained one still cancels), and calls `prove` on the dep-info under the attempt's Cargo target and build directories before the package is consumed.
4. Provenance (`sha256` of the sorted path/SHA-256 roster, file and proof counts, every deferred edit) is reported in the job log and exposed as `ProducedWasmPackageViewV1.sources`.
5. A file edited mid-compile is accepted only if a unit consumed its admitted version. A contradicting, mutually inconsistent or unproven deferred file refuses with a concurrent edit (`build` rank), so the next attempt re-admits it. Dep-info older than the attempt (cached units) can add proof but never contradicts.

## Limit (not hidden)

The compile still reads the live tree. A unit that compiles after the edit consumes the new bytes, so that attempt is refused at proof time. This removes restarts for edits that land after the consuming unit built (late edits to deep crates, the common case) and for edits to files that never reach a unit before the end, but not for an edit that precedes a unit consuming it. Eliminating that needs a private source view (see `release-source-snapshot.md`) and was not attempted.

## Files

- New: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/📎️retained/🟦️.ts`, `README.md`, `🧪️tests/🟦️.ts`, `🧪️tests/🔣️.json`
- Changed: `…/🏗️native-build/🟦️.ts` (uses the shared helpers), `…/🕸️wasm-build/🟦️.ts`, `…/🕸️wasm-build/README.md` (new), `…/🕸️wasm-build/🧪️tests/🟦️.ts`, `…/🕸️wasm-build/🧪️tests/🧫️fixtures/🔣️.json`

## Verification

Bun 1.4.2 pinned binary, `SEMIO_TEST_ARTIFACT_DIR` outside the checkout:

- retained law (9 neutral cases, independent `@noble/hashes` BLAKE3 + SHA-256) and the churn simulation (edit mid-compile through `runWasmCargoCommandV1`; the command completes, provenance carries the admitted digest and the deferred edit): 2 pass
- wasm-build tests: 8 pass; source-witness tests: 3 pass
- ticket `wasm-live-drift.test.ts` (unretained edit still exits 75, retry restart path): 1 pass; `cargo-source-drift.test.ts`: 2 pass
- `tsc --noEmit` over the touched modules: no errors except the missing `bun:test`/`Bun` types in the check config

## Not Verified

- No real WASM compile: whether dep-info under the actual lane (`CARGO_BUILD_BUILD_DIR`, unit cache wrapper) carries workspace-relative checksum paths resolved against `cargoWorkspace` as assumed; if checksums are absent a deferred file refuses, which equals the previous behavior.
- Descriptor suites beyond the source-witness test (component-pass, native-build) were not run after the helper extraction.
