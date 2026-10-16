# Selected Graph Reader Correction Audit

2026-10-16. Read-only Source successor review. No Native, SDK, compiler, image, network or runtime tests ran. Prior epoch P1 early namespace release is corrected structurally, subject to actual runtime qualification. Global physical enrollment and fresh installed SDK qualification remain HOLD.

## Corrected Namespace Ordering

Graph lines120–121 create the private ledger on the actual NativeScopeEntry and make release idempotent. Release seals and awaits `entry.readers.close()` before invalidating live/maps, closing native image and releasing lease. Graph line79 registers the complete canonical task promise synchronously via ledger.accept. Ledger lines7–8 reserve the actual Promise before invoking run; canonical requested control settlement includes accepted descendants before this promise settles. Ordinary wrapper line84 and withNamespace line130 await namespace release before nested store invalidation/image close at lines60–61. This closes the prior detached-task early-release Source path, including callback Error/undefined and reader Error/undefined; cleanup attempts still continue and AggregateError retains multiple release failures. No new Original is issued.

## P1 — Bootstrap Passes SourceRoot Authority Into a Separate Frozen Issuer Universe

Bootstrap line511 invokes `frozen.authorizers.selectedGraphSdkPortsV1` with `actualPhase`. That control comes from SourceRoot `withNxToolingUseV1` at line418, nested in SourceRoot storage/tooling routing at lines375–388. SDK line22 now immediately authenticates via `requireNxGraphNativeControlV1`; graph lines74–75 require the module-private issued-control WeakMap and NativeScope ALS.

The immutable authorizer snapshot compiler lines23–25 bundles every relative captured module into semio-authorizers.mjs. Its external allowlist (authorizers file line11) contains SDK modules and no SourceRoot routing/Graph module. Therefore that frozen function owns different private WeakMaps/ALS from the bootstrap's SourceRoot authority owners. A genuinely issued SourceRoot actualPhase is foreign to the frozen bundle and fails before SDK resolution. Binding environment strings cannot transfer those identities. The actual bootstrap route remains broken at Source level despite neutral missing-owner guard tests passing.

Required correction: use one actual module authority owner for both the port factory and its control/namespace lifetime; preserve the frozen SDK authorizer DATA/planner interface where appropriate. Do not copy controls, accept structural ports, open a second namespace, or reconstruct Original. Qualify the exact bootstrap/frozen boundary when physical effects are authorized.

## P2 — Ledger Failure Retention Is Not Bounded

Reading ledger line7 declares an unbounded failures array; line8 appends every distinct rejection. `maximumPendingOperations` bounds concurrent pending promises only. Repeated sequential rejected admissions with distinct Error objects can append indefinitely while pending returns to zero. The private NativeScopeEntry lifetime may be long. Bound retained failures independently (a first-failure cell suffices for primary propagation, or a schema-bounded aggregate), preserving thrown undefined explicitly.

## Remaining Limits

The finite metadata reader still has no new blocking Source finding; its documented same-user lstat/open ancestor swap-and-restore limitation remains. Ports can escape as callable functions but every invocation reauthenticates and late async admission is now sealed by the actual ledger. A reader directly awaiting its own namespace release would self-deadlock, as with a structured scope joining itself; the SDK callbacks do not possess that lease. Exact-law narrowed control ancestry still must remain enforced. No runtime drainage, cancellation, FD-close-failure or installed SDK execution was claimed by this audit.

## Exact Source Roster

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` — SHA-256 `a233cf978dcfaa17b722d00e344f5341c181c96ac16026d08898be398e1e7757`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/👓️reading/🟦️.ts` — SHA-256 `14904bb1681bd4c5321d1526f0a015c591f1325ab24ea584da8479928b33368a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧬️schema/🔣️.json` — SHA-256 `1599b48148c94e956fe0204e74df7a85b72af62ec8b12dd0f3ea9e24c650d3ec`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts` — SHA-256 `a21353f86ce1fad4e634cd5627c03759d0b7674a8f0401c3c0f8d4e4aeab521b`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` — SHA-256 `f1ccb454826933eb293b47fbd796de446a8a159fa6bf27526ca3f68e172a8d45`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts` — SHA-256 `78abef4e1a32d2731d5681319d693df3114749cb713a15a6d4fbb1f40f54cb41`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts` — SHA-256 `9548d11375bf11c50f774fe11b70dbdf8662737c53d8ae9476aaa65cff7dc78a`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` — SHA-256 `f2854f3a5820c190a793ba0eae625ab4acadb468d403de7ac60c2da9548ae5b3`

## Second Successor Rereview

Read the corrective Source after the findings above. Both are structurally resolved: bootstrap line1 imports the SourceRoot selectedGraphSdkPortsV1 and line512 invokes it with actualPhase, retaining frozen capture/receipt as DATA/callback consumers. Frozen installed handoff keeps its same-bundle factory. Ledger line8 now seals admission on the first observed rejection; at that point at most the finite pending limit can contribute failures, including undefined, so failures no longer accumulate over unlimited sequential admissions.

Patch Source has genuine external-wrapper ownership around main at lines1948–1949 and run-executor at lines1885–1886. Handoff consumers at lines1208,1281,1302,1336 invoke the frozen authorizer; no separate SourceRoot issuer crossing is visible in these wrapper paths. This is Source tracing only: no actual installed SDK/frozen bundle, daemon route, Native lifetime or runtime guard success was exercised. No additional blocking finding is established by this rereview. Prior findings above describe their pinned earlier epoch, not the successor status.

### Second Successor Hashes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/👓️reading/🟦️.ts` — SHA-256 `14904bb1681bd4c5321d1526f0a015c591f1325ab24ea584da8479928b33368a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` — SHA-256 `f1ccb454826933eb293b47fbd796de446a8a159fa6bf27526ca3f68e172a8d45`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch` — SHA-256 `cd4974307311ba7fc79c67f954348f91f4331998533ab757c0e4cd4e7767f90e`
