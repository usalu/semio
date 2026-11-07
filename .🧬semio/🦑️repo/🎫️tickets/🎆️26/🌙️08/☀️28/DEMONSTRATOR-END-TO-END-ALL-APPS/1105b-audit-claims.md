# 📋 Wave 1105b claims audit (aud-claims)

Checked at 13:50 against the newest non-empty compile log per lane in `🗑️generated/1105b/`. Zero-byte newest logs (energie-t10, wit-13, proc-13, store-t6, bearbeiten-13, aussuchen-16) were skipped.

Green newest logs (0 `error[`): koordinator-w17 (13:49), statik-3 (13:49), aggregator-a14 (13:49), runtime-n2 (13:49), renames-21 (13:50), wit-11 (13:42), energie-s1 (13:43), e2e-test-2 (13:50), actinfra-comp (13:50).

Red logs used: aussuchen-15 (13:48), flow-16 (13:48), ext-3-playbook-module-procedural (13:48), store-5 (13:45), proc-12 (13:39), bearbeiten-12 (13:28), verfolgen-11 (13:08), hub-9 (12:48).

## Red crates, newest log first

| Crate | Errors | Newest red log | First error file:line (tail) | Claim status |
|---|---|---|---|---|
| semio-s-artifact-stdio-txt-schema | 25 | aussuchen-15 13:48 | `sqlite/snapshot/🦀️.rs:43:135` | CLAIMED flow 13:34, still red |
| semio-s-artifact-stdio-deflate-snapshot | 36 | aussuchen-15 13:48 | `sqlite/snapshot/🦀️.rs:8:155` | CLAIMED flow 13:34, still red (absent from flow-16 13:48, unexplained) |
| semio-s-artifact-stdio-semio-image-editor | 3 | aussuchen-15 13:48 | `image/editor/🦀️.rs:185:5` | CLAIMED aussuchen 13:48 |
| semio-s-artifact-stdio-semio-flow-io | 3 | aussuchen-15 13:48 | `flow/io/editor/child/🦀️.rs:12:174` | CLAIMED aussuchen 13:49 |
| semio-s-artifact-stdio-semio-drawing-snapshot | 4 | store-5 13:45 | `sqlite/snapshot/🦀️.rs:48:281` (bearbeiten-12 13:28: 2 errors) | UNCLAIMED, statik released 13:32 ("needs design") |
| semio-s-artifact-procedural-retained-source | 12 | proc-12 13:39 | `flow-source/scene/🧷️base.rs:16:201` (store-5: 3 errors) | UNCLAIMED |
| semio-s-artifact-procedural-3d-model | 4 | store-5 13:45 | `generation3d/sqlite-codec/🦀️.rs:8:550` | UNCLAIMED |
| semio-s-artifact-energy-model-session | 1 | store-5 13:45 | `weather/🦀️.rs:221:85` | UNCLAIMED |
| semio-s-artifact-stdio-step-snapshot | 1 | store-5 13:45 | `sqlite/snapshot/native/🦀️.rs:86:151` | CLAIMED koordinator 13:25 (batch) |
| semio-s-artifact-stdio-semio-mesh-editor | 2 | store-5 13:45 | `editor/🦀️.rs:82:38` | CLAIMED koordinator 13:37 |
| semio-s-artifact-stdio-semio-brep-io | 2 | store-5 13:45 | `brep/io/editor/🦀️.rs:81:38` | CLAIMED koordinator 13:37 |
| semio-s-artifact-stdio-semio-text | 1 | store-5 13:45 | `text/editor/🦀️.rs:88:38` | CLAIMED store 13:26, koordinator 13:37 |
| semio-s-artifact-stdio-semio-audio, -video, -animation-editor | 1 each | store-5 13:45 | `editor/🦀️.rs:88:38` | WEAK: only koordinator 13:36 queue and 13:37 "..." |
| semio-s-artifact-stdio-semio-document-editor, -model-io, -value | 1 each (value: 3 in bearbeiten-12) | store-5 13:45 | `editor/🦀️.rs:88:38` | WEAK: only koordinator 13:36 queue |
| semio-s-artifact-fem-2d-model | 2 | hub-9 12:48 | `snapshot/native.rs:99:9` | UNCLAIMED, not in any newer log (stale or resolved) |
| semio-s-artifact-procedural-2d-editor-presence | 1 | hub-9 12:48 | `editor/presence/🦀️.rs:42:9` | UNCLAIMED, not in any newer log (stale or resolved) |
| semio-s-procedural-core-generation-sqlite-native | 3 | aggregator-a13 13:34 | `document/🦀️.rs:42:2` | CLAIMED aggregator 13:35, edited 13:36, GREEN in aggregator-a14 13:49 (no FIXED line) |
| semio-os-flow-host | 1 | bearbeiten-12 13:28 | `publication/pending/🦀️.rs:16:160` | FIXED proc 13:29, GREEN in proc-12 13:39 |
| semio-plugin-decl-app, semio-plugin-guest-contract | 2 / 1 | hub-9 12:48 | `turn-output/🦀️.rs:4:29`, `cold-job/🦀️.rs:22:25` | FIXED koordinator 12:57, GREEN in koordinator-w17 13:49 |
| semio-plugin-reactor-wit-bridge | 72 | verfolgen-11 13:08 | `bridge/🦀️.rs:203:9` | FIXED 13:14 (static), GREEN in wit-11 13:42 |
| semio-plugin-reactor-jobs | 2 | verfolgen-11 13:08 | `jobs/🦀️.rs:692:77` | FIXED flow 13:15, not red in flow-16 13:48 |

## Flags

**(a) UNCLAIMED red crates**
- semio-s-artifact-stdio-semio-drawing-snapshot (red in store-5 13:45)
- semio-s-artifact-procedural-retained-source (red in proc-12 13:39 and store-5 13:45)
- semio-s-artifact-procedural-3d-model (red in store-5 13:45)
- semio-s-artifact-energy-model-session (red in store-5 13:45)
- semio-s-artifact-fem-2d-model and semio-s-artifact-procedural-2d-editor-presence (red only in hub-9 12:48, status unknown)

**(b) FIXED but still red in a newer log**
- semio-s-artifact-stdio-semio-drawing-snapshot: statik FIXED line 13:24 says "patched, compile unproven". The 13:32 statik line says 4 errors remain. Still red in bearbeiten-12 13:28 and store-5 13:45. The FIXED line is stale.

**(c) CLAIMED more than 60 min ago with no FIXED line (now 13:50)**
- store 12:29: all ArtifactSqliteSnapshotHost implementors via `1105b-tools/store/port_native.py`. Marked UNVERIFIED, never FIXED. Store family still red in store-5 13:45.
- value 11:56: all callers of the old 2-generic `owner.receive`. No FIXED line. Trinity io was UNVERIFIED at 13:04.
- wit 11:56: exports.rs/macro/actor-crate impls. No FIXED line. Likely resolved, since wit-11 13:42 is green.

**Other notes**
- Newest logs for store, energie, wit, proc and bearbeiten are zero-byte, so their status is only as of their last non-empty log.
- The verfolgen and hub logs predate most FIXED lines and are not current evidence.
