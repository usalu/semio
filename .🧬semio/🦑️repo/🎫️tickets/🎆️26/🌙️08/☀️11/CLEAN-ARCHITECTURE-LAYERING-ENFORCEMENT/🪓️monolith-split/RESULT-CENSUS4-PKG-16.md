# Result CENSUS4-PKG-16 (prefix `c4p16`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c4p16-tools/` (descendant of `c4p8-tools`). Logs: `F/🗑️generated/c4p16-*` (nothing deleted).

## 0. Verdict (wave 7, 2026-10-17 08:20)

**DONE statically.** All 8 files of package 16 are split into 38 cohesive children (every child at most 714 lines, every parent at most 297 lines): `part21` 1,289 -> 50 + 5 children; `document` 2,422 -> 66 + 10 children (2 unwrapped inline modules); las `header_diff` 1,268 -> 271 + 4; `zip_io` 1,301 -> 254 + 3; `mp4_io` 1,906 -> 297 + 3 (merged from 5 in wave 7, see below); `bcf_oracles` 1,289 -> 75 + 6; `step_geometry` 1,163 -> 133 + 3; `xlsx_oracles` 1,056 -> 48 + 4.

Real compile: NOT run. `COMPILE ADMITTED` is absent from `📓️FLEET.md` (checked 08:17; `grep -n "COMPILE ADMITTED"` finds nothing), so the exact command for the compile queue is in section 0c.

Evidence (all static, all re-run or re-confirmed in wave 7):
- Losslessness: `c4p16.py lossless` OK for all 8 (0 missing lines; only explained extras: docstrings, `use super::*;`, `#[path]`, `mod`, `pub use child::*;`, region marker copies). Live tree equals the staged tree byte for byte (0 differing files of 41 unit files), so nothing was edited by hand.
- Region markers: every `//#region` / `//#endregion` preserved (open equals close in every tree; split regions get a marker pair per child).
- Scoped patched typeck oracle (`📓️RESOLUTION-ORACLE.md`, lib and `--test`, canary `E0308 E0616 E0624` reached on both sides): NEW 0 / GONE 0 for all seven families (`c4p16a` part21+document, `c4p16b` las (GONE 3 = three parse errors of `☁️las/🦀️.rs` at the base revision), `c4p16c` zip, `c4p16d` mp4 (re-run as `b2` after the wave-7 merge), `c4p16e` bcf, `c4p16f` step, `c4p16g` xlsx (`b2` after the rename)); privacy, trait-scope, resolve, type, other all 0.
- Definition scan (`defscan.py`): 0 cross-file private leftovers in all 8 trees (struct fields, free structs and enums, impls).
- rustfmt: no file fails to parse; hunk counts live 109 versus 124 on the originals (children are not worse than the monolith).
- Census (`readRustSourceClosureV1`, native, wasm32-unknown-unknown, wasm32-wasip2, lib and test): no crate is newly over the 4000-line / 32-file / 262144-byte gate. Before 06:58: `mp4-tests` test cfg went 28 -> 33 files (over); wave 7 merged the mp4 io children from 5 to 3 and re-measured: `mp4-tests` 31 files, 2,981 lines, not over. Still over at baseline and unchanged by this package: `zip` (files, 106 -> 109 lib, 152 -> 155 test) and `mp4-test-oracle` (bytes 2.7 MB, embedded asset, test cfg).
- `--check-applied` gates (`bun ./📜️script.ts new crates <rust dir> --check-applied --base=f856dff0813`): las exit 0 (772/772, 1017/1023, `onlyCombined` 0/0); zip exit 0 (1012/1010, 1481/1478, 0/0); mp4 (after the merge) exit 0 (714/714, 927/931, 0/0); bcf exit 0 (796/795, 1113/1118, 0/0; run 07:06, bcf untouched since); step exit 0 (2216/2216, 3019/3019, 0/0, identical to before); xlsx exit 0 (1265/1265, 1476/1476, 0/0, identical to before).
- Source-text readers: the las descriptor facet `rust: include_str!("🔺️diff/🦀️.rs")` in the las `🧬️schema/🦀️.rs` is a `concat!` over the five files (facet-check: 5 files, 58,523 bytes, equal to the module tree); the two lib-wiring ratchet pins (part21, document) were removed once the roots are at most 150 lines (lib-wiring-budget output identical before and after: the same 2 foreign failures, no stdio row). The hub composition fixtures and the `#[path]` mount lines of the artifact crate roots point at the parent files, which stay.
- No compatibility re-exports: the only re-exports are the `pub use child::*;` lines that keep the parent's own previous public surface (the children are private modules of the parent); no alias, shim or deprecation.
- Duplicate mounts: `🗄️stdio` multi-mounted targets 326 before and 326 after (scan of 06:58, targets 7,170 -> 7,210); the live `c4p16check.py` re-run after the mp4 merge reports no duplicate mount of any child.
- Docstring emoji uniqueness in each crate closure, test-name sets (0 tests moved or renamed), longest path (192 bytes relative, 240 limit) all pass (`c4p16check.py live`: CHECKS OK).

## 0c. Compile-queue command (not run)

For each workspace, one launcher call under the compile mutex (canonical form from `📓️WAVE5-BRIEF.md`; lib and tests):

`cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crates> --lib --tests --keep-going --message-format=short --manifest-path <workspace>/Cargo.toml`

- `✏️s/🔌️plugins/🗄️stdio/Cargo.toml`: `semio-s-artifact-stdio-contract-part21 semio-s-plugin-stdio-document-test-oracle`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/Cargo.toml`: `semio-s-artifact-stdio-las-schema semio-s-artifact-stdio-las-snapshot semio-s-artifact-stdio-las-tests`
- `…/🗿️artifacts/🎒️zip/Cargo.toml`: `semio-s-artifact-stdio-zip semio-s-artifact-stdio-zip-test-oracle`
- `…/🗿️artifacts/🎥️mp4/Cargo.toml`: `semio-s-artifact-stdio-mp4 semio-s-artifact-stdio-mp4-tests semio-s-artifact-stdio-mp4-test-oracle`
- `…/🗿️artifacts/💬️bcf/Cargo.toml`: `semio-s-artifact-stdio-bcf-test-oracle`
- `…/🗿️artifacts/📐️step/Cargo.toml`: `semio-s-artifact-stdio-step semio-s-artifact-stdio-step-geometry semio-s-artifact-stdio-step-test-oracle`
- `…/🗿️artifacts/📕️xlsx/Cargo.toml`: `semio-s-artifact-stdio-xlsx-test-oracle`

Foreign errors seen: the lib-wiring-budget law has 2 failures on lowpoly, procedural and layout roots (peers, no stdio row); the hub oracle composition test has 3 failures at baseline (60 pass), identical before and after; the `☁️las/🦀️.rs` base revision has 3 parse errors that are not in my file.

## 0b. Wave-7 changes

- Rolled back and re-split my own `mp4_io` install (SHA256 of the root equal to the snapshot before reinstall) because `mp4-tests` (test cfg) had grown from 28 to 33 files. New spec: `box_parsing_and_decode` (sample tables folded in, 590 lines), `box_builders` (330), `encode` (plan and cursor merged, 714); `newdirs.txt` re-resolved through `tax.ts` (39 dirs, 0 unresolved). Verified by oracle `c4p16d b2` (NEW 0 / GONE 0), rustfmt, `c4p16check.py live`, census and `--check-applied`.
- Re-ran the `--check-applied` after-runs of step and xlsx that died with EMFILE: both exit 0 and identical to before.

## 0a. Found state (wave 7, 07:27)

- All 8 files were already split and installed; the sections below record the pre-restart work. Live-versus-staged comparison (`c4p16.py stage <key>` into a fresh scratch dir, byte compare of every unit file): 0 differing files for all 8 specs; `c4p16.py lossless` OK for all 8 (zero missing lines).
- Unfinished at the cut: the oracle log `c4p16-oracle-c4p16f-b1` (step) and `c4p16g-b2` (xlsx) had finished with NEW 0; the `--check-applied` after-runs of step and xlsx died with `EMFILE` (machine file table exhausted, no verdict); the census after-run showed `mp4-tests` (test cfg) at 33 files, one over the 32-file gate (it was 28 before; the 5 mp4 io children pushed it over).

## 0. Found state (written first, pre-restart)

- Fresh start (no earlier `c4p16` tools or logs). All 8 files of package 16 are clean against `HEAD` (`git status` empty for each) and equal the census line counts: 1289, 2422, 1268, 1301, 1906, 1289, 1163, 1056 (11,694 lines).
- Pre-edit snapshots with SHA256 are in `F/c4p16-tools/orig/*.rs.txt`.

## 1. Status and verdict (kept current)

- In progress (nothing installed yet): `c4p16-tools/c4p16.py` (descendant of `c4p8.py`, adds `unwrap` of inline `mod x { ... }` bodies into `#[path]` file modules, dangling-doc parsing and native-empty-region protection) stages all 8 files; the line multiset is lossless for all 8 staged trees (`c4p16.py lossless <key>`). Next: typeck oracle baseline `a0`, install, verification.
- Decisions made so far: the `LasDiff` struct stays in `🔺️diff/🦀️.rs` (the schema-field laws read it there); the descriptor facet `rust: include_str!("🔺️diff/🦀️.rs")` in the las `🧬️schema/🦀️.rs` becomes a `concat!` over the children (precedent `RESULT-PLUGIN-GATES-GREEN.md` section 4); the lib-wiring ratchet pins of the two lib roots (part21, document) are removed once their roots are at most 150 lines.

## 1a. Install log (kept current; a cut leaves this as the record)

- Baselines taken before any install: pos oracle `a0` (running), `lit.py` pre (0 sites), census before (`c4p16-census-before.txt`), lib-wiring-budget before (2 foreign failures, none stdio), hub oracle composition test before (60 pass, 3 fail, foreign), duplicate mounts before (326 multi-mounted targets in `🗄️stdio`), `--check-applied` before: las exit 0 (`onlyCombined` 0 native and test), others running.
- Installed: `header_diff` (4 children; parent 271 lines) and the descriptor facet `concat!` in the las `🧬️schema/🦀️.rs` (facet-check: 5 files, 58,523 bytes, equal to the module tree).
- Installed (cont.): `part21` (5 children, root 50 lines), `document` (10 children in 2 unwrapped inline modules, root 66 lines), `zip_io` (3 children, parent 254 lines); lib-wiring ratchet pins of the two lib roots removed (the lib-wiring-budget test output is byte-identical before and after: the same 2 foreign failures, no stdio row).
- A first full-family oracle run `a0` was killed from outside (exit 143, probably a peer `pkill`); the oracle is now run per group (`c4p16a` part21+document, `c4p16b` las, `c4p16c` zip, `c4p16d` mp4, `c4p16e` bcf+xlsx, `c4p16f` step) with base `b84bf526c7b`.
- `--check-applied` before (las exit 0 `onlyCombined` 0/0; zip exit 0 `onlyCombined` 0/0, uncarved) ; mp4, bcf, step, xlsx before-runs still queued; each artifact is installed only after its before-run finished.
- Installed (cont.): `mp4_io` (5 children), `bcf_oracles` (1 unwrapped module with 5 grandchildren), `xlsx_oracles` (4 children, installed twice, see next line). `step_geometry` waits for its before-run.
- Real defect caught by the typeck oracle and fixed: the first xlsx install had a child module `shared_strings` that collided with the file's own inline `pub mod shared_strings` (the oracle reported 9 new E0425 `cannot find function read_pool in module shared_strings` in lib and 17 in test). I rolled back my own install from `orig/` (SHA256 equal), renamed the child to `shared_string_pool` (and the step child `builder` to `solid_builder`, same hazard) and added `c4p16-tools/modnames.py` (child module names versus every identifier of the original; 0 collisions). The log of that failing run was deleted by mistake; the per-run `.err` files stay.
- Typeck oracle `b1` (per group, base `b84bf526c7b`, lib and `--test`, canary `E0308 E0616 E0624` reached on both sides): `c4p16a` part21+document NEW 0 / GONE 0 (lib 1667 errors, test 2271); `c4p16b` las NEW 0 / GONE 3 (three parse errors of `☁️las/🦀️.rs` at the base revision, not my file); `c4p16c` zip NEW 0 / GONE 0; `c4p16d` mp4 NEW 0 / GONE 0; `c4p16e` bcf NEW 0 / GONE 0. Privacy, trait-scope, resolve, type and other all 0.
- `--check-applied` after, las: exit 0, identical to before (native 772/772, test 1017/1023, `onlyCombined` 0/0).
