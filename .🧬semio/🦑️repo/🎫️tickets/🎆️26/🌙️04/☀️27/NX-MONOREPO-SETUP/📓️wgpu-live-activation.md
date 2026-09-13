# WGPU Live Activation — 2026-09-13

WGPU preparation previously published extensions, fonts and a Date.now()-based reload marker despite being cacheable with no outputs. Activation executed PreparationScript again and was also cached. A no-change rerun could replay success without recreating deleted live state, and a cold path published the same live state twice.

Preparation now only validates completed prerequisites. Activation consumes the completed session, validates its identity and performs WGPU live publication; inferred WGPU activation targets are explicitly uncached. Nx owns the preparation edge, so ActivationScript no longer invokes PreparationScript internally. React activation retains its owned, cacheable variant/profile runtime directory.

The language-neutral live-activation fixture first failed on the old cached activation classification (368 ms). The native Nx test then passed in 4.1 s. It extracts and executes the actual preparation, activation and WGPU publication code, uses controlled file-copy publishers in a private workspace, and runs three cycles: cold, unchanged, and deleted live state. Preparation executes once; activation and publication execute once per cycle; missing extension/font/reload files are recreated. Ajv checks the neutral fixture, TypeScript extracts the real commands, and native Nx decides cache reuse. The test rejects nested PreparationScript execution and checks that preparation invokes no live publishers.

This is command/lifecycle qualification with controlled downstream publishers. It does not claim a full application render, native plugin installation, complete Trunk artifact restoration, concurrent production-store protection or cross-platform runtime validation. The current Trunk compiler/live-store boundary remains open; see 📓️trunk-output-boundary.md.

The earlier full suite passed the coordinator repair and then failed because its materializer cancellation harness read an obsolete package-facade path. The neutral fixture/schema now identify the actual materializer source at plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts; validation is running again.

A fresh native repository graph resolves all 122 WGPU activation targets (61 variants, two profiles) as cache:false. Every one depends on its corresponding preparation target. Scoped read-only git diff checks passed. The complete repository suite and refreshed audit are running; no full-suite pass is claimed yet.

The refreshed audit reports 704 projects, 7,281 commands, 7,902 artifact/storage entries and zero automated structural findings. The documented manual Trunk finding remains unresolved. Successful new styling and WGPU fixture runs now remove only their own private workspace; failed runs retain diagnostics under this ticket. Shared compiler stores and other active workspaces are not pruned by these tests.

## Full Repository Validation — 2026-09-13

The actual-root `repo:test` invocation completed successfully: 5m12s, both tasks executed (0/2 cache hits). Native inventory covered 704 projects and 6896 targets. Compiler output, Python/.NET wheel/assembly execution and native oracles, WGPU live publication, graph ownership, source-byte discovery, coordinator and materializer cancellation contracts all passed. The separate structural audit covered 7281 commands and 7902 artifact-storage entries with zero automated findings. This does not qualify the remaining Trunk output/serving, active retention, CI trust, or unexecuted platforms.
