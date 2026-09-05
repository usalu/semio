# TIFF Hand Repair

## Scope and method

Reviewed `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff` manually, one sibling group at a time. Every move used an exact old and new path with a destination-absence check and `mv -n`; no rename generator, palette, hash, normalization writer, Git mutation, or compatibility alias was used. Existing file bodies were retained except for exact path, descriptor-emoji, schema-sidecar, fixture-coordinate, and documentation references required by the moves.

## Baseline

The ticket's scoped read-only audit reported 331 files and 230 directories with 44 missing identities, 2 presentation-form findings, and 14 sibling-duplicate findings. The repeated/generic roots were the subset identities, oracle roots, option roots, fixture leaves, and mutation schema sidecars. Literal `Cargo.toml` and Cargo binary `src/main.rs` entrypoints were treated as conventional reserved names.

## Handpicked identities

| Meaning | Old identity | Current identity |
| --- | --- | --- |
| Baseline TIFF feature set | `✳️baseline` | `🧱️baseline` |
| Structured IFD/tag document | `✳️document`, briefly rejected `📄️document` | `🧾️document` |
| External truth implementation | `🧪️oracle` | `🔮️oracle` |
| Editor/viewer options | `🎚️options` | `☑️options` |
| Mutation regression guard | `🧬️mutation-regressions` | `🛡️mutation-regressions` |
| Set photometric interpretation | `⚙set-photometric-interpretation` | `🌈️set-photometric-interpretation` |
| Remove tile tags | `🏷remove-tile-tags` | `🗑️remove-tile-tags` |
| Remove strip offsets | `📏remove-strip-offsets` | `✂️remove-strip-offsets` |
| Set strip offsets | `📐set-strip-offsets` | `📍️set-strip-offsets` |
| Insert tile tags | `🔖insert-tile-tags` | `🧱️insert-tile-tags` |
| Set snapshot | `🔧set-snapshot` | `📸️set-snapshot` |
| Set compression | `🔩set-compression` | `🗜️set-compression` |
| Set bits per sample | `🧩set-bits-per-sample` | `🔢️set-bits-per-sample` |
| Replace pixel buffer | `🟪️replace-pixels` | `🔲️replace-pixels` |
| JSON Schema sidecars | repeated `🔣️.schema.json` | `🧬️.schema.json` |
| Fixture input/output leaves | missing-emoji `before.*` / `after.*` | `⬅️before.*` / `➡️after.*` |

The fourteen mutation fixture directories were aligned to their operation meaning: `🌈️set-photometric-interpretation`, `🗑️remove-tile-tags`, `🧱️insert-tile-tags`, `✂️remove-strip-offsets`, `📍️set-strip-offsets`, `📸️set-snapshot`, `🗜️set-compression`, `🔢️set-bits-per-sample`, `📥️insert-ifd-applied`, `📤️remove-ifd-applied`, `🏷️replace-tag-applied`, `🗑️remove-tag-applied`, `🔲️replace-pixels-applied`, and `🧭️change-byte-order`.

## Exact consumers repaired

- TIFF mutation descriptors, direct-mutation contract, aggregate Rust mounts, oracle manifests, generator/probe commands, protocol registry, native codec registry, examples, tests, and internal documentation.
- Stdio's main Rust barrel and oracle Rust barrel.
- Root dependency coordinates and the plugin policy allowlist were coordinated with the root owner after the final `🧾️document` decision.
- Central taxonomy overrides were coordinated for `🧱️baseline -> 🔮️oracle` and `🧾️document -> 🔮️oracle`; Cargo `**/src/main.rs` was registered as a conventional fixed name rather than polluted with an emoji.

No live non-ticket reference remains to TIFF's old `📄️document`, `✳️document`, `✳️baseline`, old operation basenames, old oracle/options roots, or old pixel-operation identity.

## Verification

- All TIFF JSON files parsed successfully with `JSON.parse`.
- Final ticket audit: 331 files, 230 directories, 550 governed nodes; zero missing, generic, presentation, spacing, duplicate, multiple-emoji, reserved-emoji, or oracle findings.
- `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.tiff`: 14 fixtures, 0 file problems; Nx target passed.
- Exact stale-coordinate searches across the live workspace returned no TIFF old-path matches.

The baseline mutation descriptors still refer to their pre-existing payload-schema names; this repair did not invent absent schema payloads. The canonical artifact verifier passed, so no unrelated content change was made.
