# Result CENSUS4-PKG-25 (prefix `c4p25`): plugin-family, remodel optimize-internals + energy engine and model editor

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c4p25-tools/`. Logs: `F/🗑️generated/c4p25-*` (nothing deleted).

## 0. Verdict

DONE (static evidence; real compile not admitted). Wave 7, 2026-10-17 09:05. All 8 files are split (8 roots rewritten, 34 children created: rem 4, kern 6, mod 4, pre 3, wire 3, ed 6, unit 4 `include!` fragments, epj 4). Losslessness, line order, rustfmt parse, mounts, pub-API, region markers, census, scoped typeck differential (IDENTICAL), reader probe (energy rows IDENTICAL) and both post-install `--check-applied` runs (0 new unresolved names) are green. The only open item is the real `bun repo cargo check` (no `COMPILE ADMITTED` line in `F/📓️FLEET.md` at 09:05); the exact command for the compile queue is in section 6.

## 1. File list (`F/MONOLITH-CENSUS-4.md` section 4, Package 25, 8 files, 10,758 lines)

| key | lines | file (relative to `✏️s/🔌️plugins`) |
| --- | ---: | --- |
| rem | 1,034 | `📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️optimize-internals/🦀️.rs` |
| kern | 2,162 | `🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs` |
| mod | 1,322 | `🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs` |
| pre | 1,014 | `🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️.rs` |
| wire | 1,089 | `🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📡️wire/🦀️.rs` |
| ed | 1,574 | `🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| unit | 1,518 | `🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| epj | 1,045 | `🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs` |

## 2. Found state (session 1)

- No `F/c4p25-tools`, no RESULT file, no child folders from this slice existed at the start. All 8 files unmodified by this slice; `ed` is staged against HEAD (`M `, peer/auto-commit work) with no unstaged change.
- The energy artifact has a peer working on the TS mutations (`A  ...🧬️schema/🧬️mutations/*/🟦️.ts`); unrelated to the 8 Rust files.

- Pre-edit snapshots (byte copies of the 8 live files, taken before the first edit): `F/🗑️generated/c4p25-orig/{rem,kern,mod,pre,wire,ed,unit,epj}.rs`. Stage tree (scratchpad, regenerable from the snapshots with `python3 F/c4p25-tools/c4psplit.py stage`): `scratchpad/c4p25-work/stage`.
- Tools: `F/c4p25-tools/` (copied from `c4p22-tools`, then extended: impl-method ranges `(a, b, "impl X {")` wrapped per child, `drop` lines, literal marker lines, `replace` rebases, a `pub(crate) struct` field-widening fix, `checkapplied.sh`, `census.sh`, `probe.ts`).

## 2b. Found state (wave 7, 2026-10-17 07:28)

- Process restart at about 07:05. All 8 live files are byte-identical (`cmp`) to the snapshots `F/🗑️generated/c4p25-orig/*.rs`; no child folder exists; `c4p25-installed.json` does not exist. Nothing of this slice was installed, so there is no half-applied batch to finish or roll back.
- Logs on disk from session 1: `c4p25-pos-{pre,post1,post2}.log`, census-before, lit-pre, probe-before.json (07:00, 105 kB, `.err` empty), checkapplied logs. The `--check-applied` baseline for `energymodel` ended at 07:07 with `ENFILE: file table overflow` (system-wide fd exhaustion, not a result), the `energyengine` and `remodeling` trees have no artifact package. A stale `.exit` file claims exit=1 for all three.
- The scratchpad stage and `c4p25-work` are gone; `c4plib.SCRATCH` and `checkapplied.sh` still name the dead scratchpad of the earlier session and are re-pointed to the current one.

## 3. Baselines recorded before the first edit

| Check | Command | Result |
| --- | --- | --- |
| Typeck oracle `pre`, lib and test | `POS_PREFIX=c4p25 python3 c4p25-tools/posrun.py run c4p25-tools/c4p25.json --cfgs lib,test --label pre --no-base` (7 crates: remodel optimize, energy engine kernel, model, precompute, sim-wire, energy model editor, energy model export) | canary reached (`E0308, E0616, E0624`), lib 1056 rows, test 1342 rows (log `c4p25-pos-pre.log`) |
| E0451 literal scan | `POS_PREFIX=c4p25 python3 rpv-tools/lit.py c4p25-tools/c4p25.json pre` | 0 candidate E0451 sites; 93 brace structs; 20 files in the module tree (`c4p25-lit-pre.log`) |
| Census (source boundary) | `c4p25-tools/census.sh before` (`measureRustCrate`, hard gate 32 files / 262,144 bytes / 4,000 lines) | remodel optimize 2 files 44,704 B 1,054 lines; kernel 1 / 107,829 / 2,134; model 1 / 55,583 / 1,309; precompute 1 / 48,112 / 1,012; sim-wire 1 / 44,028 / 1,087; editor 2 / 101,617 / 1,580; export 5 / 62,819 / 1,108; all inside the gate |
| `--check-applied` | `c4p25-tools/checkapplied.sh before energymodel remodeling` (`bun ./📜️script.ts new crates <artifact>/🏅️standards/🔖️1 --check-applied --base=f856dff0813 --sandbox=<dir>`) | energymodel: base 3704 / combined 3711 (native), 3926 / 3927 (`--test`), onlyCombined 0 / 0 (`c4p25-checkapplied-before-energymodel.log`, wave 7; the first try died with `ENFILE`, kept as `...-enfile.log`); remodeling: 3329 / 3341 and 4252 / 4259, onlyCombined 0 / 0 (`c4p25-checkapplied-before-rm-remodeling.log`). The energy engine tree is not an artifact package: `verifyAppliedLayout: ... has no artifact package`, exit 1 before and after (first try, `c4p25-checkapplied-try1-*.log`), so there is nothing to gate there. |

## 4. Wave 7 recovery and install

- Rebuilt the stage from the snapshots with `python3 c4p25-tools/c4psplit.py stage` (scratchpad `c4p25-work/stage`; `c4plib.SCRATCH` and `checkapplied.sh` re-pointed to the current scratchpad). The rebuilt stage is the plan as it stood after the last session-1 edit of `plans.py` (the ed child `🎛️retained-commands`; the emoji `🧵️` of the earlier name list is gone). It passed again before any write: lossless (156 added wiring/docstring/widening lines, 0 unexplained removed), sequence, rustfmt parse (42 files), mounts, pub-API, fmt hunks 76 to 75, typeck oracle `post3` against `pre` (lib 1056 rows vs 1056, test 1342 vs 1342, only-pre 0, only-post 0: POSDIFF IDENTICAL), taxonomy match of the 34 new folders (`c4ptaxcheck.ts`, 34 ok, 0 unresolved).
- Baselines (all before the first write): `--check-applied` energymodel and remodeling, reader probe `c4p25-probe-before2.json` (fresh; the 07:00 probe differed only by peer churn in unrelated rows, energy rows equal), census-before, E0451 literal scan, pos-pre.
- Install: `python3 c4p25-tools/c4pinstall.py kern mod pre wire ed unit epj` (energy, after the energymodel baseline), then `rem` (after the remodeling baseline). Each install asserted the live root byte-identical to the pre-edit blob; journal `🗑️generated/c4p25-installed.json`. Live bytes of all 42 files equal the staged bytes (`cmp`), so the staged oracle result holds for the live tree.
- Consumer re-points (`c4pconsumers.py apply`, 7 anchored edits, verified): (1) `📚️library/🔍️discovery/📖️source-access/🟦️.ts` lists the energy editor root among the module-tree-read sources; (2) `🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json` points the energy editor `source` at the moved `🖥️editor-impl` child; (3) five entries (kernel, model, precompute, sim-wire, editor roots; 150-line budget now met by 137/34/42/43/144 lines) removed from `🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json`.

## 5. Post-install gates (all run, real results)

| Gate | Result |
| --- | --- |
| lossless / sequence / parse / mounts / api / fmt (live) | LOSSLESS OK (0 unexplained removed); SEQUENCE OK; PARSE OK 42 files; MOUNTS OK (no missing target, no duplicate, every new file reached once); API OK (pub and pub(crate) top-level items 22/25/61/20/40/5/1/41 before = after, lost 0, gained 0); fmt hunks 76 to 75 |
| `c4pcheck.py` (live) | CHECKS OK: no duplicate first emoji, `//#region` pairs never split (orig = new counts), test fns 53 == 53 in unit, max child 710 lines / 239 path bytes (limit 240) |
| E0451 literal scan (`rpv-tools/lit.py ... post`) | 0 candidate sites, 93 brace structs, 93 definitions, 54 files in the module tree (pre: 20 files) |
| Census (`census.sh final`, hard gate 32 files / 262,144 B / 4,000 lines) | rem 6 files 45,803 B 1,086 lines; kern 7 / 110,702 / 2,192; model 5 / 56,556 / 1,339; precompute 4 / 48,889 / 1,036; sim-wire 4 / 44,851 / 1,111; editor 8 / 103,206 / 1,625; export 9 / 63,708 / 1,140: all inside the gate |
| `--check-applied` after, energymodel | native base 3704 / combined 3711, `--test` 3926 / 3927, onlyCombined 0 / 0, exit 0 (`c4p25-checkapplied-after-energymodel.log`) |
| `--check-applied` after, remodeling | native 3329 / 3341, `--test` 4252 / 4259, onlyCombined 0 / 0, exit 0 (`c4p25-checkapplied-after-rm-remodeling.log`) |
| Reader probe after (`c4p25-probe-after.json`) | proofs 640 = 640, proofsEnergy 23 = 23 (same rows), rowsEnergy 0, appOwnedEnergy 0, failuresEnergy 48 = 48 (same set), scan energy failures 23 = 23 (same set), completionRejectionOracle equal. Global `sources` 16997 to 17039, `failuresTotal` 1312 to 1310 and 35 scan rows in `draw` differ because of peer churn, none concerns the energy files. |
| `[DEBUG]` scan, cfg scan | no `[DEBUG]` left; the 8 files carry no `cfg(...)` other than `cfg(test)` (no wasm-gated code), so the lib and test configurations are complete |
| Engine crates (`--check-applied`) | not applicable: `verifyAppliedLayout: ... has no artifact package` for the energy engine tree, before and after; their gate is the scoped typeck differential above |

Real compile: not admitted at 09:05 (`F/📓️FLEET.md` has no line starting `COMPILE ADMITTED`), so no `bun repo cargo` call was made by this slice.

## 6. Compile-queue command (one batched call, under the fleet mutex)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-remodel-remodeling-optimize -p semio-s-plugin-energy-engine-kernel -p semio-s-plugin-energy-engine-model -p semio-s-plugin-energy-engine-precompute -p semio-s-plugin-energy-engine-sim-wire -p semio-s-artifact-energy-model-editor -p semio-s-artifact-energy-model-export --lib --keep-going --message-format=short
```

Run the same with `--tests` instead of `--lib` (the editor `🧪️tests/🔬️unit` fragments and the kernel/model unit tests only compile there). If a crate sits in the framework workspace, append `--manifest-path 🧰️framework/Cargo.toml` after `--`.

## 7. Foreign errors and hand-offs

- None attributed to this slice. The scoped oracle rows (stub-layer noise, unresolved external crates) are identical before and after the split.
- Peer churn seen while working (not touched): `draw` editor forged-factory scan rows, the `gis` editor rows in the probe, a large number of concurrent `--check-applied` runs (load 50-60, one `ENFILE` fd exhaustion during the first energymodel baseline).
- The tool folder `c4p25-tools/` and `🗑️generated/c4p25-*` are kept as the audit trail (nothing deleted).
