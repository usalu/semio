# Styling Compiler Output Contracts

The .NET styling build is being changed from an uncached compiler operation with no deliverable declaration to a finite `dist/build` publisher. Compiler intermediates stay in the native .NET store. Build and restore take the same cross-process resource lease. Publication uses the existing owned-directory transaction and recursively collects only the temporary final-output tree; each invocation removes its own temporary directory after success or failure.

The target declares generator-output hashing, the palette source, production inputs, and SDK/platform fingerprints. Dependency preparation remains uncached. Its existing editor build/dependency entries already invoke these Nx targets.

## Runtime Evidence

The new language-neutral fixture and JSON schema are validated with Ajv. It copies the actual styling project and generated C# source into a private Nx workspace. A separately compiled .NET reflection consumer executes the resulting assembly. Direct MSBuild compilation into a second compiler store supplies the native oracle.

- Initial contract test: expected failure in 589 ms because the target was uncached with an empty output declaration.
- First publisher run: compilation and reflected palette values passed. Assembly bytes differed between compiler stores; the assertion failed after 19.5 seconds.
- Canonical mapping of compiler-state paths made the independent assembly builds byte-identical. The cold, warm and deleted-deliverable restoration checks passed. The fixture then failed its source-change check after 1m01s: the canonical generated palette path is ignored by Git/Nx, so a direct source-file input alone cannot hash its changed bytes.
- The fixture now models an explicit generator producing that ignored file, and hashes its completed output using Nx's `dependentTasksOutputFiles`, matching the production target's contract. This revision is under validation.

The test checks every published file's exact bytes after restoration, executes the restored DLL, and checks that the uncached dependency target executes on each invocation. No shared styling compiler cache was deleted. Runtime qualification is currently on macOS arm64 only.

## Compiler References

MSBuild's build command supports separate artifact state and final output paths. See [dotnet build](https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-build). Deterministic C# output includes compiler inputs and their paths; equivalence therefore requires normalizing both source and intermediate paths. See [C# deterministic compilation](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/compiler-options/code-generation#deterministic).

The Python follow-up must also separate a published wheel from its build environment. Its current preparation performs editable package work and its backend constraint floats. The intended finite build will consume an explicitly prepared, locked backend. `uv build --no-build-isolation` expects build dependencies already installed; see [uv CLI](https://docs.astral.sh/uv/reference/cli/#uv-build). [Hatchling 1.27.0](https://pypi.org/project/hatchling/1.27.0/) is an available exact backend version. No Python package or lock was modified in this checkpoint.


The generator-aware native fixture passed in 1m07s: both independent compiler stores emitted identical DLL bytes, all final files survived exact-byte Nx restoration, the restored assembly executed, a generated palette change triggered a rebuild, and preparation ran on all four invocations. The production .NET implementation now lives in its own 📜️script.ts with a narrow router import; the full suite includes the permanent test and an esbuild assertion excluding Python command imports. The real repository target is being validated separately.


The real `@semio-tech/ui-styling-dotnet:build` now passed through the authored Nx graph and narrow package router: three tasks (generator, uncached restore, compiler), 8.0s task execution, 3 published compiler files plus the ownership receipt, no compiler warnings/errors. Initial graph computation was substantially longer while other graph processes were active; 8.0s is not end-to-end startup latency. Log: `🗑️generated/styling-dotnet-native-build.log`. A second native invocation is running to measure actual cache reuse.


The second real invocation passed in 8.2s of task execution. `@semio-tech/ui-styling-dotnet:build` was explicitly reported `[local cache]`; the dependency restore and styling generator executed, so the graph result was 1/3 cached. The compiler messages in that cached task are replayed output. The full repository suite subsequently passed the narrow esbuild boundary and every native styling fixture case. It later failed on a graph-catalog input missing from its earlier cached graph; the newly computed launcher graph contains all current graph-catalog inputs.


The actual package final directory contains only its 175-byte ownership receipt, 448-byte deps manifest, 12,288-byte DLL and 14,948-byte PDB. Native intermediate/compiler state is outside the cached deliverable tree. This census does not claim bounded total native-store retention.

## Python Wheel Contract — 13 September 2026

The Python wheel still writes to shared compiler state and has no Nx-owned output. The new language-neutral fixture requires a package-local wheel, a locked Hatchling build group, uncached preparation without an editable first-party build, native Nx cold/warm/deletion/source-change behavior, and byte equality with an independent uv build. Implementation has not yet been verified. uv documents --no-install-project for dependency-only preparation and --no-build-isolation for consuming installed backend tools; Hatch documents reproducible wheel construction. Sources: [uv CLI](https://docs.astral.sh/uv/reference/cli/) and [Hatch wheel builder](https://hatch.pypa.io/1.13/plugins/builder/wheel/).

The preceding full repo:test run passed the repaired .NET source path and both compiler-output proofs, then failed a shallow Python styling generator dependency assertion. The assertion now checks native Nx task reachability through build. A fresh full run is in progress.

### Native Python Evidence

The focused Nx target first failed in 171 ms because the production build was uncached. After implementation, the private native Nx/uv proof passed in 9.5 s: one cold compile; no compile on the identical second invocation; no compile after deleting the final output; byte-identical restoration; changed generated token bytes trigger a second compile. Both wheel revisions execute in isolated Python and equal independently built uv wheels byte for byte. All four dependency preparation invocations execute, and the environment contains no editable first-party package. Hatchling 1.27.0 and its four transitive build tools are now locked; the package still has no runtime dependencies. The lock was generated in a private ticket directory through Nx, without changing a shared environment.

The 49.8 s full repo:test attempt stopped before the compiler checks because concurrent taxonomy changes referenced actor-typegen and ui-axes owners that discovery did not see in that run. This is not a full-suite pass.

### Repository Python Target

The actual @semio-tech/ui-styling-py:test target passed after 5.7 s of task execution (graph startup is additional). It prepared a private ticket virtual environment, generated tokens, published the package-local wheel, and imported both canonical source and wheel. The next invocation took 1.9 s of task execution and reused 3 of 4 tasks; only dependency preparation executed. This confirms compiler and test reuse in the actual repository. The native graph includes the narrow compiler script and seven infrastructure helper files, excluding the former broad test/application closure. All four native test task graphs include generator, deps and build prerequisites. Shared package virtual environments and existing outputs were not deleted; deletion/restoration was tested only in the private fixture.

The extended wheel proof passed in 14.8 s. It additionally checks all seven Python commands in both editor seed and generated launch configuration, starts the backend with isolated Python imports, forces a real Hatchling missing-source failure, confirms that all prior publication bytes survive, and confirms private staging is empty. The launcher generator completed through native Nx.

## Full Repository Validation — 2026-09-13

The actual-root `repo:test` invocation completed successfully: 5m12s, both tasks executed (0/2 cache hits). Native inventory covered 704 projects and 6896 targets. Compiler output, Python/.NET wheel/assembly execution and native oracles, WGPU live publication, graph ownership, source-byte discovery, coordinator and materializer cancellation contracts all passed. The separate structural audit covered 7281 commands and 7902 artifact-storage entries with zero automated findings. This does not qualify the remaining Trunk output/serving, active retention, CI trust, or unexecuted platforms.
