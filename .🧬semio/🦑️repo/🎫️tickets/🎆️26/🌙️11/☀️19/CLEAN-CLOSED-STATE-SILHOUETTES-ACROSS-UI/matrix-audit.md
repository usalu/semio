# 🧭 App Sweep Matrix Audit

Read-only source audit; no browser or sweep executed. Historical results below are claims from SUMMARY.md, not new verification.

## Routes and coverage

| Surface | Server | Entry | Scope and recorded results |
|---|---|---|---|
| Elements | 5452 | / | Ticket story harness; sweep-boot.js iterates round3.list(), limit 30 each. Historical 126 stories, 354 targets, 0 failures. |
| Course | 5601 | / | Course config base /; historical English 1440, 919, 768, 375 and German 375. Current page only. |
| OS | 5602 | / | Prior ticket showcase harness, not the complete OS app; os-sweep.js iterates 72 partNames, limit 20 per part. Historical 1440, 548 targets, 0 failures. |
| CAD | 5603 | / | Prior ticket CAD plugin harness; historical 1440, 40 targets, 0 failures. |
| Hub admin | 5608 | /admin/ | Actual Vite base is /admin/; / is not reliable as an app entry. Prior ticket config mocks admin APIs, so results cover fixture state, not live backend. Historical 1440, 10 targets, 0 failures. |
| Demonstrator | 5609 | / | Vite base ./; frozen watcher/HMR, historical 1440, 15 targets, 0 failures. |
| Staffage | 5471 | / | App startup configured; historical sweep blocked by main-thread saturation, hence no passing live claim. |
| Play | 5397 | / | Vite base ./; frozen watcher/HMR, historical 1440 50 targets and 375 7 targets, 0 failures. |

apps-start.sh health checks / for every port, which does not establish that /admin/ is served correctly, nor that any app is ready. It starts Vite directly, kills matching processes, fixes absolute paths to this machine and depends on another ticket’s configs; it is not a cross-platform or repo-command invocation. It also checks every port even when invoked for a single app. Use repository-managed serve targets where available and verify the proper entry route.

## Probe invocation and limits

Load app-sweep.js into each already-open page with the elements server available at 5452; then invoke await appSweep(name, limit) and inspect appState(name). For a new matrix cell use a unique name for app, route, locale, viewport and state, or clear its sessionStorage key. The storage key only includes name and innerWidth: equal-width heights, locales, routes and revisions otherwise reuse stale results. Set keyboard modality before running so :focus-visible actually matches; historical live-round6.md confirms pointer modality silently omitted focus checks. For elements use runSweep(uniqueName) and sweepState(uniqueName); for OS use osRun() and osState(), clearing the os:<width> sessionStorage key for each distinct run. These are browser-console APIs, not automated route or locale orchestration.

silhouette.sweep takes the first limit visible, enabled controls in DOM order (app default 60, elements 30, OS 20); it does not scroll, navigate, open dialogs, select tabs or enumerate app states. It sends synthetic pointerover, which does not reproduce CSS :hover, and programmatic focus is checked only when :focus-visible matches. It does not exercise selected, pressed, invalid, loading or other requested states. Off-viewport and clipped boxes are skipped; skips are not counted as failures. Shadow DOM, canvas and native rendering are not traversed. A zero therefore means no reported issue among checked targets, not full UI or state coverage. appSweep records done=true even if the limit truncates targets, and deduplicates/truncates issue strings. A reload can change DOM ordering and invalidate its stored index. The SVG parser handles only a subset of path commands and its geometry/visibility heuristics are approximations.

## Locale matrix

Course document/config describes English and German with no assumed language; historical run covers both only at 375. Hub admin explicitly declares en/de, resolves browser preferences or shows its chooser, and exposes a language switch. Demonstrator explicitly declares en/de, reads stored UI locale then browser preferences or shows a chooser, and exposes a switch. Staffage explicitly offers English and German, resolving stored choice then browser preferences or showing a gate. Prior records do not establish separate en/de runs for hub, demonstrator, staffage, play, CAD or OS. Locale and route must be selected and verified per cell before probing; stored choices can override browser preferences. Play, CAD, OS and elements locale support was not established by this audit.

## Source references

apps-start.sh, app-sweep.js, silhouette-probe.js, sweep-boot.js, os-sweep.js, live-baseline.md, live-round6.md, live-os.md and SUMMARY.md in this ticket; prior ticket vite-preview.config.ts, vite-round3-os.config.ts, vite-round3-cad.config.ts, vite-hub-admin.config.ts, r3-vite-demonstrator-frozen.config.ts and vite-play-frozen.config.ts; app Vite and locale sources under course, hub admin, demonstrator and staffage.
