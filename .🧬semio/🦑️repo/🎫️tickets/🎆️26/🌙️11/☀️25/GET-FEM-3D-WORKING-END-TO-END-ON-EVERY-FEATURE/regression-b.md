# Regression B: FEM 3D editor browser audit

Driver: `node page-driver.mjs http://127.0.0.1:6087/ 6241` (ticket 22 folder), own PID stopped at the end. Bodies and screenshots: `🗑️generated/rb/`. Latency is wall-clock from the click to the observed change. Console lines are quoted from the driver log (none matched `[semio`, `panic`, `refused`, `failed` during the parts that returned).

## Build 1 (6087 as served at start of audit, before the coordinator rebuild)

| # | Item | Steps | Observed | Latency | Result |
|---|---|---|---|---|---|
| 1 | Select All | direct action | viewport hash 8946361e -> baa6d3d4 (highlight appears), no refusal | 0.9 s click | PASS (earlier audit F-A not reproduced) |
| 2 | Clear Selection | direct action | hash -> 0f233025, no refusal | 1.2 s | PASS |
| 3 | Set Selection Mode | domainId fem3d, mode Multiple | no refusal, effect not visible | 4.4 s | PASS (refusal F-B not reproduced; effect unverified) |
| 4 | Set Granularity | domainId fem3d, granularityId element | no refusal, effect not visible | 2.8 s | PASS (effect unverified) |
| 5 | Add Node | 5 5 0 | Nodes 17 -> 18 | visible by next step | PASS |
| 6 | Add Bar | n00_g -> n20_g, steel, hea200 | Elements unchanged at the 1.2 s check, count moved later | n/a | NOT CONFIRMED (lag) |
| 7 | Add Frame | n02_g -> n22_g, roll 0.2 | Elements -> 19 | 3.5 s | PASS |
| 8 | Add Material | Audit, E, G | Materials 2 -> 3 | visible <4 s | PASS |
| 9 | Add Section | AuditSec | Sections 1 -> 2 | visible <3 s | PASS |
| 10 | Add Support | n00_l2 tx,ty,tz | Supports 8 -> 9 | visible <4 s | PASS |
| 11 | Add Solid | concrete, 3x3x0.3 | Solids 1 -> 2 | visible 2.5 s | PASS (but see F3 below) |
| 12 | Add Solid unknown material | unobtainium | no visible notice captured in the logged part | n/a | NOT VERIFIED |
| 13 | Add Load Case | Wind | Load Cases 2 -> 3 | visible 2.4 s | PASS |
| 14 | Add Combination | SLS1 `dead:1.35,live:1.5`, SLS2 `dead=1.35, live=1.5`, SLS3 `1.35 dead + 1.5 live` | Combinations 1 -> 4 | visible 1.8-2.9 s | PASS |
| 15 | Add Combination invalid | `dead:x??` | no localized error captured | n/a | NOT VERIFIED |
| 16 | Add Nodal Load / Member UDL / Area Load | n20_l2 Tx 1234; fb1_0 wz -2500; sol1 700 | Execute aria-disabled `true` while a solve was pending, nothing applied | n/a | NOT VERIFIED (blocked by F3) |
| 17 | Set Self Weight, Set Analysis Settings | live; 4, 2, 150 | Execute disabled during pending solve | n/a | NOT VERIFIED (blocked by F3) |
| 18 | Re-solve after edits (F3) | after Add Solid concrete and later edits | caption `Case: dead · Preparing analysis` observed continuously from about 22:20 to 22:46 (more than 25 min, no console error); screenshot `🗑️generated/rb/stuck-preparing-45min.png` | > 25 min | FAIL (F3 persists) |
| 19 | Page reload to recover | reload | `Case: dead` after 110 s (Demo) | 110 s | PASS (slow) |
| 20 | Group A re-run after recovery | 11 edits in one body | body exceeded the 600 s curl limit (rc=28); page afterwards showed Demo counts only (Nodes 16, Elements 16, Solids 1), so the edits were not all applied and no per-step rows were returned | > 600 s | NOT VERIFIED (driver/body timeout) |

Not run in build 1: Set Result Display, Translate/Rotate/Scale, Remove Selection, Cut/Copy/Paste, Undo/Redo, checkpoints and alternatives, Export/Import, Set Active Example, Results panel controls, examples, roles, windows, settings, palette.

## Build 2

Pending: the coordinator is rebuilding Vite on 6087. At the time of this file, 6087 still answered 200 and the build-2 pass has not started. Items that FAIL on build 1 (F3, and every NOT VERIFIED row) are re-tested there.

### Build 2 (after the coordinator rebuild; 6087 back at 23:05:43, page reloaded in 109 s)

| # | Item | Steps | Observed | Latency | Result |
|---|---|---|---|---|---|
| B2-1 | Re-solve / F3 | Set Analysis Settings (4, 2, 150) on Demo | caption settled, solve 9.5 s | 9.5 s | PASS |
| B2-2 | Add Solid, unknown material `unobtainium` | Execute | Execute stayed aria-disabled for 73 s (previous solve), then the click ran; no localized error text captured in the notices read-out | 76 s | NOT VERIFIED (no notice captured) |
| B2-3 | Add Solid, concrete | second Solid add | Execute disabled for more than 90 s; result not captured | > 94 s | NOT VERIFIED (solve wait) |
| B2-4 | Select All | direct action | executed without refusal; hash comparison not captured | 2.3 s | PASS (no refusal) |

Not verified on build 2 (time spent on waiting for disabled Execute during pending solves): Add Combination invalid notice, Set Result Display, Translate/Rotate/Scale, Remove Selection, Cut, Undo/Redo, checkpoints and alternatives, Export/Import, Set Active Example and the examples, Results playback, roles, windows, Settings, command palette.

Driver on port 6241 stopped (only its own PID 50376 was killed). Note: the first `page-driver.mjs` background launch exited 127 in the notifications because of the earlier shell cwd, but it was serving requests on 6241 throughout.
