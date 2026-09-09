# Late Rust Mutation Fixture Layout

This late pass repairs mutation support trees added after the main Rust layout census. The canonical case sources still mount the same Rust module names, but their fixture-only source and catalog assets now live under the nearest semantic `🧫️fixtures` owner. Inline executable bodies were extracted through `#[cfg(test)] #[path = "…"] mod tests;`, which keeps private access and module namespaces intact.

## Scope evidence

- Latest input snapshot: `🗑️generated/root-layout/layout-current.json` (29,436 discovered sources, 58 findings at dispatch).
- Rust queue: 28 distinct nested Rust sources in seven fixture families.
- Classification: 9 sources contained 16 `#[test]` bodies; 19 sources were mutation rosters/leaves without executable bodies.
- Asset move: 74 files moved as 12 intact support subtrees, including Rust, descriptor JSON, schemas, and the command-close vector/schema pair.
- Exact changed-file payload: the fenced JSON array below contains 173 unique repository-relative paths: 171 implementation/asset paths plus this durable report and its retained runtime input. The temporary standalone copy is removed with this lane’s generated output.

## Support ownership

| Previous support root | Current fixture root |
|---|---|
| `plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations` | `plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations` |
| `plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations` | `plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations` |
| `plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations` | `plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations` |
| `plugin/🧪️tests/📢️publication-fixtures/{👥️presence,🫧️transient}` | `plugin/🧫️fixtures/📢️publication-fixtures/{👥️presence,🫧️transient}` |
| `plugin/🧪️tests/🖥️test-app-mutations/{🎚️config,🧬️document}` | `plugin/🧫️fixtures/🖥️test-app-mutations/{🎚️config,🧬️document}` |
| `plugin/🧪️tests/🛰️declaration-channels/{1️⃣standard-1,2️⃣standard-2}` | `plugin/🧫️fixtures/🛰️declaration-channels/{1️⃣standard-1,2️⃣standard-2}` |
| `plugin/🧪️tests/🧬️mutation-fixtures/{🎲️dummy,🔀️transaction,🪟️surface}` | `plugin/🧫️fixtures/🧬️mutation-fixtures/{🎲️dummy,🔀️transaction,🪟️surface}` |

## Extracted executable cases

| Canonical implementation | Tests | Preserved identities |
|---|---:|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/➕️dependency-contribution-add-value-unit/🦀️.rs` | 1 | `direct_leaf_contract` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/➕️contributed-mutation-wire-add-value-unit/🦀️.rs` | 1 | `direct_leaf_descriptor_and_inverse_law` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️standard-one-any-set-value-unit/🦀️.rs` | 3 | `actual_leaf_descriptor_and_provenance, assignment_inverse_and_structural_diff, source_json_codecs_and_i32_boundaries` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️standard-two-any-set-value-unit/🦀️.rs` | 3 | `actual_leaf_descriptor_and_provenance, assignment_inverse_and_structural_diff, source_json_codecs_and_i32_boundaries` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎚️test-app-config-selection-unit/🦀️.rs` | 2 | `nullable_selection_serde_text_and_binary_round_trip, structural_config_diff_serde_preserves_identity_clear_and_set` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏷️test-app-document-set-label-unit/🦀️.rs` | 1 | `descriptor_has_set_label_identity` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📝️test-app-document-set-count-unit/🦀️.rs` | 1 | `descriptor_has_set_count_identity` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔒️standard-one-strict-set-value-unit/🦀️.rs` | 3 | `actual_leaf_descriptor_and_provenance, assignment_inverse_and_structural_diff, source_json_codecs_and_i32_boundaries` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-document-mutation-roster-unit/🦀️.rs` | 1 | `direct_leaves_preserve_generic_document_codecs_and_laws` |

## Verification

- Static plugin census after the move: zero Rust sources remain below `plugin/**/🧪️tests/<case>/…`; every plugin Rust test implementation is a direct `🧪️tests/<case>/🦀️.rs` leaf.
- Literal traversal audit: all 53 affected `#[path]`, `include_str!`, and `include_bytes!` edges resolve through the literal filesystem path; zero missing targets.
- Preservation audit: the nine new canonical implementations contain all 16 original `#[test]` attributes; moved `🧫️fixtures` sources contain zero executable test attributes.
- `rustfmt --edition 2021 --config skip_children=true` completed for the nine extracted bodies and their nine fixture callers.
- `git diff HEAD --check -- <172 exact paths>` completed with zero whitespace errors.
- `bun nx run @semio-tech/framework-plugin:test-quick --skip-nx-cache` reached the plugin task after all three generation dependencies passed, then stopped before Cargo in the independent completion-rejection oracle: `writer loses the returned completion owner`. This is a preserved concurrent production-source/tooling integration failure outside the fixture paths; no passing claim is made for that target.
- Focused runtime completed after the executor stopped at its usage limit. The produced plugin test binary contains all new paths in its compiler dependency file. The coordinator normalized literal relative dependency paths and confirmed all 52 extant changed Rust inputs were compiled and no newer than the binary. Public Bun/Nx then ran the 16 fully-qualified laws individually with `--exact --nocapture`; all 16 passed (each reported one passed, zero failed). No second compilation was started. The retained input is `🧑‍💻late-plugin-runtime/📜️script.ts`.

## Exact changed paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-late-rust-mutation-fixtures-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻late-plugin-runtime/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations-add-value-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/➕️add-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/➕️add-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/➕️dependency-contribution-add-value-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/➕️add-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/➕️add-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/➕️contributed-mutation-wire-add-value-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️standard-one-any-set-value-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️standard-two-any-set-value-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎚️test-app-config-selection-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏷️test-app-document-set-label-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📝️test-app-document-set-count-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-presence/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-transient/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔒️standard-one-strict-set-value-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-config/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-unit-command-close/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-document-mutation-roster-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs"
]
```

## Exact move evidence

The following table is generated from the guarded move journal before its temporary copy is removed. Each row is an exact old file and the current file carrying the same content, aside from ownership literal updates and the nine test-module extractions.

<details><summary>74 exact file moves</summary>

| Previous file | Current file |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/➕️add-value/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/➕️add-value/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/➕️add-value/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/➕️add-value/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🧬️schema/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🔣️.json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🧬️schema/🔣️.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🧬️schema/🔣️.json` |

</details>
