# WP-N1: Norm Plugin (📕️norm, 15 Families) Contract + Integration

Slice N1, session 13 (2026-09-26 20:3x). Coordinator = main chat. Predecessor: the closed norm ticket
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS/` (Wave C/D restructure
11:58–18:28) and T13's finding (`📓️wp-t13.md` 20:1x). Ports 8200–8209 / 6700–6709. Private cargo target
`.tmp-ticket/wp-n1/target`. Captures `wp-n1/generated/` (expendable); durable data `.🧬semio/🌐hub/s13-n1-*`. Landing rows:
`📓️landing.md` § Session 13 Landing Window. Guest rebuild requests: `wp-w3/requests/n1.txt`.

## Session 13

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | repo contract gate: norm → 0 HIGH (classify, root cause, fix tree or rule) | in progress: root cause found; layout fixes + codec bridge LANDED; vector regeneration pending the native lane | `generated/contract-1.txt`, landing row N1 05:29 |
| 2 | descriptor/describe freshness, en + de for 15 families, exact manifest pins | en + de: 374 mutation labels fixed + census law green; landing pending wasm32 (queue); describe = W3's rebuild | `generated/label-census-3.txt`, `check-native-3.txt` |
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
- 21:0x–21:2x (before the usage cut) tree fixes, all by one-off codemods in `wp-n1/`:
  - deleted the two mistyped orphan families `📚️en1999/` (an 11-byte `placeholder` test under obsolete `🎡️tests`) and
    `🧱en1996/` (a stray copy of en1996's snapshot schema) — nothing referenced either;
  - `n1-case-rename.ts`: 49 en1995 leaf test cases `✏️sets-bM` → `✏️sets-b-m` etc. (+ the 49 `#[path]` mounts); the 14
    other `test-case-name` cases (en1991 10, din4108 3, en1998 1) were never mounted, `let _ = default()` smoke stubs →
    deleted;
  - `n1-inline-tests.py`: 7 inline `#[cfg(test)] mod … { }` bodies → `🧪️tests/🔬️<name>/🦀️.rs` + `#[path]` (din18599 field-meta,
    en1997 regen-assets (its `[DEBUG]` line dropped), iso16757/en1993/en1995 field-meta, en1994/en1995 op-text round trip);
  - din4108 `🧬️mutations/🧪️tests/🔬️fixture` (a test that WROTE `🎫️fixtures` into the repo) and en1992 `🔬️fixture/🧩puzzle5d`
    (vectors with before == after, never applied) removed with their mounts; the plugin's `⚖️compliance/🎫️fixtures` →
    `🧫️fixtures` (+2 readers);
  - `n1-oracle-moves.py`: compliance/schema oracle scripts out of test-case folders into `🔮️oracles/{⚖️compliance,🧬️snapshot-schema}/🐍️.py`
    (din16798, din4108, vdi3805, en1993, en1994; 6 readers rewritten), en1993's generated `oracle-report.json` and en1994's
    scenario-less stub feature removed;
  - `n1-bridges.py`: production codec bridge restored in 7 crates (see landing row).
- 21:3x usage cut (mid emitter). 04:59 resumed; edits were auto-committed at 22:00 (40a2736e661), nothing lost.
- 05:20 native `cargo check --lib --tests` of the 14 touched artifact crates + contract: **exit 0, 560 warnings**
  (`generated/check-native-2.txt`; the 05:01 run died on a peer's mid-edit of `🌱️value/🔁️codec`). 05:29 wasm32-wasip2
  `--lib` of the 7 bridge crates via the wasm mutex: **exit 0, 214 warnings** (`check-wasm-1.txt`). Landing row + `wp-w3/requests/n1.txt`.
- Plan for the rest of item 1 (test-only, runs after REBUILD START on the coordinator-approved native lane): build the
  ticket-local emitter `wp-n1/emitter/` (links the 15 crates; answers `kinds / dsl / derive (from_snapshot) / apply`)
  → `n1-vectors.py` derives one production vector per kind from the committed examples (single edit → `from_snapshot`
  must emit exactly one mutation, dispatch applies it with no diagnostic, its own inverse restores) → `n1-materialize.ts`
  writes `🧫️fixtures/🧬️mutations/**`, the manifest (from leaf descriptors) + catalog in `🔮️oracles/🔣️.json`, and the
  `mutate-<fam>-1` feature + Rust subject + Python oracle adapters; then `test inventory` per family (norm 🏭️bridge) and
  the contract gate again.
- 05:3x item 2 audit: norm `🔣️.json`/`🛂️.descriptor.semio` are from 2026-09-25 18:38 (pre-Wave C: 15 examples, old
  vocabularies) → W3's describe-all regenerates them; re-verify after the rebuild. Descriptor strings: 3120 localized
  objects, 0 missing a locale. SOURCE defect: 236 mutation leaves (en1991 80, din4108 43, din16798 37, en1994 25, en1998
  22, en1997 20, en1999 18) returned `label()` = kebab kind for en AND de (undo/redo history, MCP transcript), en1990's 30
  were half German (`Ändern: seismic`), din18599's 19 `attachment ändern`, en1992 passed raw field names (`"width"`,
  `"effective_depth"`, `"m_k"`) into both locales, en1993's 32 insert/remove labels had English nouns in German
  (`member #{} entfernen`, `cold-formed-member an #{} einfügen`) and CRUD "Upsert", iso16757 `productSeries`/`productClass`/
  `productIndex`, vdi3805 `correction-as-of`/`max-file-bytes`. Coordinator 05:3x: land only if native + wasm32 green and the
  row is written by 06:55.
- 05:4x fix: `n1-label-glossary.json` (239 kind rows + noun table; Eurocode/DIN terminology: Schadensfolgeklasse,
  Einbindetiefe, Steifemodul, Gesamtenergiedurchlassgrad, Kopfbolzendübel, Verkehrslastgruppe …) + `n1-labels.py` → 297
  static labels in 9 families; `n1-labels-format.py` → 77 interpolated labels in en1992/en1993/iso16757/vdi3805.
  **Law** `✏️s/🔌️plugins/📕️norm/🧪️tests/🏷️mutation-label-census/🟦️.ts` (bun): reads every leaf's `label()` (direct and
  `🦠️mutation/` split layout) across all 15 families; fails when a locale equals the kind, carries a field identifier
  (verb-led or ≥3-part kebab id, camelCase, snake_case except short subscripted symbols like `q_k`), starts lowercase,
  or the German reuses an English literal / equals the English. Red before the fix (`generated/label-census-1..2.txt`:
  hundreds of rows), **green after: 2/2** (`label-census-3.txt`, 0.6 s). Registered in the norm TS `test` verb
  (`@semio-tech/norm-js:test`), whose hardcoded example list was stale (all 30 paths missing since the
  `🧪️tests/🧩️example/` move) → now discovered from the tree.
- 06:0x native `--lib --tests` of the 13 label crates: **exit 0, 540 warnings** (`check-native-3.txt`, 6.6 min).
  wasm32 queued behind s17 (holding 06:04), ld, wg10 (ticket 05:51).
