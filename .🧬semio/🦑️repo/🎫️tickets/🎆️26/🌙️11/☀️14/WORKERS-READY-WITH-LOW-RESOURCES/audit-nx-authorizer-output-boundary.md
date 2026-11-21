# Nx Authorizer Output Boundary

On 2026-11-21, the workspace stage on `g1-m05` (Windows), `g1-m11` and `g1-m12` (macOS) ended at Nx tooling publication. Each `ready-log-workspace-failed.txt` under `/Users/Shared/semio-store/v1/origins/` names `output-read` for an unpublished `.cache/b/t/m/nx/.stage-*` directory, followed by `Authenticated derivative material reference required` (reference `6313f7d6`). Staging cleanup removed the failed outputs before their lengths could be measured. Ten retained local generations ranged from 7,822,105 to 8,235,403 bytes.

The publisher compiled under a 33,554,432-byte output policy. Receipt validation and Node preflight still used an 8,388,608-byte limit. The patched Nx loader used a 16,777,216-byte file limit, and selected retirement included the compiled output in its smaller metadata budget. The receipt reproduced the exact `output-read` diagnostic using a valid receipt for an 8,388,609-byte output.

Receipt and Node preflight share the compiler's output policy and hash the original module in fixed-size chunks. The patched Nx loader applies the equivalent bounded digest read to the named bundle, with its ordinary metadata readers retaining their current bounds. Selected retirement reads the fixed output separately under the compiler ceiling and retains its digest in the generation evidence.

| Output bytes | Compiler policy | Receipt before | Receipt and Node preflight after | Patched Nx loader after |
|---:|:---:|:---:|:---:|:---:|
| 8,388,609 | Accept | Refuse | Accept | Accept |
| 33,554,432 | Accept | Refuse | Accept | Accept |
| 33,554,433 | Refuse | Refuse | Refuse | Refuse |

Both checked-in preflight tests passed through the registered runtime, including the esbuild fixture and independent Web Crypto digests. The patched Nx inspector test passed with the actual patch applied and validated by the independent `diff` library. Both selected-retirement receipt tests passed with their SQLite expectations. The isolated physical retirement fixture also authenticated and rechecked a 33,554,432-byte generation bundle. Scoped `git diff --check` passed.

At 08:47 UTC, `repo run repo:test-toolkit-preflight --skip-cache --output=stream --lang=en` was still inspecting the source inventory after its graph planning capacity wait. The three physical readiness attempts had restarted naturally. The local tests establish the size boundary, while a completed post-fix workspace attempt on each physical worker remains the fleet confirmation.

The graph-gated process subsequently exited. Its stdout and stderr were attached only to the expired PTY, so its terminal status could not be recovered. The checked-in focused tests retained their verified outcomes.

The replica notifier accepted all eight paths. By roughly 09:00 UTC the optional graph-gated local PID 98503 was absent and its original PTY handle was no longer accessible. It had last been observed working through graph inventory. Its terminal result remains unknown. Local focused boundary checks and the physical Ready retries are separate evidence.
