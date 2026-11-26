# Worker cost of the procedural requester (2026-11-26)

## Verdict

The served requester (b4, core SHA-256 `c2824a5e…`) is an unoptimized build. The dominant cost is the build profile, not one algorithm. I found no quadratic scan in the code I could read. I did not isolate an algorithmic hot spot, because the CPU profile cannot be symbolized any more (see "Not measured").

## Evidence that the served core is opt-level 0

- Cargo omits `-C opt-level` from a rustc invocation exactly when the profile's opt-level is 0. The captured final-crate invocations `🗑️generated/bridge-current/b4/rustc.json`, `boolean-link/rustc.json` and `boolean-precision-link/rustc.json` carry only `embed-bitcode=no`, `metadata=…` and `strip=debuginfo` as `-C` options. None has an `opt-level`, so all three compile at opt-level 0.
- Their cargo target directory is `g3d-p-n25/wasm32-wasip2/debug`, which is the plain `dev` profile and not the policy's `prototype` profile.
- The 1,531,412 defined functions and 324 MB in `📓️requester-WASI-2026-11-25.md` match an unoptimized monomorphised build. The policy's own `s` build is much smaller.
- `debug-assertions` and `overflow-checks` are on in the policy's prototype profile as well (`render-requester-profile.ts` output). They are on in the b4 build too, because rustc enables them at opt-level 0.

## What the repository policy produces

`render-requester-profile.ts` renders the actual policy for `semio-hub-procedural-component` (`wasm-component`, mode prototype). The result is:

| Setting | Value |
| --- | --- |
| profile role | component, lean (lean composition, one package) |
| `opt-level` | `s` |
| `codegen-units` | 16 |
| `lto` | off |
| `debug` | 0 |
| `debug-assertions`, `overflow-checks` | true (preserved by policy) |
| package overrides | 86 rows, level 1 to 3 |

So the canonical path, `compile-lean-procedural.ts` or `repo run semio-hub-procedural-component:component -c prototype`, already serves an optimized build. The b4 core did not go through it. The prototype profile ships no `wasm-opt` pass by design (`optimizeComponentCoresV1`), and V8's tier-up makes that acceptable.

The hot reactor, job and value crates are not in the authored package overrides (`📋️constraints/🔣️.json`, group `runtime`, wasm rows). They run at `s`. I did not raise them, because I have no measurement for the gain.

## Fix

`rebuild-procedural-requester.py` now refuses to capture and link a final `semio_hub_procedural_component` crate compiled at opt-level 0. It exits with a message that names `compile-lean-procedural.ts`. The check is `requester_is_unoptimized(args)`.

Law: `requester-optimization-law.py` loads that function. It failed before the change (`AttributeError`) and passes after:

| Input | Verdict |
| --- | --- |
| b4 `rustc.json` | unoptimized |
| `boolean-link/rustc.json` | unoptimized |
| explicit `opt-level=0` | unoptimized |
| policy-style `-C opt-level=s` | optimized |
| `-Copt-level=3` | optimized |

## Code read, no defect found

- `advance_typed_operation_output_inner` is a thin wrapper. Its inclusive time is the inlined `app.advance_typed_operation_publication` ladder: up to 256 units per call, 4 ms wall each (`INTERACTIVE_TURN_WORKER_WALL_US`). Each unit moves the whole `MountedTypedCommandFullOperation` out of and back into `tool_operations`, and runs the revalidation pass. At opt-level 0 those moves and the forwarding closures are probably the 4 s of self time. I could not confirm that.
- Contributions registration: 425,209 characters cross as 105 pages of 4 KiB. The page size equals the receiver's per-turn copy grant (`receiverGrant.maximumCopyBytes` 4096, `TOOL_WIRE_PAGE_BYTES`). 1,680 continuations is therefore about 16 per page, each a time-sliced turn of roughly 8 ms. In the stable run `setContributions` finished at about 158 s after a first call of about 14 s (the 142 s to 156 s gap). The 49th continuation came 1.4 s after the first call returned, so the registration compute is about 16 s at opt-level 0. Nothing in it scales quadratically.
- The registry fold parses each manifest up to four times (`parse_contributions`, metadata, `register_contributed_manifest`, `installed_flow_extensions_shared`). That is linear and small next to the 16 s, so I left it.

## Not measured

- The profile's wasm function indices cannot be mapped back to Rust names. The b4 link inputs (`proc.core.wasm` and 499 of 948 link inputs, build-output rlibs) live in `/Volumes/semio-store/v1/tmp/…` and `.cache/b/r/m/tmp/g3d-p-n25`, which are gone. The b4 core has no name section.
- No native before/after number exists. The host load average was about 11 to 13 with 85 MB free memory during this pass. A rebuild of the 1,276-unit graph and a Chromium replay were not started.

## Remaining

1. Rebuild the requester through the policy (`bun compile-lean-procedural.ts` with `RUSTC` set, then the relink wrapper) and re-run the 6021 journey. Expect the 320 s from first Edit tick to the first Math invoke, and the 16 s registration, to shrink by the unoptimized-to-`s` factor.
2. If it stays slow, keep the core's name section (`strip` off for the profiled build) and re-profile `advance_typed_operation_output_inner`. Consider opt-level 3 package overrides for the reactor, job and value crates, and check the per-unit `tool_operations` remove and insert move.
3. Page size: the 105 pages cost 16 time-sliced turns each. Raising the receiver grant above 4096 would cut the count, but the grant is a protocol constant tied to the retained-copy accounting. I did not change it.

## Files

- `rebuild-procedural-requester.py` (guard)
- `requester-optimization-law.py` (law)
- `render-requester-profile.ts` (policy render)
