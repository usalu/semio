# IFC Emoji Repair

## Scope

Hand-repaired `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc` without Git state mutation or generated rename plans.

## Handpicked taxonomy

- IFC 2x3 subsets: `🧱️base`, `🏢️cobie`, `🤝️cv20`, and `🧮️sav`.
- IFC4 standard: `4️⃣4`; wildcard subset remains the semantic `✳️any` singleton.
- Subset-local configuration and oracle: `☑️options` and `🔮️oracle`.
- Snapshot mutations: `📸️set-snapshot`.
- IFC4 mutations: `➕insert-entity`, `🧩insert-entity-arg`, `➖remove-entity`, `🧹remove-entity-arg`, `🎛️set-entity-arg`, `🏷️set-entity-name`, `🗒️set-file-description`, `📛️set-file-name`, and `🧬️set-file-schema`.
- Fixture directories mirror their mutation identity; fixture carriers use `⬅️before.ifc` and `➡️after.ifc`.
- Real-world fixture identities use `🏥️wellness-center-sama-street-level`, `🏗️wellness-center-sama-structural-seed`, and `🏢️nakagin-capsule-tower` with identically named carrier files.

## Reference repair

- Updated subset-local source, manifests, tests, documentation, include paths, and ownership fields.
- Updated exact Stdio Rust barrel, oracle mount, and policy allowlist coordinates.
- Added explicit central taxonomy oracle overrides for all five repaired subsets.
- Removed stale IFC standard, local-oracle, option, fixture-carrier, and mutation coordinates from current IFC source.

## Verification

- IFC4 scoped audit before its tracked parent move: 190 files, 117 directories, 307 governed entries; every count zero.
- Parsed 69 IFC JSON documents successfully.
- Resolved 28 fixture manifests and 56 fixture files; byte counts and SHA-256 digests all match.
- `validateTaxonomy()` returned `[]`.
- `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.ifc`: 28 fixtures, 0 file problems.

