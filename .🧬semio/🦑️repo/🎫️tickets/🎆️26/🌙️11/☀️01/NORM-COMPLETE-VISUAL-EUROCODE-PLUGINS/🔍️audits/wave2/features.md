# Wave 2 features agent: norm step-adapter layer

## How the repo runs `.feature` files (finding)
There is no free-standing gherkin runner. The repository test platform (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) discovers every `🧪️tests/<emoji><slug>/🥒️.feature`, parses it ONCE with the restricted Gherkin profile (`🧰️framework/🔨️modules/🧪️test/🥒️gherkin/🟦️.ts`) and hands the expanded plan to native hosts. Steps are NOT matched by regex; a handler is registered per scenario id (`@id-<base>` serves every Examples row) in a per-language adapter (`🐍️.py` `def adapter()`, `🦀️.rs` `pub fn adapter() -> Adapter`), in the role `oracle` (Python) or `subject` (Rust, linked through `sut` and the owner's `subjectHostPackages`), and the two projections are compared by the `@comparison-` profile. A feature is valid only with `@capability-`, `@oracle-`/`@no-oracle-`, `@comparison-` and per scenario `@id-`, one `@level-` (fundamental|quick|long|exhaustive), one `@mode-`, and an `id` cell per Examples row. Before wave 2 only the `mutate-*` cases and ~8 app-surface cases satisfied this; the norm catalogue, clause, storey and scene features were free prose read by hand-written Rust unit tests, so `:test` never ran them.

## What was built (shared, domain-neutral, schema-first)
`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps/`
- `🧬️schema/🔣️.json`: the ONE step grammar. Templates are literal text with `{hole}`s (no shared regex dialect). Families: `catalogue-fixture-case`, `catalogue-example`, `clause-vector`, `published-example`, `oracle-check`, `check-outcome`, `scene-law`, `mutation-round-trip`, `oracle-comparison`, each with its projection shape. Scenario base id == family name.
- `🐍️.py`: grammar loader, template matcher, input parser (`a=1;b:text`), tolerance rule, isolated oracle runner (`-I`, JSON and line protocol), projection builder, `Binding` (what an artifact contributes) and `norm_adapter(binding)` (oracle role, all families).
- `🦀️.rs`: the Rust twin (same grammar file via `include_str!`, same hole matcher, same tolerance, same projection keys), the `Binding`/`Paired` types, the repo-resolved interpreter and `run_oracle` (replaces `norm_compliance::python_oracle`, which is `cfg(test)` and unreachable from the generated host), `subject_handler`, `norm_adapter`. The subject runs the Python oracle itself and asserts every oracle-named value within the profile bound, so both projections carry the oracle's key set.
- `🧪️tests/🔬️unit/🐍️.py`: unittest, third-party validators `jsonschema` (grammar against its schema) and `numpy.isclose` (tolerance rule), regex built from each template as a second matcher. Run green: 7 tests.

## Features registered (normalised by hand-script `🧰️scripts/wave2-features-normalise.py`, no aliases)
Tags, `id` column, canonical step wording; prose scenarios without a family moved into the Feature description as "Further specification assertions" (they stay asserted by the Rust unit tests). Parser check: 0 errors for all nine.

| artifact | case | family | capability / oracle / profile | Python oracle adapter (dry-run) | Rust subject adapter |
|---|---|---|---|---|---|
| en1990 | 📇️catalogue-en1990-1 | catalogue-fixture-case (demand/capacity pairs) | en1990-1-evaluate / -compliance-independent / numerical-v1 | 56/56 | written |
| en1995 | 📇️catalogue-en1995-1 | catalogue-fixture-case | en1995-1-evaluate / -compliance-independent / method-spread-v1 (1 % asserted by the subject) | 18/18 | written |
| en1997 | 📇️catalogue-en1997-1 | catalogue-fixture-case | en1997-1-evaluate / same / method-spread-v1 | 107/107 | written |
| en1991 | 🏋️en1991-catalogue | catalogue-example | en1991-1-evaluate (oracle entry ADDED to its oracles manifest) / numerical-v1 | 41/48 | written |
| en1996 | 📇️en1996-catalogue | catalogue-example | en1996-1-evaluate / numerical-v1 | 14/19 | written |
| en1992 | 📇️catalogue (schema/tests) | catalogue-example | en1992-1-evaluate / numerical-v1 | none | none |
| en1993 | 🧮️clauses | clause-vector (12 keys mapped, E12 theta split into 3 rows with 5e-3 relative instead of 3 K absolute) | en1993-1-evaluate / numerical-v1 | 17/17 | none |
| en1993 | 🔭️special-parts | clause-vector + oracle-check | same | 98/98 | none |
| en1994 | 🧮️catalogue-en1994 | published-example | en1994-1-evaluate / numerical-v1 | 29/29 | none |

Dry-run = `🧰️scripts/wave2-features-dryrun.py` (fakes the Python host classes, expands features with the platform's own parser through node, runs every oracle handler against the real oracle scripts). It does NOT prove the host path; the platform `test-contract`/`test-parity` must confirm.

Consumers adjusted for the new tables: en1991 `🔮️catalogue-oracle/🦀️.rs` (7 cells, `name:text` inputs), en1996 `📇️catalogue/🦀️.rs` (`"id"` header skip), en1992 `📇️catalogue/🦀️.rs` (`col("entry")`). Feature input spelling fixed in en1991 (text inputs `k:v`) and fixture URIs `shared://🧫️fixtures/📇️catalogue/…` -> `shared://📇️catalogue/…` (the scheme already resolves under `🧫️fixtures`).
Manifests: `subjectHostPackages` added to the subset oracle manifests of en1990, en1991, en1995, en1996, en1997 (checks/kernel/masonry crate + `semio-s-plugin-norm-compliance`); en1991 got `en1991-1-compliance-independent`.

## Open items (not done, with the reason)
1. Oracle gaps surfaced honestly by the Python adapters (platform rows will fail until the owners extend the oracle, no skipping): en1996 M-06 `e-lt`, M-31 `alpha1` (oracle yields `w-rd`), M-45 `lb`, M-61 `lintel-deflection`, M-81 `n-rd`; en1991 E11.01 `g`, E11.02 `delta_q`, E11.09 `pressure`, E12.03 `q_fd`, E13.02 `s_k` (oracle raises). 
2. en1992: `en1992-1-compliance-independent` is registered but no oracle script exists (`🔮️oracles` holds only the manifest); needs an independent Python formulation for the 20 rows, then the 6-line adapters.
3. en1993 clauses/special-parts and en1994: Rust subject adapters missing (the quantity -> kernel call mapping lives in hand-written Rust tests `clause_oracle_matches_rust_clause_functions` and `🔭️special-parts/🦀️.rs`; port each into `Binding.vector`/`Binding.utilisation`).
4. Not yet normalised to the profile (still free prose, parse errors, no adapters): scenes of en1990-en1999 (`🎬️scene/🧪️tests/🥒️.feature`, currently carry invalid feature-level `@level-unit @mode-edit`), en1993 `🌐️storey`, `⚖️compliance` (design-case, incremental-evaluation, units, check-stations, ndp-registry), `🧮️mechanics`, `🎬️structural-scene/**` and `📇️registry/**/app-surface/**` (about 40 files). The grammar already carries `check-outcome` and `scene-law`; the scene laws are per-scenario prose (entity ids, hulls, colours) and need a typed scene probe per artifact (fixture -> entity kinds, handles, diagrams) before they can be rewritten as Examples tables; do that after the scene crates compile.
5. `scene-law`, `check-outcome`, `mutation-round-trip`, `oracle-comparison` have grammar, Python projection and (check-outcome) Rust dispatch but no artifact binding yet.
6. The en1995 pinned worked examples (E2/E6), document-case, 6.10a/b and domain-geometry scenarios and the en1990/en1997 prose scenarios now live in the Feature description only.
7. Unverified: nothing was compiled. Rust risk points to read first in a failing build: closure inference in `🦀️.rs` `norm_adapter`, `#[path]` depth (8 `../`) in each adapter, `AnnexChoice` and kernel imports against the `subjectHostPackages` crate names.
8. Originals of the nine rewritten features are the staged index versions (`git show :path`); the normaliser script is kept as input.
