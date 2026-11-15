# Package R16: why units are compiled more than once, and what was fixed (2026-11-15)

Scripts: `T/🧰️scripts/R16/` (`attribution.py` = table below, `land.py`, `files.json`). T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`. UC = `…/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/`.

## 1. Attribution (built)
- Schema first (`UC/🧬️schema/🔣️.json`: `Lookup`, `Eviction`, policy `lookups`, layout `y`). Every wrapper lookup (local hit, fleet hit, miss) appends one record to `<cache root>/y/lookups.jsonl` (bounded 8 MiB, rotates to `lookups.1.jsonl`): `toolchain`, `source`, `platform`, `roots`, one `extern:<name>` per dependency artifact digest, one `env:<NAME>` per observed variable, `args.<class>` for edition, kind (crate type/emit), target, codegen, metadata, features, cfg, lints, remap, diagnostics, link, unstable, source; plus the text of small classes (`unitKeyComponentsV1`, `UC/🔏️key/🟦️.ts`; `recordLookupV1`, `UC/💾️store/🟦️.ts`; call sites `UC/📜️script.ts` `lookup(...)`).
- Report: `--semio-unit-cache churn --root <unit cache root> [--since-minutes n] [--crate name] [--limit n]` prints, for every crate with more than one key, the differing components, texts seen, and crates grouped by cause. `stats` now also prints `notShared` (bypass / remote-skip / publish-skip / incomplete by reason), `evictions`, `lookups`. No `repo` subcommand was added (door/catalogue are shared with other packages); the operator command is the existing unit-cache report.

## 2. Churn causes
The recorded logs hold only hashes, so the 745 multi-key crates were classified from the local entries of each key (output names carry Cargo's metadata hash, output bytes are content hashes; `attribution.py`), and the argument-level causes were then found by running REAL Cargo through the real wrapper on a scratch workspace (builds of 5 tiny crates, new lookup records diffed). Cause -> crates (of 745):

| cause | crates | verdict |
|---|---|---|
| same metadata, same output BYTES, only the command line differs (204 alone, 242 more where dependents follow through changed externs) | 446 | noise, fixed for the causes found below |
| emit differs (cargo check, metadata only, against build): other `-C metadata`, other rmeta | 181 | real variant, not mergeable |
| same metadata, bytes differ: 57 cascade from a dependency, 11 proc-macro dylibs (link bytes), 6 root (2 rlib object bytes, 3 rmeta incl. incremental) | 74 | proc-macro identity already by producer inputs; rest unresolved (see 5) |
| entry already collected | 32 | not attributable |
| real input (source bytes changed) | 12 | real |

Observed in real runs (before/after in the new test): 
- `--diagnostic-width=N` (Cargo passes it only when stderr is a terminal, so each window size of a human's pane was a unit of its own; agents have none): split `core` and, through the proc-macro identity, every dependent (`extern:mac` differed). Now dropped from the key (`normalizeRustcArgumentsV1`, Python oracle and corpus fixture updated). This is the best explanation of the 446 noise crates (lane-global, cosmetic, bytes equal); not provable from the logs alone, the next window's `churn` output will confirm or name the rest.
- `CARGO_PRIMARY_PACKAGE`: a member selected on the command line has it, the same crate as dependency of another selection does not -> two keys. Removed from the rustc key allow-list (`UC/🧬️schema/🔣️.json` key.environmentNames; build-script runs keep it). Safe: variables the code really reads are recorded by the compiler as env-deps and checked at lookup (`lookupUnitV1`).
- `--remap-path-prefix=<target dir>=/cargo/build-dir`: a target dir outside the lane root (`CARGO_TARGET_DIR`, other checkout/machine) stayed in the key; now `⟨cargo-build-dir⟩` (`UC/🧭️remap/🟦️.ts`).
- Observed and left (real inputs): other feature sets per member selection (`args.features`+`args.metadata`; `core` with fa / fb / both), `CARGO_INCREMENTAL` (metadata hash), `--release`, check vs build, rustflags. The report names them. `resolver.feature-unification=workspace` passed with `--config` had no effect on this nightly, so it is NOT recommended or applied.
- Not causes (measured): colour flags, `--message-format json`, member selection with equal features, checkout path (local key differs only in `roots`, fleet key equal), lane, terminal without width.

## 3. Early cutoff
Dependency identity is the artifact content digest (`digestFileV1`), not path, mtime or the dependency's key (`script.ts` digests map); only proc-macro dylibs use the producer's input identity on purpose (linked bytes differ across machines). Cargo passes rmeta to library dependents (and the repo sets `embed-metadata=DoNotEmbed`), so a dependency recompiled to the same rmeta leaves dependents' keys alone. Law test proves it (rebuild with other key and path, same bytes -> same dependent key; changed bytes -> new key). Honest limit: rustc embeds a crate hash in the rmeta, so a real body change changes the rmeta; cutoff holds for rebuilds that produce equal metadata (noise-driven recompiles, other lane/machine), not for semantic edits.

## 4. Reach (units that never reach the fleet) and counters
- `UC/🔏️key/🟦️.ts:54-62` bypass: no source/crate name/out dir, print/version queries, no artifact messages, no emit, uncacheable crate type, `native-link` (bin/cdylib/dylib/staticlib with `-l`/`-L native`): host-bound by design (output depends on the machine's native libraries); each is logged as `bypass: <reason>`.
- `UC/📜️script.ts:113` `unreadable-input` (source/extern unreadable); `:145-146` fleet off, breaker down/denied, no compiler identity (`remote-skip: breaker`); `:171-173` `publish-skip` (no outputs, incomplete, no dep-info/refused); `:176` `path-dependent` (compiled bytes hold a machine path: shared only with equal paths).
- Build script RUNS are local-only (`UC/🏗️buildscript/🟦️.ts` has no fleet code; they read machine files and probe the host): host-bound; the `build_script_build` compile is shared.
- Offload takes only `OFFLOAD_POLICY_V1.crateTypes` units.
- Every case above is now counted by reason in `stats.notShared`. New counters: unit-cache evictions by kind (lanes, incremental, sessions, units, superseded, entries, forgotten; `recordEvictionV1`, hooked in `UC/🧹️gc/🟦️.ts`, prune and forget), remote cache server `evicted {entries,bytes}` in `/status` (schema `🏃️execution/🧬️schema/🌐️remote-cache.json`, `🌐️remote-cache/💾️store`, `🖥️server`). Non-rust task kinds (Nx/tsc-like): the server already counts hits/misses/puts per kind (`counters.nx`); there is still no per-task-NAME counter (the Nx API sees only hashes).

## 5. Tests (BUN_RUNTIME_TRANSPILER_CACHE_PATH=0)
New: `UC/🔏️key/🧪️tests/🧬️components` (4: one input moves exactly one component, schema, Python oracle same digests), `UC/🧪️tests/🔁️churn` (9: width/primary/lane laws; two machines with other roots+target dirs -> one portable key; early cutoff; proc-macro identity; report; log rotation; evictions+stats; REAL Cargo 6 runs over 3 member selections, odd target dir, 2 terminal widths (pty) -> one key per crate, each compiled once; real Cargo with different features -> report names `args.features,args.metadata`), remote-cache eviction test (1). Landing run (`land.py --land`, under the lock, 7 sources load ok): 33 test files, all green except `UC/🧪️tests/🟦️.ts` "build scripts that exec $RUSTC" (the replicated wrapper script has no executable bit, unrelated, red before). Includes fleet tier (real Cargo + cache server), offload cluster and determinism. Before my run, unrelated reds seen and not touched: `🌱️incremental` (expects `incremental` events nothing emits), lane/retention/pool-model/live-holder tests. Record: `T/landings.md` "Package R16".

## 6. Not verified / remains
- The actual production cause of the 446 noise crates is inferred (terminal width) and unproven on the fleet; run `churn` after a day of use. Other terminal-dependent flags would show as `args.diagnostics`.
- Edits were made in place on the live files (anchored read-modify-write, each re-read before its edit), then gated by `land.py`; no backup directory exists (process deviation).
- 17 root nondeterministic-bytes crates (rlib object bytes, a few rmeta, incremental vs non-incremental) are unexplained; 181 check-vs-build and feature-per-selection splits are real by Cargo's design.
- Windows/Linux not run (pty scenario uses `script`, darwin and linux branches).
