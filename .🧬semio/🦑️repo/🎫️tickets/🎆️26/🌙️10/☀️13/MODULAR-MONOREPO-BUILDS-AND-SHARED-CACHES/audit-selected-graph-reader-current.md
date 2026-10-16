# Selected Graph Reader Source Audit

2026-10-16. Independent read-only Source review. No Native, SDK loading, compiler, network, image, lease, runtime test or physical effect was executed. Source-only conclusions; global physical enrollment and fresh SDK qualification remain HOLD.

## P1 — Accepted SDK tasks do not retain the namespace or nested pool until completion

Graph `withNxGraphNativeTaskV1` lines77–78 tracks work against `entry.store.control` and optionally the current graph-law context. It does not enroll the actual namespace callback or cache consumer frame. In the ordinary `withRepositoryNxGraphNativeStorageV1` route, line83 immediately enters namespace release in its finally when the callback returns; `openNxGraphCacheStoreV1` then invalidates its store at line59 and closes its nested image at line60. These actions are inside the outer cache callback, before the outer control can settle/drain work registered on that control. `withNxGraphOperationTaskV1` line33 has no law-context enrollment when no graph operation exists (`Promise.resolve().then(body)`). Even an enclosing law drains outside this inner namespace finally.

Concrete Source schedule: start accepted `ports.scope` or `ports.capture`, suspend within metadata `file.read`/journal RPC, and return the owner's callback without awaiting that Promise. Line83 releases the namespace while accepted reader work remains. Subsequent guards can reject after release but cannot undo an FD/RPC already in flight. Control tracking preserves eventual process drainage, not the required before-namespace-release ordering. The graph cache consumer bridge exists in store lines51–53 but is not used here, and alone would still drain outside the inner namespace finally.

Required correction: drain an authenticated bounded task frame inside the namespace owner and nested pool callbacks before their finally blocks release anything; also enroll the actual cache consumer and law frame with exact received/returned phase. Verify successful return, Error, thrown undefined and cancellation while a real task is suspended, with namespace/pool release observed only after close/settlement. Do not substitute fake Native ports or a new Original.

## Reviewed without another blocking finding

SDK lines21–24 capture environment/args privately, authenticate control/binding before the first SDK resolution, gate synchronous hashing before/after, and issue actual requested descendants for async metadata and journal work. Escaped callable ports reauthenticate, but this does not repair the accepted-work drainage defect above. Pending count is privately bounded per port instance; canonical owner admission still needs to supply aggregate bounds.

Scope lines12–23 finite descriptor reads cap allocation before Buffer creation, demand one-link normal files and exact descriptor/path stamps, check ancestry before/after and retain descriptor-close failure with a primary including undefined. Its lstat/open checks do not supply arbitrary same-user atomicity: ancestors can be swapped and restored between observations. The record already disclaims this, so this audit makes no stronger claim. No hostile race or close-failure runtime schedule was tested here. Private cloned reader schema and primitive policy avoid mutation through exported JSON. Scope final value is ordinary DATA; SDK capture returns validated witness DATA. No fresh Original issuance was added: receive routing lines60–61 reuses the existing exact in-process invocation.

Bootstrap line511 consumes ports under actualPhase and awaits capture; this ordinary awaited call does not exercise detached work. The immutable handoff SDK lines28–33 reuses the incoming Original but creates a sibling compiler control; require-refinement must keep refusing any genuinely narrowed current law/phase it does not inherit. Full installed wrapper behavior remains unqualified here.

## Exact Source Roster

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🟦️.ts` — SHA-256 `d9a8042f22c4f802470277b40122535acd28b223934e56bcda4bc086f4c5030f`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts` — SHA-256 `4d2823f7b6daa12b81a1c246ede5c0625ff6e59242d01e2ab4353047a669965e`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` — SHA-256 `8506871dfc70c1da755c4d0de4647da9e82572291d3e823fc63e3c3e32f5807a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🎛️operation/🟦️.ts` — SHA-256 `3537a9f8cb4a069345649f8ec05b4a54b8285b4fbcaccbed7a2397feb9e1f942`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts` — SHA-256 `cb4d7dcd3d8419b11a7a1e807dfbc593e5a87944ae09adc9b7f1f440d5330490`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` — SHA-256 `f2854f3a5820c190a793ba0eae625ab4acadb468d403de7ac60c2da9548ae5b3`
