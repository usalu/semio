# FND-Source-Authority-07 Evidence

The private `MutationSourceAuthority` region is mirrored in the DSL derive glue and component sources. It accepts an explicit compiler source path and compiler working directory, resolves relative `Span::local_file()`-style paths lexically, and then proves only these facts: workspace root, mutation root, owner, source path, sibling descriptor path, and taxonomy path.

The proof finds the nearest ancestor bearing the exact regular-file `nx.json` plus `📋️project.json` marker pair. It reads only `metadata.semio.taxonomy`, validates the locator before target access, derives the canonical Rust and JSON filenames from taxonomy file-kind fields, finds exactly one taxonomy mutation collection, requires a direct owner leaf, and requires the descriptor to be a JSON object whose `owner` string equals the byte-for-byte normalized repository-relative owner path.

No taxonomy path or compatibility filename is hardcoded. It does not validate the other descriptor metadata fields and does not wire any public trait, derive, registry, or generated behavior.

The authority-local neutral fixture and schema are:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🧬️mutation-source-authority/🧫️fixtures/🔣️cases.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🧬️mutation-source-authority/🛂️schema/🔣️cases.json`

The 18 vectors contain normal and relative-parent source positives plus missing/malformed locator, parent/source/descriptor/taxonomy symlink, missing root-pair, wrong primary filename, descriptor-owner mismatch, virtual absent compose, raw case-folded `COMPOSE` before parent reduction, nested unpaired `nx.json`, symlink ancestor, symlink erased by parent reduction, native regular-file parent erasure, and Unix/Windows non-UTF-8 source components. The file-parent vector independently asserts the native `ENOTDIR` oracle. The valid descriptor is the authoritative exact fourteen-field shape (`schemaVersion`, `owner`, `semanticKind`, `displayName`, `emoji`, `aggregateVariant`, `payloadSchema`, `textOpcode`, `binaryTag`, `invertibility`, `diffParticipation`, `outcomeClasses`, `composition`, `requiredLanguageSurfaces`); this helper intentionally checks only its object form and required `owner` field.

The unit test materializes each vector below the canonical real path given by `SEMIO_TEST_ARTIFACT_DIR`, retaining those artifacts when supplied. Without that environment variable it uses a unique path below the system temporary realpath base. It never materializes an actual `compose` subtree. The test is mirrored in both derive sources, using source-relative fixture inclusion. No Cargo command was run by this packet; root owns the registered crate execution.

No-Cargo checks run: Ajv accepted the local fixture schema (`[DEBUG] Ajv accepted 18 source-authority vectors and exact 14-field descriptor`) and accepted the fixture descriptor, after its test-local owner injection, against `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json`. Root `📜️script.ts` imported successfully; scoped `git diff --check` was clean. The registered Rust unit test is pending root's controlled execution.

Final SHA-256 source bytes:

- `cca2fbb1ad1452389e33c89efc7af5f1700f4bf32827277a38dc5a64f70730eb` — `📦️glue.rs`
- `aacc698db3d5ef28535a51da906967bfbb9588d2818d9feaf29d52fa5dd1efcd` — `🦀️component.rs`
- `2949f6a0332158ca6c7ec5d116e4c16dc62dce4e7314cf570e018f03d7286312` — `Cargo.toml`
- `340d240327a6a4c249c43ddc7c27d9e12799a0a3a670497e2821ae6fd9bbfb90` — fixture
- `124e6c1511959efe09949bd96a08ee66c1371bc5fbd0eb8f1c265f50b101448b` — schema
