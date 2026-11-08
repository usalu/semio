STATUS: UNVERIFIED. Source port complete for the whole demonstrator closure (32 components) and 90+ off-closure hosts; no compile of the ported crates has run. hosts-101 (wasm32-wasip2, gismap manifest, log 🗑️generated/1105b/hosts-101.txt) is blocked upstream by crates owned by others: semio-framework-replication-mutation, -job-failure, -pack-record-encode, -pack-record-encode-cursor, -pack-record-retained, -artifact-reference, semio-ui-viewport (old close_step/retirement shapes). Rerun: `1105b-tools/chk.sh hosts-102 ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml wasm -p semio-s-artifact-gis-gismap` once those clear; then the store codec crate (`semio-os-store-codec-registry`) first, since it holds the new helpers.

## New shape (trait: store/space-history lifted/io)
Hosts take a trailing `output:&mut Option<SnapshotReceivingOutput<T>>` and return `SnapshotReceivingOutcome` (`store::os_store::serialization::{SnapshotReceivingOutput,SnapshotReceivingOutcome}`). Pre-checks that used `?` run in a closure and map to `SqliteStageFailure::NotStarted`; no `native_snapshot_turn` wrapper in plugins.

## Helpers (store/codec/snapshot-capability, re-exported by `store`)
- `decode_sqlite_snapshot_record_native(payload, envelope, spec, &mut |record,out,native|->Result<(),ValueError>, control, owner, output)`; closure lost the `_body` wallet param.
- `decode_sqlite_snapshot_record_receiving(..., &mut |record,out,owner,sql|->RetainedWorkOutcome<(),NativeSnapshotFailure>, ...)` for constructors that run nested receivers or need the SQL control (fem, cad). The first is a thin layer on this one.
- `decode_/encode_sqlite_snapshot_prefixed_record_native` for the Generation2d discriminator (replaces the hand-rolled allocation_stage_native paths).
- `encode_sqlite_snapshot_record_native(encoding, envelope, spec, &mut |native|->Result<RecordValue,ValueError>, control, owner, output)`.
- Work accounting: operation runs inside `receive_nested` (decode) / `receive` (encode) with `ReceivingSettlement`; output is published into the caller slot; settlement refusal maps to `Stage::Operation(refused_failure)`; an occupied slot or pre-check refusal is `NotStarted`, work 0. Effects come from the caller's owner (funded action); no AuthoringWallet anywhere.
- `SqliteStageFailure::map_operation` (io/sqlite-snapshot model) re-types retained custody for the puzzle play wrappers (5d, 3d).

## Ported by script (1105b-tools/hosts/port.py, fail-closed: skips unknown shapes) or by hand
Closure: gismap (exemplar, snapshot host + pack host), gisterrain, flow, playground, playbook, process3d, cad (+owner), energy (native + host), fem 2d/3d (receiving helper), generation2d/3d (shared `procedural/foundation/.../native/document` rewritten on the prefixed helpers), puzzle 2d/5d/3d (hosts + play wrappers), sourcing curation.
Off-closure: ~60 further hosts converted by the script (writer, vcs, animate, bim, architect, forms, layout, imperative, remodel, rewriting, dag, drawing, raster, block, space, home, norm family, wfc subset, ...).

## Not done (off closure, shapes the script refuses; all need a hand port)
Derive the list with a grep for files still matching `fn (decode|encode)_sqlite(_snapshot)?_native.*RetainedWorkOutcome<.*NativeSnapshotFailure>` (about 80): stdio family (~45 files, `semio` subsets), wfc text/native, equation, jack, lowpoly, shooting, en1998, dwg, dxf, ply, pdf, rewriting, note, space, framework-side dag/mcp workspace/flow capability/workflow/renderer/plugin-testing/print. Partially converted files (encode ported, decode skipped by a `tail` post-step: shooting, lowpoly, en1998, jack, space, dwg, dxf, ply, wfc text) are inconsistent with the trait until finished.
- Store codec crate tests (`native-decoding/tests/control`, `native-encoding/tests/control`) still call the previous helper signature: UNPORTED.
- Fem receiver files still define their own `forecast_decoding(owner)` unchanged.

## Risks to check on first compile
- `decode_framed`: the two `source` match arms must unify (`&[u8]`); `construct` is `&mut impl FnMut` re-entered per receive call.
- `receive_nested` operation closures capture `settlement` by reference while `scoped_native` holds the observer on it (same pattern as port-f's receiving macro).
- Generation `bind` closure adapts a FnOnce constructor into the FnMut slot via `Option::take`.
