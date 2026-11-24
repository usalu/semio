# Norm mutation vectors, 2026-11-23

Status: in progress. Sections marked TODO are not yet verified.

## How a produced en1999 diff differs from its committed one

Compared as parsed JSON (the law table compares parsed values), 220 committed `diff.json` files against the diff production dispatch raises for the same `before` + `mutation`:

| Difference | Bundles | Committed | Produced |
|---|---|---|---|
| `reliabilityClass` | 202 | `"RC2"` | `null` |
| `forceMethod` in connection patches | 32 | key absent | `null` |
| empty row deltas (`transverseStiffeners`, `connections`, `sheetProfiles`, `ringStiffeners`, `outlineVertices`) | 32 | `{"removed":[],"inserted":[],"moved":[],"modified":[]}` | `null` |
| redundant patch field | 1 | `patch.productForm = "extrudedProfile"` | `null` |
| `reliabilityClass` of `change-reliability-class` | 1 | `"RC2"` | `"RC3"` |

Concrete example, `change-sheet-action-category/apply` (`before` and `after` both have `"reliabilityClass": "RC2"`, only a sheet action category moves):

- committed `diff.json` line 60: `"reliabilityClass": "RC2"`
- produced: `"reliabilityClass": null`

Concrete example 2, `change-reliability-class/apply` (`before` RC2, `after` RC3):

- committed diff: `"reliabilityClass": "RC2"`. Applying it leaves RC2, so the committed diff does not reach the committed `after` (RC3). It is wrong on its own terms.
- produced: `"RC3"`.

## Why the produced form is correct (and what I could not show)

I could NOT find a framework change that altered the producer. What I verified:

- `En1999Diff.reliability_class` is `Option<ReliabilityClass>` since commit 3315848ba6d (2026-11-03, rc-threading); the diff Rust file is unchanged for these fields since. `None` means unchanged and serialises to `null`; the diff schema (`🔺️diff/🔣️.json`) has no `required`, `null` is valid.
- The law `the produced diff is not the committed diff` exists since 793f63533f2 (2026-10-15) and `🧪️tests/🧪️mutation-vectors/🦀️.rs` is unchanged since 2026-11-07. The writer `to_json_string` is `to_string(from_dsl_value(value.to_value()))`.
- The committed `RC2` has a traceable authoring origin that is not the producer: the ticket tool `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/rc-threading` (`rc_leaf.py::flat_fixtures`, `rc_run.py::fixtures`, 2026-11-01..03) threaded the new `reliabilityClass` field through all committed vectors by text edits and cloning of the `change-annex` bundles. It wrote `RC2` into diffs where the producer yields `None`.
- Project intent for a scalar diff: `norm-fixture-authoring/emit-scalar.py` (2026-08) documents "one snapshot field moves, one diff field is `Some`, every sibling stays", and `En1999Diff` is documented as a "keyed sparse diff". A diff that sets `reliabilityClass = RC2` in every bundle would reset the class of any document whose class is RC3, so the committed form is not a valid sparse delta.
- `forceMethod` (and other later patch fields) are absent in the committed diffs because the fields were added after the vectors were authored; empty row deltas are `Some(empty)` that `is_empty()` treats as no change.

So this is stale committed data, not a producer regression and not a new canonical form. This differs from the original task hypothesis (a shared diff or JSON-writer change); I did not find one.

## Actions taken

- `✏️s/🧪️tests/🧪️mutation-vectors/🦀️.rs`: added `record_produced` (env `SEMIO_MUTATION_VECTORS_PRODUCED_DIR`) which writes each mismatching produced diff as `<leaf>__<case>.json`. This is the regeneration input.
- `regenerate-mutation-vector-diffs.py` (this ticket folder): writes those files over the committed `diff.json` with the committed format (`indent=2`, no ASCII escaping, trailing newline; all 220 committed files round-trip in this format).
- en1999: 220 `diff.json` regenerated under `🪶️en1999/…/🧪️tests/🧫️fixtures/🧬️mutations/*/*/🔣️.diff.json`. The only source for the regeneration is production dispatch; the other laws (applied == committed after, inverse restores before, messages, wire, binary codec) stay independent of the regenerated files.

## Other defects found on the way (not vector content)

- `compliance` `test-support` feature did not enable `design-case/compliance-testing`, so `CaseRow` lacked serde derives and en1990 `mutation_vectors` did not compile. Fixed in `⚖️compliance/📦️packages/🦀️rust/Cargo.toml`.
- en1990 `mutation_vectors` pulled in `🧮️physical/🦀️.rs` (a replay-preparation probe written against an older framework API, 20 compile errors). Moved to its own `[[test]] replay_preparation_physical` in the en1990 mutations `Cargo.toml` and removed from the vectors file; that target is unverified and still fails to compile.
- en1990: three committed `before.json` have duplicate permanent key `G` (`change-reliability-member-id`, `change-test-pairs`, `change-test-theory-variation`); the applier rejects them (`mutation.apply.duplicate-target`).

## TODO

- Other families, TS oracle and wire-twin tests, final test lines per family.
