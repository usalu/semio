# WP CX1 — stdio txt/tsv/html Native Codecs + Nx Dist-Wasm Inputs

Slice CX1, session 13 (Claude Code fleet, Opus executor). Coordinator = `main`. Prepared patch, lands in window 3 only
(rule 33: no source edits after 15:45 until the final chain reports 7800 READY). Patch: `wp-cx1/cx1-apply.py`
(`--dry-run` default / `--write`), payloads `wp-cx1/payload/`. Overlay: `.🧬semio/🌐hub/s13-cx1-overlay/` (gitignored),
private build dirs `.🧬semio/🌐hub/s13-cx1-{build,target}`. Captures: `wp-cx1/generated/`.

## Session 13

| # | Item | Status | Evidence |
|---|------|--------|----------|
| 1 | txt/tsv/html native codec factories (stdio registry, receipts JSON, fence 26 → 29, descriptors) + laws + fixture + oracle | patch written (80 hunks / 34 files); dry run live **0 problems** (16:0x); applied in overlay; Python oracle **7/7**; Rust laws: build running | `cx1-apply.py`, `generated/dry-live-1.txt`, `write-overlay-1.txt` |
| 2 | publisher law `⛓️linked-codec-ownership` + T12 census law: stdio unowned = ∅ | in the patch (real-registry vitest case; census law gains the linked-codec check) — not run yet | `cx1-apply.py` |
| 3 | Nx: registry targets declare consumed dist wasm (`dependentTasksOutputFiles`), drop `--skip-nx-cache` | exploring | — |

### Log

- 14:58 started; read AGENTS.md, preamble rules 1–33, `📓️wp-lb.md` 13:2x blocker + follow-up.
- 15:0x scope census (git grep, read-only): the 26/28/29 stdio codec count is pinned in ~25 places — stdio registry (6 checks +
  unit ledger + schema JSON + catalog-surface fixture + home-io fixture/schema/TS type), stdio `native_openable_provider`
  test (5), `native-codec-factories.json` (3 new rows), hub provider consts (STDIO 26, SET 29), hub fence (`!= 26`), hub
  trusted-catalog tests (28 ×3, synthetic `0u8..26`, test name `twenty_eight`), hub bin-unit (28, 26), hub `📜️script.ts`
  (fixture/oracle checks 26/28/29 ×8 + stale law names), bootstrap fixture (codecCount 26/28 + generationId, which hashes
  the real receipts), framework `trusted-stdio-catalog` (TS, test, fixture, schema), `native-catalog-selection` fixture
  (26/27/28/29). txt/tsv/html already own snapshot/mutation schemas + `ArtifactDsl` (`EXTENSION` txt/tsv/html) and
  `*_artifact_schema_descriptor()`; they are `definition_only_assembly` with no codec rows and (tsv/html) no runtime
  capabilities. Snapshot protocol SHA-256: txt `d416b7a0…`, tsv `8c65ab63…`, html `563a2751…` (md/csv recomputed = committed).
- 15:1x overlay `.🧬semio/🌐hub/s13-cx1-overlay` (tracked tree via `git ls-files -z | tar`; openrsync `--files-from` hung 13 min
  at 0 B, stopped) + 25 untracked generated `.rs` / 586 `🤖️generated` files (the build needs `🎨️styling/🔤️tokens`).
- 15:5x warm-up `cargo test --no-run -p semio-s-plugin-stdio --lib --test native_openable_provider` in the overlay (private
  build/target dirs, nice 10, detached pid 76903, `generated/warm-stdio-2.txt`); first attempt died on the missing generated
  tokens file (`warm-stdio-1.txt`).
- Descriptors: the committed stdio/gis/vcs `🛂️.descriptor.semio` + `🔣️.json` carry the 26-codec catalog topic (gis/vcs link it)
  and are describe outputs of the wasm components → they regenerate in the chain after window 3; not hand-edited (hashes).
- 16:0x patch `wp-cx1/cx1-apply.py` (80 hunks, 1 new file, 34 files): per artifact (txt/tsv/html) `native_codec()` +
  factory `stdio.native.<x>.v1` (snapshot protocol SHA-256), `definition()` binds the executable, `runtime_assembly` with a
  minimal `declaration()` (schema + formats + `document_codec_bare`, the facets the codec needs; composers/languages stay
  unregistered as today), JSON codec row + codec/schema runtime capabilities, `semio-framework-hash` dep; registry
  `NATIVE_CODEC_FACTORY_COUNT = 29` replaces six literal 26s; 3 receipt rows; counts in every law/fixture/schema (hub
  provider set 29 → 32, fence stdio 26 → 29, stdio+GIS 28 → 31, selection fixture 26/27/28/29 → 29/30/31/32); fence test
  renamed `…_thirty_one_codecs_…` (the hub script's law lists named a test that no longer exists:
  `…_and_one_map_editor_and_viewer`); publisher doc no longer calls txt/tsv/html unowned; linked-ownership fixture's
  synthetic case uses `s.fixture.*` kinds + a new real-registry case (stdio descriptor ∖ linked registry = ∅); census law
  adds `linked_codec_registries()` (stdio + GIS committed registries) and asserts every open target of a linked package
  binds a linked codec. New neutral fixture `📇️registry/🧫️fixtures/📇️native-text-codecs` (7 cases, schema def
  `NativeTextCodecs`) + law `text_document_codecs_compile_their_neutral_fixture_through_the_linked_receipts`.
- 16:0x third-party oracle `wp-cx1/cx1-oracle.py` (Python stdlib `str.splitlines`, `csv` QUOTE_NONE tab, `html.parser`):
  **7/7** cases equal the hand-authored fixture snapshots.
- 16:00 overlay resync (35 tracked files changed since the copy) → `--write --root overlay`: 34 files.
