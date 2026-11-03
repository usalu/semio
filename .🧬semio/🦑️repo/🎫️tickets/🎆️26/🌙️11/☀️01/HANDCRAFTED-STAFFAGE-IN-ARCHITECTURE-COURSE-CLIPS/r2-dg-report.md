# 🧹️ Round 2 Style Check (agent dg)

## Scope and method

- Input: `r2-changed-files.md`, sections Created and Modified, `.py`/`.ts`/`.tsx` only. Deleted and Uncertain were excluded. 123 files checked (70 py and 55 ts/tsx entries in the inventory, minus the two deleted py entries).
- Created files: every line counted. Modified files: only lines added against `HEAD` (`git diff HEAD -U0`, read-only).
- Checker: `T/🧰️dg/check_style.py`. Python uses `tokenize` and `ast`. TypeScript uses a small lexer that handles strings, template literals, and regex literals, and tracks bracket depth.
- Cross-check: `grep` for indented `#` lines and `//` lines in the same files.

## Results

- (a) Comments inside definitions, added lines: 0 violations. The only hits were `// @ts-expect-error` pragmas in `🧪️tests/🤐️textless-depiction/🟦️.ts:40,42`. These are the TS counterpart of the allowed type-ignore pragma, so they were kept.
- (a) Indented `#endregion` markers (6 hits in `📦️volume` and `🧮️calculation`) are not added by this ticket. Left alone.
- (b) `[DEBUG]` markers: 0.
- (b) `print` calls in added lines, kept as intentional CLI output of standalone generator scripts under `__main__`:
  - `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🚀️export-performance/🐍️.py:60` prints a case count.
  - `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🫕️phase-change/🐍️.py:123-124` prints the oracle table and the CoolProp provenance line. Review if you consider these debug dumps; removing them is a one-line edit each.
- (c) Docstrings and JSDoc not starting with an emoji, added lines: 0. Two earlier hits were false positives from my checker, for emoji written as a base character plus U+FE0F (`〰️`, `↔️`). The checker now accepts those.

## Edits

None. No line in the scope violated (a), (b) or (c) beyond the kept items above.

## Test runs (`T/m9_tests.sh`)

- Staffage guard `🧪️tests/🐍️staffage_only`: Ran 22 tests in 285.8 s, OK. Log in `T/🗑️generated/dg/guard.log` (deleted afterwards).
- Visuals suite `🎨️visuals/🧪️tests/🐍️`: Ran 7 tests in 0.4 s, OK.

## Still open

- The TS scanner does not parse regex literals that start after an identifier. These are treated as division, so a comment inside such a line could be missed. The cross-check grep found no `//` comments in added TS lines beyond the pragmas, so the risk is low.
- The 13 Uncertain-attribution files were not checked, by design.
- Checker and probes kept in `T/🧰️dg/` (`check_style.py`, `probe_scanner.py`, `probe_depth.py`).
