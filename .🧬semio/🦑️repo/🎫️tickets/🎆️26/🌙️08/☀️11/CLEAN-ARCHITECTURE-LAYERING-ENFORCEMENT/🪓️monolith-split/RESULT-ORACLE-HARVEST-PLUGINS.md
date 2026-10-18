# Result: ORACLE-HARVEST-PLUGINS (prefix `ohp`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/ohp-tools/` (see section 3). Logs: `F/🗑️generated/ohp-*`. Sandboxes: `/Users/ueli/.cache/ohp/`. Started 2026-10-17 17:15. `COMPILE ADMITTED` is not in `📓️FLEET.md` (checked at start): no real cargo, no `bun repo` launcher call.

## 0. Verdict (kept current)

IN PROGRESS (22:15). Static only: no cargo (`COMPILE ADMITTED` absent), no `bun test` (Bun segfault at start, recorded not fought).

* Trinity section 7 leftovers: DONE and proven (lib 39 to 38, test 127 to 59 resolution/privacy rows, 0 new; the remaining rows are macro or rewriter artifacts, ledger).
* Draw coordinator item: the 17 `--check-applied --test` rows are down to 1 (`fixture_text!` of a declared dependency, gate-sandbox artifact of a peer's concurrent runtime-fixture edit). 7 private host-arena items made `pub`, imports restored, 3 removed-API laws re-authored (section 5).
* Families swept (typeck oracle lib+test): trinity, mathematical, vcs, dag, playbook, reasoning, demonstrator, shooting, animate, forms, note, space, architect, imperative, lowpoly, gis, flow, process, cad, fem, remodel, wfc, puzzle 3d/2d/5d, block 2d/3d/5d, layout, draw, energy, procedural, dev-misc, trinity-dev, 35 stdio artifact groups. Not swept (owner slice still running or BD10): pdf, bim, writer, curation/sourcing, sequence, raster.
* Automated repair passes (`ohp-tools/fixloop.sh`: autoimport + autowiden + autotestkit, rerun until fixed point; `autodep.py` for undeclared dependencies) are still running family by family; per-family proof is in `ORACLE-HARVEST-PLUGINS.md` section 2 (regenerate with `python3 ohp-tools/mkledger.py`).

## 1. Method

* Oracle: the analysis-only "pos" typeck oracle (`ohp-tools/pos/`, a copy of `tad-tools/pos` with the scratch dir moved to `/Users/ueli/.cache/ohp`, `POS_PREFIX=ohp`): single-crate union of a family with a stub layer for everything outside it, `rustc +nightly-2026-07-20 --emit=metadata -o <sandbox rmeta that is deleted>` for lib and `--test`. The canary (E0616, E0624, E0308) is reached in every run, so body type-check ran.
* Harvest: `ohp-tools/summ.py` (counts by wanted code), `triage.py` (resolution/privacy rows grouped by missing name; tags A = oracle rewriter artifact, A3 = macro or derive generated or undefined, EXT = name lives only in the framework, REAL = defined in the family and not inside a macro block), `typerows.py` (E0599/E0308/E0061 with a LOCAL tag when the method name is defined in the family but not in the framework), `undeclared.py` (E0433 "cannot find module or crate X" where X is an indexed workspace crate) plus `deps.py`/`cyc.py` (is the dependency declared; would adding it create a normal-dependency cycle).
* Classification rules that held up: (1) `crate` in paths start position, `..crate::`, `crate::store`, `crate::__ext` are rewriter artifacts; (2) names from `app_labels!`, `*Delta`, `*Command`, `*Apps`, `register_*_descriptors`, `transient_root!`/`window_transient_owners!` (`from_snapshot`, `addressed`, `register`, `current`) and plugin-prelude glob names (`text`, `button`, `NoConfig`, ...) are macro or external glob artifacts; (3) E0599/E0308/E0061 "unit type", derive-provided and external-trait methods are stub-layer artifacts; (4) a `pub(crate)` or private item used by a test or consumer that the carve moved into another crate is REAL (E0603/E0616/E0624/E0425/E0433).
* Proof of a fix: the keyed row diff `ohp-tools/rowdiff.py before.json after.json` (gone / new) and, for lib+test, the next run of the same family.

## 2. Families

Filled in section 4 as each family finishes.

## 3. Files

Tools: `F/ohp-tools/{pos/,step.sh,run.sh,runbatch.sh,mkfam.py,mkdefs.py,summ.py,harvest.py,triage.py,typerows.py,rowdiff.py,ex.py,show.py,undeclared.py,deps.py,cyc.py,families/*.json,ledger.tsv}`.
