# UI Contract Hand Repair

Scope: `🧰️framework/🔨️modules/🖱️ui/🧬️contract`. No nested AGENTS files. The parent owns surrounding UI roots. Names and moves are decided individually; no Git mutations or rename/replacement scripts.

## Baseline

The initial physical audit included three old Cargo output caches (`target-root-ui-contract-check`, `target-root-ui-contract-native`, and `target-root-ui-contract-wasm`). Those compiler-managed contents are not authored source and will not be renamed. Authored conformance cases, retained fixtures, module files, and role-directory collisions remain in scope.

## Rust module choices

Under `📦️packages/🦀️rust`, each former `🦀️<name>.rs` becomes the exact meaningful name below; `🦀️.rs` remains the single format entry.

| Module | Exact new basename | Responsibility |
| --- | --- | --- |
| accessibility | `♿️accessibility.rs` | Accessible node properties |
| action | `🎬️action.rs` | Versioned actions, triggers, intents |
| builder | `🏗️builder.rs` | Semantic UI construction |
| component | `🧩️component.rs` | Closed component vocabulary |
| conformance | `🔬️conformance.rs` | Cross-implementation corpus assertions |
| document | `📃️document.rs` | Flat revisioned UI documents |
| layout | `📐️layout.rs` | Renderer-neutral geometry |
| limits | `🛡️limits.rs` | Quotas and transactional validation |
| presence | `👥️presence.rs` | Multi-user ephemeral presence |
| style | `🎨️style.rs` | Token-based visual style |
| surface | `🗺️surface.rs` | Embedded product surfaces |
| text_edit | `🪢️text_edit.rs` | Immutable paged text editing |

The explicit integration target leaf `tests/typegen_export.rs` becomes `tests/🧬️typegen_export.rs`; Cargo's conventional `tests` directory stays literal. The empty non-reserved `bindings` directory becomes `🔗️bindings`.

Verification and subsequent handpicked batches are recorded as they complete. Evidence lives under `🗑️generated/ui-contract/`.

## Conformance corpus

All 62 case-directory identities were handpicked in `📚️examples/🧪️conformance/📇️catalog.json`, preserving the existing case IDs. Each old `<id>.snapshot.json`, `<id>.expect.json`, and optional `<id>.patch.json` moved to its catalog-declared directory with distinct `📸️snapshot.json`, `🎯️expect.json`, and `🩹️patch.json` role leaves. All 146 payload SHA256 values match the pre-move bytes. The new language-neutral catalog/schema is checked by Ajv and independently deserialized/checked by Rust serde. The owned Nx conformance target first rejected the still-flat patch group, then passed all 62 catalog cases and six Rust corpus tests after exact moves and consumer repairs. Renderer test IDs still use the unchanged catalog case IDs.

## Role-directory repairs

Root `📋️copy` → `🪞️copy` (typed copying), `🧬️typed` → `🧾️typed` (field roster), retirement `📋️patch` → `🩹️patch` (patch retirement), and resident `🟣️root` → `🌳️root` (owned document root).

The individually inspected collision owners below use `🧫️fixture` for the existing neutral data directory, `📐️schema` for its existing fixture schema where present, and retain `🧪️tests`:

- `♻️retirement`, its `🌳️typed`, `📋️list`, `🩹️patch`, `📮️handback`, and `🩹️patch/📨️pending/📦️whole`.
- `⚖️compare` and `⚖️compare/📃️document`.
- `🎟️resident` and `🎟️resident/🌳️root`.
- `📃️document/🎟️assembly`, `🪞️copy`, `📋️list`, and `🔗️bindings/📋️copy`.

Typed retirement leaves: `🦀️component.rs` → `🧱️component.rs`, `🦀️document.rs` → `📃️document.rs`, `🧪️document.rs` → `🔬️document.rs`, `🧪️components.json` → `🧩️components.json`, `🧪️components.schema.json` → `🧬️components.schema.json`. Copy's `🧪️bytes.rs` → `🔬️bytes.rs`.

Within retained resident ownership, the builder, page, page/binding, reader, payload, slot, evidence, evidence/copied, evidence/cancellation, and metadata owners each distinguish their fixture `📐️schema` directory from their retained `🧪️fixture`. Their concrete `🧬️contract.json` data becomes `🤝️contract.json`, retaining `🧬️schema.json`; the resident root and operations/wire/pages concrete contract use the same distinct identity. All exact actor, renderer, Rust, and declaration references are repaired together.

Other exact choices: root fixtures `🔣️presence-overlay.json` → `👥️presence-overlay.json`; retained scene/typed `🔣️catalog.json` → `📇️catalog.json`, `🧩️pack` → `📦️pack`. The latter denotes the binary pack format and stays distinct from the JSON projector.

## Retained fixture case choices

Under `🧵️retained/🧪️fixtures`, each named data/schema pair now has a case-owned directory below with `🔣️.json` and `🧬️.schema.json`. This keeps the actual data and schema roles distinct without arbitrary glyph palettes. Existing tests refer to these exact paths, not a guessed prefix.

| Existing `🔣️<stem>.json` / `🔣️<stem>.schema.json` stem | Handpicked case directory |
| --- | --- |
| instance-close | `🚪️instance-close` |
| instance-maintenance | `🛠️instance-maintenance` |
| instance-owner | `🪪️instance-owner` |
| intake-close-fault | `🚨️intake-close-fault` |
| intake-notification | `🔔️intake-notification` |
| intake | `📥️intake` |
| native-child | `👶️native-child` |
| owned-hash | `🔏️owned-hash` |
| owned-nodes | `🌳️owned-nodes` |
| owned-operations | `⚙️owned-operations` |
| owned-scene | `🎬️owned-scene` |
| owned-surface | `🗺️owned-surface` |
| owned-validation | `🛡️owned-validation` |
| patch | `🩹️patch` |
| read-lease | `🎟️read-lease` |
| read-publication | `📤️read-publication` |
| resident | `💾️resident` |
| root-source | `🌱️root-source` |
| scene-binding | `🔗️scene-binding` |
| scene-generic-pack | `📦️scene-generic-pack` |
| scene-json-document | `📃️scene-json-document` |
| scene-json-string | `🧵️scene-json-string` |
| scene-json | `🔣️scene-json` |
| scene-numeric | `🔢️scene-numeric` |
| scene-pack-field | `🧳️scene-pack-field` |
| scene-text-bytes | `🔤️scene-text-bytes` |
| surface-child | `🪆️surface-child` |
| typed-scene | `🧾️typed-scene` |
| wire-operations | `📨️wire-operations` |

The resident pair's relative capacity-source reference gained one additional `../` after this relocation; its data and schema constant changed together. The other payloads moved unchanged.

Wire fixtures under `🧵️retained/📦️wire/🧪️fixtures` now use individually chosen `📤️decode`, `🏷️fields`, `🗺️surface-bytes`, and `🧾️typed` case directories. Each contains the distinct data `🔣️.json` and schema `🧬️.schema.json` roles. All exact renderer and Rust consumer paths were patched manually.

## Final audit and verification

The complete authored tree contains 601 physical entries, 597 governed entries, zero emoji-statute findings, and zero unknown semantic directories. The three previously identified Cargo cache trees remain compiler-managed evidence, not renamed source. Cargo.toml, the conventional tests directory, README variants, and the fixed Nx manifest retain their exact tool-authorized names. New retained fixture cases and implementation roles are registered by exact owner, without a generic prefix fallback.

- 874 literal relative dynamic imports in UiDocumentStore, actor lifetime, and actor shard-client resolve to existing files, accounting for Vite's `?raw` query.
- `@semio-tech/ui-contract-rs:test-quick`: all 160 native tests and 75 language-neutral fixed-list checks pass after regeneration of the owned TypeScript schema mirror.
- `@semio-tech/ui-scene-rs:test-quick`: all 108 native scene tests pass after its three fixture import repairs.
- `@semio-tech/framework-renderer-react:test-quick`: four tests pass. The exhaustive target completed successfully: all 747 tests pass across seven files, including UiDocumentStore's ownership matrices and the catalog-backed Interpreter conformance cases.
- `@semio-tech/framework-actor:test-long`: 198 of 199 pass. The failure is response error serialization (`{}` versus `Error: post-after-observation`), not a missing fixture path. No unrelated runtime change was made to suppress it.
- Canonical WGPU generation refreshed one of six artifacts after the catalog-input path repair; its subsequent check reports all six fresh. The current package catalog bytes and digest are unchanged. Frozen historical package/source witnesses remain untouched.

Source-reference scans of root scripts, framework, and s find the old core Rust names only in frozen package-purity witnesses, which are deliberately preserved. No Git state was modified and no bulk rename/replacement scripts were used.
