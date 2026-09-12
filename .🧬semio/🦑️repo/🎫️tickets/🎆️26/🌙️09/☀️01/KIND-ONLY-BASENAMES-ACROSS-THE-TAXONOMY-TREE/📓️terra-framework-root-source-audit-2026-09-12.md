# Framework Root Source Audit

## Acceptance

The six extracted framework owners satisfy the focused taxonomy goal. Each implementation is an anonymous physical language leaf under a domain owner, and both package roots are bounded declaration glue.

| Concern | Owner | Leaf | Real-ancestry result |
| --- | --- | --- | --- |
| Schema metadata/typegen | `🧰️framework/🔨️modules/🧬️schema/📽️projection` | `🦀️.rs` | accepted |
| Lease pool | `🧰️framework/🔨️modules/⏳️async/🎟️lease-pool` | `🟦️.ts` | accepted |
| Jittered backoff | `🧰️framework/🔨️modules/⏳️async/🔁️jittered-backoff` | `🟦️.ts` | accepted |
| Latest-wins | `🧰️framework/🔨️modules/⏳️async/🥇️latest-wins` | `🟦️.ts` | accepted |
| Event wait | `🧰️framework/🔨️modules/⏳️async/🔔️event-wait` | `🟦️.ts` | accepted |
| Fetch timeout | `🧰️framework/🔨️modules/🚪️io/🌐️fetch-timeout` | `🟦️.ts` | accepted |

`inventoryTaxonomy` evaluated each full path, including all actual ancestors from `🧰️framework` through its semantic owner. All six scopes had zero violations and each leaf resolved to its correct language kind.

The Rust root is 201 lines and keeps the typegen-gated path mount to the schema projection. The TypeScript root is 30 lines and explicitly reexports the five public operations and their types. The focused portable fixture validates the two package identities, the active source-mode test registration, the typegen feature, the exact contexts, and the anonymous basenames.

The five new TypeScript owners occur exactly once in both WGPU `browserProfile.sourceModulePaths` and generator `inputPatterns`; both arrays are strict UTF-8 byte sorted. The WGPU check regenerated no bytes, so the checked browser authority and its six generated artifacts agree.

The OS source-topology schema, fixture, and test contain no live owner-body SHA field or implementation-hash assertion. The retained move-time hashes in `📓️sol-os-source-2026-09-12.md` are provenance only. The OS test still checks source roles, anonymous basenames, package identities, consumers, browser compilation, and configured Cargo targets. Generated-output integrity checks remain separate and active.

## Independent verification

| Check | Result |
| --- | --- |
| `bun nx run @semio-tech/repo-lib:test-framework-root-source-topology --skip-nx-cache` with isolated Nx state | 5 passed, 67 expectations |
| Full-path `inventoryTaxonomy` for the six owners | 0 violations in every scope |
| WGPU source/input authority membership, uniqueness, UTF-8 ordering | all five owners present exactly once; both arrays sorted |
| `bun nx run @semio-tech/framework-os:check-wgpu --skip-nx-cache` with isolated Nx state | 6 exact artifacts, 0 changed |
| `bun nx run @semio-tech/framework-rs:check --skip-nx-cache` with isolated Nx state | `exports_typescript_bindings` passed; 236 filtered; TypeScript mirror fresh |

The Rust check emitted one existing `AtomicU64::fetch_update` deprecation warning in the trace package. It does not originate from these six owners. The async package's absent `async/🤖️generated/🟦️async.js` remains an observed package-level limitation from the executor report; this focused root-framework gate and the native typegen check do not reproduce or attribute it.

No focused defect was found. This audit made no source, taxonomy, fixture, authority, or generated-artifact changes.
