# WGPU Runtime Isolation

The WGPU activation branch previously copied selected extensions into a shared OS Dev installation directory, rebuilt missing font data directly with Cargo, and rewrote a shared module reload marker. That mixed renderer/profile/variant state and mutated completed materializer outputs outside their Nx owners.

Development runtime roots now explicitly include renderer, profile and variant: `dist/runtime/<renderer>/<profile>/<variant>`. All current React consumers and its cached output contract use that same authority. WGPU uses the existing completed-artifact digest, extension publisher and activation receipt. Its activation remains uncached because it publishes live completion. It no longer calls the global extension publisher or the hidden font compiler. Compiled modules continue to be consumed from their existing profile owner, without another mirror.

The WGPU server mounts extensions from its selected runtime namespace and observes that namespace's completion receipt. Fonts are served directly from the existing `semio-framework-os-infinite:fonts` output at the worker's vendor URL. The font producer was already an explicit preparation dependency.

## Verification

- Language-neutral renderer/profile/variant vectors with Ajv schema and lodash projection: the red test reproduced shared paths in 365 ms.
- Native Nx executes the actual preparation, activation, digest and installation definitions against controlled catalog/input fixtures. Eight runtime namespaces publish concurrently; warm activation preserves installation mtime and receipt bytes; deleted live state is reconstructed; shared module/global installation paths are not written. Passed in 5.5 seconds.
- Native Vite serves both profiles, including the completed font producer, with byte parity against Vite's public-file server. Reload, strict port ownership and cancellation pass. Four deployment route tests pass. Combined invocation: 4.1 seconds.
- The test now resolves definitions from their current semantic owners and asserts their presence. The previous actual-root full suite failed after 5m18s because its old root-router extraction produced no PreparationScript definition. That stale harness is repaired here; a fresh complete run remains required.

The native Nx fixture controls catalog and import-rewrite dependencies; it does not qualify real plugin execution or WebGPU painting. Active-reader-safe retention and the native runner's nested activation remain open. No shared cache or prior live installation was deleted.

## Output Ownership And Cancellation

The uncached WGPU activation target now declares its complete live namespace as its output, so inventory can account for it without caching completion events. The output-owner red assertion failed in 376 ms; the corrected native matrix passed in 6.9 seconds.

Cancellation now reaches extension rewriting and the asynchronous artifact publisher/lease. The new cancellation fixture first published despite an already aborted signal and failed its rejection assertion (8.8 seconds). The corrected test rejects before creating the destination; the full focused matrix passes in 11.2 seconds. These times include native Nx startup and the existing publication cycles.

An initial actual activation invocation used the nonexistent `puzzle` variant and Nx rejected it before execution. The Cargo manifest identifies `puzzle2d`, `puzzle3d` and `puzzle5d`; the corrected `activate-puzzle3d-wgpu-dev` invocation is running. No real application result is claimed yet.

## Actual Workspace Qualification

The complete puzzle3d activation run ended after 7m22s with compiler failures (terminal exit130): the renderer attempted Display formatting of InputGeneration, which only derives Debug; the current pack consumer still called the old two-argument DeflateRetainedCursor constructor and removed close method. The renderer debug formatting was corrected to the existing Debug implementation. The pack API changes are concurrent source work and have not been altered here. No successful fresh application activation is claimed.

The existing-artifact activation leaf is now being checked explicitly with Nx --excludeTaskDependencies. This checks publication and HTTP consumption of already completed module/font artifacts, independently of whether current application sources compile; it is not a substitute for the failed full task graph. The ordinary activation target keeps every prerequisite. The renderer compiler is being retried separately after the format fix.

The 3m47s full contract run passed runtime namespaces and inventory (704 projects /6914 targets), then failed the old Apple-tooling function extraction. Its fixture now names the current engine publication owner, and the platform vectors passed in 290ms. A new complete suite is running.

## Collected Runtime Evidence And Current Validation

The real Puzzle 3D publication using completed prerequisite artifacts passed in 672 ms (0/1 Nx hit). This deliberately selected only the activation task with --excludeTaskDependencies to qualify the publication operation; it is not a successful current-source full pipeline. Its real session contains one component and zero extensions.

The actual Vite HTTP consumer passed in 801 ms. Compiler JavaScript (177202 bytes), WASM (79553006 bytes), browser boot (59947 bytes), frame worker (1136242 bytes), and canonical font bytes matched their source artifact digests over HTTP. The runtime receipt selected puzzle3d/dev; abort closed the owned listener. This qualifies routing/lifecycle for existing artifacts, not GPU painting or real extension activation.

The final 1m7s full-suite attempt and the 2.5s actual compiler retry failed before compilation because three generator contracts temporarily referenced absent owner projects. Current taxonomy now points all three at the existing OS Dev TypeScript project. No taxonomy repair was made by this task. Full-suite and actual compiler validation were restarted against that current ownership state.

The current-owner full repo:test invocation passed in 2m28s with both tasks executed (0/2 hits). This is the complete existing suite after the runtime isolation, cancellation and test owner repairs. The subsequent native runtime refactor adds further work and requires fresh full validation.
