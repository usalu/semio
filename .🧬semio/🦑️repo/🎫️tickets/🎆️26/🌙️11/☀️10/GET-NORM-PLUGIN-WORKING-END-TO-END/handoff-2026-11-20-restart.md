# Norm Play restart checkpoint

Recorded 2026-11-20T02:20:33+01:00. Work paused at the user's request. The existing ticket stays open. Background builds and servers were left running. A machine restart can end them.

## Resume

1. Check the modification times and receipts in `.cache/b/p/m/browser/prototype/modules/` and the last lines of the logs below. Confirm which runs published before restarting any of them. Component publication can succeed before the shared-source verification returns exit 75.
2. Read `din4108-archive-play.md` and the latest `din4108-original-replacement*` diagnostics in the generated folder. The round-three base guest `wkJLdaI` successfully hydrated the captured archive and reached 117/118 in Chromium. It predates the Stage 14 store-replacement fixes. On disk, the replacement owner now quotes the empty-source handoff correctly, accounts for outcome acknowledgement and composes its full bounded loan. The worker callback receives the active replacement identity. The older diagnostic progressed beyond bootstrap once that identity was supplied. A later zero-progress point was under investigation. Continue through actual store replacement, archive acknowledgement and sustained UI, then build all affected components. Do not count the isolated hydration success as whole-app success.
3. Read `eurocode-surface-and-example-audit.md` and the latest diagram gallery notes. Production scene data for EN 1990–1993 painted and zoomed in Chromium. Both large diagram canvases painted. The last shared gallery change accounts for the 128-node retained-document ceiling and pages multi-diagram galleries. Rerun the first/last-page subtree-limit assertions and real UI probes against this version. EN 1994 verification was waiting for the structural component.
4. The joint run `wOltbvh` compiled earlier sources; its completed previews provide intermediate evidence. All tested routes from its first module groups failed at archive transfer. Complete the final rebuild for all five modules after the archive and diagram changes. Apply the generated common shard/bridge and activate all five lanes as described in `play-worker-bridge-materialization-2026-11-20.md`. Restart the native Vite server after lane activation, then run `verify-play-components.mjs browser` on port 6033. Require all 15 actual Play routes, including example selection and editor/viewer, before closing the ticket.
5. Retain inputs and Markdown reports. Remove the ticket's generated directory only after verification, then close with the changed-file inventories in the reports.

## Live processes and evidence

- Joint build log: `🗑️generated/combined-component-build.log`. Run directory: `.cache/b/r/m/components/runs/wOltbvh`. See `🗑️generated/joint-preview.jsonl` for per-route Chromium results.
- Latest focused base build: `🗑️generated/norm-hydration-round3.log`, run `wkJLdaI`. Previous base run `wyRBFq7` is superseded.
- Real Standards Play native Vite listens on port 6033. The private preview runs on 6034. Launch via `SEMIO_TECH_PLAY_PORT=6033 repo "$ticket/serve-play-native.ts"`. Chromium requires `--in-process-gpu` and the repo's installed browser location in `PLAYWRIGHT_BROWSERS_PATH`.
- All 79 typed DSL example round trips passed. See `play-verification-2026-11-19.md`, `iso-vdi-din-typed-examples-2026-11-20.md` and `curated-byte-cohort-2026-11-20.md`. These checks do not exercise the mounted app.
- The original Nov 19 handoff retains the normal build and lane commands. `component-warm-rebuild-audit.md` documents incremental build constraints. Check the live disk and compiler leases before starting a duplicate.

## Build counters at pause

- combined-component-build — [native-build] prototype-00-b0 prototype 🌎️hub: exit 0, 4/4 component(s) linked, 1531 unit(s) in 6818 s
- norm-hydration-round3 — [native-build] 664 unit(s), 1 component(s) staged, 10 stage run(s), stage cache 0 hit(s)/3 miss(es), provenance 7879 file(s) 5964 MiB, 0 failure(s) in 1809 s; report /Users/ueli/Documents/semio/.cache/b/r/m/components/runs/wkJLdaI/r/report.md
