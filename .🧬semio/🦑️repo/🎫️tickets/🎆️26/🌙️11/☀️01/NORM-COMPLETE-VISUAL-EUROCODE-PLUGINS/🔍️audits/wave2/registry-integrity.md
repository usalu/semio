# Registry integrity (wave 2)

Registry: `✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`. Tool: `🧰️ndp/registry_integrity.py` (one-shot, idempotent), then `🧰️ndp/codegen.py` (`--check` passes, 3754 entries).

## 1. 46 verified EN 1995 rows with contradicting notes
- Contradiction: `asWritten` carried the earlier spec value, `recommended` another. 36 EN 338 rows (hardwood Table 3 and softwood Table 1) were verified by the blue-prints library, 10 EN 14080 glulam rows (gl24h/gl28h/gl32h) by non-authoritative public sources.
- The printed tables were not available and the library values disagree with the spec values in a pattern that suggests library faults (e.g. D24 f_c,90,k 4.9 vs 7.8, D70 rho_k 800 vs 900, C16 f_t,0,k 8.5 vs 10). No note states the source value unambiguously, so no value was changed: all 46 rows are now `unverified`, with `asWritten` stating both values and an `evidence` prefix "registry-integrity 2026-11-03: downgraded". Values stay as they were, so no fixture, test or oracle pin changes; verdicts of checks reading these rows turn from Pass to UnverifiedBasis (3754 rows: 3542 unverified now, was 3496).
- Human action: check the 46 ids against the printed EN 338:2016 Tables 1/3 and EN 14080:2013 Table 5 (list: rows whose `evidence` starts with "registry-integrity").

## 2. 36 EN 1998 rows citing the EN 1997-1 NA
- All 36 are `ndp.en1997.*` rows (xi factors Tables A.9-A.11, SLS limits Annex H.2) wrongly coded "EN 1998". The document `DIN EN 1997-1/NA:2023-11` is correct for them, the code was wrong: `code` set to "EN 1997" (source untouched). Their status stays unverified.

## 3. Law test
- `⚖️compliance/🧪️tests/🔖️ndp-registry-law/{🐍️.py,🥒️.feature}`: fails when a verified row has a note (specRef, asWritten, evidence) with "contradicts", "differs", "disagree", "should be", "TODO", "FIXME" or "?", or an asWritten value that differs from `recommended`; fails when the id prefix differs from `code` or the source document does not name the row's EN number (allowed product standards: EN 338, EN 14080, EN 14374, EN 16351 for EN 1995; DIN 1054 for EN 1997). Has built-in negative controls. Passes on the current registry.

## Runs
- codegen `--check`: ok. `🔖️ndp-registry/🐍️.py` and `--check`: ok (fixture `🔖️ndp-annex-differences` regenerated with `ndp_annex_differences.py` for 2 unrelated en1996 rows that had drifted). Law test: ok.
- Oracle dry runs (run directly with nice 19, not through heavy.sh, because the lease waited on low swap behind build-en1996; the dry run is a light Python+node job): see end of this section for results.
- Rust test pins fixed: `en1995/.../🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` asserted `verified` for C16, C30, GL24H, GL28H, GL32H, C20, D30; now asserts C24 verified and the others unverified. Check-outcome fixtures pin utilisations only and are unaffected (values unchanged).
- "- ready for rebuild: registry integrity" appended to build-en1995, build-en1997, build-en1998 `latest.md`.
- Dry-run results: check-outcome en1995 80 ok / 0 failed, en1998 98 ok / 0 failed; oracle-comparison en1995 4 ok, en1998 4 ok.
