# UI Card and Band React Index Registrar Acceptance

Both Terra source leases reported their disjoint component/story changes complete before the coordinator touched the shared React package index.

The coordinator serialized the shared edits:

1. Rehashed the index at `f6936957c8044acaa7af426e671d9a9fe83491ca2c2b4146c9b6a242e77c1aa2`, preserving the accepted Steps removal, then removed only the Card import/export region. Intermediate SHA-256: `dec9ae039492af1ac9c751120ef66e9ed2f15d2bd2684f45a9c8a4b249e5050a`.
2. Rehashed again, then removed only the Band import/export region. Final SHA-256: `7872a8bcbcf3990d623d0dc4486e8b16e199c7cd0f053fb9c76ab2b0cd9d2eb6`.

During this registrar sequence, HEAD advanced externally from `0727b80aa6a802cac1760f90fb7a148f74035413` to `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`. Read-only reconciliation showed the new HEAD already contains the accepted Steps registrar deletion. Against the new HEAD, the current index ordinary diff is exactly ten Card/Band deletion lines, the cached diff is empty, and scoped `git diff --check` passes.

The final index has no Steps, Card/CardGrid, or Band registration. Distinct `StatCard` remains untouched. Both Terra leases were signalled to complete active/excluded reference scans and the JavaScript Nx gates without editing the index.
