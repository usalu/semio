# OS Dev Production Build Completion

The generic `@semio-tech/framework-os-dev:build` entry point now depends on the existing `build-s-react-release` target. Nx owns that variant's 67 prerequisites and final distribution output. The generic entry executes a narrow, argument-free completion script and declares no files of its own. Its package script uses the same generic Nx target.

The old `BuildScript` and router registration were removed. They previously called plugin compilation and engine compilation, then Vite, or invoked the WGPU script directly. The generic entry no longer performs that hidden orchestration. Other OS Dev/WGPU service paths still require separate lifecycle work.

The existing production browser fixture now exercises the generic completion dependency through native Nx and real Vite. Its new language-neutral contract failed first on the missing explicit production prerequisite (153 ms). After implementation, the fixture passed in 11.5 seconds: one cold compiler invocation, warm reuse, exact deleted-output restoration, runnable relocated plugin/extension imports, and a second compiler invocation after a runtime dependency changed. The completion command itself uses only repository routing and does not invoke compilers or another Nx process. No complete production site or the full native 67-task closure was built in this test.

The editor seed now exposes the generic default build and canonical CAD, Puzzle 3D, Puzzle 5D and Shooting production targets. Four obsolete `build -- <variant> fixture ...` command strings were replaced. The old implementation forwarded these fixture words as raw Vite arguments; no fixture-selection mechanism was found in the production configuration. Their misleading fixture-specific names were removed, and the duplicate Puzzle 5D entry was consolidated. Launch regeneration and the extended editor assertions are pending.

## Adjacent Validation

The .NET styling build passed the real authored Nx graph. Its second invocation reused the compiler target from the local cache; restore and generator still executed (1/3 tasks cached, 8.2s task execution). The full suite also passed the new narrow .NET command/esbuild test and all its MSBuild/restoration cases.

The full suite stopped after 6m23s because its cached project graph lacked a graph-catalog schema input that the currently read taxonomy declares. The graph was computed before the suite began; the inventory/audit reads that cached graph rather than recomputing it. This remains to be checked against a newly computed graph. The full suite did not reach the repaired support-library Compile assertion. No complete pass is claimed.

The subsequent cached-graph audit passed in 15.0 seconds with 704 projects, 7,276 commands, 7,901 artifact/storage records and zero automated structural findings. It still describes the earlier OS Dev command, so it is not evidence for the new production completion graph. Python wheel and finite WGPU Trunk output ownership remain open.


A newly computed native graph now confirms the generic OS Dev build depends exactly on `build-s-react-release`, calls the completion script and hashes only its three-file routing/workspace import closure. The same fresh graph has zero missing graph-catalog inputs, resolving the earlier stale-snapshot mismatch without changing the generator contract.


Launch regeneration passed through the real registry target (1m29s task execution). The editor test initially assumed one entry per target, but the generated launch file deliberately includes both curated build entries and catalog-generated playground entries. The test now checks availability of every required canonical command and absence of the obsolete generic argument form; it does not reject these existing curated/catalog entry pairs. The source seed contains one curated entry for each selected target.


The complete editor/native production fixture now passes in 32.0s with both launch files, the package entry, real Vite, native Nx completion, cache reuse, deleted-output restoration and dependency invalidation. The full repository suite is running again against a newly computed graph.
