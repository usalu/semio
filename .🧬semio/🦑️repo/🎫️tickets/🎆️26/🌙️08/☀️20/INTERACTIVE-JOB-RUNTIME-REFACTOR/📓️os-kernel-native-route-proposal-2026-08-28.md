# OS-Kernel Native Test Route Proposal

Source-only request to taxonomy. No package metadata, Cargo feature, runner implementation, or native execution was changed here.

## Exact Adapter

Add a separate test-native command to the existing @semio-tech/framework-os-kernel package. Preserve its existing test command and scalar-wire behavior. Reuse the existing library import for resolveTestLevel and runCargoTestBudgeted.

```typescript
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(
      ["semio-framework-os-kernel"],
      this.repoRoot,
      ["--lib", "--features", "sync,ureq", ...rest],
    );
  }
}
```

Register test-native in the existing ScriptRouter; project target calls only bun ./📜️script.ts test-native with argument forwarding and canonical project cwd. Register the matching launch row through the existing taxonomy process. No new target directory, profile, runtime, pool, raw Cargo launch, central-runner changes, Vitest stage, timeout, test-thread, stack, or retry override.

The fixed feature selection reproduces the already-existing native WGPU dependency, not a new workaround: current WGPU Cargo.toml99 names semio-framework-os-kernel with features=["sync","ureq"]. OS-kernel Cargo.toml21 retains default=["deflate"]; sync36 provides the real services/WS runtime edge and ureq28 the actual Directory native transport. Directory native cfg requires sync+ureq+not(wasm32). No --no-default-features or feature-disabling invocation is proposed.

For this packet, the externally supplied rest contains only exhaustive level selection, exact nextest filters, --no-fail-fast and the existing -- --nocapture separator. Do not add caller feature overrides or target overrides. The inert router oracle must record the actual composed argument vector, including fixed --lib and --features sync,ureq, at repoRoot rather than package cwd.

## Inert TDD Matrix

Use the existing parser/router harness pattern already accepted for WGPU, never launch a compiler from an inert oracle.

- Default and explicit exhaustive select the canonical level through resolveTestLevel and call the budgeted helper once.
- Exact filter and separator survive unchanged after the fixed three build arguments.
- --no-fail-fast remains an execution argument in the central partition, not Cargo build metadata input.
- The actual package list is exactly semio-framework-os-kernel, not semio-framework and not renderer.
- No runCargo/raw runner, runVitest, spawn, compiler, artifact mutation, or alternate runtime is invoked by the new adapter.
- Existing commands keep their original dispatch; three new source-only commands reject extra args and lazily import only their own domain oracle.

## Native Selection After Metadata And Source Release

The six authored owner-crate laws are selected by the union of directory_native_runtime_identity_, document_codec_native_send_, and backbone_detach_refusal_. Their Rust paths and hashes are in 📓️os-kernel-r17-native-test-first-source-2026-08-28.md. Use the central supported nextest filter expression and preserve actual enumeration when scheduled; do not infer six executed from six source names. Current production still has the actual original R17 errors and the newly mounted Send assertions, so an initial compiler RED may prevent semantic test execution.

Retain the same master Cargo target, jobs2, explicit SEMIO_COVERAGE=0, exhaustive/no-fail-fast, unchanged existing budgets and per-test profiles. WGPU's separate test-native remains the eventual actual single-enqueue semantic RED route only after the dependency library compiles. It cannot execute these OS-kernel cfg(test) laws.
