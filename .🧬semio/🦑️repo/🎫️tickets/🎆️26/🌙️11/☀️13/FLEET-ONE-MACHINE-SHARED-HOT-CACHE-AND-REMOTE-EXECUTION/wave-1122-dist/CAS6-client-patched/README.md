# CAS6-client-patched: the CAS6 client half for the play snapshot (REQUESTER-ONLY, HOT-LOADABLE)

Base: the play clone `/Users/ueli/.sinc/play` as it is now (its three files are byte-identical to the running wrapper's sealed copy `4114c4d6a52d7c25`, used by cargo pid 10580 through `RUSTC_WRAPPER=.../u/bin/rustc-wrapper-4114c4d6a52d7c25`; the newer `4d134194c8bb5e85` (13:04) holds the same three files). Nothing was written to the clone or its sealed copies.

## Apply: sealed mode FIRST (picked up by every wrapper process spawned afterwards, no restart)
```
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist/CAS6-client-patched/apply.sh /Users/ueli/.sinc/play/.cache/b/r/m/u/sealed/4114c4d6a52d7c25 sealed
```
Same line for `.../sealed/4d134194c8bb5e85` if a process spawns wrappers from it. Tree mode (taken by the next seal): the script with `/Users/ueli/.sinc/play` and no second argument. Stage 1 checks every target against its pre-image sha256 and refuses on any mismatch before it changes anything (already-patched files are skipped, idempotent), stage 2 writes `<name>.cas7tmp` and renames.

## What it hot-loads (3 files under `🌐️remote-execution/`)
1. `🗄️content/🟦️.ts`: `release` (DELETE with the lease header), intent `input:<action>`, `status` on content errors, 507/5xx/408/429/unreachable retry (`retryableStatusV1`), release passed through the outage-tolerant wrapper.
2. `🚀️offload/📜️action/🟦️.ts`: the lease name `input:<first 32 hex of the action digest>` on existence checks and uploads, `release()` on the built action. Not taken (other lanes, not CAS6): the `inputs-too-large` decline.
3. `🚀️offload/🟦️.ts`: `patientUpload` (uploads retry up to 30 s with doubling pause before the local fallback), `void built.release()` in both `finally` blocks (rustc and command offload). Not taken: the live file's removal of `+patience` and the codegen-threads note.
Effect: each action's input lease ends right after the action (60 s grace instead of a fixed 30 min), and a 507 or restart of the store no longer drops a unit to a local compile at the first answer. Server side was already live.

## Files
| file | clone pre (16) | post (16) |
|---|---|---|
| `content.ts` | `bd7d7768838324d4` | `095b09107b121c7d` |
| `action.ts` | `67e10680d08bcef1` | `5aebec4caf691bfc` |
| `offload.ts` | `98b462cadf36d659` | `7dbf5f63634bbc8d` |

## Verified (scratch copy of sealed `4114c4d6a52d7c25` in the session scratchpad, 5.2 MB, deleted afterwards)
`apply.sh <copy> sealed`: 3 files written, second run `0 file(s) to write`; `bun build` (snapshot bun) of the copy's unit-cache `📜️script.ts`: ok (2.0 MB), the bundle contains `built.release()` (2x), `retryableStatusV1` (2x) and the `DELETE` send.
