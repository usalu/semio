# H15 result: cross-origin output determinism of Rust units

## Divergence classes (server `fleet/divergences.json`, 971 unit rows since 11-21; 714 rows listed)
By differing output: `.rlib` only 223; stderr/stdout only (no output differs) 100; `.rlib+.rmeta+.semio-deps` 95; `build_script_build` binary 78; `.d+.rlib+.rmeta+.semio-deps` 57; `.rmeta` only 56; `.rlib+.rmeta` 30; `.d` only 12; `.dylib` 10(+3); other 4.
By day: 11-22 100, 11-23 194, 11-24 107, 11-25 223 (mostly fixed by earlier waves: incremental CGU token, path remaps), 11-26 41, 11-27 2.
Evidence kept on disk (`.cache/b/p/m/fleet/cache-server/fleet/evidence`): 64 unit entries: 43 stderr-digest only, 11 `.d`, 10 `.rmeta`, 3 `.rlib`, 2 `.semio-deps`.

## Cause found and fixed: `.d` (dep-info) holds the per-run proc-macro observation files
`with_compiler_resources_v1` writes `semio-compiler-resources/<epoch-ms>-<pid>-<n>/observation.json` and registers it with the compiler, so every `.d` listed 5+ such paths in the target lines, as `path:` lines and as `# checksum:blake3=` lines (content differs per run). The key already ignored them (`compilerWrittenV1`) but the stored output kept them, so every unit with a proc-macro had a different `.d` per run and origin. Also the `# checksum` hashes change with each run.
Fix: `canonicalDepInfoV1` (`🔏️key/🔗️sources/🟦️.ts`) drops exactly those entries; applied (1) in the unit wrapper to every finished `.d` before key gap check, publish and store (`📜️script.ts`, atomic temp+rename) and (2) in the uploader to every `.d` output (covers spooled jobs from before the fix, `🛰️fleet/📤️uploader/🟦️.ts`).

## Proof (stored outputs from two origins, real server evidence)
11 `.d` pairs (kept vs refused origin, same key): before 11 differ; after canonicalisation 10 identical sha256, 1 still differs (`durable_group_wire`, g1-m06 vs g1-m11 at 11-26 16:55): `#[path]`/`[lib] path` spelled with `../..` on one origin; `canonicalSourceArgumentsV1` (earlier wave) already normalises that spelling, entry predates it. Script: scratchpad `H15/proof.ts`.
Test: `🧪️tests/🧳️portable/🟦️.ts` new test (two runs with other stamp, pid, checksums give identical text and sha; idempotent): passes (36 pass; 1 pre-existing fail `a unit that was compiled but not shared says why` = H3's `unsharedReasonV1` expectation `spool-full` vs `spool-error`, not mine).
Counts: `.d` classes (12 `.d`-only, 60 combined with rlib/rmeta/deps; 11 of 11 recent) -> expected 0 new from this cause; combined rows may stay if rlib/rmeta differ on their own.

## Not fixed (open classes, with what is known)
- `.rmeta`/`.rlib` (10 recent): byte diffs are table reorderings and +7 value shifts (position-like fields), strings identical, so a source-map/span-offset or ordering effect, not a path. NOT caused by `-Z threads=0`: a 400-item synthetic crate and `syn 2.0.119` (full) compiled 6x with threads=0 and 4x threads=1 gave identical rmeta/rlib shas on this machine. Needs a two-origin diff of one proc-macro crate (e.g. `semio_framework_job_clock`, 183 bytes differ) with `-Z` span dumps.
- stderr-only (43 of 64 evidence entries, 100 rows): blobs already garbage-collected from the server, cause unverified (diagnostic order or text with paths).
- `build_script_build` Mach-O (78): link output, not examined.
- No two-checkout real build was run (cache hits would skip compile; wrapper experiment too heavy at load 50).

# Round 2 (coordinator follow-up)

1. `unsharedReasonV1` test: the code is right. H3 returns `spool-full` only when the spool length reaches the bound, else `spool-error`; the old test used a fake unit with no root and an empty spool. Test fixed (real empty spool root gives `spool-error`); portable file 37/37.
2. stderr-only divergences: the evidence kept no blobs for them, because `differing` listed output files only and streams are not output files. Fixed in `🌐️remote-cache/🔬️action-result/🟦️.ts`: the stdout/stderr digests are now `named` rows (`⟨stdout⟩`, `⟨stderr⟩`), so a stream-only divergence names them and the existing bounded evidence retention keeps both blobs (limits unchanged). Next divergence is diagnosable with `repo remote-cache evidence show <id>`. NOT done: stderr normalisation, because no stderr blob of a diverged pair exists to say what differs (local CAS layout holds none either); normalising blind (sorting diagnostics, stripping numbers) could hide real text. Likely suspects to check on the first captured pair: diagnostic order under `-Z threads=0`, rendered paths, timing.
3. `.rmeta`/`.rlib` and the observation files: the proc-macro `owned_json_file` (value-derive-resources) does not embed the observation paths in its output (it only writes observation.json and registers it with `proc_macro::tracked::path`; the expansion has no HashMap iteration, HashSet only checks duplicate keys). Experiment (`T/🧰️scripts/H15/exp.sh`, nightly-2026-07-20, `-Z threads=0`, remaps, `SEMIO_COMPILER_RESOURCE_ROOT` set): real hash crate, compiler-resources crate, the real proc-macro crate and a dependent crate that calls the macro, built in two roots of different path length. Dependent `.rmeta` and `.rlib`: identical shas (67cbcc9c12 / b437b90c37). Proc-macro and compiler-resources `.rmeta`: identical. Only the dependent `.d` differed (the per-run observation entries, fixed in round 1). So the observation files are not the cause of the rmeta/rlib divergences; the recent 11-26 rmeta rows (job_clock +7 shifts, manifest_plugin reordered table entries) remain unexplained and need a pair with both sides' inputs (extern rmeta digests are the first thing to compare: a dependency interface that differs while the key sees it equal would shift positions).
4. `build_script_build` and `.dylib`: no divergence rows after 11-25 (78 bs rows 11-22..25, 13 dylib rows 11-23..25). Experiment: a debuginfo bin built in two roots differs without `-Wl,-oso_prefix` (0bb9f979 vs 53d979a5) and is identical with it (0d14d501 both); the proc-macro dylib differs by install name and LC_UUID without `-install_name`/`-oso_prefix` and is identical with them (02a36d9c both). The wrapper already adds exactly those flags (`linkRootArgumentsV1`, landed 11-25 17:12), which explains the stop. No code change needed; LC_UUID follows the content, no `-no_uuid` required.

Landing: lock remote-cache (released), tests action-result 4, provenance 10, portable 37 pass, typecheck passed, dangling-imports 0.
Unverified: live divergence counts after restart; the rmeta/stderr causes.

# Round 3: root cause of the `.rmeta`/`.rlib` divergences found and fixed (landed 04:00)

## Classification of the divergences recorded after 01:00 CET (81 unit rows, 01:00-03:14; 64 evidence entries)
- `.rmeta` only 34 rows (m13/m15 10, m01/m15 5, m02/m14 4, m06/m11 4, ...); no-output-diff (stream or metadata only) 40 rows (m07/m08 26, m02/m07 5, m13/m15 4, ...); `.d+.rlib+.rmeta+deps` 1, `.rlib+.rmeta` 1, `.rlib` 1.
- The server had not restarted yet with the round 2 stream change (restart policy 3 h), so the stream-only pairs have no blobs: that class (40) is still unclassified; the evidence will keep them after the next server restart.
- Kept vs refused `.rmeta` (e.g. `replication_crypto`, 15106 vs 14996 bytes, `-Z ls=root` and `=items` identical: same SVH, same stable crate id, same dependencies, same items): same content, entries of the lazy tables in another order and shifted positions.

## Root cause
`.cargo/config.toml` builds with `-Z threads=0` (parallel front end, one thread per core). With more than one thread the compiler numbers the definitions it creates while lowering (opaque types of `async fn` in a trait, return-position `impl Trait`) in the order the threads arrive, so the metadata tables and the rlib differ between two compilations of one unit, on one machine too. The crates that diverge are exactly those with async-fn-in-trait (`replication_crypto`, `async`, `actor_pack`, ...). Reproduction with one real compiler (nightly-2026-07-20, 60 traits with `async fn`, same command line): `-Z threads=0` 6 runs gave 6 different `.rmeta` shas; `-Z threads=4` 3 runs gave 3; `-Z threads=1` 5 runs gave 1. (The earlier synthetic crates without async-fn-in-trait, and `syn`, were deterministic: that is why round 1 ruled threads out wrongly.)

## Fix (landed 04:00, lock caching-cargo released)
`deterministicFrontEndArgumentsV1` (`🔏️key/🟦️.ts`): every spelling of `-Z threads=N` becomes `threads=1`. `📜️script.ts` applies it to the compile of every unit that is shared through the fleet (`shared`); units that are not shared (no fleet) keep the parallel front end. The key keeps what Cargo gave (all machines give the same), so no unit is re-keyed. Cost: the compile of a shared unit uses one front-end thread (codegen stays parallel); accepted for bit-identical outputs.
Tests: portable 39 pass: argument rewrite (both spellings, `--cfg threads=0` untouched) and a real-compiler check (4 compiles of an async-fn-in-trait crate give one `.rmeta` sha).
Dangling imports 0; typecheck result below.

## Still open / measure
- Stream-only (40 rows, m07/m08 pair dominant): blobs kept after the next cache-server restart; classify with `repo remote-cache evidence show <id>` then. No guess normalisation landed.
- Rate: 1135 at 03:14 (53/h). Measure again at about 05:15, after workers restart on the new wrapper and the server restarts; expected: `.rmeta`/`.rlib` classes (34 of 81 recent) stop, only stream-only and pre-existing keys remain. Not measured yet.
