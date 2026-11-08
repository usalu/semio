STATUS: BLOCKED on claimed store crates (15:55 11/07). No gis smoke has run. Not green, not red: gis crates were never reached by the compiler. Log: 🗑️generated/1105b/verfolgen2-4.txt (rc 1).

## Exact path (verfolgen2-4: `check --tests -p semio-s-artifact-gis-gismap-gis2d-unit-tests --keep-going`, native, 15:33-15:52)
Compiles clean up to the store layer (115 `error` lines, all in three crates; no gis crate, no dsl-record-model, no pack-json, no value/job error remains on this path):
- semio-os-store-operation-wire: 88 errors (E0308 x70, E0609 x23 over all). Claimed store5 (15:51 "all of it") and store4 (one-item/publication/lifted/operation-wire).
- semio-os-store-space-history: 24 errors (codec snapshot-capability/workspace-receiving 8, io/sqlite/snapshot/receiving 5, validation 3, native 2). Claimed store3 (14:08, 15:34: workspace-receiving, receiving/validation/native arities).
- semio-os-store-snapshot-clone: 6 errors (🧬️snapshot-clone root 3, metadata 2, law 1). Claimed store4 (15:48).
No unclaimed red crate on the path, so nothing for me to claim. Gis crates (gis2d, gis2d-unit-tests) stay unverified until these three compile; keep-going skips every dependent.

## What changed
- artifact-app-laws `pane-smoke-law/🦀️.rs` `assert_pane_smoke_by_example`: added redo (probe == applied) and a second undo (probe == loaded) before close, mirroring `assert_pane_smoke`. Every by_example pane (verfolgen, generator, koordinator, aussuchen, bearbeiten, statik) now proves redo. Claimed in 1105b-claims.md 14:4x. UNVERIFIED (artifact-app-laws compiled as part of verfolgen2-4 without error lines, but the changed fn is only type-checked, not run).
- Verified fact: dsl-record-model blocker from 1107-result-verfolgen.md is gone (dslowned-4/5 FIXED); `verfolgen2-1` (`--lib` check, rc 0) is meaningless because dev-deps are not built by `--lib`; use `--tests`.
- Verified fact: gis `io/sqlite/snapshot` host impl already has the current trait signatures (decode/encode_sqlite_snapshot_native with output slot), contrary to 1107-result-gis.md "Blocker routed to runtime"; only a compile can show body errors.
- Transient: verfolgen2-2 and -3 died on "Unbalanced Rust source delimiter" in semio-os-store-space-history (store3 mid-edit, fixed 15:12:45; all store .rs now balance under the repo tokenizer via a scratch bun check).

## Remaining coverage gaps in the verfolgen smoke (static)
- de: `verfolgen_pane_body_renders_in_every_locale` renders en and de; apply/undo/redo/close run once with the process language only. `tst.sh` hardcodes SEMIO_LANGUAGE=en, so a de apply run needs a tst variant with SEMIO_LANGUAGE=de (not created; the test never reads the env, per 1107-audit-paneready.md).

## Next (when the three store crates are green)
`C/1105b-tools/chk.sh W-verfolgen2-N "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --tests`, fix gis errors (source was statically ported 11/07 09:xx, see 1107-result-gis.md), then the smoke: `C/1105b-tools/runtime/tst.sh W-verfolgen2-N <same manifest> native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen --nocapture --test-threads=1`.
