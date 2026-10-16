# Selected Graph Task Final Source Audit

2026-10-16. Read-only Source review; no tests, fake positive ports, Native/SDK/runtime/compiler/image/network/lease effects. Collector results are Root-owned and not inferred here.

## P1 — Reordered Law Task Creates a Sibling Control Branch

Graph task line79 synchronously creates caller child `actual`, then withNxGraphOperationTaskV1 creates a different child from the current operation effect. Its callback runs under this latter operation frame while requireNxGraphNativeControlV1 validates the former `actual` against that frame. Routing require-refinement line80 requires real ancestry where time/cancellation restrictions exist. A narrowed law adds a signal to its effect control (operation line30); a new child inherits that non-Original signal. The operation child is not an ancestor of `actual`, so the guard can reject genuine admitted caller work with Actual inherited control restriction required. Zero requested duration does not remove inherited signal restrictions.

The reorder correctly makes canonical caller admission synchronous before returning from the API; retain that property. Fix the sibling identity by having the law task owner accept/authenticate the actual caller phase and create its tracked child from that phase, install that same child as the accepted operation Context control, and pass that same child to work. Preserve exact storage/root/current-effect verification and current operation pending/failure registration. A dedicated explicit required-phase task seam is cleaner than silently rebasing or structural control reconstruction. No new Original is needed.

## Omitting Operation Tracking Alone Is Insufficient

Canonical caller-child enrollment plus namespace ledger retains process/control and namespace lifetime, but the operation frame has independent accepting/live transitions. If namespace custody was acquired outside a shorter law and a task escapes the law callback, operation settle (line15) sets accepting=false and sees no pending task; later graph guards in the retained reader fail even if canonical controls and namespace remain held. Canonical drainage occurs too late to keep the operation frame open. Therefore retain actual law task enrollment or explicitly constrain and verify a namespace-owned law lifetime; do not simply drop withNxGraphOperationTaskV1 to resolve the sibling mismatch.

An accepted law task's own operation frame supplies the necessary accepting=true while its parent closes admission and drains pending work. Its child must inherit both the actual caller restrictions and the actual operation signal without creating parallel authority branches. A genuine neutral refinement/drain test can cover this using real issued controls; no Native stand-in proves the physical route.

## Journal DTO Projection

SDK lines22–23 calls existing Core readFilesystemJournalScopeV1 on a fresh roots/files object. Core lines58–63 validates path DATA, rejects repeated paths, copies sorted arrays and freezes arrays/object. SDK capture line29 authenticates first, performs this synchronous immutable projection before task enqueue, then sends the captured DTO with actual child control. This prevents later caller array mutation and strips selection-only fields without exposing private observer authority. No new blocking Source finding in this projection. Replacing mocked SDK forwarding with a pure projection assertion is appropriately a Source/DATA test, not installed SDK qualification.

## Exact Source Roster

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` — SHA-256 `d321b03f892fd5500571611f59419f83b1c495c1ba374435e7dcf9b1fe5773aa`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🎛️operation/🟦️.ts` — SHA-256 `3537a9f8cb4a069345649f8ec05b4a54b8285b4fbcaccbed7a2397feb9e1f942`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/👓️reading/🟦️.ts` — SHA-256 `14904bb1681bd4c5321d1526f0a015c591f1325ab24ea584da8479928b33368a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts` — SHA-256 `d32e3bb3b1734b7284e51581494e0d13f901311ea01ff87b4d420602a24a6c71`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` — SHA-256 `f2854f3a5820c190a793ba0eae625ab4acadb468d403de7ac60c2da9548ae5b3`
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts` — SHA-256 `bb9d2fec3d4592a12a99ee747952e926b54e609e4cae6a9e0ad956d5cb196917`
