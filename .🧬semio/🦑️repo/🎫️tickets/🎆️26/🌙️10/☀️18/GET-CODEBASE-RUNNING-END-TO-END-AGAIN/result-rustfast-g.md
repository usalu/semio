FOR gate-g: batch ready C/rustfast-g-tools/staged/rustfast (10 edited library/process files + 16 new files; `python3 -I stage.py check|apply|revert [--root DIR]`, exit 2 = drift; rebuilt by `python3 -I C/rustfast-g-tools/build.py` from the live tree). Touches only: process native-build `🕸️units` (+capture), unit-cache `📖️admission`/`🔏️key`/`📜️script.ts`/schema, cargo `⏱️timing`, cargo `🚪️launcher`. NOTHING applied live, NO official run, NO edit ON.

# result-rustfast-g (checkpoint, usage cut): why a hot one-edit Rust rebuild spends 327 s before rustc

## State
- STAGED, verified on mirror `rfast` (`$TMPDIR/sm/rfast`, batch applied): new laws 4+1+2+1+1 pass (memo key, key parts, foreign digests, ledger, plan stability; the live real-Cargo test is skipped in the mirror); RED on the base tree (all five fail without the code edits); existing unit-cache suite 26/26 (one old assertion adapted: `id` no longer keys the memo), key suite 8/8, units duplicate-edges 1/1.
- NOT verified: tsc strict on the touched files; `launcher` end to end (marks/ledger/foreign port are only exercised by an official run); native-build admission memo test and timing oracle test (need a process invocation, not runnable single-file); real-Cargo plan-stability law (needs the live toolchain: runs after landing, ~30 s).
- Next command: `python3 -I C/rustfast-g-tools/staged/rustfast/stage.py check` then `apply`; verify with `sh`-less single files: `bun test ./<file>` for the five new test files (paths in new.json) + `unit-cache/🧪️tests/🟦️.ts`; then ONE official `print-rs:build` run twice (cold memo, then hot) and read `.cache/b/p/m/bazel/outputs/server/*/execroot/_main/bazel-out/_tmp/actions/stderr-N` or the new ledger `<rust store>/timing/cargo-phases.jsonl`.

## Measured facts (from rustedit-g's e1 run; the Bazel action stderr is KEPT in `.cache/b/p/m/bazel/outputs/server/s4/execroot/_main/bazel-out/_tmp/actions/stderr-2`, copy: `C/rustfast-g-tools/e1-action-stderr.txt.gz`; the `native:owner-command` heartbeat gives seconds since the nested command started)
| nested seconds | phase | evidence |
|---|---|---|
| 0-100 | silent boot (bun repo, store, lane, toolchain verify) | no progress rows, owner heartbeat only; boot-g's area |
| 100-140 | cargo preparation, fetch, metadata, unit-graph | `cargo:preparation`, `workspace-selection`, `unit-metadata` rows |
| 140-185 (45 s) | FOREIGN SOURCE: every file of every registry crate re-read and sha-256 hashed each run, ~4,100 files, one publish+yield per file | 4,130 `cargo:foreign-source` rows; code `actualCargoCompilerPlanningPortV1.sourceInputs` |
| 185-300 (115 s) | SOURCE ADMISSION: 375 units, 375 misses, 0 memo hits (3,845 reads, 2,144 files, 45 MB) | `Source admitted: i/375` rows: read counter grows for every unit |
| 300-327 | recipe inputs, scheduler, broker | |
| 327-827 | rustc `-Zthreads=1` incremental (500 s, 4.2 GB) | rustedit-g |
| 827-850 | artifact capture: 110 MB rlib, EXCLUSIVE `artifact-capture` lease, 36 s; 3 `[lease-order] Lock order violated` notices (artifact-capture rank 6 requested while compiler-writer rank 7 held) | rows + stderr |
| 850-940 | silent (consume / Bazel finish), then a last `workspace-selection` | no rows (the new `consume` phase mark covers it) |
Raw hashing of the 107 MB rlib in 64 KB async chunks costs 1.7-2.0 s wall (0.2-0.7 s at 1 MiB): the 36 s is per-operation custody overhead in `📤️publication/📥️sources/📦️capture` (chunk `64 KiB` schema constant `chunkBytes`, checks/proofs per operation), NOT hashing. OPEN, not staged (publication owner): raise chunkBytes to 1 MiB (also hard-coded 65536 in `📤️publication/🟦️.ts` lines 18/103 and capture 27/34/45) and measure; fix the lock-order violation.

## Root causes found (measured on the real tree, graph-free)
1. ADMISSION MEMO NEVER HITS. `cargo build --unit-graph` run twice back to back on the print workspace gives 81 of 372 workspace units a different `planningDigest` (it hashes the raw unit including each dependency's array `index`; Cargo renumbers). `C/rustfast-g-tools/memo-key-stability.ts` (bun, pure functions on two real plans + `cargo metadata`): 291 equal, 81 different keys, the only differing field `planningDigest`. The memo key hashed the WHOLE unit including that digest, the ids, the dependency edges, `configurationDigest` and `compilerEnvironment` (PATH, RUSTUP_HOME of the invoking process), and `remember` re-hashed the unit AFTER the admission rewrote its fields. Memo files are fine (393/400 recent memos still validate against the files); `a/` holds 232,596 files / 2.4 GB with no GC (open: layout `admissions` is never collected).
2. FOREIGN INVENTORY has no memo (every registry crate file hashed per run).
3. 94 UNCHANGED CRATES RECOMPILED IN THE PRIME (unit cache): the first-level key of one crate differs between runs although sources, `-C metadata`, outputs and rlib bytes are identical (libc: keys `84788511d9`, `4d6e0bcfa6`, `73ddcc7030` all with extra-filename `2c0aaee73bd67d20`; `b5d5c415e3`, `2f42705861` both `48fe2980cd1d8ee6`; entry records equal apart from `key`). Toolchain digest is content based and unchanged since 10-18; so the cause is in normalized args or the key environment (`SDKROOT`, `MACOSX_DEPLOYMENT_TARGET`, `CARGO_MANIFEST_DIR`, `RUSTC_BOOTSTRAP`, `CARGO_ENCODED_RUSTFLAGS`) of the launching context. NOT PROVEN. Staged instrument: every `miss` event now carries `reason: "toolchain:xxxxxxxx args:... source:... externs:... environment:..."` (short digests per key part); the next prime shows which part differs (compare two events of one crate in `u/x/*.jsonl`; `events.py` of rustedit-g reads them).
4. No timing rows in a Bazel action: stderr is swallowed on success, but Bazel keeps it (see above). Staged: an always-on ledger (one JSON line per phase mark, bounded 1 MiB + one generation) at `<rust store>/timing/cargo-phases.jsonl`, independent of `SEMIO_CARGO_TIMING`, plus a new `consume` phase for the post-rustc 146 s.

## The staged batch
- `🕸️units/🟦️.ts`: `planningDigest` resolves dependency indices to unit ids (plan-order independent).
- `📖️admission/🟦️.ts`: memo key = `ADMISSION_KEYED_FIELDS_V1` (declared once, fixture-checked) + compiler environment without `ADMISSION_VOLATILE_ENVIRONMENT_V1` (PATH, SystemRoot, WINDIR, PATHEXT, RUSTUP_HOME); recall returns the current volatile values; a unit is remembered under the key it was recalled with.
- `capture/🟦️.ts` + new `📖️admission/📚️foreign/🟦️.ts` + launcher call site + schema layout `foreign: "f"`: foreign inventory through a stamp memo (dev, ino, size, mtime, ctime; racy window 2 s; books per crate dir; poisoned book ignored), progress every 64 files instead of per file.
- `🔏️key` `unitKeyPartsV1`, `📜️script.ts` miss reason; `⏱️timing` ledger + `consume` phase; launcher marks.
- Laws (each with a language-agnostic fixture and an independent Python oracle; plan stability also with real Cargo): `📖️admission/🧪️tests/🔑️key-stability`, `📖️admission/📚️foreign/🧪️tests`, `🔏️key/🧪️tests/🧩️parts`, `⏱️timing/🧪️tests/📒️ledger`, `🕸️units/🧪️tests/🔁️plan-stability`.

## Projected per-phase budget (one-line print edit, hot, 16 GB class, WITH the staged batch; CPU-based, not measured live)
| phase | measured now (load 30-50) | projected |
|---|---|---|
| capacity queue / replan | 1012 s / 250 s | quietfc-g, toolsetup-g, verify2-g (not mine) |
| nested boot | 100 s | boot-g; ~10-15 s on an unloaded host (rust-g: ingress 27 s idle) |
| cargo prep + unit graph | 40 s | ~20 s (cargo itself 19 s) |
| foreign source | 45 s | ~1 s (4,100 lstat + readdir; 0 reads when stamps hold) |
| source admission | 115 s | ~2-5 s once memos exist (a first run per plan still pays ~100 s and writes 375 memos; hit rate on repeats projected ~100 %: 291 of 372 keys were already stable, the rest by the digest fix) |
| rustc incremental, 1 thread | 500 s | ~40-60 s with admitted threads on an idle host (CPU ~125 s at load 40; non-incremental 10 threads 67-87 s); cheap leaf needed to measure |
| post (capture + consume) | 146 s | open: publication chunk size + lease order; unknown until the `consume` mark is read |
Total unloaded estimate for the Rust part after queue/replan: boot 15 + prep 20 + admission 5 + rustc 50 + post 30 = ~2 min. Not measured.

## Open
1. Land + measure (cold then hot) after gate-g; read the ledger.
2. 94-crate key cause (instrument staged, cause open).
3. Publication capture 36 s + lock-order violation (publication owner), admissions `a/` GC (2.4 GB, 232k files, never collected).
4. rustedit-g's edit hit a function with a JSON-expanding macro; a cheap leaf measurement is still missing.
5. tsc strict over the touched files not run.
Tools: `C/rustfast-g-tools/` (build.py, src/, staged/rustfast, memo-key-stability.ts, chunk-bench.ts, e1-action-stderr.txt.gz). Mirror `rfast` is mine (drop with `python3 -I C/mirror-tools/mirror.py drop rfast` when done).
