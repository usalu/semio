# DOCX Emoji Repair

## Scope

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx`

The initial strict audit reported 636 governed entries and 129 findings: 110 missing emojis, seven emoji-presentation violations, and 12 sibling-duplicate violations.

## Handpicked identities

- Subsets: `base → 🧱️base`, `strict → 📏️strict`, `transitional → 🔄️transitional`.
- Each subset-local oracle contribution: `🧪️oracle → 🔮️oracle`.
- Each editor/viewer window: `🎚️options → ☑️options`, distinct from its `🎚️config` sibling.
- Every mutation fixture role leaf: `before.docx → ⬅️before.docx`, `after.docx → ➡️after.docx`.
- Base fixtures: `💪️bolds-the-tower-run-of-the-opening-paragraph`, `📊️insert-block-appends-a-pricing-table`, `🚧️insert-block-rejected-invalid-index`, `💬️insert-style-adds-a-quote-style`, `👯️insert-style-rejected-duplicate-id`, `⏸️no-mutation-no-op`, `✂️remove-block-drops-the-closing-paragraph`, `🕳️remove-block-rejected-missing-index`, `🧹️remove-part-drops-core-properties`, `🧭️remove-part-rejected-missing-path`, `🗨️remove-style-drops-the-quote-style`, `🪪️remove-style-rejected-missing-id`, `💤️set-block-content-no-op-unchanged-content`, `📝️set-block-content-replaces-the-closing-paragraph`, `🧩️set-part-adds-core-properties`, `🟰️set-part-no-op-identical-content`, `🪄️set-run-formatting-italicizes-the-closing-paragraph`, `🚩️set-run-formatting-no-op-identical-flags`, `🪞️set-run-text-no-op-identical-text`, `✍️set-run-text-rewrites-the-closing-paragraph`, `📸️set-snapshot-no-op-identical-snapshot`, `🔗️set-style-based-on-rejected-missing-id`, `🌳️set-style-based-on-reparents-the-quote-style`, `🏷️set-style-name-rejected-missing-id`, `🔤️set-style-name-renames-the-body-style`.
- Strict fixtures: `🔀️insert-alternate-content-applied`, `🖼️insert-vml-part-applied`, `🧹️remove-alternate-content-applied`, `🗑️remove-conformance-attribute-applied`, `✂️remove-vml-part-applied`, `✅️set-conformance-attribute-applied`, `🌐️set-main-namespace-applied`, `🔗️set-relationship-base-applied`, `📸️set-snapshot-applied`.
- Transitional fixtures: `🗑️remove-conformance-attribute-applied`, `✅️set-conformance-attribute-applied`, `🌐️set-main-namespace-applied`, `🔗️set-relationship-base-applied`, `📸️set-snapshot-applied`.
- Presentation repairs: `✍️set-block-content`, `🏷️set-style-name`, `🖌️insert-style`, `🗑️remove-part`, both `⚙️set-relationship-base` owners, and `🏷️insert-vml-part`.
- The style-inheritance mutation became `🌳️set-style-based-on`, distinguishing it from the sibling GraphQL schema file while directly representing its inheritance-tree semantics.

All selections were made individually from entry semantics. No emoji chooser, rename planner, migration script, or Git-mutating command was used.

## Reference reconciliation

- Reconciled exact DOCX paths in all local schema owner metadata, Rust module paths, oracle manifests, tests, features, UI surfaces, and the generator.
- Reconciled exact incoming DOCX paths in the shared Stdio Rust barrel, oracle library, native-codec registry, runtime registry, plugin policy allowlist, and the XML/XLSX references.
- Added the complete DOCX subset-directory roster and all three local oracle-directory overrides to the central taxonomy.
- Extended the existing DOCX generator with the explicit handpicked logical-recipe-to-physical-directory map and exact `⬅️before.docx` / `➡️after.docx` role names.

## Verification

- Strict ticket audit: 381 files, 258 directories, 636 governed entries; `missing=0`, `generic=0`, `presentation=0`, `spacing=0`, `duplicate=0`, `multiple=0`, `reserved-emoji=0`, `oracle=0`.
- Central taxonomy validation: `[]`.
- JSON parse: every JSON file in the DOCX artifact passed `jq empty`.
- Oracle coordinates and integrity: all 71 recorded fixture paths resolve and every file matches its recorded SHA-256 digest.
- Generator runtime: an applied recipe emitted `📊️insert-block-appends-a-pricing-table/{⬅️before.docx,➡️after.docx}`; a rejected recipe emitted only `🚧️insert-block-rejected-invalid-index/⬅️before.docx`. Temporary output was removed after verification.
- Global stale-reference scans found no old DOCX subset, local-oracle, mutation-owner, fixture-directory, or role-leaf coordinates outside ticket history.
- Stdio `test-quick` now compiles past the formerly stale Semio Drawing coordinate, but is blocked outside DOCX by the concurrently moving PDF path `🌳️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations/🧽️remove-trim-box/🦀️.rs`. No DOCX diagnostic was emitted.
