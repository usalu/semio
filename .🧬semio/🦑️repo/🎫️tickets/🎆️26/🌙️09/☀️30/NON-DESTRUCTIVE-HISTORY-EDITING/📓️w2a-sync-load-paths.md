# Remaining synchronous whole-document load paths (audit W2A-6) — 2026-10-03 12:10

This census was taken with S3-W2A for the split with S3-LOAD. Every whole-document load goes through the stepped document archive
load: admit, poll, acknowledge (see `📓️api-stepped-document-load.md`). Senders no longer emit `AppCommand::LoadDocument`
(S3-LOAD, `DocumentArchiveLoadHost`).

The census uses `git grep`, which counts tracked files only:

- `load_document_pack(` and `load_document_text(` appear at **96 sites in 56 files**.
- `AppCommand::LoadDocument` has **17 references**.

## 1. Production paths

| # | Path | Where | Disposition | Owner |
|---|------|-------|-------------|-------|
| P1 | `AppCommand::LoadDocument` handler | `🔌️plugin/🦀️.rs` ≈43718 (`plugin_load_document_pack`) | Delete with the command tag | coordinator, §20.7 bump wave |
| P2 | `AppCommand::LoadDocument` codec, tag 6 | `📡️spr/🧵️channel/🦀️.rs` 1978/1981/2007/2010/2154/2347/3405/3793 + unit tests 98/632/810 | Delete in the bump wave | coordinator, bump wave |
| P3 | Decoder arms that answer `LoadDocument` | `🌉️mcp/🏠️workspace/🦀️.rs:2178`, `🏃️run/🦀️.rs:2265` | Delete in the bump wave (no sender left) | S3-LOAD, bump wave |
| P4 | `PluginApp::load_document_pack/text`, `plugin_load_document_pack/text` | `🔌️plugin/🦀️.rs` 14610/14616, 34740/34755, 41610/41730 | Leave the host-facing surface in the bump wave, once T1–T3 have moved | S3-W2A |
| P5 | `consume_media` default: a `Document{schema}` media artifact loads through `load_document_pack` | `🔌️plugin/🦀️.rs` ≈14718 | **Live sync path.** Move it onto a guest-driven archive load, the same reactor queue as checkpoint restore | S3-W2A |
| P6 | `hydrate_document_lane` (pure command) | `🔌️plugin/🦀️.rs` | Already head-only, O(snapshot), no fold (decision §4 b). Keep | — |
| P7 | Checkpoint restore and cold pair | `⚛️reactor/📸️checkpoint`, `⚛️reactor/🔄️turn` | Done: stepped archive loads, driven by the turn | S3-W2A (done) |

## 2. Test and tooling paths (T)

These call the synchronous trait method directly. They need one harness helper so that every law proves the stepped path.

The helper is `artifact_app_laws::load_document(app, &ArtifactPackFiles)`, plus `load_document_text(app, &ArtifactTextFiles)`
for text pairs. It:

1. stamps the pair with the live store's identity, which is exactly what the runtime does to an `Effect::LoadDocument`;
2. admits it as a document archive load (`members: []`);
3. polls it until it is terminal;
4. acknowledges it;
5. answers the archive's own fault when it does not land.

S3-W2A writes the helper in the plugin crate.

| # | Sites | Owner |
|---|-------|-------|
| T1 | Plugin crate: `🧪️time-travel` 6, `🧪️history-label-reload` 2, `🧪️history-edit-acceptance` 2, builder contract 2, `declared_verb_fixture_app` boot example 1 (`🦀️.rs` ≈8368) | S3-W2A |
| T2 | `✏️s` plugin tests: about 45 files, among them `🗄️stdio` ×17, `🖨️raster` ×6, reasoning `🔌️wires` ×5, `🏗️fem` ×4, note ×3, cad ×3, and puzzle 2d `history-edit-runtime` 2 (S3-W2A's own law) | S3-LOAD sweeps them onto the helper; S3-W2A takes puzzle 2d |
| T3 | `✏️s/🧑‍💻dev/🧩️composition` tests (2) | S3-LOAD |

## 3. Order

1. S3-W2A lands the helper and moves P5 and T1.
2. S3-LOAD sweeps T2 and T3.
3. The coordinator's bump wave deletes P1–P3 and drops P4 from `PluginApp`. No caller is left by then, because the helper builds on
   `begin/poll/acknowledge_document_archive_load`.

## 4. Session 4 status (S4-LOAD, 2026-10-04)

| # | Status |
|---|--------|
| P1–P3 | Gone with S4-BUMP's channel bump (v21): `AppCommand::LoadDocument`, its codec, the guest arm, `plugin_load_document_pack`, the exhaustive seq arms. |
| P4 | Gone: `PluginApp::load_document_pack/text`, their `VcsArtifactApp` impls and `plugin_runtime::plugin_load_document_text` (03:31, plugin lib check green 03:52). |
| P5 | Closed: `consume_media` answers `MediaConsumption::DocumentLoad(archive)`; the runtime admits it under the `MediaIn`'s seq and answers `DocumentArchiveLoad{Pending}`; hosts drive it with `DocumentArchiveLoadHost::admitted` (`📓️api-stepped-document-load.md` §9). |
| P6 | Replaced: `PureCommand{seq, command, head}` + `PluginApp::hydrate_pure_head` (no `resolve_ready`). |
| T1 | Done by a peer before session 4 (9 plugin-crate sites on `artifact_app_laws::load_document_text`) + builder contract by S4-LOAD. |
| T2/T3 | Done: 72 sites in 53 `✏️s` files (`🧪️s4-load-sweep-document-loads.py`); hub close-ladder/idle-turns onto `artifact_app_laws::plugin_load_document[_text]`. Compile of the touched ✏️s/hub test targets OWED (rule 43). |
