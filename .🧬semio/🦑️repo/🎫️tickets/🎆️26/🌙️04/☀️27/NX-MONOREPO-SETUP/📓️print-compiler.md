# Print Compiler Inputs and Ownership

The repository currently discovers any Tectonic executable on PATH and otherwise downloads 0.16.9 during compilation. The cached macOS arm64 0.16.9 binary exists and reports that exact version. It supports explicit bundle selection, cached-only resource access, Makefile dependency reporting, and output-directory selection. No executable is currently available on PATH in this shell.

The [official compiler reference](https://tectonic-typesetting.github.io/book/latest/v2cli/compile.html) documents dependency rules, explicit bundles and deterministic builds. SOURCE_DATE_EPOCH is generally sufficient for reproducibility; deterministic mode also removes absolute source paths but changes SyncTeX behavior. The [document configuration reference](https://tectonic-typesetting.github.io/book/latest/ref/tectonic-toml.html) accepts a pinned bundle URL or a local .ttb/.zip bundle.

Next implementation boundary: explicit tool/dependency provisioning, one registered document per Nx build leaf, fixed and exclusive output roots, source staging separate from final PDFs, token/font prerequisites, and compile-time dependency validation. Aggregate Print/Report selections should become Nx prerequisites, with no compiler loops in aggregate commands. Long tests should consume completed document outputs. Existing source watchers should become Nx watch routes.

The current compiler regenerates the shared LaTeX token stylesheet during every compile, downloads dependencies, writes light and dark PDFs into a shared dist root, and retains sources/intermediates there. The report helper permits arbitrary source and destination paths; these behaviors still need refactoring before caching PDF producers.

The official [Tectonic 0.16.9 release](https://github.com/tectonic-typesetting/tectonic/releases/tag/tectonic%400.16.9) exposes SHA-256 digests for its archives through the GitHub release API. The selected platform archives are:

| Platform Archive | Published SHA-256 | Bytes |
| --- | --- | ---: |
| tectonic-0.16.9-aarch64-apple-darwin.tar.gz | sha256:edb67c61aba768289f6da441c9e6f523cfaff4f8b2a5708523ef29c543f8e88e | 20590132 |
| tectonic-0.16.9-aarch64-unknown-linux-musl.tar.gz | sha256:f9aa39017dbd51f111fdb93dda222178cbe51c8193508fc567b523cc74fff9c1 | 9923433 |
| tectonic-0.16.9-arm-unknown-linux-musleabihf.tar.gz | sha256:518eec6784010d12c7c1015d72abce5f1baeea25c7c0984817069662f70770be | 9779190 |
| tectonic-0.16.9-i686-unknown-linux-gnu.tar.gz | sha256:7a2b3cf2f61fd9025992d89e92abc39291dcfbdd5d3b6c2adf5d8fabf668c0f8 | 20708451 |
| tectonic-0.16.9-x86_64-apple-darwin.tar.gz | sha256:79d8839fa3594bfea9b2bf2ac0a0455bcc4d0de956a5e5c403107e9a72f79e86 | 20572838 |
| tectonic-0.16.9-x86_64-pc-windows-gnu.zip | sha256:61176116a515bb1f1a4ce659e99949a33c59586d6200afe28b2cad3e79d30053 | 11404822 |
| tectonic-0.16.9-x86_64-pc-windows-msvc.zip | sha256:131a24604785a9600989a3d91225f597df52ac06f00aeffe86fd529f99ee5cdd | 20035039 |
| tectonic-0.16.9-x86_64-unknown-linux-gnu.tar.gz | sha256:f3c825128095dc3399ea11c08c18035b33050a216930c295c79e8eb11bd21de4 | 21568986 |
| tectonic-0.16.9-x86_64-unknown-linux-musl.tar.gz | sha256:60b13a0826ae7ad9ce34b4a2df06bff2cfcfa6dda8a915477c0cbb84e1a4a902 | 10146030 |

The local Tectonic cache currently points at the default_bundle_v33.tar alias with bundle identity `6ffe055852f8faf66c0acbe1a7fb27f87b869a90bad1204f3bf4d9683f597c7c`. This observation is not yet an explicit repository bundle pin. Tool preparation still needs a schema-owned archive manifest, platform-specific owned installation, verified download/extraction, and an uncached Nx dependency target before PDF compilation can lose its installer fallback.

## Explicit Compiler Preparation — Implementation in Progress

A schema-owned 0.16.9 distribution manifest now records seven host/architecture variants, exact archive bytes and published SHA-256 digests. `deps-tectonic` is an uncached Nx preparation target with a version-and-platform-specific owned tool directory. The downloader bounds archive bytes, verifies the published digest, extracts through the system archive tool, atomically publishes executable files and a hash receipt, and handles cancellation before publication. Existing installations are verified without downloads. No PATH-selected compiler is accepted.

The PDF helper now reads this prepared compiler and no longer installs Tectonic or rewrites the shared token stylesheet during compilation. Print and Report producers declare compiler preparation, generated tokens and fonts as Nx prerequisites. `test-toolchain` checks the actual executable version. Both targets were added to the editor seed; generated launch refresh is pending.

The platform-selection test passes against a retained projection of the publisher's release metadata. Malformed archive and cancellation tests pass, confirming no executable or preparation tree survives either failure. The real preparation and executable targets are running. Full PDF output ownership, bundle pinning, compiler-free long tests and Nx watch routes remain unfinished. Concurrent first-time installations can still duplicate the download and race for the short publication lease; a shared installation guard remains to be completed. Native Windows/Linux execution has not been exercised.

Both normal shared-daemon compiler targets passed. The installation downloaded and SHA-256 verified the published 20,590,132-byte macOS arm64 archive, extracted it and published its executable receipt. The executable verification returned exactly Tectonic 0.16.9. Preparation reported 20.4 seconds on its critical path but 4m27s overall Nx run duration; the second preparation and executable check together reported 1.3 seconds critical path but 1m overall. This overhead needs profiling and is not attributed to any unverified plugin behavior.

## Document ownership implementation, 2026-09-08

Added a schema-first catalog for six templates and 81 visualization documents. Nx infers separate build and watch targets with exclusive `dist/documents/<id>` PDF ownership. Aggregate commands consume dependencies and do not compile. Long and visualization tests now consume PDF deliverables. Leaf caching remains disabled until bundle identity and byte restoration are qualified. Compilation now uses asynchronous cancellable native processes, a fixed SOURCE_DATE_EPOCH, deterministic mode, and temporary compiler directories removed after completion; it publishes only final PDFs through the existing atomic artifact boundary.

The first native viz-api probe failed because Tectonic `--untrusted` disables explicitly supplied source search paths; its help confirmed that behavior. Removed that option, preserving the compiler's default disabled shell escape. The retry has compiled the light document and is still running; it is not yet a passing build.

The command-boundary regression failed before extraction. Moving pure Oklab functions next to styling and replacing public repository-library imports reduced the compiler to 11 source files and token renderer to three (esbuild independent import oracle), before the document catalog was added. The permanent quick test passed in 4.1 seconds through normal Nx. Additional catalog/router checks are pending.

Primary source investigation: [Tectonic indexed tar implementation](https://raw.githubusercontent.com/tectonic-typesetting/tectonic/master/crates/bundles/src/itar.rs) identifies byte offset/length entries and a bundle SHA256SUM file. [Bundle detection](https://raw.githubusercontent.com/tectonic-typesetting/tectonic/master/crates/bundles/src/lib.rs) also accepts local directory bundles. A direct HTTP response resolves the current default to `https://data1b.fullyjustified.net/tlextras-2022.0r0.tar`, a 2,881,562,112-byte archive. The existing native cache contains 486 files totaling 19,446,990 bytes. These are measurements, not a completed or validated pinned dependency manifest. The proposed acquisition uses checksummed file ranges, avoiding a whole-archive download.

### Verified support-file acquisition

The existing cache's 486-file set was converted to a source-controlled dependency lock with URL, total archive size, exact byte ranges, per-file SHA256, and upstream identity. Every one of the 19,446,990 support bytes was independently fetched from the publisher and verified against that lock by the actual `@semio-tech/print:deps-tex` target. The run passed in 27.5 seconds, with 26.7 seconds on the critical path. No native cache file was copied into the new installation. The local directory bundle has a SHA256SUM derived from its lock, and verification checks every file before compiler use. A curl/Bun range oracle agreed byte-for-byte. The schema, pre-cancel and corrupted-range tests passed. The first build consuming this local bundle is still pending.

The preceding default-bundle viz-api build passed in 3m39s and published two PDFs through its exclusive artifact owner. Inspection found no panel-glass render operations in that run. Compilation now leaves rerun scheduling to Tectonic and omits a second panel pass when no panels exist; the resulting PDF content is not yet qualified against the prior output.

Registry generation passed in 22.9 seconds after adding 174 document build/watch editor commands. The new `deps-tex` editor seed entry was added afterward and still needs regeneration.

### Native repeat and first cache-restoration probe

A second native local-bundle compilation produced byte-identical light and dark PDFs. The first deletion/restoration run then reported the document leaf as `[local cache]` and finished in 1.0 seconds (194 ms critical path), but the probe failed before PDF parsing because it expected three cache hits and observed two. Inspection identified an omitted cache flag on the independent LaTeX token generator; fonts and the PDF leaf were cached, while token generation still ran. Added the flag and narrowed its generator-contract inputs to its actual production implementation and source tokens. The restoration probe is being rerun. The initial probe's rollback restored the original owned PDF directory; no outputs were lost.

Normal shared-daemon graph construction delayed the native repeat by approximately three minutes before task execution. A bounded daemon-tail inspection showed concurrent source changes and project reconstruction. The attempted 1-second worker sample could not run because that worker had already exited; no profiling conclusion is drawn.

### PDF restoration qualified

The completed retry passed: repeated build output bytes match, deleting `viz-api`'s owned deliverable directory produces a native Nx `[local cache]` restoration, all bytes match, and PDF.js independently extracts every required fixture text from both restored documents. The warm run reports 3/5 cache hits; Tectonic and TeX dependency verification intentionally execute uncached. Evidence: `🗑️generated/print-document-cache-complete.log` and `print-document-cache-restored.log`.

After this qualification, narrowed document source lists to each entry TeX file and its explicitly used content, bibliography, appendix and image files. Gallery documents no longer hash/copy the unrelated visualization taxonomy or source fonts; their font dependency inputs remain declared separately. The three Zukunftbau templates no longer hash one another's entry/content sources. A full collection build is next to check the source lists and bundle completeness.

### Repository contracts

The full normal-daemon `repo:test` run passed with the new Print invocation vectors, inferred document targets and editor changes (about 65 seconds in the test task). The collection build is running independently; no final all-document result is available yet.

### Complete catalog build and PDF consumers — 2026-09-08

The first complete catalog build passed: all 87 leaves published 174 PDFs through their fixed output owners. Nx reported 93 tasks, 2 cache hits and 62m59s task-run duration. All six templates and 81 visualization documents consumed the pinned local bundle successfully; layout warnings remained visible, without compiler errors. This run began before the final shared publisher extraction and dead compiler API removal, so it does not establish a final-source warm cache.

Removed the obsolete arbitrary compiler selector API and its compatibility assertions; authored root invocation vectors cover selection. The subsequent Print quick suite passed in 3.1s producer time (5.8s task run). Added a language-neutral PDF-consumption fixture for 12 template PDFs and 162 visualization PDFs. Long/full-visualization tests now use PDF.js to read every page of every PDF and validate nonempty text, with the existing API content fixture retained. Their Nx inputs include transitive PDF producer outputs. The final-source collection rebuild plus these stronger consumers is still running; this newer validation is not yet claimed passing.
