# WP-N1: Norm Plugin (📕️norm, 15 Families) Contract + Integration

Slice N1, session 13 (2026-09-26 20:3x). Coordinator = main chat. Predecessor: the closed norm ticket
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS/` (Wave C/D restructure
11:58–18:28) and T13's finding (`📓️wp-t13.md` 20:1x). Ports 8200–8209 / 6700–6709. Private cargo target
`.tmp-ticket/wp-n1/target`. Captures `wp-n1/generated/` (expendable); durable data `.🧬semio/🌐hub/s13-n1-*`. Landing rows:
`📓️landing.md` § Session 13 Landing Window. Guest rebuild requests: `wp-w3/requests/n1.txt`.

## Session 13

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | repo contract gate: norm → 0 HIGH (classify, root cause, fix tree or rule) | in progress | — |
| 2 | descriptor/describe freshness, en + de for 15 families, exact manifest pins | pending | — |
| 3 | norm in `s` (after W3 restage) + hub creation of norm kinds (after all-package publish) | pending (blocked on W3) | — |

### Log

- 20:3x start. Read AGENTS.md, preambles 13 + 12, `📓️wp-t13.md`, norm ticket `📓️coordination.md`. Load 83, 6 rustc,
  128 GiB free. Contract command = `bun ./📜️script.ts contract` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`
  (writes `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`).
- 20:52 **Measured** (`wp-n1/generated/contract-1.txt`, 19 min at nice 10): **2551 HIGH repo-wide, 2365 in norm**, 76 other
  (T13's 1714 grew by 597 `missing-fixture`: feature doc-string URIs that no longer resolve). Per family (census
  `wp-n1/n1-census.py`): din4108 657, en1991 412, en1995 359, en1992 262, din18599 127, en1994 121, en1999 110, en1990 68,
  din16798 67, en1996 54, en1993 39, iso16757 36, vdi3805 25, en1997 19, en1998 6, plugin 3.
  By rule: missing-fixture 597, obsolete-testing-category 567 (561 = din4108 `🎫️fixtures`), mutation-without-fixture 283,
  manifest-only-mutation 235, contribution-manifest-invalid 209 (a rejected catalog cascades into every kind it owns),
  runtime-only-mutation 114, test-only-mutation 99, inline-test-body 83, test-case-name 63, test-data-in-case 30,
  feature-syntax 15, adapter-entry-point-missing 10, mutation-outcome-mismatch 10, 5×(no-scenarios/missing-comparison/
  unknown-mutation-catalog/…) for the stub cases.
- **Root cause.** Wave C rewrote all 15 mutation vocabularies (leaf dirs + `dsl::Mutations` enum + manifest/catalog kinds
  via generators) and verified only per-crate nx tests plus `norm-artifact-contract` 51/51. The repository test-platform
  layer was never re-run: runtime inventories are from 2026-09-25 16:05 (so production dispatch still shows the OLD
  vocabularies), catalog vectors/fixtures/`mutate-<fam>-1` features and adapters still describe the old kinds (en1990's
  353-line subject adapter was replaced by a 10-line smoke test), and five families lost the production codec bridge
  (`decode/apply/inverse_<fam>_mutation`) the test host needs. din4108's fixer additionally wrote a parallel 43-kind
  vector tree into the obsolete `🎫️fixtures` category from a *test that writes the repo* (placeholder `🔺️diff = {}`), and
  several families left sham tests (en1992 `🧩puzzle5d` "vectors" with before == after; en1991/din4108 `let _ = default()`
  smoke stubs). The contract rules are right; the tree is wrong → fix the tree.
