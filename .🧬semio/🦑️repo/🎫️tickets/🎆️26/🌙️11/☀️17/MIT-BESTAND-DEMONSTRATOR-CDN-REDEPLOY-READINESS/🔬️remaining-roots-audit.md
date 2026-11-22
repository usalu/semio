# 🔬️ Remaining Roots Audit

Read-only audit of roster modules not yet confirmed (`🗑️generated/confirmed/*.json`). Roster = `pluginIds` + `extensionIds` in `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json` (32 modules). Confirmed: 10 (cad, energy, fem, flow-extension-bim, flow-extension-dictionary, gis, procedural, process, puzzle, sourcing-module-windows). Not confirmed: 22.

Log timestamps are file mtimes (local time); the logs carry no internal timestamps. "Last log" = newest `browser-<module>.log*` or pair log that mentions the module. Pair logs are named `browser-pair-<a>+<b>.log`.

## Table

| module | last log time | cause | still present? |
|---|---|---|---|
| demonstrator | 2026-11-22 05:33 (`.log.prev`) | lease-order: `LeaseOrderErrorV1` (`SEMIO_LEASE_ORDER` enforce). `artifact:.../components/c42e5b019a72cb8cdafb58f2` exclusive unavailable 15 s while process holds `artifact-scratch-private:a606f2...` | Mechanism yes (`🏃️process/🔒️leases/🧭️order/🟦️.ts:262`, enforce throws). Contention is runtime-only, not reproducible statically |
| flow | 2026-11-21 21:13 | taxonomy: `generatorContracts["cad-geometry-bindings"]` and `["spatial-kernel-session-bindings"]` lack `previewTarget` and have non-sorted or duplicate `inputPatterns` | No. `🔣️taxonomy.json` modified 2026-11-21 21:26:45 (after the failure). Both contracts now have `previewTarget` and sorted unique patterns; all 35 owned contracts pass a static check. Validator not executed |
| playbook | 2026-11-21 21:13 | taxonomy (same two contracts) | No (same as flow) |
| sourcing | 2026-11-21 22:12 (`.log.prev`) | emitter stale: 2 files, first `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs` | Yes, recurring. Emitter rebuilt 2026-11-22 04:22; 129 `.rs`/`.ts`/`Cargo.toml` under `🧰️framework` are newer than that binary (edited 04:23 to 05:40). A new build would fail the same check |
| cad-extension-aec-building | 2026-11-21 21:14 | taxonomy (same two contracts) | No (same as flow) |
| cad-extension-aec-building-energy | 2026-11-21 21:14 | taxonomy | No |
| cad-extension-aec-building-structure | 2026-11-21 21:14 | taxonomy | No |
| cad-extension-spatial-shape | 2026-11-21 21:14 | taxonomy | No |
| flow-extension-brep | 2026-11-21 21:14 | taxonomy | No |
| flow-extension-draw | 2026-11-21 21:14 | taxonomy | No |
| flow-extension-list | 2026-11-21 21:14 | taxonomy | No |
| flow-extension-logic | 2026-11-21 21:14 | taxonomy | No |
| flow-extension-math | 2026-11-21 21:42 (own log: SIGTERM cancel). Pair `flow-extension-text+flow-extension-math` 21:41 | emitter stale (pair log). Own log ends in operator SIGTERM, not a failure | Yes, recurring (see flow-extension-text) |
| flow-extension-primitive | 2026-11-21 21:14 | taxonomy | No |
| flow-extension-text | 2026-11-21 21:42 | emitter stale: 124 files, first `🧰️framework/📦️packages/🦀️rust/🦀️.rs` | Yes, recurring (129 newer files now) |
| playbook-module-procedural | 2026-11-21 21:14 | taxonomy | No |
| process-extension-concrete | 2026-11-21 21:14 | taxonomy | No |
| process-extension-metal | 2026-11-21 21:13 | taxonomy | No |
| process-extension-robotic | 2026-11-21 21:14 | taxonomy | No |
| process-extension-wood | 2026-11-21 21:13 | taxonomy | No |
| sourcing-module-beams | 2026-11-21 21:13 | taxonomy | No |
| sourcing-module-slabs | 2026-11-21 21:13 | taxonomy | No |

## Compile errors, ENOSPC, describe failures

- No roster module (unconfirmed) has a compile error in its newest log. The `cargo-release-00.jsonl` files were not needed for that.
- No `ENOSPC` or "No space left" in any `browser-*` log, including the `.gz` files.
- Compile errors exist only in logs of confirmed modules or historical logs. None is in an unconfirmed module's newest log:
  - `E0063` `NaturalFileImportJob` missing `pack_export`, `pending_genesis`, `publication_work` (+1). `semio-plugin-decl-surface`, `📥️natural-file/🦀️.rs:450:18`. `browser-flow-extension-dictionary.log.prev`, 2026-11-19. Dictionary is confirmed.
  - `E0061` method takes 5 arguments, got 4. `semio-plugin-window-config`, `🧭️owner-registry/🦀️.rs:185:220`. `browser-sourcing-module-windows.log.prev`, 2026-11-20. Windows is confirmed.
  - `E0599` `original_report_key` not found on `ArtifactS...`. `browser-procedural.log.prev`, 2026-11-22 01:49. Procedural is confirmed. The method now exists at `🏪️store/⚙️engine/🔭️derived-replay/📜️report-replay/🦀️.rs:25`.
  - `browser-process.log.prev` (2026-11-21 14:25): errors in `semio-s-artifact-process-process3d-editor`. Process is confirmed.
- Older describe failures (`browser-flow.log.r2`, 2026-11-18): guest trap, `[semio-plugin panic] ... decl-registry-0.1.0/../../🌳️tree/🦀️.rs:183:33`. Superseded by the 2026-11-21 taxonomy failure for flow. The site was not located in current source.
- Older `browser-partA/B/C/rest` `.gz` logs (2026-11-18/19) and `browser-partD.log` contain source-admission and describe lines only, no compile error for unconfirmed modules.

## Summary

- 21 of 22 unconfirmed modules last failed on 2026-11-21 with the taxonomy error, or on the emitter-stale check. The taxonomy error no longer reproduces statically.
- Emitter stale is the blocker that still applies. Rebuild `@semio-tech/os-plugin-describe-rs:build` before the next run, otherwise sourcing and flow-extension-text (and flow-extension-math via its pair) fail again.
- demonstrator fails on a lease-order violation (most recent, 2026-11-22 05:33). That is a runtime contention issue, not fixed by a source change.
- No unconfirmed module is blocked by a compile error in the current logs.
