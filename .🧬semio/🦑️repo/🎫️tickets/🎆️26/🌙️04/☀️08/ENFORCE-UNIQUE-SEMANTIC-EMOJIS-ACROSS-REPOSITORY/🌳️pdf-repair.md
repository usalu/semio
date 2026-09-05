# PDF Emoji Repair

## Scope

The canonical PDF artifact tree is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌳️pdf`. Every physical rename was selected explicitly from the represented PDF concept; no emoji-selection or rename generator was used.

## Handpicked coordinates

- Standards: `4️⃣1.4` and `7️⃣1.7` distinguish the two supported PDF revisions.
- Subsets: `🗄️a` for archival PDF/A, `🧱️base` for the base document model, `📐️e` for engineering PDF/E, `⚕️h` for healthcare PDF/H, `♿️ua` for accessibility PDF/UA, `🧾️vt` for variable-data printing PDF/VT, and `🖨️x` for print-production PDF/X.
- Local test authority: `🔮️oracle`; editor and viewer options: `☑️options`.
- Generator sources: `🏗️generate.rs` and `📖️reader.rs`; the second Base generator is `⚖️lopdf-engine` so it does not collide with `🦀️engine`.
- Fixture carriers: `⬅️before.pdf` and `➡️after.pdf`; conformance seed: `🧬️conformance-seed.pdf`; thesis asset: `🎓️bachelor-thesis.pdf`; demo asset: `🎬️demo.pdf`; two-page asset: `2️⃣two-pages.pdf`; report-strip fixture and asset: `📊️report-strip` and `📊️report-strip.pdf`.
- Mutation directories and their descriptor sidecars share one semantic operation emoji. Notable disambiguations are `📰️set-info-title` and `✅️set-mark-info` in PDF/UA, `✂️remove-trim-box` versus `🧽️remove-output-intent` in PDF/VT and PDF/X, and `♻️replace-page-text` in PDF 1.4 Base.
- Mutation payload schemas use `🧬️.schema.json`, distinct from the descriptor `🔣️.json` at the same level.

## Reference closure

All PDF-local owner IDs, directory-name fields, generator commands, reader executable paths, Cargo source declarations, documentation paths, shared Stdio Rust mounts, policy entries, and dependency-lock coordinates were updated to the final physical names. A no-ignore scan reports no old `📄️pdf`, `🔖️1.4`, `🔖️1.7`, or old generic subset coordinates in the PDF source tree.

## Verification

- The full repository statute audit snapshot contains no finding under the canonical PDF root.
- All ten fixture generator `manifests` commands returned valid JSON. Together they describe 97 generated fixtures, and every declared file path resolves.
- `@semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.pdf` reports 105 fixtures and zero file problems.
- All 283 PDF JSON files parse with `jq`.
- Central taxonomy validation returns an empty error list after adding the ten exact PDF subset oracle overrides.

