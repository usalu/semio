# WASI and Native Codegen Profile Policy

## Boundary

The target-blind nine native `profile.dev.package.*` single-CGU overrides are removed. Explicit `wasm-dev` inherits `dev` and applies one codegen unit only to commands selecting that WASI profile. Native `dev` retains Cargo's parallel default. The existing `wasm-release` profile, stdio immutable publication command, hash limits and marker-last contract are unchanged.

One shared strict selector accepts only `wasm-dev` and `wasm-release`, with dev/ship defaults respectively. OS plugin and extension component builders use it; the common descriptor helper and both scale component-link paths explicitly select `wasm-dev`. The extension output resolver now honors the same `CARGO_TARGET_DIR` used by Cargo.

Runtime native OS, root preflight and MCP directory order is exactly `wasm-dev`, then `wasm-release`. The registry owner generates the Rust constant. Publication hash verification is a separate canonical `wasm-release` path with no first-found development fallback; absence leaves publication identity unverified.

## Neutral and registered evidence

- Strict JSON Schema and twelve language-neutral profile vectors cover six accepted defaults/overrides and six rejected native/unknown/empty/whitespace profiles. An independent mapping and AJV validate the vectors; `@iarna/toml` independently validates actual Cargo profile settings.
- Initial `bun nx run @semio-tech/framework-os-dev:test --skip-nx-cache -- -t 'WASI codegen profile policy'`, session 44182: RED exit 1 before test selection on two concurrently missing kernel builder/composition schema imports. No policy test result is credited.
- `bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache`, session 59031: GREEN exit 0; registry Rust and launch output regenerated through the permanent owner.
- `bun nx run @semio-tech/plugin-registry:test --skip-nx-cache -- -t 'WASI codegen profile policy'`, session 26659: GREEN exit 0, exactly three tests passed. They cover all twelve routes plus Cargo policy, generated/root/MCP directory parity and descriptor/scale link selection, and development/stale-debug artifacts failing to substitute for canonical release identity.
- `bun nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache`, session 12890: GREEN exit 0, registry and launch bytes fresh.
- Final source-policy rerun 6226: GREEN exit 0, three tests passed. Final owner freshness rerun 47016: GREEN exit 0. Only the intentional stale-debug negative retains a debug component path in the current builder/resolver census; the optional native note probe no longer consults an old ticket target.
- Final OS-dev wrapper/argument rerun 99334: RED exit 1 before test selection. The prior schema files are now resolved, but concurrent taxonomy still leaves the kernel input-authority and composition `🧪️fixture/🔣️.json` imports missing. The independent registry policy test remains separate positive evidence.
- Isolated actual-link attempt `CARGO_TARGET_DIR=<ticket>/🗑️generated/wasi-profile-policy-sol-target SEMIO_BUILD_BUDGET_MS=180000 bun nx run @semio-tech/framework-os-scale-fixture:build-wasm --skip-nx-cache`, session 52015: RED exit 1, `ETIMEDOUT` at the 180,000 ms bound. Cargo selected `--target wasm32-wasip2 --profile wasm-dev` and advanced through active WIT dependency compilation; no Rust source diagnostic or linked component was produced. The generic timeout text speculates about a target lock, but the retained isolated target and sampled active compiler processes do not support that attribution. No retry was launched under the coordinator's memory-pressure serialization policy.
- Owned-file `git diff --check`: GREEN.

## Pending and nonclaims

OS-dev wrapper/stack-argument tests await the shared fixture import repair. No new WASI link, native timing improvement, immutable publication identity, provider law, readiness, or client mount is claimed by these source-policy tests. Active stdio provider session 26201 was not interrupted; its already-running compiler retains its original selected codegen settings. Subsequent Cargo commands use the new policy.
