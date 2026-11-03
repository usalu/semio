# NDP orphans (wave 2)

## Result
- Orphan lookups: **0**. The "134 orphans" came from the raw `ndp\.` token scan (`refs.json`), which counts doc text, `.py`/`.pyc`, table prefixes (`ndp.en1996.k`), test negatives and `format!` stems. Nothing to retype, add or remove at call sites.
- Refined scan (`🗑️generated/ndp-orphans/scan-lookups.py`, `scan-exact.py`, `scan-dyn.py`): 460 direct-call literals (`ndp`, `ndp_number*`, `ndp_common`, `ndp_is_verified*`, `ndp_lookup`, `ndp_awaits_value`, `.ndp(s)`), 755 id literals, all literal-key `ndp_cell*` calls, 98 `format!("ndp.…")` patterns (checked as regexes against the registry).
  - Only unregistered exact ids: `ndp.does.not.exist`, `ndp.nobody.unknown`, `ndp.en1990.a2.group.gr9.road_traffic`; deliberate negative controls in tests.
  - Prefix-only literals are table families whose cells exist (`ndp.en1990.psi.<cat>.{0,1,2}`, `ndp.en1993.psi-self.{0,1,2}`, `en1996.{k,alpha,beta,fvk0,fxk,m70.bed-joint}`, `en1999.web-crippling`, `en1994.1-2.ks`, `en1997.gk1.presumptive`, ...). Key sets compared with the key helper functions (mortar/material/group) for en1996, row counts consistent (k 3x4x4 = 48).
  - `ndp.en1993.gamma-m` is a string-prefix probe (`annex_parameter_rows`), allowlisted.
- Not provable statically: `ndp_cell` calls with computed keys beyond the families above (for example `cpe.wall.{zone}` zone/height combinations); a runtime panic there would show in the first test run.
- No missing rows, no values invented, no dead code removed.

## Clauses
48 rows with clause `?` filled (clause and locator only; values untouched, rows stay unverified):
- Rows scraped from spec notes (`l482`, `l642`, `e1-12`, `rw-0x`, ...): clause = their spec section title (the registry `locator`).
- `ndp.en1993.v-255/256/257.1`: EN 1993-1-3 6.1.5 / 6.1.7 / 6.1.8 (the artifact check titles and spec items V-255..257); `v-305.1`: EN 1993-1-4 Annex (spec V-305; the using check is a stainless fire check, so the clause should be re-examined at verification); `v-800.1` EN 1993-1-6 section 8; `v-851.1/.2` EN 1993-3-2; `v-905.1-.3` EN 1993-4-3.
- Script: `🧰️ndp/wave2_clause_fill.py`. Note: many of these rows (`lNNN`, `eN-NN`, `rw-NN`) are spec-note artifacts rather than real NDPs and are unreferenced in code; the owners may delete them (registry cleanup, not done here).

## Regeneration and tests
- `codegen.py` run, then `--check`: ok, 3057 entries (other agents added rows meanwhile).
- Registry Python test extended (`⚖️compliance/🧪️tests/🔖️ndp-registry/🐍️.py`): clause must not be `?`/empty; static scan asserts every direct lookup and literal-key `ndp_cell*` id exists, every other id literal is a registry id or a dotted table prefix; negatives and `STRING_PREFIX_PROBES` allowlisted. Result: ok (322 direct lookups, 758 literals).
- `🔖️ndp-adoption` test fails independently: `private NDP lists remain: [('🔥️beam-fire', 'pub const NDPS')]` (another owner's area, en1994 beam-fire).

## Files
Registry `🔣️.json`, generated `🗃️table/**` and `🗃️table.rs`, test `🐍️.py` (ndp-registry), `🧰️ndp/wave2_clause_fill.py`, `📌️important/requests.md`, `build-en19*/latest.md`, `integrate/latest.md`.
