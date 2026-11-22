# Generator wRATjNi Linked Guest Probe

Command: `PROBE_SITE=<private clone> bash probe-linked.sh procedural wRATjNi 8793 both` (acceptance boot test, `generator boot`, workers=1, one browser). `probe-linked.sh` gained an optional `PROBE_SITE` override (skips the captured-site lookup). The site is a copy-on-write clone of captured Vite release `5dc1b077e86de2aa` with `purpose:{tag:"ordinary"}` inserted into both stale `instance-open` producers (`assets/🟦️-x1UQdzrR.js`: 1 payload producer + 1 poll producer; `assets/🌐️-DLq6yBBu.js`: 1 payload producer). Diagnostic only; clone deleted afterwards. Guest bytes verified by `probe-custody` (core SHA-256 39cf30c3...c6cc6, 159,530,089 bytes). The `purpose` boot refusal is gone.

## Result: FAIL in both locales

| locale | result | time |
|---|---|---|
| en | FAIL | 18.5 s |
| de | FAIL | 16.9 s |

Failing assertions (identical):

1. `pane "generator": expected its shell to reach "ready"` (received `error`). The shell alert is `typed-operation failed: {"code":"interactive-job.publication-stalled","message":"typed operation 'setActiveExample' advanced 4096 publication units without moving its ladder", retainedProgress all 0, retryable false}`.
2. `unexpected browser console errors`: a single leaked guest stderr line `[DEBUG] boxclose#0 grant=RetainedCloneGrant { maximum_items: 5652011, maximum_copy_bytes: 2094757945, maximum_capacity_bytes: 263862856, maximum_release_bytes: 467039831, maximum_depth: 64 } work=47727873 depth=Ok(2) term=false`.

## What this means

- The empty-preview fix (display writer batching) was never reached: the Generator no longer gets to a ready shell, so the preview cannot be judged. The stall is the same symptom family as Demonstrator `wd3qizX` (`🔬️demonstrator-wd3qizX-probe.md`, `🔬️stage24-archive-stall.md`): a typed-operation publication that waits on a live document archive load / store replacement that never moves. The earlier wmr64VB Generator build completed `setActiveExample`; this build regresses it.
- The `boxclose#0` line is emitted at about 3.4 s after boot, right after maintenance stage 104, and before the stall. It is a Box (depth 2) close with a demand of 5,652,011 items, 2.09 GB copy, 263 MB capacity, 467 MB release and work 47,727,873, against a mounted per-turn ceiling of 64 items, 32 KiB copy, 16 MiB capacity and 1 MiB release. A close this large can never be funded by one mounted turn, and `term=false` means it did not terminate. This is the leading candidate for the never-moving ladder (an unfundable retirement quote, i.e. funding shortfall rather than a lost wake).
- The string `boxclose#` is present in the wRATjNi core but exists nowhere in the current source tree (searched all `*.rs` in `🧰️framework` and `✏️s`): the build contained a peer's temporary debug print that has since been removed, so the emitting owner cannot be named statically. No source fix was made: without the owner, any change would be a guess (relabelling or raising limits is unsound per earlier audits).
- Console trace (diagnostics run, 45 s): the instance keeps handling `command_ingress` more-work turns (stage 16/104/19 maintenance all `Ok`), the preview host never appears (`host:false, meshes:null`), status stays "local only".

## Next evidence needed

1. Identify the emitter of `boxclose#` in the current tree (git history of the peer's edit) and print its owner type; quote of 5.65M items suggests a retained Source/Value tree sized by the whole Generator catalog or pack rather than by one document.
2. Rebuild Procedural with that witness and rerun this probe; the red regression is a mounted-grant law asserting the owner closes under the per-turn ceiling.
3. Only after `setActiveExample` completes can the display-writer fix be verified (`generator-preview-content-probe.cjs`).

Procedural retry: not queued (no source change).

## Status

No servers or browsers remain running. The private site clone is removed; probe output directories deleted.
