# Process Extensions Report (metal, wood, robotic, concrete rust)

Status: NOT verified green. No source file was changed. Blocked by repo infrastructure.

## Attempts
1. `repo test @semio-tech/process-extension-wood-rust` (MCP): ran 34m, graph slice worker failed 3x with exit 75:
   `Export named 'admissionReserveBytesV1' not found` in framework native-build admission module
   (launcher `🚪️launcher/🟦️.ts` imported it before it existed; another agent added it minutes later).
2. Native `cargo test --workspace` in each extension workspace: refused by the repo's compiler ingress
   ("Original compiler ingress requires the repository launcher and authentic enforcing store"). Policy, not bypassed.
3. `mcp__repo__cargo check` gate: same missing-export failure at the time.
4. Retried after the export landed: `Closed host capacity Policy required: /swap/activeMegabytesPerSecond is required`,
   then `/swap/activeBasisPointsPerSecond is required`: host-capacity schema and `🏗️build/🔣️policy.json`
   were being migrated concurrently by another agent (basis points vs megabytes), flapping between each other.
5. Retry once both were consistent: ran 1h48m, stuck in "Reading the project graph slice" (file inventory
   6447 of 600), then the resident service crashed (ref 639a8c20, log
   /var/folders/mr/6b0sgm814b5bkd4zfm3tf5sm0000gn/T/semio-repo/501/host-b74e39ba1273.log).

## Not done
package/test targets and component builds of all four extensions remain unchecked. Re-run once the framework
repo-tooling migration (host-capacity policy, admission) settles and the graph slice loads.
