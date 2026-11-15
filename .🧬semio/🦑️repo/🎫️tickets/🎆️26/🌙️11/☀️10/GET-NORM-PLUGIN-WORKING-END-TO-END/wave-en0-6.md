# wave-en0-6: en1990-en1996 norm artifacts

Result: component check log `T/🗑️generated/normcomp-en0-6-10.log` shows 0 errors in en1990-en1996 crates (only en1997/98/99 viewers remained, fixed afterwards by the same pattern). `en0-6-errors.py` filters the log for these crates.

## Fixes by root cause
- Generated sqlite readers/writers: const name drift (`FATIGUE_STRESSE`, `*_FLOATS`), stray `Real(n)` inside `Enumeration` lists (en1993, en1996), missing fields (`bearing_width` en1992, `web_spacing_m` en1996).
- `Default` for all seven Snapshots (needed by `live_report!`), via `impl Default` calling the worked example.
- Diff crates: `cases` is non-Option (en1994), four non-Option row deltas (en1991), triplicated `JointPlate` list_delta removed (en1993), `semio-framework-diagnostic` dependency (en1992/93/95/96), text/binary diff impls mounted in the diff crate (orphan rule) and re-exported by io (en1992/93/95/96).
- Mutations: `requiredLanguageSurfaces` key (en1990 theory-variation), `usize`->`u32` payload counts (en1992/en1993), renamed delta/patch types (en1993), `CanonicalJsonTree` on en1992 grade rows, binary `Malformed` offset u64, `OpBinaryReceiving` via `original_pack_value_operation_receiving!` (all seven), `mutation_bridge` import path (en1992/93/95/96), `ParseBoolError` path.
- Checks/scene: moved misplaced fire block (en1993), float annotations, shadowed `ratio` (en1994), missing imports/deps (en1991 rail-silo helpers, ndp), `ActionStage` typed stage (en1994), child-module `super::super` paths (en1993 diagrams), MAX_STUDS/raw_value moved to kit (en1994/en1990), `cut_ring` reimplemented and `ActionKind: Display` (en1992), en1996 handles/members binding aliases, gauges and catalogue derived depth.
- Viewers/editors (en1990-1996, plus en1997-1999 viewers): `norm_viewer_commands!` with fully qualified snapshot, `semio-framework-job`/`action-bus` deps, `viewport_act` import, `#[cfg(test)]` on `norm_live_acceptance!`/`norm_latency_bench!` (macros exist only under test/compliance-testing).
- din16798 editor manifest had an incomplete `rust-source-boundary` (graph refused unit 607); completed with the three required keys.

## Not done / notes
- Full component check `wave-full2` hit the 1 h wrapper timeout (exit 124) on a cold cache with no errors; `wave-full3` rerun in progress; build not yet run.

## Update
- wave-full3 wrapper killed (pid 89896) per coordinator. Describe build log: .cache/b/r/m/u/l/q91aad72f06e/semio-cargo-logs/cargo-artifacts-yDJ15z.log (16:20: still compiling framework crates, 0 errors). No source fixes needed yet; .wasm path/size pending.
