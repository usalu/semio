# Node Native Linker First-Cause Trace

Read-only follow-up to actual full caller-environment83522 exit1. No runtime, tool registry, PATH or compiler Source was edited; no retry or compilation was launched.

The actual selected SDK was .cache/b/t/m/nx/363d3244f321fa2795bc45f2e92f6a01d86d8755485802968f6f9fca894af544/semio-authorizers.mjs. Bounded literal inspection confirms that this derivative contains the current registered-driver linker DATA parser, projection, cache identity, rustc -C linker= and ordered -C link-arg= logic. It also contains the current withNxDaemonServiceBirthV1 projection call. This is not evidence of an old derivative lacking the parent linker fix.

The failing log names linux-x64-f6e353a9518bd18e. Independently applying the current canonical SHA recipe to the actual Rust source (SHA256 2e632cc1d4b23c4cdc979ab2bbbb45842c1137247acef97a7bad8f07527dd2ec) and current schema, with linker=null, yields EXACTLY linux-x64-f6e353a9518bd18e. This proves the failing plan did not carry registered linker DATA. With a declared linker, that DATA participates in the cache identity.

The corresponding reconstructed argument prefix is --edition 2021 --crate-type cdylib -C opt-level=3 -C panic=abort -C strip=symbols -C target-feature=-crt-static --crate-name semio_node_native, followed by ordinary output/source arguments. There is no -C linker= nor declared linker suffix. The log does not expose the actual absolute rustc path or final staging output name, so those are not asserted or recovered from private environment.

## Causal Ordering

Actual immutable SDK nx/dist/src/daemon/client/client.js830–834 loads the authorizers and calls withNxDaemonStartupV1(..., process.env, ...). In current caching/graph/startup/🟦️.ts31, requireMaterializedPathV1(resolve(directory)) executes BEFORE order.receive, order.store and order.birth. The first failing child reports nx:daemon-startup resource-directory-admission immediately before Node native-core build failure.

The registered compiler selection and nodeNativeLinkerEnvironmentV1 projection occur only later, in startup/🚦️service/🟦️.ts119–121 inside withNxDaemonServiceBirthV1. Thus the initial Native materialized-path demand can need the Node native core before the existing linker projection is reached. The fixture's direct owned Node child inherits scriptInvocationEnvironment(original, process.env); its startInBackground stub is downstream of startup admission, not a substitute for the missing pre-admission linker preparation.

The current addon reader preserves exact explicit provider/addon/linker choices and includes registered suffix arguments in rustc link-arg order. Nothing here establishes a dropped already-projected value across the child. The demonstrated gap is the earlier caller-side Native boundary. Parent runtime/tooling owners were notified with exact anchors; no ambient PATH workaround, synthesized capability, linker shim or acceptance relaxation was applied.
