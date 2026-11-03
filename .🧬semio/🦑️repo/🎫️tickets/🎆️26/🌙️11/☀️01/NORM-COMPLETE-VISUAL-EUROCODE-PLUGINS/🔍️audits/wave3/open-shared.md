# 🧭 W3-OPEN-SHARED report (2026-11-03)

Scope: groups "shared" (100 items) and "infra" (13 items) of `open-items.md`. No cargo, no git writes, no heavy.sh needed (grep/sed only).

## Counts

- Verified done (code checked, `[done W3-OPEN-SHARED]` line with original line number appended to `📌️important/requests.md`): 40 of the 51 "likely done: verify" items.
- Fixed at the source (also appended as done lines, uncompiled): 3 (lines 1880, 1877, 18).
- "likely done" still unverified (21 ids, a mention-only match whose proof is a compile or a manifest/gate run): 31, 133, 135, 220, 248, 255, 256, 269, 291 (compile/test requests), 290 (needs the printed standard), 374, 377, 433 (build-owners: 44 missing rows are down to 7, all new en1991/en1993 crates of locked agents), 808, 939, 1272, 1273, 1536, 1691, 1736, 1788, 1891, 1972.
- Still open (not touched): the remaining "open" shared items. Largest ones: 8, 61, 62, 134, 205, 206, 216 (five non-Eurocode apps still use the default full-evaluate `live_report`), 271, 344, 349 (host merge of world-3d binding args), 493, 497, 528 (English `Err(String)` reasons in catalogue evaluators), 617, 669, 863 (English-only diagnostics, 40 sites), 1063, 1131, 1132, 1270, 1878 (`Tier`/`InspectorOptions` Default, needs a UI-spec decision; 12 test callers), 657/1129 (blocked on viewport-config green build).
- Skipped by rule: build-queue items (194, 195, 297, 371, 455, 470, 1245) and print/human items (1200, 1271).
- Locks: no shared/infra item sat under an active owner lock; the shared crates edited (mechanics, app-surface checks) are not named in any open lock.

## Fixes (all uncompiled)

1. 1880: umbrella `📜️script.ts` for `🪟️results/🎚️config` and `🪟️viewport/🎚️config` (identical gap; mirrors `🧮️mechanics`).
2. 1877: `SectionOptions::default()` replaced by `SectionOptions::standard()` in `🧮️mechanics/🧬️schema/🦀️.rs`, `🔲️section`, and the tests (4 sites).
3. 18: `UtilisationBand::Unverified` (glyph `?`, fill `▚`, warning tone) so an unverified-basis check no longer shares the warning glyph; test-first in `📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/✅️checks/🧪️tests/🦀️.rs` (band, `? Unverified basis`, `? Ungeprüfte Grundlage`, five distinct fills).

Rebuild: "- ready for rebuild: open-shared" appended to `🗑️generated/build-mechanics/latest.md` and `🗑️generated/integrate/latest.md` (app-surface).
