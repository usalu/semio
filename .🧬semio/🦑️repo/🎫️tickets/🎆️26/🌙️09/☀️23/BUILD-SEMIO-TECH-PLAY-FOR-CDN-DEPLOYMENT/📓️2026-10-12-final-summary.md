# Semio-Tech Play Redeploy — Final Summary (2026-10-12)

Ticket `26/09/23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT`. Close-out on the user's "Finish". Details are in the per-agent reports `📓️2026-10-1[0-2]-*.md`, the decision log `📓️2026-10-10-migration-corrections.md` (#1–#75) and the timeline in `📓️2026-10-10-coordinator-handover.md`.

## Status at close

| Gate | State |
|---|---|
| Release-path compile: hub workspace (35 compositions) on wasm32-wasip2, 132 plugin manifests native, 21/22 wasm-bindgen targets | green (renderer-wgpu, the unwired playground, excluded) |
| Cold fresh build | **#9 running** since 08:17 (`⚡️cache/fdkxMAb`, log `⚡️cache/play-fleet/coordinator/fresh-9.{log,status}`), 0 failures at 16:32 |
| Catalog / prepare-release / site build / `dist/pages` | **not yet reached in this ticket** |
| Play unit, `test-release-routing`, `test-release` (149 Chromium cases) | **not run** — need a green build first |

**Not yet verified redeployable.** P1/P2/P3 (#74) and the provenance changes (#75) landed while #9 was compiling, so #9 mixes pre- and post-fix plugin SDK builds. Ship from a final fresh build on a quiet tree (#10), then run the release gates in `📓️2026-10-10-release-verification-path.md` §4.

## Root causes fixed (selection)

- **Remote half-refactor:** a fast-forward brought a half-applied grant-based retained-clone protocol. Bottom-up migration of core, OS, plugin SDK, plugins and hubs.
- **Build tooling:**
  - Nested script invocations, and an nx child selection the script runner refused.
  - The fresh build reported exit 0 on failure (`runOwnedNxInvocationV1`).
  - The provenance path check ran before normalization.
  - The fresh-generation layout was 26 units too long (now `⚡️cache/fXXXXXX/{t,b,n,d}`).
  - Throttled provenance logging and bounded concurrency.
- **Link limits:** `wasm-dev` at opt-level 0 crossed the 1 M wasm function ceiling and the 256 MiB size bound (stdio, norm). The hub `[profile.wasm-dev] opt-level = 1` fixes it.
- **Editing:**
  - The generic `mutation_apply` and `config_apply` factories replaced the refusing bounded factory.
  - An operation wire in owner catalogs, so every store can author.
  - `RetireOwned::controlled_retirement_supported` is now required; 55 of the explicit `false` owners went to 4.
  - The typed-session close.
  - The window-config mounted publication path (re-entry, plus the missing factory).
  - Tool-run completion, batch digest and publication `is_closing`.
  - The worker session cancel alias.
  - A 10.9 MB jobs table built on the stack.
  - An archive-load close larger than its grant.
- **Performance:**
  - Bulk bitwise clone; a 4 MiB `Vec<u16>` takes 66 turns instead of about 27 M.
  - Paged png/bmp validation and paint, and a paged canonical sealer; a 64×40 bmp paint takes 1,313 turns instead of 922 k.
  - Admission reserves its exact extent; this was the close livelock.
  - Host-turn batching; payload init takes 2 turns instead of 256.
  - One `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` policy for all 78 embeddings.
- **stdio:**
  - SQLite export/import recipient and shared receiving defaults.
  - The png `ReplaceImage` panic.
  - step/ifc print, pptx detail controls, pdf17 field drift.
  - semio subsets publishing.
- **Descriptors:** generation3d inference claims, bim duplicate panel ids, describe fuel cap of 32e9.

## Tests at close

- Plugin lib: 957 ok out of about 1,230, against 759 at the start of the test waves.
- os-kernel: about 1,353 ok out of 1,466.
- Value: 307 ok out of 307.
- Replication: 346 ok out of 346.
- Hub stdio `editor_catalog`: 160 of 177, up from 4. The 17 remaining:
  - 11 editors have no edit fixture.
  - 5 fail on the lossy reopen (jpg ×2, pdf17 ua/vt/x).
  - 1 is a known debug timeout.
- `test-retained-raw-source`: 20 of 20.

## Open follow-ups

1. Final fresh build #10, then the play unit tests, `test-release-routing` and `test-release`.
2. Editor fixtures for docx, svg, las, tiff and xml. Lossy reopen for jpg and pdf17 ua/vt/x.
3. `ArtifactChild` retirement should lease the composed local owner generally; only block 3d and wires do so now.
4. Remaining plugin and os-kernel test debt, by bucket, in `📓️2026-10-11-plugin-tests.md` and `📓️2026-10-11-framework-tests.md`. Also the stale-test errors in the puzzle-5d, shooting, jack and stdio-contract test targets.
5. norm is at 227 MB, 85% of the dev-component bound.
6. Repeated EISDIR dep-info noise: replace the DSL derive directory walk with a tracked schema index (a task chip was offered).
7. The stale puzzle-5d editor version is still staged in the git index; it must never be committed.


## Update 2026-10-12 23:45 — optimization wave and close-out

The user asked to use the idle compute. Measured: build #9 ran one task at a time for 98% of its wall time (`📓️2026-10-12-build9-timeline-audit.md`). #9 also died at 16:34 with nx `RangeError: Invalid string length`, because the outer task buffered about 950 MB of child output.

- **Parallel wave (Sonnet).** Seven agents ran concurrently and machine utilization rose from about 15% to 80–97% of 10 cores. Results:
  - os-kernel lib 1465/1465 (`📓️2026-10-12-kernel-tests.md`).
  - Hub stdio `editor_catalog` 177/177 (`📓️2026-10-12-editor-catalog-complete.md`).
  - Compiler warnings −95%, from 18.5k to 920 (`📓️2026-10-12-warning-flood.md`). Its script broke 6 files; the coordinator repaired them (#76b).
  - All 222 plugin manifests compile.
- **Weekly usage limit (resets 2026-10-17 08:00).** It stopped three agents mid-work:
  - **build-parallelism.** One cargo pass per workspace (`componentPrebuildPlanV1`, `SEMIO_CARGO_PREBUILD_PLAN`), plus lease and native-build changes. It never wrote its report. Verified after it stopped:
    - fresh-build contract 8/8, `test-native-artifacts` 37/0, `test-cargo-test-leases` 3/0.
    - `test-resource-leases` fails, but only because its path-length assertion depends on the environment; the file is unmodified.
    - Hub wasm workspace check and the plugin/os-kernel lib checks are green.
  - **plugin-tests-final** and **artifact-tests.** Partial; no reports.
  - Coordinator cleanup: killed their orphaned cargo processes, deleted the `semio-sn` scratch snapshot and the private targets.
- **Build #10 launched 2026-10-12 23:40.** It is the final fresh build on the new pipeline, in `⚡️cache/fChPzBn`, logging to `⚡️cache/play-fleet/coordinator/fresh-10.{log,status}`, with `CARGO_BUILD_JOBS=10`. When `fresh-10.status` shows `exit 0`, run in order: `⚖️gate🏢️semio-tech🎡️playprefinal-complete-unit`, `⚖️gate🏢️semio-tech🎡️play🛣️routing`, then `test-release` (149 Chromium cases), all listed in `📓️2026-10-10-release-verification-path.md` §4. On a nonzero exit, read the `Failed tasks` block at the end of the log.
- **Still open after the limit resets.** In addition to the list above:
  - The build-parallelism report, and a full-build utilization measurement.
  - Remaining plugin lib failures, about 270.
  - Artifact test targets.
  - Raising the history-read release bound back to 4096 (paging the 10.5 KB registry frame).
  - The outer build-fresh output buffering. Throttled provenance logging and the warning cut shrink the stream, but buffering is not removed structurally.
