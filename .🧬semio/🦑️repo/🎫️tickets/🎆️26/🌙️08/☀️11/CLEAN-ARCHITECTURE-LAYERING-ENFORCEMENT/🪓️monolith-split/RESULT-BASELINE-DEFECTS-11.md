# RESULT: BASELINE-DEFECTS-11 (`bd11`)

Executor: Sonnet, 2026-10-17 from about 20:00. Foreground, no sub-agent, no modifying git command, no ticket tool. Continues `RESULT-BASELINE-DEFECTS-10.md` and ledger row O-13. Tools in `bd11-tools/`, logs in `🗑️generated/bd11-*`, pre-edit copies of every touched test in `bd11-tools/orig/` (index `INDEX.tsv`).

## 0. Verdict

IN PROGRESS (written early; rewritten at the end).

## 1. Found state

* Baseline sweep (`bd11-tools/sweep.sh`, 6 parallel, `🗑️generated/bd11-sweep-before.tsv`): about 71 failing files / 174 failing tests, as BD10 measured; a few rows are load artefacts (60 s cap hit under parallel load: `descriptor-verification` passes 21/0 alone, `composition` is 37 pass / 1 fail alone).
* Transient peer churn: `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` was syntactically broken for a while (catalog `host-activation` and `semantic-source-pairs` printed `Expected ")" but found ";"` from that file); not ours, re-checked at the end.

## 2. Tools

* `sweep.sh <list> <out.tsv> [parallel] [cap]`, `one.sh <cap> <file>` (pass, fail, exit, file, first error).
* `needles.py <test.ts>`: runs the file with soft expects (`bd8-tools/softexpect.ts`), prints every failing assertion with its literal needles and whether each literal exists in the plugin family text (FOUND file:line) or not (ABSENT plus near matches).
* `repoint.py <test.ts>`: in every test block with a failing assertion, replaces `readFileSync(new URL("<non-test .rs>", import.meta.url),"utf8")` by `pluginFamilyText()` (BD8 reader) and adds the import.
