# SVG Handpicked Emoji Repair

## Scope

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg`

The initial strict ticket audit counted 411 files, 254 directories, 658 governed entries, 87 missing emojis, 7 presentation mismatches, and 26 sibling duplicates.

## Handpicked identities

- Subsets: `✳️base → 🧱️base`, `✳️basic → 🔰️basic`, `✳️tiny → 🔬️tiny`.
- Test contributions: each local `🧪️oracle → 🔮️oracle`; each local `☑️options` keeps the option/checklist meaning.
- Shared source fixtures: `qr-code.svg → 🔳️qr-code.svg`, `mouse.svg → 🐁️mouse.svg`.
- Every mutation fixture pair uses the role identities `⬅️before.svg` and `➡️after.svg`.
- Base fixture cases: `➕️insert-element-applied`, `➖️remove-element-applied`, `🏷️set-attribute-applied`, `📣️set-declaration-applied`, `📜️set-doctype-applied`, `🔤️set-element-name-applied`, `✍️set-text-applied`, `🔄️set-transform-applied`, and `🖼️set-view-box-applied`.
- Basic fixture cases: `➕️insert-basic-element-applied`, `✂️insert-clip-path-shape-applied`, `➖️remove-element-applied`, `🏷️set-basic-attribute-applied`, `🔗️set-clip-path-reference-applied`, `📸️set-snapshot-applied`, `✍️set-text-applied`, `🔄️set-transform-applied`, `🖼️set-view-box-applied`, and `🪪️stamp-base-profile-applied`.
- Tiny fixture cases: `➕️insert-tiny-element-applied`, `➖️remove-element-applied`, `📸️set-snapshot-applied`, `✍️set-text-applied`, `🏷️set-tiny-attribute-applied`, `🔄️set-transform-applied`, `🖼️set-view-box-applied`, `🪪️stamp-base-profile-applied`, and `🧹️strip-non-tiny-applied`.
- Base mutation owners: `🏷️set-attribute`, `📣️set-declaration`, `📜️set-doctype`, `🔤️set-element-name`, `✍️set-text`, `🔄️set-transform`, and `🖼️set-view-box`.
- All nine base schema sidecars use `🧬️.schema.json`; Basic and Tiny presentation names were specialized by their mutation semantics.

All names were chosen individually from the asset, role, mutation, or subset meaning. Reserved basenames remain literal.

## Reference reconciliation

The three oracle manifests, generator, quick-xml codec, Rust package barrel, Stdio oracle library, native codec registry, policy allowlist, dependency lock, XML fixture include, and the incoming Semio Drawing documentation now use the exact physical SVG coordinates. The generator keeps logical mutation IDs distinct from physical fixture directory names and emits exact handpicked role filenames. External source asset paths and runtime scratch-copy names intentionally remain unchanged.

Central taxonomy additions are limited to the SVG subset roster and the three exact local oracle overrides.

## Verification

- Final strict audit: 411 files, 254 directories, 658 governed entries; all eight finding categories are zero.
- Taxonomy validation: `[]`.
- JSON parsing: all 52 SVG JSON files parse.
- Fixture integrity: 28 manifests and 56 file records resolve; every byte count and SHA-256 digest matches.
- Isolated generator source check: `set-doctype-applied` emitted exactly `📜️set-doctype-applied/⬅️before.svg` and `📜️set-doctype-applied/➡️after.svg`, with manifest paths and digests matching the committed files.
- `cargo test` for `quick-xml-svg-codec`: 3 passed, 0 failed. One upstream deprecation warning for `quick_xml::Attribute::unescape_value` remains unrelated to paths.
- Live stale-coordinate searches for the former SVG subset, oracle, schema-sidecar, mutation, shared URI, and fixture coordinates returned no findings after the incoming Semio reference was reconciled.
- Nx Stdio quick integration: the run reached `cargo-nextest nextest list` but made no progress for more than seven minutes behind a pre-existing Stdio Cargo lock; the run and its orphaned child were terminated cleanly. No integration result is claimed. The independent codec tests and exact reference/fixture checks above are complete.
