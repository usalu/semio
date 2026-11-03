# NDP Registry and Verdict Gate (compliance kernel)

## Design
- Schema: `⚖️compliance/🧬️schema/🔣️.json` (+ `.ts`, `.proto`, `.graphql` twins) gained `NdpRegistry`, `NdpEntry`, `NdpSource`, `NdpValue`, `NdpStatus`, `CheckResult.ndps`, `CheckStatus.UnverifiedBasis`, and `unverified` counters on `PartVerdict` / `CheckReportSummary`.
- Data (language-agnostic source of truth): `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`. Rust: `🔖️ndp/🦀️.rs` (types, `ndp(id)`, `ndp_is_verified`, `unverified_ndp_entries`) over the generated `🔖️ndp/🗃️table.rs` (`static NDP_TABLE: &[NdpEntry]`, id-sorted, binary-search lookup, no allocation). Generator: `🧰️ndp/codegen.py` (ticket). Entry: id, code, part, clause, symbol, recommended, naDe, unit, source{document, edition, locator}, status `verified|unverified`, specRef, asWritten.
- Gate: `CheckBuilder::ndp(id)` records ids; `build()` turns `Pass`/`Warning` into `UnverifiedBasis` when any cited id is unverified or unknown (explanation EN+DE lists ids; utilisation kept). `Fail` stays `Fail`. `CheckReport::complies()` is false while any check is `UnverifiedBasis`; `unverified()` and `unverified_ndps()` expose the worklist to the UI. `app_surface::status_label` renders "Unverified basis / Ungeprüfte Grundlage"; scene kits map it to `scene::CheckStatus::Warning`.
- Registry content: 520 entries extracted mechanically (`🧰️ndp/extract.py`) from the six target specs, all `unverified`; typed numbers only where the spec wrote `EC x / DE y`. Verifying = edit JSON (status, edition, locator, values), regenerate table.

## Task 2 (CheckBuilder::minimum / worst_utilization)
Both live in the shared kernel and were wrong. Fixed: `minimum` failures now report `minimum/computed` (> 1; capped by `UTILIZATION_CAP = 1e6`, NaN/zero computed fail with the cap); `utilization()` against a zero limit failed open (utilisation 0, Pass) and now fails with the cap unless demand is also 0; explicit `status(Fail)` is reported with utilisation >= 1; report/part `worst_utilization` uses `max(util, 1)` for failing checks.

## Tests
- Rust unit (`⚖️compliance/🧪️tests/🔬️unit/🦀️.rs`): minimum failures, zero limit, worst utilisation, sorted/unique/static lookup, JSON == static table, unverified blocks verdict, Fail stays Fail, fixture round trip with an `UnverifiedBasis` row.
- Language-agnostic: `⚖️compliance/🧪️tests/🔖️ndp-registry/🥒️.feature` with independent Python oracle `🐍️.py` (reads the same JSON, same verdict rule; run directly: passes). Ajv (third party) validates the fixture and the registry against the schema in `🧪️tests/🔬️check-report-fixture/🟦️.ts`.

## Deliverables
- Worklist: `📌️important/ndp-verification-worklist.md` (51 conflict rows first, then all entries by code).
- Adoption recipe: `📌️important/requests.md` ("NDP registry adoption").

## Limits
- Extraction is heuristic: symbol/clause may be `?`; granularity is spec line segment. EN 1991 and EN 1997 audits list further unverified values (`en1991-completion.md`, `en1997-completion.md`) that may not be in specs; those agents register them per the recipe.
- The `.feature` is not yet bound to the cucumber platform harness (no kernel project target owns features); the Python oracle runs standalone.
- Per-entry status covers both recommended and NA-DE values (a single flag).

## Update (second round)
- Registry: 583 entries (coarse `t-01.1` / `m-01.1` split into 30 + 5 atomic entries; 30 EN 1993 special-part ids and all other ids cited in artifact sources added; `evidence` field added, public-source context only; editions recorded for EN 1997 (DIN 1054:2021-04), EN 1998 (NA 2023-11) and EN 1999 (NA 2013-05/2017-05 withdrawn)). All remain `unverified`. Baseline extraction output and conflict map are kept in `🧰️ndp/ndp.json` and `🧰️ndp/conf.json`; pipeline: `extract.py` -> `curate.py` -> `codegen.py` -> `worklist.py`.
- Kernel: `CheckStatus::Info` + `CheckBuilder::informative`, `ratio` guard for non-positive capacity (EN/DE message), `info` counters.
- Build status: NOT verified. `repo run @semio-tech/norm-artifact-contract-rs:check` first died of the 10 min no-progress watchdog, the second run reached cargo and was refused by source admission in a crate that is not mine: `semio-framework-plugin-retained-command (Unbalanced Rust source delimiter)`. Kernel unit tests, Ajv test and artifact compilation therefore have not run; the Python oracle passes. Rebuild after that crate is fixed.
