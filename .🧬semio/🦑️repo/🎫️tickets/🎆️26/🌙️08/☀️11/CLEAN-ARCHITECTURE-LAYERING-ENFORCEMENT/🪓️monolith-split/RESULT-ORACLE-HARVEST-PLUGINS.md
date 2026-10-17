# Result: ORACLE-HARVEST-PLUGINS (prefix `ohp`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/ohp-tools/` (see section 3). Logs: `F/🗑️generated/ohp-*`. Sandboxes: `/Users/ueli/.cache/ohp/`. Started 2026-10-17 17:15. `COMPILE ADMITTED` is not in `📓️FLEET.md` (checked at start): no real cargo, no `bun repo` launcher call.

## 0. Verdict (kept current)

IN PROGRESS. Trinity section 7 leftovers fixed and proven by the typeck oracle (lib 39 to 38 resolution/privacy rows, test 127 to 59, 0 new). Family sweep running; the per-row ledger is `ORACLE-HARVEST-PLUGINS.md` (this folder); the machine ledger is `ohp-tools/ledger.tsv`.

## 1. Method

* Oracle: the analysis-only "pos" typeck oracle (`ohp-tools/pos/`, a copy of `tad-tools/pos` with the scratch dir moved to `/Users/ueli/.cache/ohp`, `POS_PREFIX=ohp`): single-crate union of a family with a stub layer for everything outside it, `rustc +nightly-2026-07-20 --emit=metadata -o <sandbox rmeta that is deleted>` for lib and `--test`. The canary (E0616, E0624, E0308) is reached in every run, so body type-check ran.
* Harvest: `ohp-tools/summ.py` (counts by wanted code), `triage.py` (resolution/privacy rows grouped by missing name; tags A = oracle rewriter artifact, A3 = macro or derive generated or undefined, EXT = name lives only in the framework, REAL = defined in the family and not inside a macro block), `typerows.py` (E0599/E0308/E0061 with a LOCAL tag when the method name is defined in the family but not in the framework), `undeclared.py` (E0433 "cannot find module or crate X" where X is an indexed workspace crate) plus `deps.py`/`cyc.py` (is the dependency declared; would adding it create a normal-dependency cycle).
* Classification rules that held up: (1) `crate` in paths start position, `..crate::`, `crate::store`, `crate::__ext` are rewriter artifacts; (2) names from `app_labels!`, `*Delta`, `*Command`, `*Apps`, `register_*_descriptors`, `transient_root!`/`window_transient_owners!` (`from_snapshot`, `addressed`, `register`, `current`) and plugin-prelude glob names (`text`, `button`, `NoConfig`, ...) are macro or external glob artifacts; (3) E0599/E0308/E0061 "unit type", derive-provided and external-trait methods are stub-layer artifacts; (4) a `pub(crate)` or private item used by a test or consumer that the carve moved into another crate is REAL (E0603/E0616/E0624/E0425/E0433).
* Proof of a fix: the keyed row diff `ohp-tools/rowdiff.py before.json after.json` (gone / new) and, for lib+test, the next run of the same family.

## 2. Families

Filled in section 4 as each family finishes.

## 3. Files

Tools: `F/ohp-tools/{pos/,step.sh,run.sh,runbatch.sh,mkfam.py,mkdefs.py,summ.py,harvest.py,triage.py,typerows.py,rowdiff.py,ex.py,show.py,undeclared.py,deps.py,cyc.py,families/*.json,ledger.tsv}`.
