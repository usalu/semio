# XML Handpicked Emoji Repair

## Scope

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml`

The initial strict ticket audit counted 293 files, 180 directories, 466 governed entries, 42 missing emojis, 3 presentation mismatches, and 17 sibling duplicates.

## Handpicked identities

- Subsets: `✳️base → 🧱️base`, `✳️valid → ✅️valid`.
- Local test contributions: `🧪️oracle → 🔮️oracle`; each colliding `🎚️options → ☑️options` while `🎚️config` remains unchanged.
- Every mutation fixture pair uses `⬅️before.xml` and `➡️after.xml`.
- Base fixture cases: `➕️insert-element-applied`, `➖️remove-element-applied`, `🪪️set-attribute-applied`, `📣️set-declaration-applied`, `📜️set-doctype-applied`, and `✍️set-text-applied`.
- Valid fixture cases: `📜️declare-doctype-applied`, `🏷️declare-entity-applied`, `🌳️rename-document-element-applied`, `🔗️set-external-subset-applied`, `📚️set-internal-subset-applied`, `📸️set-snapshot-applied`, `🏳️set-standalone-applied`, and `✍️set-text-applied`.
- Valid source fixtures: `🧪️macos-uttype-plist → 🍎️macos-uttype-plist`, `🧪️reuse-marketplaces-plist → ♻️reuse-marketplaces-plist`.
- Base mutation owners: `🏷️set-attribute`, `📣️set-declaration`, `📜️set-doctype`, and `✍️set-text`.
- Base schema sidecars: all six `🔣️.schema.json → 🧬️.schema.json`.
- Valid mutation presentation and semantics: `✍set-text → ✍️set-text`, `🏳set-standalone → 🏳️set-standalone`, `🏷declare-entity → 🏷️declare-entity`, and `🟤️set-snapshot → 📸️set-snapshot`.

`🪪️set-attribute-applied` was selected instead of the schema-owner `🏷️` identity because the base fixture root already contains the sibling `🏷️.xml`; this preserves both semantic meaning and sibling uniqueness. Reserved basenames remain literal.

## Reference reconciliation

The two oracle manifests, generator, quick-xml codec, source tests/features, schema owner descriptors, schema sidecar references, shared fixture URIs, dependency lock, policy allowlist, Stdio oracle library, native codec registry, PPTX cross-artifact grammar reference, and all documentation coordinates now use the exact physical XML paths.

The generator preserves the six logical recipe IDs while explicitly mapping each to its handpicked physical directory and role filenames. It does not derive an emoji. Central taxonomy additions are limited to the XML subset roster and the two exact local oracle overrides.

The generated shared Stdio Rust barrel was not edited or regenerated. Two read-only snapshots found 54 XML `#[path]` attributes, all resolving, with the identical XML-line digest `8b25f6af5d501afa1e729363477b8c8827ac0c9d5040219812f8a1d25ded2ece`.

## Verification

- Final strict audit: 293 files, 180 directories, 466 governed entries; all eight finding categories are zero.
- Taxonomy validation: `[]`.
- JSON parsing: all 35 XML JSON files parse; the native codec factory registry also parses.
- Fixture integrity: 14 manifests and 28 file records resolve; every byte count and SHA-256 digest matches.
- Isolated generator source check: logical `set-attribute-applied` emitted exactly `🪪️set-attribute-applied/⬅️before.xml` and `🪪️set-attribute-applied/➡️after.xml`; both generated hashes match the committed fixtures.
- `cargo test` for `quick-xml-oracle-codec`: 4 passed, 0 failed.
- Live stale-coordinate search across dependencies, plugins, framework, and Hub: no old XML subset, oracle, mutation, shared fixture URI, or fixture paths remain.
- No repository-wide Stdio integration result is claimed because the earlier independent SVG integration attempt was blocked behind a pre-existing Stdio Cargo lock, and regenerating the shared barrel was explicitly prohibited during this pass.
