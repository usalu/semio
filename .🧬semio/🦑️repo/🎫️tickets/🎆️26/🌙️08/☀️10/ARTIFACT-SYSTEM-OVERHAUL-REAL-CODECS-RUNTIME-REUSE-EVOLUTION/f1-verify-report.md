# F1 Wave — Independent Verification Report

Scope: independent, on-disk re-verification of the 6 F1 fan-out agents covering 7 standards —
`xml/1.0`, `zip/2.0`, `json/rfc8259`, `deflate/rfc1950`, `csv/rfc4180`, `txt/utf-8`,
`binary/raw`. Nothing in this report is taken on the agents' word; every claim below was
independently checked against the files on disk and/or a real `cargo test` run.

## Headline finding

**The shared crate (`semio-s-plugin-stdio`) currently does NOT compile its test binary.**
`cargo test -p semio-s-plugin-stdio --lib "artifacts::binary::standards::v_raw"` (and, by
necessity, every other filter in this crate) fails with **9 compile errors**, none of which are
the "unrelated concurrent churn" that blocked earlier attempts by the F1 agents themselves — this
is not the `pdf` `✳️a-2b`→`✳️a` in-flight rename (that blocker was independently confirmed present
at session start via polling, and confirmed resolved by another session ~13 minutes later, at
which point this clean compile attempt was made). All 9 errors are real, own-code defects inside
this wave's own artifacts:

| # | Artifact (owner) | File | Defect |
|---|---|---|---|
| 1–3 | `binary/raw` | `⚙️engine/🦀️component.rs:124,126,137` (`field_sweep_covers_every_byte_level_change`) | `.apply()` called without `MutationDiff` in scope — only `use protocol::os_spr::command::DiffAlgebra;` is imported locally; `.apply()` lives on `MutationDiff`, not `DiffAlgebra` |
| 4–5 | `txt/utf-8` | `⚙️engine/🦀️component.rs:149,151` (`field_sweep_covers_every_mutable_field`) | identical bug, same pattern, same missing import |
| 6 | `xml/1.0` | `⚙️engine/🦀️component.rs:350` (`between_roundtrip_law`) | `MutationDiff::apply(&DiffAlgebra::between(&a, &b), &a)` — bare trait-qualified call, type inference cannot pick which impl of `DiffAlgebra` to use (needs `<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(...)`, the fully-qualified form correctly used two tests later in `field_sweep_law` in the same file) |
| 7 | `json/rfc8259` (fallout in `gltf`, same crate) | `🗿️artifacts/🧊️gltf/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:10` | `JsonSnapshot { value }` expects `JsonValue`, gets `serde_json::Value` — a downstream consumer of json's own redesigned `JsonSnapshot.value` field type that was not updated |
| 8–9 | `txt/utf-8` | `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:315-322` (`insert_then_remove_before_matches_canonical_shape`) and `:358-364` (`modify_then_remove_drops_the_modify`) | `let ld = merged.lines.expect(...)` moves `Option<TxtLinesDiff>` out of `merged`, then `merged.apply(&base)` on the next line borrows the now-partially-moved `merged` — E0382. A sibling test in the same file (`add_then_set_field_patches_into_added`) correctly used `merged.lines.clone().expect(...)`, so this is an inconsistent oversight, not a structural problem. |

**Practical consequence: real, on-disk `cargo test` pass/fail counts could not be obtained for
any of the 7 artifacts this session**, because they all share one compiled test binary and a
single compile failure anywhere blocks every filter. `tests_passed`/`tests_failed` below are
`0`/`0` for all 7 — this reflects "not measurable due to a real, current compile failure", not "0
tests ran cleanly".

### Why the self-reports missed this

- **binary/txt** (`f1-txt-binary-report.md`): claims `cargo check -p semio-s-plugin-stdio`
  confirms "every line of txt/binary's own code is well-typed" — but bare `cargo check` (without
  `--tests`) does not type-check `#[cfg(test)] mod tests` bodies at all. The agent's own report
  never shows a `--tests` flag for this claim, which is consistent with it never having
  typechecked the very `field_sweep` test it also reports as unable to run (blocked, per the
  agent, entirely by the concurrent `pdf` rename — true at the time, but the import bug was
  already latent underneath it).
- **xml** (`f1-xml-report.md`): claims `cargo check -p semio-s-plugin-stdio --lib --tests`
  compiled the xml module clean, "zero errors" — this contradicts the reproducible error found
  here. Possible explanations: the check was run before the file reached its current state, or the
  claim is simply inaccurate; either way, the current on-disk file does not compile.
- **json** (`f1-json-report.md`): this one **is accurate and transparent** — it explicitly
  predicted and documented the exact `gltf`→`json` bridge error ("`expected JsonValue, found
  Value`") as "the ONLY compile fallout from my change within the crate itself," correctly scoped
  it as cross-artifact fallout outside its ownership boundary, and recommended a follow-up. Fully
  corroborated.
- **csv, zip, deflate**: own files contribute **zero** of the 9 errors (none of the 9 errors are
  in a `csv`/`zip`/`deflate` file). Their own-code compile cleanliness is corroborated by this
  independent run, though real pass/fail counts remain unobtainable while the other 4 defects
  block the shared binary.

No fixes were applied by this verification pass — the exact defects and file:line locations above
are handed off as-is for the wave's closer, per this task's read-only verification scope.

## Static / structural checks (all independently re-verified on disk)

For every artifact: `impl DiffAlgebra<...Snapshot> for ...Diff` present, no `snapshot: Option<...>`
full-replace field anywhere in a diff struct definition (only explanatory doc-comments referencing
the banned pattern by name, e.g. deflate's `//! No snapshot: Option<DeflateSnapshot>...`), and a
test function whose name contains `field_sweep` present (search widened beyond the diff file itself
— `xml`, `csv`, `binary`, `txt` keep their field_sweep test in `⚙️engine/component.rs` rather than
the diff file, which is a valid "existing test region" per the recipe).

`json`'s `JsonSnapshot`/`JsonValue` model was spot-checked and contains **no** `serde_json::Value`
anywhere (`Number` keeps the raw lexeme, `Object` is an order-preserving `Vec<JsonMember>`).

All 6 laws (`mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`,
`codec_retention_law`, `field_sweep`) are present, under varying test-fn names, for all 7
artifacts — confirmed by grep across each artifact's full standard directory, not just the diff
file.

CSV-specific check (per the wave-verify brief's explicit instruction #5): the csv report's claim
that the earlier "mid-edit, −77/+15" observation on the snapshot file had already settled before
it started, and that it did **not** defer any work, was independently confirmed: `git status` on
`📊️csv/🏅️standards/🔖️rfc4180/` today shows the snapshot/diff/mutations component.rs files as
**not modified** in the working tree (i.e. already committed by the repo's auto-commit system,
consistent with completed work) — no "deferred due to live edit" language appears in the csv
report at all, and none was needed.

## Manual code-quality review (since real `cargo test` counts were unavailable)

Full, close reading of `deflate/rfc1950`'s and `zip/2.0`'s diff files (the two extremes — deflate
is the simplest, all-scalar/LWW case at 338 lines; zip is the most structurally complex, a
name-keyed collection with rename-tracking absorb, at 393 lines) and spot-checks of `binary/raw`'s
mutations file and `csv/rfc4180`'s absorb tests. All four read as genuinely handcrafted,
well-reasoned, and carefully documented:

- `deflate`'s `field_sweep_between_covers_every_field` exercises the tri-state `dict_id` in both
  directions (`Some(Some(id))` and `Some(None)`), and `codec_retention_law` is split into a
  self-round-trip byte-exact case and a real-third-party-fixture case with an explicitly documented
  normal form (fixed- vs dynamic-Huffman re-encoding) rather than a silently-loosened assertion.
- `zip`'s absorb implementation handles rename-tracking through a base-name/mid-name φ map, entry
  annihilation (insert-then-remove-of-the-same-added-item), and an honestly documented
  best-effort limitation for `added`-index bookkeeping when an unrelated base survivor removed by
  the second diff sits *after* the pending add (an inherent information limit of name-keyed
  collections' `removed` list carrying no position, not an oversight) — flagged inline and in the
  agent's own `deviations`.
- `csv`'s `absorb_law` test explicitly covers the recipe-mandated `Insert+Remove-before` and
  `Insert+Insert-same-index` (both survive) canonical cases by name in code comments.
- `binary`'s mutation dispatch (`apply_binary_mutation`, `BinaryMutation::diff`/`inverse`) is
  fully handcrafted per-variant, not apply-and-capture, matching the recipe.

This gives reasonable confidence that once the 9 compile errors above are fixed, the underlying
diff/absorb/inverse algorithms themselves are very likely to pass — but this is inference from
code reading plus each agent's own scratch-crate corroboration (self-reported, not independently
re-run by this pass), **not** a substitute for the real, currently-unobtainable `cargo test` count.

## Per-artifact table

| artifact | tests_passed | tests_failed | diff_algebra_present | full_replace_slot_gone | field_sweep_present | notes |
|---|---|---|---|---|---|---|
| xml (1.0) | 0 | 0 | true | true | true | Own compile error: `between_roundtrip_law` ambiguous `DiffAlgebra::between` call, `⚙️engine/🦀️component.rs:350`. Self-report's "compiles clean" claim not reproducible on current disk state. |
| zip (2.0) | 0 | 0 | true | true | true | Own files contribute 0 of the 9 crate-wide errors. High-quality, thoroughly documented absorb/rename handling on manual review. Real test count blocked only by sibling defects (xml/txt/binary/json-fallout). |
| json (rfc8259) | 0 | 0 | true | true | true | Own files contribute 0 errors; confirmed no `serde_json::Value` in `JsonSnapshot`/`JsonValue`. Downstream `gltf`→`json` export bridge breaks on `JsonSnapshot.value` type change (`JsonValue` vs `serde_json::Value`) — accurately self-reported by the json agent as known, out-of-scope fallout. |
| deflate (rfc1950) | 0 | 0 | true | true | true | Own files contribute 0 errors. Manual review: clean, well-documented, tri-state field handled correctly in field_sweep. |
| csv (rfc4180) | 0 | 0 | true | true | true | Own files contribute 0 errors. csv's "not deferred, clean start" claim independently confirmed via `git status` (files already committed, not mid-edit). |
| txt (utf-8) | 0 | 0 | true | true | true | Own compile errors: 2x missing `MutationDiff` import in `⚙️engine/🦀️component.rs:149,151`; 2x E0382 partial-move of `merged.lines` in `🔺️diff/🦀️component.rs:315-322,358-364`. |
| binary (raw) | 0 | 0 | true | true | true | Own compile errors: 3x missing `MutationDiff` import in `⚙️engine/🦀️component.rs:124,126,137`, identical root cause to txt's engine-file bug (same F1 agent covered both). |

## Recommendation for the wave's closer

Four minimal, well-understood fixes unblock the whole crate's test binary:
1. `binary/raw` and `txt/utf-8` `⚙️engine/🦀️component.rs`: add `use protocol::MutationDiff;`
   alongside the existing `use protocol::os_spr::command::DiffAlgebra;` in each `field_sweep`
   test.
2. `xml/1.0` `⚙️engine/🦀️component.rs:350-351`: replace the two bare `DiffAlgebra::between(&a,
   &b)` calls in `between_roundtrip_law` with the fully-qualified `<XmlDiff as
   DiffAlgebra<XmlSnapshot>>::between(&a, &b)` form already used correctly in `field_sweep_law`.
3. `txt/utf-8` `🔺️diff/🦀️component.rs:315,358`: change `merged.lines.expect(...)` to
   `merged.lines.clone().expect(...)` (matching the pattern already used correctly at line ~340 in
   the same file).
4. `gltf`'s json export bridge (or, per the json agent's own recommendation, a
   `JsonValue::to_serde_json()`/`from_serde_json()` conversion pair) — out of this wave's 7
   artifacts' ownership, flagged for the ~120-file follow-up the json agent already scoped.

Once applied, re-run `cargo test -p semio-s-plugin-stdio --lib "artifacts::{binary,txt,xml,zip,csv,deflate,json}::"` for the first genuine, on-disk pass/fail count.
