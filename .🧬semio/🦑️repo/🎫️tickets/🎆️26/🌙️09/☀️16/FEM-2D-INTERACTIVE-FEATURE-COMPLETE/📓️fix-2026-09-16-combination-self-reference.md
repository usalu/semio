# 🪞️ Fix — a combination may not weight itself (`create-`/`replace-combination`)

Date: 2026-09-16 · Crate: `semio-s-artifact-fem-2d` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/`)
Audit source: `📓️w-c-mutations-2026-09-16.md` §3 (the ten byte-identical Python engine copies).

## 1. The gap

`guards::combination_term_references` resolved each `FemCombinationTerm.case_id` against
`base.load_cases` **or** `base.combinations` and accepted anything it found. Under
`replace-combination` the selected combination *is* in `base.combinations` — it is the target — so a
`new_combination` whose term cites `payload.id` resolved against itself and **applied**. The
document then carried a combination that superposes its own superposition.

`create-combination` looked safe only by accident: the fresh id is not in the base yet, so the very
same term fell out as `mutation.target-missing` (Error) — the wrong code, the wrong level, and a
diagnostic that invites the caller to go and create the missing "case".

`guards::combination_referrers` two definitions below already carried the correct exclusion
(`combination.id != target`) for the delete lane; the reference lane did not.

The Python twin (`check_record`'s `"combination"` branch) had the identical gap, in all ten copies.

Proven, not assumed — the committed oracle run against the new vector with only that branch
restored to its pre-fix form:

```
AFTER-FIX: refused mutation.invariant [fatal] ['uls1']
PRE-FIX:   APPLIED — combinations now: [[{'caseId': 'dead', 'factor': 1.35}, {'caseId': 'uls1', 'factor': 1.0}]]
```

## 2. The outcome class, and why

`mutation.invariant` (Fatal), **not** `mutation.target-missing` (Error).

The level discipline is stated at the top of `mod guards`: the three Fatal codes say the PAYLOAD is
inadmissible on any base, the two Error codes say THIS base cannot host it. A self-weighting term is
a cycle inside the payload — no base anywhere can host it, no merge policy may absorb it, and no
later base can make it right. `mutation.target-missing` would promise the opposite. The address is
the combination's own id (one entry), matching `combination_factors`, the other payload-level
combination invariant.

## 3. The change

* `🌐️any/🧬️schema/🧬️mutations/🦀️.rs` — `combination_term_references(base, id, combination)`. The
  identity is now **threaded in** rather than inferred: `replace-` passes `&payload.id` (what it
  selects), `create-` passes `&payload.combination.id` (what it will store), so the guard does not
  depend on `identity_matches` having already run. One `find_map` pass keeps the per-term order, so
  the first offending term wins in both implementations: self-reference → Fatal `invariant`,
  otherwise unresolvable → Error `target-missing`. Docstring records the choice.
* `🏋️load/🧬️schema/🧬️mutations/🔁️replace-combination/🔺️diff/🦀️.rs` and `🔗️create-combination/🔺️diff/🦀️.rs`
  — call sites plus their guard-order module docstrings.
* All **ten** `🐍️.py` engine copies — the `"combination"` branch of `check_record` and the shared
  docstring. `REFUSALS["replace-combination"]` 3 → 4 in the two load-lane copies
  (`🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/`, `🌐️any/🧪️tests/🏋️mutate-fem2d-1-any-load/`), which stay
  byte-identical apart from their own `shared://` prefix.

## 4. New coverage

**Unit** — `🌐️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`:
`replace_combination_self_reference_is_refused`, `create_combination_self_reference_is_refused`.
Both assert `mutation.invariant` / `Fatal` / `[<own id>]`.

**Fixture scenario** — `👻️self-term-0f54d1` (`sha1("refuses-a-uls-term-that-weights-the-combination-itself")[:6]`,
the `<slug>-<sha1(full-kebab-name)[:6]>` convention the siblings use; verified against
`👻️dangling-case-65b8a8` = `sha1("refuses-a-uls-term-weighting-a-case-the-steel-frame-never-had")[:6]`).
Braced steel frame, `replace uls1` with terms `dead 1.35` + `uls1 1.0`; before == after;
`🎯️outcome` declares `mutation.invariant` on `["uls1"]`. Registered on every surface:

| Surface | Entry |
|---|---|
| `🏋️load/🧫️fixtures/🧬️mutations/🔁️replace-combination/👻️self-term-0f54d1/` | quartet + `🔺️diff/🚫️.absent` |
| `🏋️load/🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/👻️self-term-0f54d1/🦀️.rs` | 5 tests |
| `◻️2d/🦀️.rs` | `mod tests_refuses_a_uls_term_that_weights_the_combination_itself` |
| `🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/🥒️.feature` | `@id-reject` row `replace-combination-4` |
| `🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/🦀️.rs` | vector `reject-replace-combination-4` |
| `🏋️load/🔮️oracles/🔣️.json` | scenario `self-term-0f54d1` |
| both load-lane `🐍️.py` | `REFUSALS["replace-combination"] = 4` |

`🌐️any/🔮️oracles/🔣️.json` needs no edit: it declares mutation manifests (kinds, outcome classes,
oracle requirements), never per-scenario rows — `replace-combination` already declares `fatal`.

## 5. Verification (foreground, logs under `🗑️generated/`)

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-fem-2d --tests --message-format short` | `rc=0`, `Finished dev in 2m02s` → `🗑️generated/check-combination-self-reference.txt` |
| `cargo nextest run -p semio-s-artifact-fem-2d --profile fundamental --no-fail-fast … -- combination --skip quick:: --skip long:: --skip exhaustive::` | **96 passed**, 943 skipped → `🗑️generated/nextest-combination-self-reference.txt` |
| `cargo nextest list … \| grep self_reference` | all 7 new tests registered (2 unit + 5 scenario) |
| `python3 🔨️run-fem2d-load-oracle.py combination` | 7 passed, incl. `reject-replace-combination-4` |
| `python3 🔨️run-fem2d-load-oracle.py` | **19 passed, 0 failed** — every `@id-reject` row of the load subset |

`🔨️run-fem2d-load-oracle.py` (new, kept in this ticket folder) drives the committed Python twin
through the repository's own host (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py`),
building the plan by READING the feature's `@id-reject` table — rows, URIs and directory names are
never retyped, so a row added to the feature is picked up automatically.

`--features component-app-assembly` was deliberately NOT passed: `✏️editor/**` is under concurrent
edit by other agents and is out of this fix's scope.
