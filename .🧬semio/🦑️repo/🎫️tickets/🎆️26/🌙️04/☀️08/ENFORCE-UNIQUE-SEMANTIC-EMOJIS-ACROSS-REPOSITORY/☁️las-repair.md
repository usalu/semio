# LAS Emoji Repair Evidence

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las`.

The initial strict audit counted 345 governed entries and 53 findings: 41 missing, 8 duplicate, and 4 presentation violations. Every affected basename was reviewed case-by-case. Subsets are now `🎩️header`, `📍️points`, and `📼️vlr`; subset-local oracle directories use `🔮️oracle`; window options use `☑️options`; mutation presentation forms and the snapshot schema sidecar were normalized. Fixture identities are semantic (`📐️set-bounds`, `📅️set-creation-date`, `↩️set-points-by-return`, `⚖️set-scale-and-offset`, `💻️set-software-info`, `🖥️set-system-identifier`, `🔖️set-version`, `🛩️survey-strip`, `➕️insert-point`, `➖️remove-point`, `✏️set-point`, `📥️insert-vlr`, `🗑️remove-vlr`, `🗃️set-vlr-data`) and before/after roles are `⬅️before.json` and `➡️after.json`. The generated survey asset is `☁️survey-strip.las`.

Verification:

- Final strict audit: 345 governed entries; all eight violation categories are 0.
- All LAS JSON documents parse.
- Every fixture path in all three oracle catalogs resolves to a file.
- No old subset, oracle, mutation-presentation, fixture-directory, or fixture-file coordinate remains in LAS or its shared Stdio mounts.
- Generator module import passes.
- `bun …/🎩️header/🏭️generator/📜️script.ts manifests` passes and emits `../🧫️fixtures/🛩️survey-strip/☁️survey-strip.las` with SHA-256 `e452a27f1bc94c4311067b60e9d2b4f704c89f3aa287dee3d9cc9167604c8af8`.
- Taxonomy validation returned `[]`.
- Focused Nx Rust verification reached only the unrelated concurrent Semio missing-old-path failure documented in the STEP evidence.

The Stdio Rust package sibling collision was also repaired by hand as `🧫️fixture` → `🧭️wiring-fixture`, with its exact script reference updated. Its scoped audit now reports 21 governed entries and all eight violation categories at 0.

