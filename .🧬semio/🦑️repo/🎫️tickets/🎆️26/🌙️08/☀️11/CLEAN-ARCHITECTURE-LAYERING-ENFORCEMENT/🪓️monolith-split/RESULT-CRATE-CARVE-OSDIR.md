# Result: CRATE-CARVE-OSDIR (prefix `osd`): `semio-framework-os-directory`

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder, `M` = `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory`. Tools: `F/osd-tools/` (`orig/` + `orig.SHA256` = snapshot of the module tree before the first edit; `carve.py` + `carve2.py` source edits, `manifests.py`, `facade.py`, `registries.py`, `osd-census.ts`, `pos/` = copy of the POS union oracle with sandboxes under `/Users/ueli/.cache/osd/`, `depcheck.py`, `visscan.py`, `lossless.py`, `fmt.py`, `tomlcheck.ts`, `taxcheck.ts`, `trees.ts`, `catalog-diff.ts`). Logs: `F/🗑️generated/osd-*`.

## 0. Verdict: DONE (static evidence only)

The crate was at 3,983 of 4,000 lines; it is now six crates, the largest at 2,741 lines. No real cargo ran (`📓️FLEET.md` has no `COMPILE ADMITTED` line), no Rust test was executed. Real-check commands are in section 6.

| Crate (lib = test closure unless noted) | Before files / bytes / lines | After files / bytes / lines | Ratio of gate (lines) |
| --- | --- | --- | --- |
| `semio-framework-os-directory` (facade + read model) | 23 / 187,887 / 3,983 | 1 / 14,692 / 215 | 0.996 -> 0.054 |
| `semio-framework-os-directory-schema` (new, `M/🧬️schema`) | | 10 / 124,827 / 2,741 | 0.685 |
| `semio-framework-os-directory-browser-actor` (new, `M/🧬️schema/🌐️browser-actor`) | | 1 / 8,997 / 192 | 0.048 |
| `semio-framework-os-directory-check-in` (new, `M/🧬️schema/📌️document-check-in-v1`) | | 2 / 7,256 / 181 | 0.045 |
| `semio-framework-os-directory-binary` (new, `M/🚪️io/🧱️binary`) | | 5 / 23,383 / 470 | 0.118 |
| `semio-framework-os-directory-access-policy` (new, `M/🛡️access-policy`) | | lib 4 / 9,239 / 172; test 6 / 47,321 / 209 | 0.052 |
| `…-tests-1` companion (test closure) | 13 / 244,885 / 491 | 11 / 206,585 / 447 | bytes 0.93 -> 0.79 |
| `…-tests-2` companion (test closure) | 14 / 126,820 / 391 | 14 / 126,820 / 391 | unchanged |

Every new or shrunk crate is inside the target (<= 3,000 lines, <= 24 files, <= 200 KB). Source: canonical counter `readRustSourceClosureV1` via `osd-tools/osd-census.ts` (`🗑️generated/osd-census-before.tsv`, `osd-census-final.log`). Not mine and unchanged: `…-client-tests` test closure is 248,067 bytes (0.95 of the byte gate) and `…-tests-1` still holds 206,585 bytes of embedded JSON fixtures (`command-receipt-v1.json` alone is 99,643 bytes); splitting those fixture sets needs its own companion carve (hand-off in section 7).

## 1. Design

No source file moved: each new crate takes an existing taxonomy node as its lib root (`[lib] path = "../../🦀️.rs"` beside the new `📦️packages/🦀️rust`), so the TypeScript and Rust text readers of those paths stay valid.

Findings that fixed the design:

- `🚪️io/📝️text` holds inherent `impl` blocks on schema types (`DirectoryEventPageV1::validate`, `DirectoryCommandRequestV1::canonical_json`, `DocumentCheckInV1::parse_canonical_json`, ...). Rust forbids inherent impls outside the defining crate, so the canonical text IO stays with its types: it is mounted from the schema root (`#[path = "../🚪️io/📝️text/🦀️.rs"] pub mod text;`), and the Check In text child is mounted from the Check In crate.
- `events`, `views` and `execution-target` are one strongly connected component (`DirectoryEvent`, `DocumentView`, `DocumentOpenPlanV1`, `DirectoryStreamMessage` refer to each other), so the core cannot be cut finer (`osd-tools/xref2.py`). `document-index` and `space-artifact-creation` feed core types.
- Cut candidates found by the same graph: `browser-actor` (leaf: no dependency on the core, used by `views` and `execution-target`, so it sits below), `document-check-in` (above: depends on `EditedArtifactFrontierV1` and `valid_document_open_hash`), the canonical checkpoint pair (value type used by nothing in the core, so it moves with its binary codec), `access-policy` (independent: serde plus its own JSON; the type and its `parse`/`declared` impl share a crate).

| Crate | Content | Direct first-party dependencies |
| --- | --- | --- |
| `…-browser-actor` | browser actor identities | value, value-derive, replication (`protocol`) |
| `…-schema` | wire contract core (events, commands, admin, views, document open, execution target, stream, session authority, space artifact creation, document index) plus `text` (canonical JSON, receipts, page validation) | value, value-derive, replication, artifact-reference, os-channel, os-identity, plugin-component-receipt, hash, pack-json, serde, browser-actor |
| `…-check-in` | Check In schema + its text IO | value, value-derive, pack-json, serde, schema |
| `…-binary` | `artifact_hash`, `descriptor_digest`, `checkpoint_pair` codec, and the `canonical_checkpoint_pair` value (mounted from `🧬️schema/🪢️canonical-checkpoint-pair-v1`) | hash, schema |
| `…-access-policy` | policy schema, `directory_access_permits`, its unit test (`#[cfg(test)]` mount, moved out of companion 1) | serde, serde_json |
| `semio-framework-os-directory` (stays) | `DirectoryReadModel`, `fold`, `fold_all` and the facade | value, async, schema, check-in, binary, access-policy |

Dependency direction is downward only (browser-actor < schema < check-in, binary; access-policy independent; facade on top); `visscan.py`/the corpus graph show no cycle.

Facade by design: the root keeps the public paths that `semio_framework_os_kernel::os_directory` re-exports and about 110 plugin and hub files name (`os_directory::{X, schema::X, io::binary::…, io::text::…, access_policy::…}`). It does so with `pub use semio_framework_os_directory_schema as schema;`, `pub use semio_framework_os_directory_access_policy as access_policy;`, an inline `pub mod io { pub use …_binary as binary; pub use …_schema::text; }` and the existing explicit root `pub use` lists (the Check In and checkpoint-pair names now come from their crates). This is the same pattern as the kernel's existing `client`/`identity` re-exports and the print-server precedent; no glob and no new alias beyond these three.

## 2. Edits

- Manifests (new): five `Cargo.toml` + `📋️project.json` under the new `📦️packages/🦀️rust`; facade `Cargo.toml` rewritten (removed `protocol`, value-derive, pack-json, hash, schema-validator, artifact-reference, os-identity, os-channel, plugin-component-receipt, serde, serde_json: no longer named in its closure; `schema-validator` was already unused).
- Sources: `crate::{DslValue,ToValue,FromValue,ValueError}` -> `semio_framework_value::…` (browser-actor 8, events 19, text 3); `crate::schema::`/`super::…::schema` -> `semio_framework_os_directory_schema::` in binary and Check In; `use crate::*` / `use crate::session_authority::*` / `use crate::space_artifact_creation::*` in the text files; binary root gains the pair mount and re-export; schema root drops three mounts and gains `pub use semio_framework_os_directory_browser_actor::{…}` (same 7 names) and the `text` mount; new `M/🛡️access-policy/🦀️.rs` (11 lines); `M/🚪️io/🦀️.rs` deleted (mounted by nobody).
- The only visibility change: `valid_document_open_hash` `pub(crate)` -> `pub` (used by binary and Check In). `visscan.py`: the other five `pub(crate)` items and the `pub(super)` one are used by no other crate.
- Companions: companion 2 mounts the browser-actor and Check In tests through the new crates (two dependencies added) and takes `CanonicalCheckpointPairRefusalV1` from the facade root; companion 1 loses the access-policy mount (now in the crate).
- Consumers re-pointed: directory client `📌️document-check-in` and `🪢️canonical-checkpoint-pair` (names now come from the facade root); eight hub files `use directory::os_directory::io::text::access_policy::directory_access_permits` -> `…::os_directory::access_policy::directory_access_permits` (`🌎️hub/🏗️bootstrap/{🌱️artifact-creation (2), 🎟️socket-grants, 📇️directory-commands, 📣️directory-events, 🔌️document-socket, 🛡️guard, 🛡️guard/🔐️sessions}`). The kernel and the 110 plugin/hub files need no edit.
- Registrations: root `Cargo.toml` (5 byte-sorted members), `📋️build-owners.json` (5 rows, 3,197 names sorted and unique), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (30 routes each, orders `900.99001..10`, `922.99001..10`, `903.99001..10`, checked unused), OS corpus `🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (5 rows), `🔣️schema-catalog.json` (one hash: the schema root facet, verified against the renderer output).

## 3. Verification (real results)

| Check | Command / tool | Result |
| --- | --- | --- |
| Lossless | `osd-tools/lossless.py` (`osd-lossless.log`) | 4,503 -> 4,492 non-blank lines; lost 80 / added 69, all deliberate (removed mounts, `crate::`/`super::` paths, imports, facade wiring, docstrings) |
| rustfmt parse | `osd-tools/fmt.py` | 30 changed or new files, 0 parse errors |
| Union differential vs the pre-carve tree (git index) | `POS_PREFIX=osdpos python3 pos.py run families/osd.json --cfgs lib,test,wasm,p2,p2test` (`osd-pos-r2.log`); 10 crates cur (facade, 5 new, both companions, client, client-tests) vs 5 base | NEW 0 / GONE 0 in all five cfgs (363, 891, 273, 234, 739 errors on both sides, all stub-layer noise); typeck canary reached on both sides; privacy, trait-scope, resolve, type, other: 0 non-stub |
| Oracle sensitivity | `--mutate` (`osd-pos-m1.log`, `m2`) | renaming an import in binary: 3 new E0425 caught; private `valid_document_open_hash`: 18 new E0432/E0425 caught. The union flattens crates into modules, so crate-restricted visibility is invisible to it: covered by `visscan.py` (section 2) |
| Dependencies vs sources | `osd-tools/depcheck.py` | named-but-undeclared: none in all six; declared-but-unnamed: none (facade's `access_policy` alias is named by `pub use … as`) |
| Manifests | `osd-tools/tomlcheck.ts` | 8 manifests parse, workspace resolves to the repo root, lib and dependency paths exist, owners row and member present, directory members byte-sorted, longest manifest path 155 bytes |
| Taxonomy | `osd-tools/taxcheck.ts` (`matchDirectoryKind`) | `checked=10 unresolved=0` (`packages`, `rust-language`, `crate-module-children`) |
| Census | `osd-tools/osd-census.ts` | section 0 |
| Dependency ratchet | `bun ./📜️script.ts verify dependencies` (`osd-verify-deps.log`) | exit 0, "clean — no new third-party dependencies" |
| Cross-crate `cfg(test)` | `bun F/ctc-tools/ctc-scan.ts --only <8 crates>` (`osd-ctc-scan.log`) | 3,097 crates enumerated, 11 scanned, hits 0 |
| Descriptor-digest reader law | `bun test` on `M/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts` | 8 pass 0 fail (module-tree reads of the schema root still resolve) |
| Source-text readers | `osd-tools/trees.ts` + literal comparison of the six reader scripts (four hub checks, `📇️directory-checks`, descriptor-digest) against the old and new schema module tree | no needle is lost (three generic words and one error string only); `readModuleTreeSource` does not follow the outward `text` mount, so the tree shrank by exactly the three moved files |
| OS crate-boundaries law | `bun test --timeout 240000 O/🧪️tests/📦️crate-boundaries/🟦️.ts` (`osd-law-os-crate-boundaries.log`) | 116 pass 0 fail, including the five new corpus rows (launch routes, TOML agreement, lib path, forbidden dependencies, caps, token scan) |
| Other laws touched | `bun test` of `📇️directory-read-model`, `🛡️access-policy`, `🪢️canonical-checkpoint-pair`, `🏷️schema-vocabulary`, `lib-wiring-budget` | read-model 5 pass; access-policy 3 pass; checkpoint-pair 32 pass 1 fail and schema-vocabulary 2 fail are inherited (missing `🧬️schema/🟦️.ts`, missing hub schema file); lib-wiring-budget fails only for plugin crates in `✏️s` |
| Schema catalog | `osd-tools/catalog-diff.ts` | rendered catalog contains the new schema-root hash; the remaining drift is foreign (mutation schema hashes) |
| JSON | python `json.loads` | `launch.json`, the seed (strict JSON), `📋️build-owners.json`, the corpus all parse |

## 4. Not done / limits

- No compiler ran: borrowck, trait resolution through the new glob/alias re-exports, the `unused_imports` family and macro hygiene are decided by the real check. The differential runs against stubbed externals.
- Lock files: the new packages are missing from `Cargo.lock` at the repo root and from 177 more lock files that contain `semio-framework-os-directory` (`✏️s/**`, `🌎️hub/Cargo.lock`, `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.lock`). Not hand-edited; refresh per workspace in the compile window.
- The hub check scripts that read the schema tree are cargo-driven and were not run; their needles are unaffected (section 3).
- `📋️dependencies.json` `users` rows were not rewritten; run `verify dependencies write-baseline` only if the coordinator wants the new manifests recorded.

## 5. Files

Edited: `M/🦀️.rs`, `M/📦️packages/🦀️rust/Cargo.toml`, `M/🧬️schema/🦀️.rs`, `M/🧬️schema/{🌐️browser-actor,📌️document-check-in-v1,📖️events-commands-and-admin,🪟️views-and-document-open,🪢️canonical-checkpoint-pair-v1}/🦀️.rs`, `M/🚪️io/📝️text/{🦀️.rs,🌱️space-artifact-creation-v1,📌️document-check-in-v1,🪪️session-authority-v1}/🦀️.rs`, `M/🚪️io/🧱️binary/🦀️.rs` and its three children, `M/🛡️access-policy/🚪️io/📝️text/🦀️.rs`, `M/🔌️client/{📌️document-check-in,🪢️canonical-checkpoint-pair}/🦀️.rs`, both companions (`🦀️.rs`, `Cargo.toml`), eight hub files, root `Cargo.toml`, `📋️build-owners.json`, both launch files, the OS corpus, `🔣️schema-catalog.json`. Created: `M/🛡️access-policy/🦀️.rs` and five `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`. Deleted: `M/🚪️io/🦀️.rs`.

## 6. Commands for the compile queue

```
cd /Users/ueli/Documents/semio
bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline
bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-directory-browser-actor -p semio-framework-os-directory-schema -p semio-framework-os-directory-check-in -p semio-framework-os-directory-binary -p semio-framework-os-directory-access-policy -p semio-framework-os-directory -p semio-framework-os-directory-tests-1 -p semio-framework-os-directory-tests-2 -p semio-framework-os-directory-client --lib --tests --keep-going --message-format=short
bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-os-directory-access-policy -p semio-framework-os-directory-tests-1 -p semio-framework-os-directory-tests-2 --lib
bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-directory -p semio-framework-os-directory-schema --target wasm32-wasip2
```
Then the kernel and hub: `-p semio-framework-os-kernel` and the hub workspace check (the eight re-pointed imports). If a build exposes a missing `pub`, widen that one definition.

## 7. Hand-offs

- Companion 1 (`…-tests-1`, 206,585 bytes of fixtures) and `…-client-tests` (248,067 bytes) stay close to the byte gate; the schema unit tests could be split by fixture set (command receipt 99,643 bytes, lease 32,399, open plan 21,153).
- Stale cargo filter in `📇️directory-checks` (`os_directory::io::binary::checkpoint_pair::tests::` against `semio-framework-os-kernel`) predates this slice; the law now lives in `…-tests-2`.
