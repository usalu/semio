# Build Writer Final Read-Only Audit

Source observation on 2026-10-15. No production files changed, compiler/build invoked, lock EX probe attempted, or daemon/process altered. Storage ownership code is changing concurrently; these conclusions describe the inspected Source, not a runtime qualification of its pending ordinary SDK pair.

## Effective Configuration: New Wiring Is Present

The checkpoint's formerly isolated configuration leaf is now consumed by repository compiler ingress at `storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts`. Both `withRepositoryCargoInputsV1` and `withRepositoryCargoResolutionV1` wrap their fetch/resolve/consumer body in `withRepositoryCargoConfigurationViewsV1`. Launcher compilation planning and awaited terminal command/result consumption occur inside that scope. The old statement that the helper is entirely unreferenced is no longer current.

The ingress observes actual `cargo -Zunstable-options config get --format=json` under the default refusing compiler guard with bounded capture. It admits all observed flag sources before choosing active flags, refuses profile/config-env flag sources for this immutable receiver, computes actual compiler cfg/host, selects matching target flags, validates selected rustc/rustdoc tokens, publishes digest-addressed `view.json` under actual compiler backing, and holds a shared configuration-view resource until the callback and final recheck finish. Closed view use is refused. Multiple manifest selections must agree on encoded compiler flags.

The delivered enforcement is an immutable **flag projection**: child environment sets `CARGO_ENCODED_RUSTFLAGS` and `CARGO_ENCODED_RUSTDOCFLAGS` to the admitted token bytes. The held JSON is evidence for those bytes. It is not passed as Cargo's complete configuration file, and `recheck()` authenticates that held JSON rather than rereading original config. Cargo children still execute from original cwd; no private config-free cwd or complete immutable CARGO_HOME configuration dispatch is implemented here.

Consequently, the specific post-observation mutation from ordinary build/target rustflags to MIR writer flags is addressed by forced encoded flags in Source. It is inaccurate to describe the current implementation as only an unused admission helper or a second-hash preflight. Conversely, the full effective Cargo configuration is still mutable at actual dispatch: nonflag keys, such as target linker/runner or Cargo config-env entries, are neither frozen into child dispatch nor globally refused by this flag receiver. The immutable view contains these observed values but Cargo rereads the original configuration independently. Existing argument/environment admission, forced target/build/temp paths, and the final rustc adapter remain separate defenses; their presence does not make that reread immutable. No exploit or runtime escape was attempted by this audit.

The runtime oracle now authors a real post-observation Source-config mutation and runs both direct held-view and actual dependency-input callback consumers. Its presence is meaningful test coverage in Source, but this audit did not execute it or infer terminal success from its definition. Whole-config immutability remains a scope gap if described more broadly than admitted effective compiler flags.

## Daemon and Watch Lifetime Frontier

The ready filesystem journal worker does hold exact `nx-tooling:<identity>` SH at its real acquisition owner. Its close path stops server/journal/watcher before releasing that loan. No claim that every ready daemon lacks generation custody is supported.

The separate root server frontier persists in inspected Source: patched daemon client wraps startup in `withNxDaemonStartupV1`, which retains startup EX only through the readiness callback. That is not detached server terminal lifetime. The journal worker acquires generation SH after native module loading; it does not open actual tools/leases/Rust fixed-image custody. Therefore ready-worker generation protection does not establish image custody, pre-worker protection, or server custody after worker refusal/restart. No new absence claim was inferred merely from stale process observations.

Dedicated watch environment still sets `.nx/workspace-data/watchers/<identity>` and a separate socket namespace. The ordinary CLI's finite Nx binder qualifies its own canonical namespace; this override does not automatically inherit that namespace qualification. Current worker deployment accepts an absolute deployment directory, checks physical ancestry and then constructs native context there; it does not itself require an issued finite-store namespace. Normal admitted deployment may derive a finite namespace, but arbitrary worker deployment and separate watch dispatch require their own proof.

## Claim Boundary

The code establishes finite managed backings and several scoped writer/reader loans. These do not establish a bound on all bytes in the checkout, arbitrary child/runtime outputs, mutable configuration-selected tools, or detached server/watch lifetimes. No whole-checkout boundedness, cross-platform runtime qualification, daemon lifetime repair, or pending ordinary SDK pair success is established by this read-only audit.
