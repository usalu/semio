# W3-POSTBUILD-FIX report

Edited: `🗑️generated/build-runner/post-build.sh` (rewritten in place), `runner-next.sh` (one line, replaced atomically), new `lock-check.py`, `latency-kernels.py`, new `📌️important/bless-allow.txt` (comment header only), docs `📌️important/post-build.md` (section "Wave 3 changes"). No builds started, no sources touched, runner not killed.

## Gaps closed
1. Hook: `finish_run` now fires on `:build` rc 0 and on every `:test` (any rc), for every `build-*` dir. Test queuing after a green build already happens in `decide()` (TEST_EXIT empty), so `auto` only runs the lock check then. On test completion: red goes to classify/bless, green goes to locks, catalog, spr/op, acceptance, kernel latency, oracle. Old runner process keeps the old hook until restarted.
2. `bless-allow.txt`: created. First bless pass of an artifact (`FIRST_DONE` empty and `BLESS_COUNT` 0) keeps all real diffs (`kept(first-green)`), 0-byte goldens as `kept(empty)`. Later passes need allow lines, otherwise restored plus `golden-diff-n.txt`. Budget 2 unchanged. `FIRST_DONE` persists in `post-<code>.env`.
3. en1995 bless env now `EN1995_REGEN_PACKS=1 EN1995_REGEN_MUTATION_VECTORS=1 SEMIO_UPDATE_GOLDEN=1`. Docs name only `SEMIO_UPDATE_GOLDEN=1` plus the regen filter; the stale names remain only in append-only history logs (`requests.md`, wave1/2 audits, `build-en1991/latest.md`), which I left as is.
4. `.spr/.op`: fixture sweep runs inside the bless pass with the bless env (golden_files now includes `*.spr.semio`/`*.op.semio` anywhere in the subset) and as a plain check on a green test. Catalog/index: `repo run @semio-tech/dsl-derive-rs:generate-schema-index` (30 min `timeout`, memory gated, graph-gate) once per build. Cargo.lock: `lock-check.py` (tomllib, no cargo) after a green build and after a green test. Script accepts any `[a-z]*[0-9]` code; the runner PROJECTS list still has only en1990..en1999 (din/iso/vdi would need entries there).
5. Latency: `stage_latency` runs mechanics `latency_tests` in release (once per hour, flock-shared, log in `build-mechanics/latency-kernels-n.txt`) and the en1993 frame-envelope test (only where the test exists in the subset), and appends a "Kernel latency" table with targets 12/400 ms to `build-<code>/latency.md`.

## Caveats
- p95 values are not printed by passing tests (they assert and print p95 only on failure). The table shows PASS/FAIL, duration and p95 if present; `--no-capture` is passed so a future `eprintln!("p95 .. ms")` is picked up. Real numbers need that print added in Rust (not done, sources out of scope).
- Unverified, as before: `repo run ... -c release -- --no-capture -E ...` forwarding, `generate-schema-index` target name/cost (it once ran 90 min), and `SEMIO_UPDATE_GOLDEN` being honoured by the fixture sweep.
- Showcase DSL authoring and hand-pinned test numbers are still manual (plan G8, G16).

## Verification
`bash -n` on both shell scripts: ok. Dry run with a fake `repo`, fake artifact and fake runner dir in the scratchpad (overridable via `POSTBUILD_ROOT/NORM/PY/NOGATE`): red test -> first pass kept 2 files (first-green, empty), `ready for test` written; second red -> real diff rejected, restored, diff file written; green -> lock check flagged a missing package, then with a fixed lock catalog, spr, acceptance, kernel latency table, oracle all ran in order and state saved.
