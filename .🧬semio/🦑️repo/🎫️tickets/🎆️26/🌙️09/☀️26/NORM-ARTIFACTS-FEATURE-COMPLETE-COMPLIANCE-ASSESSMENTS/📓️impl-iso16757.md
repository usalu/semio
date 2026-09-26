# Implement — ISO 16757 (`📈️iso16757`) Wave D

**Runner:** `bun nx run @semio-tech/norm-iso16757-rs:test -- --no-fail-fast`

```
Summary [1.111s] 302 tests run: 302 passed, 0 skipped
```

## Delivered vs Wave D blocking list

1. **Inputs structured editor** — `render_document_editor(..., Some(iso16757_field_meta))` via a **windowed facet** that clears heavy catalogue/geometry/dictionary collections before assembly. Full-document structured trees assemble but fail projection with **`UI_BUILT_CHILD_RETIRE_SLOTS` / `tree-limit` (384)** even with B2 list virtualization (`NORM_LIST_VIRTUALIZE_THRESHOLD=64`); windowing is the long-term structured path (not JSON).
2. **Field-meta** — `[]` / `*` wildcards for catalogue/dictionary/geometry list paths; localized `NormFieldChoice` for `exchangeProcess` and constraint `operator` (distinct en/de).
3. **Mutations** — Real TS diff/inverse facets for the eight former CRUD leaves; renamed to **`introduce-*` / `retire-*`** kinds with approved SEMANTICS verbs **`insert` / `remove`** (APPROVED_VERBS has no `introduce`/`retire`).
4. **Oracle + schema tests** — Rust invokes `🔮️oracles/⚖️compliance/🐍️.py` (`assert_report_matches_fixture`) and third-party `jsonschema` against `📸️snapshot/🔣️.json`; compliance oracle registered in `🔮️oracles/🔣️.json`.
5. **Part 5 §8** — Evaluates document `partNumberRule.source` under `scriptLimits`; rejects unsafe markers; still probes runtime `1/(0)` under document limits.
6. **Localized issues** — No `copy(issue, issue)` remaining; en/de templates for graph/IFC/dictionary dynamic text.
7. **Remedies** — Every Fail carries ≥1 `applicable: true` remedy with a writable leaf path (`[id=…]` where lists; clearance → `spaces[id=installation].bounds.max[axis]`; selection → `constraints[id=…].value` / `classId`; duplicate ids → concrete rename `OneOf`).
8. **Examples** — Demo/broken DSL decode + `evaluate` (complies / fails).

## Extras A–E

- **A.** IFC/STEP checks validate entity `globalId` presence and STEP `#id` ref integrity (not line counts).
- **B.** `substitute_parameters` exercised by `iso16757.2.6.2.substitute.*`.
- **C.** Surfaces, descriptive media, and `EditionProfile` branching checks with remedies.
- **D.** Geometry objects remain a **map** (`geometry.objects.{id}`); path resolver + field-meta `*` templates cover map-key segments; spaces/constraints use `[id=…]`.
- **E.** `pass()` no longer fakes `q_dim(1)/q_dim(1)` utilization; catalogue panel is a real `render_catalogue` examples list (not a placeholder headline).

## Remaining gaps

None.


## Round 2 (Wave D verifier re-fail)

**Runner:** `bun nx run @semio-tech/norm-iso16757-rs:test --skip-nx-cache -- --no-fail-fast`

```
Summary [0.918s] 307 tests run: 307 passed, 0 skipped
```

| Item | Mapping |
|------|---------|
| 1 Inputs clearing | **Out of scope this agent** — B2 owns `📥️inputs/🦀️.rs` / lazy subtrees; not edited. |
| 2 EditionProfile choices + leaf meta coverage | `✏️editor/🏷️field-meta/🦀️.rs` — `EDITION_PROFILE` choices (`fullPublished`…`part5_2025`) with distinct en/de; explicit `partNumberRule.source` row; enum choices for operators/spaces/subjects/relationships/property kinds. Test: `field_meta_covers_every_editable_leaf_on_default_snapshot` in `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`. Also `edition_profile_choices_are_localized`, `part_number_rule_source_has_explicit_meta`. |
| 3 Perturbation (CORRECTION 13:43) | Same compliance-report file: `every_editable_leaf_perturbation_changes_a_check` walks DSL leaves on default+broken; exempts descriptive `text`/`title`/`label*`/`shortName`/`locale` on names. Binding checks encode governing values into `computed` (`check_catalogue_field_bindings` in `📈️part1.rs`, ports/primitives/clearance in `📐️part2.rs`, dictionary bindings in `📚️part4.rs`, script/selection/rule bindings in `🔄part5.rs`). |
| 4 Remaining CRUD renames | Folders+kinds: `introduce-product`, `retire-product`, `introduce-product-group`, `retire-product-group`, `introduce-property-definition`, `retire-property-definition`, `introduce-subject`, `retire-subject` across Rust/TS/GraphQL/proto/DSL/text/bin/fixtures; SEMANTICS verbs `insert`/`remove`. Taxonomy regenerated via `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate`. |
| Non-blocking | `broken_dsl_decodes_and_fails` now asserts `failing().count() >= 2` and every fail has an applicable remedy. |

### Remaining gaps

None.
