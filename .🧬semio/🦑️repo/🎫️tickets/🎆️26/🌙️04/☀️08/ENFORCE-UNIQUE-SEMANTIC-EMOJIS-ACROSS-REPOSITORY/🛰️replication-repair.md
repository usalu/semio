# Replication Hand Repair

Final directory-classifier verification after the parent lane's contextual matcher and vocabulary repairs: the existing `@semio-tech/repo-lib:test` Nx route passes the actual-classifier/independent-compiler test, 103 scenarios, 909 assertions, zero failures. This includes both newly added replication-specific mutation-test roles while retaining the global vocabulary cases.

Scope: `🧰️framework/🔨️modules/📡️replication`. Each name below was selected after reading its meaning, fixture consumers, and sibling context. No generated name selection, rename script, replacement script, Git mutation, or whole-file restore is used. Tool-owned `node_modules` caches are excluded; literal `Cargo.toml`, `package.json`, and Vitest's `vitest.config.ts` are retained. Unique format-marker leaves remain unchanged.

## Initial Findings

Authored paths contain repeated mutation-contract test prefixes, fixture/schema/test role collisions, five schema/fixture leaf collisions, and nineteen wire specimens sharing the same package emoji. No stacked authored basenames were found. TypeScript's wire fixture test also uses nonexistent flat `.bin` paths despite the current owner-directory/`💾️.bin` layout.

## Handpicked Moves

Paths are relative to the scope root. Descendants retain their contents and follow the explicit parent move.

| Old Path | New Path | Meaning |
| --- | --- | --- |
| `🎮️mutation/🧪️tests/🧬️mutation-leaf-contract` | `🎮️mutation/🧪️tests/🤝️mutation-leaf-contract` | Descriptor/provenance agreement. |
| `🎮️mutation/🧪️tests/🧬️mutation-leaf-source-contract` | `🎮️mutation/🧪️tests/🧭️mutation-leaf-source-contract` | Source ownership/scope qualification. |
| `📡️wire/🏠️local-interaction/🧪️fixtures` | `📡️wire/🏠️local-interaction/🧫️fixtures` | Specimen data distinct from tests. |
| `📡️wire/🏠️local-interaction/🧫️fixtures/♻️retirement/🔣️.schema.json` | `📡️wire/🏠️local-interaction/🧫️fixtures/♻️retirement/🧬️.schema.json` | Retirement specimen structure. |
| `📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🔣️.schema.json` | `📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🧬️.schema.json` | Local-interaction specimen structure. |
| `📡️wire/🏠️local-interaction/🧫️fixtures/📃️query/🔣️.schema.json` | `📡️wire/🏠️local-interaction/🧫️fixtures/📃️query/🧬️.schema.json` | Query specimen structure. |
| `📡️wire/🏠️local-interaction/🧫️fixtures/🔐️topology-authority/🔣️.schema.json` | `📡️wire/🏠️local-interaction/🧫️fixtures/🔐️topology-authority/🧬️.schema.json` | Authority specimen structure. |
| `📡️wire/🏠️local-interaction/🌳️root/🧪️fixture` | `📡️wire/🏠️local-interaction/🌳️root/🧫️fixture` | Root specimen. |
| `📡️wire/🏠️local-interaction/🌳️root/🧪️schema` | `📡️wire/🏠️local-interaction/🌳️root/📐️schema` | Root fixture constraints. |
| `📡️wire/🏠️local-interaction/🌳️root/🩹️update/🧪️fixture` | `📡️wire/🏠️local-interaction/🌳️root/🩹️update/🧫️fixture` | Update specimen. |
| `📡️wire/🏠️local-interaction/🌳️root/🩹️update/🧪️schema` | `📡️wire/🏠️local-interaction/🌳️root/🩹️update/📐️schema` | Update fixture constraints. |
| `📡️wire/🏠️local-interaction/📡️transport/🧪️fixtures` | `📡️wire/🏠️local-interaction/📡️transport/🧫️fixtures` | Transport specimens. |
| `📡️wire/🏠️local-interaction/📡️transport/🧪️schema` | `📡️wire/🏠️local-interaction/📡️transport/📐️schema` | Transport fixture constraints, distinct from wire schema. |
| `🧫️fixtures/🧫️artifact-bootstrap` | `🧫️fixtures/🚀️artifact-bootstrap` | Artifact bootstrap/startup. |
| `🧫️fixtures/🚀️artifact-bootstrap/🔣️.schema.json` | `🧫️fixtures/🚀️artifact-bootstrap/🧬️.schema.json` | Bootstrap specimen structure. |
| `🧫️fixtures/🧫️wire` | `🧫️fixtures/📡️wire` | Network wire frames. |

The following paths are relative to the renamed `🧫️fixtures/📡️wire` parent. Every old prefix is `📦️`:

| Old Name | New Name | Meaning |
| --- | --- | --- |
| `📦️client-bye` | `👋️client-bye` | Client farewell. |
| `📦️client-commands` | `🕹️client-commands` | Client command input. |
| `📦️client-credit-grant` | `🎟️client-credit-grant` | Client-issued flow-control allowance. |
| `📦️client-frontier-advertise` | `🚩️client-frontier-advertise` | Advertised causal frontier. |
| `📦️client-presence` | `🙋️client-presence` | One client's presence. |
| `📦️client-preview-publish` | `📣️client-preview-publish` | Client publishes a preview. |
| `📦️server-ack-accepted` | `✅️server-ack-accepted` | Accepted acknowledgement. |
| `📦️server-ack-rejected` | `⛔️server-ack-rejected` | Rejected acknowledgement. |
| `📦️server-ack-transformed` | `🔀️server-ack-transformed` | Transformed acknowledgement. |
| `📦️server-commands` | `🎮️server-commands` | Server command delivery. |
| `📦️server-credit-grant` | `🎫️server-credit-grant` | Server-issued flow-control allowance. |
| `📦️server-error` | `🚨️server-error` | Protocol error. |
| `📦️server-presence` | `👥️server-presence` | Peer collection from server. |
| `📦️server-preview` | `👁️server-preview` | View of published preview. |
| `📦️server-session` | `🪪️server-session` | Session identity. |
| `📦️server-snapshot-chunk` | `🧩️server-snapshot-chunk` | One snapshot piece. |
| `📦️server-snapshot-done` | `🏁️server-snapshot-done` | Snapshot completion. |
| `📦️server-welcome-snapshot-inline` | `📸️server-welcome-snapshot-inline` | Welcome with inline snapshot. |
| `📦️server-welcome-tail` | `🔗️server-welcome-tail` | Welcome continuing the causal tail. |

`🚫️legacy-client-hello-rejected` remains a specifically rejected obsolete wire tag, not legacy support.

## Verification

All 35 handpicked moves are applied, with absence checks before each exact move and precise context patches for incoming references. All 20 binary specimens retain their pre-move SHA-256 hashes. No specimen bytes changed.

Passed:

- `bun nx run @semio-tech/framework-replication:test --skip-nx-cache`: all five TypeScript tests pass. The initial run had four passes and one ENOENT from a flat `📦️client-commands.bin` reference. Each of the nineteen affected calls now names its exact semantic owner plus `💾️.bin`; decoder and byte-roundtrip assertions remain unchanged.
- `bun nx run @semio-tech/framework-replication-rs:test-local-interaction-source --skip-nx-cache`: local interaction, retained root/update, mutation descriptor, retirement, query, topology authority, and transport fixture checks pass with Ajv, Immer, jsonc-parser, lodash, node-crypto, and the independent leb128 codec. Three already-invalid flat `🔣️schema.json` references were corrected to the exact fixture-schema leaf; one already-invalid singular transport fixture path was corrected to its real specimen.
- `bun nx run @semio-tech/framework-replication-rs:test --skip-nx-cache -- long --lib`: 242 native tests pass, zero failures or ignored tests. Build outputs stay under this ticket's `🗑️generated/replication/cargo`.
- Read-only resolution of 61 direct Rust/TypeScript/schema references finds no missing targets.
- Full basename review with `Intl.Segmenter`, independent `emoji-regex`, and the repository statute checker finds no missing, multiple, duplicate, generic, presentation, or reserved-name violations. Tool-owned node_modules subtrees and the three documented fixed manifest/config names are the only exclusions.
- After the parent lane's registry update, a complete fresh traversal resolves all 73 directory semantic roles. `validateTaxonomy` returns no problems; all 152 authored entries remain free of statute and independent emoji-oracle findings.

Two preexisting, empty, unreferenced directories (`🧪️fixtures/🧪️artifact-bootstrap` and its empty parent) were removed with exact `rmdir` calls. Neither contained files; the empty directories can be recreated. No authored input was removed. After this removal the scoped tree contains 79 files and 73 directories excluding node_modules.

## Incoming Reference Edits

Within the scope, changes are limited to imports/includes, fixture path literals, and the root path documentation. Outside the scope:

- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`: one artifact-bootstrap fixture import.
- Rust test files beneath `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction` for `📖️capture`, `♻️retirement`, `📡️live`, `📡️live/🧪️dispatch`, `🔐️authority`, and `📃️query`: exact local-interaction fixture includes only.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️package-language-kind-handoff/🔣️.json`: the two renamed mutation-contract scenario names and their canonical expectations.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`: the exact `wire_fixtures_stay_byte_identical_across_rust_and_ts` function and its related fixture documentation. Its old implementation would recreate flat generic fixture names under a stale owner and delete old fixture files. With parent approval it now performs 19 read-only committed-byte comparisons (6 client, 13 server), retains the current socket hello's lane/frame round-trip in memory, and explicitly rejects the committed obsolete hello. All 20 exact committed paths resolve. No filesystem write, create, rename, or delete call remains in that function; all 20 binary hashes are unchanged.

Parallel renderer checks also found two additional value references; those repairs are recorded separately in `🌱️value-repair.md`.

## Retained Names Reviewed

The root modules retain their distinct semantic identities: `⚙️codec` (encoding machinery), `⚔️conflict` (conflict handling), `🎮️mutation` (mutation command contract), `📖️dictionary` (dictionary storage), `📐️format` (binary format layout), `📡️wire` (network frames), `🧾️wire` (protocol records/errors), `🔐️crypto` (cryptography), `🔗️causal` (causal ordering), `🔢️scalar` (scalar types), `🆔️ids` (identifiers), and `🚰️source` (source access). Package, task, project, and unique implementation-format markers retain their roles. Local interaction retains its local scope, tree root, patch update, transport, retirement, query, and authority roles. Causal mutation specimens retain distinct add/descriptor/schema roles. Format marker leaves each reflect their real format and are unique in their sibling group.

The parent lane owns central semantic-role registration. No shared taxonomy file was edited by this lane.

## Additional Verification

The OS golden-fixture test was attempted through `@semio-tech/framework-os-kernel:test-native` with the exact test-name filter and this ticket's replication Cargo output directory. Its build exceeded the existing 1,200,000 ms budget before assertions. The isolated build was actively compiling `semio_s_plugin_stdio`, not waiting on another lane's target directory. No native golden pass is claimed. It was never run in its prior mutating form; no fixture bytes were overwritten.

The package-language classifier test initially failed twice in its isolated compiler harness: it extracts `canonicalDirectory`, whose new `pathEmojiStatuteFindings` dependency was not passed to `new Function`. The parent lane fixed that dependency injection and the wrong-parent ambiguity bug exposed next. The parent also aligned the handpicked framing vocabulary. The two neutral global mutation-vocabulary cases retain their original `🧬️` data values (they are not filesystem references); two added cases now separately check the exact `🤝️`/`🧭️` names under `tests` with their scoped semantic roles. This preserves the original global-vocabulary coverage. Final rerun passes 103 scenarios and 909 assertions, as recorded above.

The exhaustive obsolete-reference search finds no remaining authored references to the moved replication names. One generated WGPU `🟨️frame-worker.js` comment still embeds the former `🧫️fixtures/🧫️wire` documentation path; its authored source is corrected. The parent was notified to regenerate the owning bundle once its concurrent source repairs stabilize. This lane did not hand-edit generated JavaScript.
