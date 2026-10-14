# Graph Fact Queue Identity Static Audit

Static reads only, 2026-10-15. No executable Source imports, tests, storage, locks, native operations, or process observations. This describes current bytes, not the modules loaded by Actor18/CE2/CX1. Their terminal outcomes supplied by Root do not identify a lease holder or establish self-wait.

## Current Trace

- Graph `🕸️graph/🟦️.ts:101–106` enters fact-store custody before acquiring the separate `build-graph-state:<store.root>[:selected:<scope>]` exclusive lease. Its callback keeps both held until completion.
- Fact storage `📥️inputs/💾️storage/🟨️.mjs:78–80` reuses a live same-workspace writer only for the exact invocation object. A different invocation under that same writer ALS is refused before queue acquisition. The same object is checked through the retained store; workspace and mounted device/inode remain checked at line89.
- Without an existing writer scope, storage81–83 opens the leases store, acquires `repository-graph-facts:<resolved workspace>` exclusive, performs graph maintenance, opens the graph store, then installs writers ALS around the body. Different top-level invocations therefore serialize intentionally. Writer ALS is absent during preparation; a reentrant callback during that interval would need separate proof, rather than being treated as established reuse.
- Graph discovery214 passes `{workspaceRoot,invocation,registry}` to the primary inference entry; graph230 passes the same invocation into dependency context. Inference2006–2009 uses `context.invocation` directly and only receives a process envelope when absent. The proposed missing-context explanation is disproved for these current callers.
- Graph1 and inference1 statically import the same canonical storage file, with different relative spellings. Inference's revision reload targets its own URL; its relative storage import has no query. This supplies no static evidence of a second storage ALS instance. Actual loader duplication or context loss requires an isolated reproduction.
- Cache store32 reuses only an exact invocation held under the same partition key; otherwise it independently opens another image. A derived invocation with the same policy/control is still a different object. That distinction cannot be silently relaxed into authority equivalence.

## Maintenance Integration Read

**Superseded lifetime interpretation:** the paragraph below described opened handles only. Root's subsequent contract decision requires an explicit `receiver: retained | released` discriminator because `provisionNativeImageStorageV1` retains its receiver, whereas opened Mac/fixed handles consume it. Universal consumption is not a valid common-interface assumption. The new discriminator and corresponding owner changes are in progress and are not accepted by this report.

Current opening23 now calls the closed two-argument API with `{version:1,wait:true}` and reopens after completion. This required path does not intentionally skip maintenance on contention. Current graph optional retirement51 uses `{version:1,wait:false}`, returns `result.acquired`, and fact-store83 closes the attempted image before opening its writer store. Mac image58 releases its shared image before exclusive acquisition; both successful maintenance and an optional miss leave that image closed. Reopening is therefore necessary, and present in these observed call paths. The required opening result is currently discarded; review its final owner delta against the platform guarantee that a `wait:true` request either acquires or throws. No claim about the old one-argument call causing terminated runs follows from this repair.

## Remaining Bounded Proof

An isolated law should distinguish: separate originals waiting normally; nested exact original reusing custody; shifted invocation refusing before queue; and any reentrancy before writers ALS installation. A self-wait finding additionally needs the same actual held resource and original owner shown at both acquisition sites. Current stage labels and terminal deadlines alone provide neither fact. Do not bypass identity, queue, Source, or budget checks to obtain progress.

## Observed SHA-256

Paths below are relative to Repo library unless marked General.

| Source | SHA-256 |
| --- | --- |
| `🏗️build/📥️inputs/💾️storage/🟨️.mjs` | `7e7994ba5bc5646ac1567d9ad3027079c4bc5a976b067f47102199f681bf260b` |
| `🏗️build/🕸️graph/🟦️.ts` | `ca2d0eacc709bf6f9f31c4221d728e106d62417b846921ffb3cba6f82afb9cf8` |
| `🏗️build/🕸️graph/📥️inference/🟨️.mjs` | `41df042f593314326f548972cd2a648a4f9b58b1ebb290f48b692519581bbb29` |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts` | `a5ebcb406f44bd918dc4e55d24af118ff4354d7e0fcb68966d4fd1d554cdb664` |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts` | `b319658fc60ae1afb206290f6ac7e101eab6939f5cfb20c023105cc05d831174` |
| General Process `📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts` | `4f747a6186dea7418d1a93a14c3377164262f4846140aa65f1cfa2a29bf212ec` |

Concurrent owner edits are in progress; these hashes are read observations, not a joint freeze or runtime acceptance.
