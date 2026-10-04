# Release Component Optimization

The first component in family check22 compiled and extracted successfully but produced a92.6MiB core. The shipped component contract admits64MiB. The framework already distinguishes this shipping bound from its256MiB compiler-input bound.

The Stdio permanent task router now uses the installed JCO/Binaryen development tooling to optimize the component before extraction, descriptor generation and distribution admission. Both `editor-component-check` and `catalog-root` use this path. Compiler outputs remain untouched. Optimization publishes through a private sibling directory, preserves an existing release on failure or cancellation, and still rejects release outputs over64MiB. No runtime dependency or shipping-limit change was introduced.

A schema-first neutral component fixture contains an exported integer computation and an unused function. The registered catalog-root contract test parses it independently with wasm-tools, optimizes it, compares the WIT before and after, inspects the extracted core, executes its export with the native WebAssembly runtime, repeats publication, and checks cancellation leaves no output. Red1 failed because the release helper was absent. Green1 and green2 passed; the fixture component shrank from163 to131bytes and its export still returned42.

Receipts: `🗑️generated/release-component-optimization-red-1.log`, `🗑️generated/release-component-optimization-green-1.log`, `🗑️generated/release-component-optimization-green-2.log`. The dedicated launch entry is authored in `.vscode/🧩️launch.seed.jsonc`; its generator is running.

The fresh family component check23 is running in `🗑️generated/editor-components-current-23`, with its log in `🗑️generated/editor-component-check-current-23.log`. No real Stdio release size or full family pass is claimed yet. The neutral fixture proves the build mechanism, not the size or behavior of the complete editor fleet.

## Rust Component Feature Validation

Family check 23 completed its 30-minute native release compilation but failed in JCO/Binaryen input validation on `i32.trunc_sat_f64_s`. The optimizer's default feature set omitted nontrapping float-to-integer conversions. A new language-neutral component vector reproduces the same error (release-component-features-red-1.log). The controlled optimization invocation now explicitly retains size optimization, bulk memory and stripping, and enables nontrapping float conversion and sign extension used by the native target. The registered neutral gate passed fresh (release-component-features-green-1.log). Real family size admission still needs a successful optimized run.

## Family Check 24 and Current Check 25

Check24 failed before optimization against an intermediate CSV Details call/ArtifactView trait bound while the shared producer signature was changing. The native Details producer witness subsequently passed on the corrected source. Check25 now runs through the registered `@semio-tech/stdio-plugin:editor-component-check` target, with output in `🗑️generated/editor-components-current-25` and log `editor-component-check-current-25.log`; no result is claimed yet.

Check25 ended at the configured20-minute catalog deadline during native compilation. No optimizer or64MiB result was reached. The earlier measured compile took30m44s; the next current-source run will use the existing explicit finite60-minute build budget override. This is a runtime budget change for the gate, not a shipping size or cancellation waiver.

The existing VSCode component gate now carries the same explicit60-minute build budget in the launch seed. The registered plugin-registry generator completed successfully (`stdio-component-budget-launch-1.log`), so developers using launch.json receive the measured compile allowance. Shipping byte limits and cancellation remain unchanged.
