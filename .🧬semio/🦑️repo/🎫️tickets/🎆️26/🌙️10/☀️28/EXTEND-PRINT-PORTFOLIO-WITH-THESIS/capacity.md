# Verification Capacity Plan

The original thesis build session 9756/process 173076 was admitted after its initial capacity wait. It completed all 1920 input closure verifications and entered project inference. The same live process subsequently requested a child exec reservation of 3584 MB while holding its parent grant.

Repository MCP doctor (capacity only) and leases_status were inspected read-only. The lease report showed no lease cycles or ordering violations; a deadlock is not claimed. It did show simultaneous growing grants and negative memory headroom:

| Our operation | Parent process | Reserved MB | Observed MB |
| --- | --- | --- | --- |
| Thesis build | 173076 | 5382 | 3298 |
| Contract target | 175147 | 2574 | 2872 |
| PhD build | 186244 | 1638 | 2757 |

The contract and PhD runs had not reached their tests or TeX compiler. Their simultaneous graph validation consumed memory required for the original thesis build's child command. The render target remained queued without a grant.

The coordinator explicitly chose to relieve this contention: gracefully cancel only our contract process 175147 and PhD process 186244 through their owning agents, retain and observe the original thesis build 9756, and rerun the cancelled checks sequentially after that build. This is a scheduling change based on current resource evidence, not a restart due to an expired observation. Unrelated agents' jobs remain untouched. The queued rendering job was also selected for graceful cancellation: its older small request would otherwise take freed memory before the original build child exec, recreating contention. Its full unchanged matrix will run sequentially after the original build.

Cancellation completion and subsequent live states must be confirmed before launching replacements. Artifact inspection remains gated on actual successful publication; no compiler success is inferred from admission or source validation.

Contract71495/process175147 and render50782/process177409 both terminated exit143 after deliberate SIGTERM. The owning agent confirmed explicit leases-released messages and no matching commands or direct children. Neither reached tests or compiler. Replacements are not yet authorized to launch until the original thesis build completes.

PhD71002/process186244 also terminated exit143 with explicit lease release. Its owner verified the parent, children and process group are gone; it had reached1154/1920 input closures and no TeX invocation. Root build9756 remains the sole live thesis verification command. It advanced to109/128 inferred projects after resource relief; its nested exec still follows the normal shared capacity queue. No unrelated process was cancelled.

The original live command later reported worker recycling after1875MB and a partial graph result102/128 projects,1failed,5restarts. It automatically continued project inference under the same unified session9756 and reached38/128 in the next observed pass. No manual restart occurred and no TeX failure is claimed. This remains a verified live process with unresolved repository planning/resource admission, not successful artifact evidence.

## Supported Native Backend Attempt

Independent read-only exploration (backend.md) established that --tool=semio is a supported tool-neutral backend and removes Nx-specific execution. It still uses the portable graph and declared nested admissions, so no reduced graph peak is claimed. After repeated graph-worker memory restarts and child-exec waits, the coordinator deliberately replaced the five-worker Nx run with one native worker. Exact original PID173076 command identity was checked, SIGTERM yielded terminal143/leases released, and parent/known children196890/196988 were confirmed absent. Only then was replacement52427 launched with `repo --tool=semio run @semio-tech/print:build-thesis --parallel=1`. It requests3112MB through normal capacity scheduling. Same target, sources and validation scope; no bypass or concurrent duplicate.

Latest continuation is a verified wait on native52427, not a no-progress restart. Repo MCP capacity_status independently identifies native parent214878 with no own active grant;3112MB request, queue advanced from3ahead to1ahead. At955s it reported swap restrictions and169MB evaluated headroom; at1016s the same live handle reported1131MB spare and1ahead. Other owners hold the active grants; none were modified. No terminal result or compiler evidence exists yet, so the full completion gates remain open.
