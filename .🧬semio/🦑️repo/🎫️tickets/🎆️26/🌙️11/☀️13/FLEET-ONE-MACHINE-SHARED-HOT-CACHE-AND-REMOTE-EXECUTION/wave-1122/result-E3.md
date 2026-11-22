# Result E3: AC divergences

## Diagnosis (measured on the live server directory `.cache/f/cache-server`, read only)

The "1000 divergences" is the cap of the persisted row list (`ROWS=1000`), not the real count; `count` was `divergences.length`.
Classification of the 1000 rows by what the kept result is (blobs read from the host CAS `~/.cache/semio/cas`):

| class | rows | meaning |
|---|---|---|
| `failure.br` (owner pool failure note) | 426 | by design different per machine (note carries `origin`, `atMs`, `cause`); the first writer's stale note blocked every fresher note (409) |
| project record (`record.br`, graph inference) | 219 | two machines inferred the same record key with different observation rows; consumers verify rows against their tree, so benign but wasteful; cause not isolated (refused copy is not kept) |
| `costs.br` | 9 | measurement sheet, differs by design |
| other (owner pool waves, pointer, closures) | 23 | wave `head.br` carries publisher and createdMs |
| compiled Rust units | 5 (+ 8 in the old `semio` namespace) | all `semio_framework_value_*` crates (derive_expansion x2, native_encoding, control_contract), kept g1-m10/g1-m11, refused g1-server/g1-m12 |
| gone (superseded, kept result no longer readable) | 310 | class unknown |

So Rust units are 1 percent of the divergences. Per-origin: the refused side is `g1-server` 419 times (this Mac: it writes failure notes and records the other machines already wrote).
Kept unit blobs are clean (no machine path in strings). The refused unit bytes were not retained anywhere (409 never stores them, the local entry on g1-server was replaced by the adopted fleet copy), so the exact nondeterministic byte could not be named for units.

Evidence for the unit cause (not proven on these crates): E5 measured that a unit whose working directory or module directory is covered by no `--remap-path-prefix` gets a different crate hash (SVH, in rlib and rmeta) per checkout path. The wrapper's `sensitive` path logic only looks at the source file, the manifest dir and the cwd, not at module directories reached through `#[path = "../../x.rs"]`, and a hash cannot be found by the string leak check. That matches the shape of all 5 unit divergences (crates built from shared `../..` module files). Synthetic check I ran: `-Z threads=0` with 1, 2, 4, 8, 0 threads gives byte-identical rlib and rmeta (not the cause). The existing `🎯️determinism` test (peer-edited at 00:48) already fails for the uncovered-cwd case, which is the reproduction.

## Server policy decision

- Deterministic results (units, graph records, closures, waves, pointer): keep first-writer-wins with 409 (a valid build of a content-derived key is valid for every reader; the uploader already adopts the fleet copy, so the fleet converges). Refusing to serve a contested slot was rejected: every nondeterministic unit would become a permanent miss, which is "computed twice". The heads test pins first-writer-wins for owner pool waves, so they stay refused.
- Notes (`failure.br`, `costs.br`): never refused; newest replaces older; counted under `advisory` (not a divergence). Declared in `PROVENANCE_ADVISORY_OUTPUTS_V1`.

## What changed (all under `🏗️build/🌐️remote-cache/`)

- `🔬️action-result/🟦️.ts`: stable result also returns `named` (path + digest of every non-volatile output file).
- `🧾️provenance/🟦️.ts`: `DivergenceV1` gains `sort` and `differing` (output paths whose digests differ, max 16); `provenanceClassV1` (unit, project-inference, closure, owner-pool, other; advisory failure-note, costs), `provenanceDifferingV1`; `diverged(row)`, new `advised(sort)`; `divergences.json` now stores lifetime `total`, `classes` and `since` besides the newest 1000 rows; `summary()` returns the lifetime count, `byClass` (survives restarts, was empty after every restart) and `advisory`.
- `🖥️server/🟦️.ts`: class from the union of both results' paths (a unit that lost its manifest in the second write is still a unit); advisory classes replace and count; divergences carry `differing`; status type has `advisory`.
- `🛰️fleet/🪪️identity/🧬️schema/🔣️.json`: `Divergences` schema (schema first) with `since`, `total`, `classes`, row `sort`, `differing`.
- Tests: `🧾️provenance/🧪️tests/🟦️.ts` (class and differing vs Python oracle `🐍️.py` over fixture `🧫️fixtures/🔣️.json`; lifetime count beyond the cap, restart persistence), `🧮️by-class/🟦️.ts` (classes incl. unit; a note is replaced and counted as advice; a unit divergence names `out/libx.rlib`; byClass survives a server restart).
- Landing script `T/🧰️scripts/E3/land.py`; entry in `T/landings.md`.

Results: provenance 4 pass, by-class 2, nx 1, server heads 5, access 8, action-result 4, unit-cache fleet "divergence" test 1.

## Not done / not verified

- Root cause of the 5 unit divergences and the 219 record divergences is not proven; the new `differing` field will name the differing output on the next occurrence (live server needs a restart on the new code, it restarts itself on code change; the old divergences.json fails the new schema and is dropped, which resets the old evidence).
- The reproducibility fix for uncovered module directories (add a cover `--remap-path-prefix` to the local compile and feed the same to the offload planner, so the worker names equal local names) is NOT implemented: needs the local and the worker to use the same virtual name, which couples to E5's planner (`virtual.package`); also the unit key would need a salt so old path-dependent entries are not served. Recommended next lane.
- Full unit-cache fleet test file and offload signed-fleet test were not run (the file exceeds 2 minutes in the foreground).
- A project-record divergence needs the refused copy; graph lane should drop timing/host fields from `record.br` or exempt them if they are the cause.

# Round 2

Landing in `T/landings.md` ("Wave 1122 E3 round 2"), under locks execution and remote-cache.

## 1. Evidence of refused writes
- New `🧾️provenance/🗂️evidence/🟦️.ts`: on every refused write the server keeps both action results and, for each differing output, the kept and the refused blob (16 MiB per blob, a larger or collected blob is named by digest only), in `<directory>/fleet/evidence/<id>/` with `meta.json` last. Bounds: newest 64 per class, 256 MiB in all, oldest dropped first; one entry per key and refusing origin (a repeat replaces it); a cut-short directory is removed at open.
- Server: `GET /evidence` (list of entries) and `GET /evidence/<id>/<name>` (`kept.ac`, `refused.ac`, `kept.N`, `refused.N`; `meta.json` lists which `N` is which path), same authentication as `/status`, names validated. `GET /status` has `divergences.evidence` {count, bytes, byClass}. No CLI subcommand was added (a new `repo remote-cache` operation needs help, parity and texts in both languages); the status section and the endpoint are the view, fetch with the signed client (`remoteCacheFetchV1`).
- Found and fixed: the closed `ServerStatus` schema (`🏃️execution/🧬️schema/🌐️remote-cache.json`) would have rejected the round 1 status (`advisory`, row `sort`/`differing`): `repo remote-cache status` would have printed `malformed` and no server numbers once the server restarted onto round 1. Schema now has `advisory`, `evidence`, optional row fields; live status validates (checked against the still running old server).
- Tests: provenance (evidence keeps both sides, replaces per key and origin, 64 per class, survives restart), by-class (a unit divergence is listed by `/evidence`, `refused.ac` equals the refused bytes, a traversal name answers 404, the status validates against the closed schema).

## 2 and 3. Record and failure-note divergences
Neither refused copy exists any more (blobs collected), so no diff was possible; reasoning and candidate fields in `W/findings-E3.md` (provider ids with module location hash, directory listings, exists/lstat rows, unsorted row table). 426 failure notes: every machine that fails a deterministic owner writes its own note, so they differ in origin and time by construction; fix suggested for E1 (skip publishing when a live note exists). Not landed in graph-state (E1 folder, no evidence yet).

## 4. Re-runs
`🎯️determinism` 6 pass 0 fail (includes E5's uncovered module directory case: the remap works); unit-cache fleet 18 pass; offload signed-fleet 1 pass; provenance 5, by-class 2, nx 1, server 9, access 8, heads 5.

## Not verified
- The live server still runs the pre-round-1 code (restart waits for requests in flight); `/evidence` and the new counters have been tested only in the test servers. The first real divergence after the restart will show whether the unit and record causes are as guessed.
- No live unit with an uncovered module directory was compiled through the fleet to check that its output now equals the kept one.

# Round 3

Landings in `T/landings.md` ("round 3" and "round 3b"), under the locks execution, mcp, operations, remote-cache and later graph.

## 1. Failure notes are not written again
`publishOwnerPoolFailureV1` (`🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts`) reads the live note of the record state and revision first: the same cause writes nothing; another cause is added to a new `causes` array (schema `Failure.causes`, at most 4) while `cause`, `origin` and `atMs` stay those of the first failure, so the note cannot flap between machines and does not outlive its first failure. `readOwnerPoolFailureV1` returns `causes`. Test: puts counted on the server (the same cause adds none, a new one adds exactly one), causes merged in order, capped at four, a new revision starts a fresh note. Owner-pool tests 32 pass. No failure-note divergence has appeared since the server restart (consistent with the cause being the write churn).

## 2. `repo remote-cache evidence [list|show <id>]`
Schema first (`🏃️execution/🧬️schema/🌐️remote-cache.json`: `EvidenceRow`, `EvidenceList`, `EvidenceShow`, `EvidenceDiff`; the instance name `evidence` is reserved), then the catalogue (`⚙️operations/🔣️catalog.json`: synopsis, subcommand with English and German summary, example), the command (`⌨️command/🟦️.ts`: parser words, `remoteCacheEvidenceV1`, route; signed fetch, timeout, cancellation, no token in the output), texts (English and German: `evidence.syntax|off|fault|unknown`, `error.syntax`), the comparison module `🧾️diff/🟦️.ts` (first differing byte, differing byte count, lines only one side has, Brotli and JSON unpacked), MCP parity (`🔌️mcp/🖼️assets/🏗️build-surface.json` tool `remote_cache_evidence` with `action` and `id`, fixture tool list) and the help snapshots (regenerated from the catalogue; only `root`, `remote-cache` and `remote-cache evidence` changed, English and German).
Proof: `🧾️diff/🧪️tests` 3 pass (fixture shared with a Python oracle `🐍️.py`); `🧪️tests` of remote-cache 23 pass (the new test seeds evidence, lists it, shows both sides, validates against the schema, refuses a wrong id, a missing id, a wrong word, a missing endpoint and a wrong token in both languages, never prints the token); operations 15 pass, requires 3, MCP build-surface 6. Live: `repo remote-cache evidence list` and `repo remote-cache evidence --help` work.

## 3. Records: E1 owns the writer; see `W/findings-E3.md` (updated with the real diffs).

## 4. Live verification and the first real divergences
The server runs the round 1 and 2 code since 00:53Z (E5's handover). `GET /evidence` works through the CLI. Six `project-inference` divergences were captured within 5 minutes; all differ in `record.br` and were diffed with `evidence show`: (a) file rows of the same source file at two versions (tree churn), (b) directory listing rows of `node_modules` (machine-local content), (c) index shifts that follow, (d) one record with stamp rows on one side. No unit and no failure note diverged in this window. Details and the policy question (newest-wins for records) in `findings-E3.md`.

## Not verified
- No unit divergence has been seen since the restart, so the unit cause (uncovered module directory, E5's remap) is still unconfirmed live.
- The newest-wins policy for graph records is not implemented (E1 contract).
- MCP tool `remote_cache_evidence` was validated by the surface test only, not by a live MCP call.

# Round 4

Landings in `T/landings.md` ("round 4" and "round 4c"), under the locks execution, graph-state, remote-cache and caching-cargo.

## 1. Newest record wins (server)
`PROVENANCE_SUPERSEDING_CLASSES_V1 = ["project-inference"]` (`🧾️provenance/🟦️.ts`). When another origin writes a different graph record for a key, the server replaces the held one, keeps the replaced result as evidence (`outcome: "superseded"`, new schema field), counts it in `divergences.superseded` per class and does not count a divergence and does not answer 409. Units, closures and owner pool waves stay first-writer-wins (heads test unchanged and green). Evidence sides: `kept` is the earlier result, `refused` the later one; `outcome` says which one is served. Tests with the real server: the record is replaced and the served bytes are the newer; the wave is still refused; `/status` shows `superseded {project-inference: 1}` and divergences `{owner-pool: 1}`; `/evidence` lists the replaced record with `outcome superseded` and `kept.ac` equals the older bytes (by-class 3 pass, provenance 5, access 8, heads 5, remote-cache command 25).

## 2. Installed-package probes out of owner records
`installRowsV1` (`🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts`, used by `portableRowsV1` and `neutralRowsV1`): rows of the new policy list `installKinds` (exists, directoryExists, fileExists, lstat, stat, readdir, readdirNames, directories) about a path with a `node_modules` component outside a generation directory (`node_modules` itself, `<above root>/node_modules/ajv`, `node_modules/typescript`, listings) are dropped; for every dependency generation the remaining rows name, one row `exists <root>/.cache/b/d/m/j/<id>/node_modules = true` is added (before tokenizing: two machines of one install write identical rows whatever they probed; another generation id writes another row; a machine without the generation replays the row with the name `generation-unavailable` and infers the project itself). Rows below a generation directory stay: that content is addressed by the id. Schema first (`Policy.installKinds`), Python oracle updated (`install_rows`), two fixture cases (machine A and B of one install with different probe results give equal portable rows), test with a calm record and a record without a generation. The expected values of the existing fixture records were regenerated from the oracle (only the rows with node_modules probes changed). Tests: portable 13 and all portable subfolders, project-records tests 7, 8, 6, 3, 10, 8, wanted/read 23.
Not done: the row table of a record is still unsorted (candidate 5 of the findings), and a machine without an install no longer says so (its record equals an installed machine's for these rows; the taker verifies the generation row).

## 3. Watch (live)
Cache server restarted onto the code at 01:23:12Z (02:23 CET). Before: 14 `project-inference` divergences in 27 minutes (00:53Z to 01:20Z, 0.5 per minute). After, 01:23Z to 01:42Z (19 minutes): 0 `project-inference` divergences (still 16 as lifetime count), 7 `superseded` records, replaced ones kept as evidence. Effect of the install-row change cannot be separated: the machines run the new code only after their supervisors restart; the drop comes from policy 1. Units in the window: 3 divergences (6 lifetime, from 3).

## 4. What the unit divergences are (evidence, three kinds)
1. `liblibm.rlib`, `libsemio_framework_test_fixtures.rlib`, `…process_routing_workspace.rlib`, `…replication_retained_field_prep.rlib` (only the rlib differs, rmeta and `.d` equal): root cause found and fixed in round 4c: with `rust-src` installed rustc names the inlined `core` code by `<sysroot>/lib/rustlib/src/rust/library/…` in the debug info (7 occurrences in the libm rlib of g1-m12, `/rustc/<commit>/…` in g1-m02's). `rustSourceRemapArgumentsV1` now adds `--remap-path-prefix=<sysroot>/lib/rustlib/src/rust=/rustc/<commit>` to every unit of a trim-paths build (`wrapRustcV1`; token `⟨rust-src⟩` in the key so old entries are not hit). Proof: real rustc test (7 paths without the flag, 0 with it), remap 7 pass, unit-cache 27, offload 30, determinism 6. The three newest divergences happened on machines still running the old wrapper; confirm after they restart.
2. `libsemio_framework_value_persistent` and `…pack_json_canonical` (rlib, rmeta, deps and `.d` differ): the `.d` differs in `checksum:blake3` and `file_len` of a module file (43115 against 42470 bytes): the same key, two versions of a source file of the crate. The unit key hashes only the entry file (`sourceSha256`, `🔏️key/🟦️.ts`); module files (the shim includes `../../🦀️.rs`) are covered by the dependency list that a local lookup re-verifies (`💾️store` line 151) but not by the fleet key, so a machine at another tree version recomputes under the same AC key and its upload is refused for ever. Recommendation (not done, it contradicts the decision that units stay first-writer-wins): treat a unit whose `.d` output differs as superseding too (dependency content differs, every taker verifies the dependencies), or put the digest of the module closure into the fleet key.
3. `libm` older divergence: see 1.

## Not verified
- The effect of the install rows and of the rust-src remap on live divergences (machines must restart onto the code).
- `repo` itself failed at 02:16 with `Undeclared compiler-captured physical Source read refused … 🕸️graph/📥️load/💾️owner-checkpoint` (another lane's landing); I used the signed client directly for the watch.
- The key test `the observed environment … neutralised` fails (`/sdk` against `⟨sdk⟩sdk`), caused by an SDKROOT change of another lane, not by this round.

# Round 5

Landing in `T/landings.md` ("round 5"), lock caching-cargo (no other holder at the time), files under `⚡️caching/🦀️cargo/🗃️unit-cache/`.

## 1. The unit key covers every source file
- `🔏️key/🔗️sources/🟦️.ts` (new): `unitSourcesV1` finds before the compile what rustc will read: the files of `mod name;`, `#[path]`, literal `include!`/`include_str!`/`include_bytes!` (the scan of the offload planner, `reachedFilesV1`, so one scan for both), the files of the build script output directory (`OUT_DIR`, at most 256), and the names that `env!`/`option_env!` ask for. The source part of the key is now `sourceKeyV1(entry digest, rows)` under the version `sources-v2` (a row is the path below the entry's directory with forward slashes and the file digest, so another checkout of the same content has the same key); every unit re-keys once. Environment names that the key does not select already become key rows (an unset name is its own row, so setting it moves the key).
- After the compile `keyGapV1` reads the dependency file that rustc produced: a listed file the key did not cover, or an `env-dep` name the key neither selects nor scanned, is a gap. A gap means no fleet upload and the skip reason `key-incomplete` (event `remote-skip`, counter); the local store keeps the unit (it re-verifies the dependency digests on every lookup).
- Proof: `🔏️key/🔗️sources/🧪️tests` 6 pass. The digest equals an independent Python serialisation; the scan result equals the dependency file of a real `rustc --emit=dep-info` run (oracle: rustc; module, nested module, `#[path]`, `include_str!`, `env!`, `option_env!`); the key moves with a change of a module, a nested module, a `#[path]` module, an included file and with an environment value, and an identical copy in another directory has the same key; a computed include (`concat!`) and an unscanned environment name are reported as gaps while the `OUT_DIR` file is covered; through the real wrapper the same unit is a hit and every one of the three changes is a miss under another key although the entry file never changed.
- Consequence to know: units whose dependency file lists something the scan cannot find (computed include paths, proc macros that track files or variables) are now never shared (`key-incomplete`) instead of being shared under a key that cannot protect them.

## 2. New finding and fix: incremental compiles (the real source of the rlib-only unit divergences)
At 01:54Z 64 units diverged between g1-m07 and g1-m10, all `.rlib` only, same sizes, no path in either: the object members of the archive carry names that differ (`…rcgu.o`: `10vw42r` against `0850chk`). Cause measured with rustc: a compilation with `-C incremental=DIR` gives another rlib on every run (two runs, two digests) and another one than without a session, while the unit key leaves the incremental directory out. Cargo gives workspace members a session, so two machines uploaded different bytes under one key. Fix in `wrapRustcV1`: a unit that is shared through the fleet and is not focused is compiled without `-C incremental` (`withoutIncrementalV1`); a focused unit keeps its session and is not uploaded (skip reason `incremental`). Test with real rustc: two runs with a session differ, two without are equal, the stripped command line equals the compilation without a session.

## 3. Sysroot-remap units re-check
At 02:59 CET the live evidence held 3 `rlib`-only divergences from before the remap landed and the 64 incremental ones above, none with a `/Users/...` path in either side after 01:30Z. The workers (g1-m07, g1-m10 at least) still run code from before round 4c and 5 (the burst has no path leak and no `.d` difference, so it is the incremental case). Re-check after the machines restart onto round 5: `repo remote-cache evidence list` unit entries; expected: none with `incremental` or `rust-src` signature.

## Tests (pass/fail)
sources 6/0, unit-cache tests 26/1 (the one failure is the vector with `--json=diagnostic-rendered-ansi`, from another lane), freshness 2/0, churn 9/0, incremental 1/0, leaks 2/0, portable 27/0, fleet 18/0, key 8/0, offload 30/0, determinism 6/0, signed-fleet 1/0.

## Not verified
- The `key-incomplete` and `incremental` skip paths through a real fleet upload (the gap function and the stripping are tested; the share branch is covered by reading, not by a fleet test).
- Live effect: machines restart onto the code only after their supervisors do; expect one wave of cache misses (re-key).
- Windows and Linux hosts: the scan uses `/` in rows and `resolve` for coverage; not run there.
