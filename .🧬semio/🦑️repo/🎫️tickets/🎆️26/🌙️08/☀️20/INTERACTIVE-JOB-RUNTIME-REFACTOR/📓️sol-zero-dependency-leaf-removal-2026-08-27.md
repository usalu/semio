# Sol Zero-Dependency Leaf Removal — 2026-08-27

## Result

The three scout-prioritized zero-dependency leaves are removed from their direct owners without changing `🔒️dependencies.json` or any language-neutral golden that existed before this packet.

| Leaf | Direct source replacement | Manifest ownership | Lock ownership |
| --- | --- | --- | --- |
| `rust:byteorder@1.5.0` | The retained-command checkpoint test oracle now writes each little-endian `u64` through an owned shift/mask loop. The fixture-driven encode/decode differential remains intact. A hostile test freezes every byte lane with `0x8877665544332211` plus `u64::MAX`. | Removed from `semio-framework-plugin` dev-dependencies. | Removed only from the `semio-framework-plugin` dependency list. The `byteorder` package remains because unrelated transitive users such as SQLx still own it. |
| `js:fast-glob@3.3.3` | Every repo-library test import/call was replaced by a shared Node filesystem-boundary enumerator. It uses `Dirent` recursion, UTF-8 byte ordering, file/directory/symlink identity, and never descends through symlinks. All existing schema, mapping, inventory, transaction, and language-neutral-golden assertions remain. | Removed from `@semio-tech/repo-lib` dev-dependencies. | Removed from that workspace importer only. The package remains because `globby` still owns it transitively. |
| `js:@types/semver@^7.7.1` | No replacement or ambient compatibility declaration was added. The only source hits in the repo-test domain remain string fixtures that test JavaScript import-pattern recognition. | Removed from `@semio-tech/repo-test` dev-dependencies. | Removed from the workspace importer and, because there is no remaining owner, removed as a package row. |

The root retained-command source law now names `checkpoint_binary_matches_schema_fixture_and_owned_oracle`, matching the owned oracle rather than retaining stale `byteorder` terminology.

## Owned discovery hostile fixture

Added `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔍️filesystem/🧫️fixtures/🔣️.json`. It is language-neutral and freezes:

- hidden directory and file discovery;
- lexical UTF-8 byte ordering;
- distinct composed/decomposed Unicode path segments;
- file, directory, and symlink identity;
- no traversal through a directory symlink whose target contains a poison file;
- an absent-root empty census.

## Direct Bun and source evidence

Passed:

```text
bun install --lockfile-only --frozen-lockfile
  1555 packages; lockfile accepted

bun x tsc -p 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/tsconfig.json --noEmit --types node,vite/client
  exit 0

bun test …/📦️packages/🟦️typescript/🧪️index.test.ts --test-name-pattern '(sorts hostile Unicode|closes generator ownership|artifact-example-model-catalog-projection is schema-owned|artifact-editor-command-projection preserves|all physical mutation catalogs|language-agnostic mutation projection golden|inventory bytes are deterministic|accepts the canonical language-neutral manifest|keeps checkout-hostile sentinels virtual)'
  9 pass, 0 fail, 88 expectations

bun test …/📦️packages/🟦️typescript/🧪️index.test.ts --test-name-pattern 'resumes an installed symlink retarget'
  1 pass, 0 fail, 18 expectations

bun test …/📦️packages/🟦️typescript/🧪️index.test.ts --test-name-pattern 'relocates three complete embedded ticket roots'
  1 pass, 0 fail, 21 expectations

bun test …/🧪️tests/🧪️transaction-v2/🟦️.test.ts --test-name-pattern '(keeps the language-neutral golden aligned|journal allocation publishes immutable)'
  1 pass, 0 fail, 5 expectations

git diff --check -- <packet files>
  exit 0
```

The initial repo-test `tsc` invocation with its manifest's narrow `types: ["node"]` reached two unrelated existing `ImportMeta.env`/`ImportMeta.glob` errors in framework UI styling. Re-running the same package resolution with the existing Vite platform declarations (`--types node,vite/client`) passed cleanly after `@types/semver` was absent. No semver declaration or resolution error appeared in either run.

One broader transaction-v2 aggregate probe, `rolls back a process-tree-killed mixed generator and commits ordinal two`, stopped at its pre-existing missing boundary-golden hash before reaching its journal census. It is not used as acceptance evidence for this packet.

## Fresh dependency boundary

Authoritative command:

```text
bun ./📜️script.ts verify dependencies summary --format json
```

| Measure | Scout before | Fresh after | Delta |
| --- | ---: | ---: | ---: |
| Rust raw / literal external | 77 | 76 | -1 |
| JavaScript raw | 71 | 69 | -2 |
| JavaScript literal external after mandated-toolchain correction | 68 | 66 | -2 |
| Total literal external identities | 160 | 157 | -3 |
| Total production-reachable identities | 107 | 107 | 0 |

The fresh full totals are 162 raw identities, 160 third-party identities, two first-party identities, three mandated-toolchain rows, 157 corrected/literal-external identities, and 107 production-reachable identities. There are zero oracle conflicts, zero toolchain conflicts, and zero unauthorized toolchain rows.

## Deferred compiler evidence

No Cargo, Nx, or rustfmt command was run because the coordinator has not granted the compiler lease while Store owns it. The Rust hostile test and fixture differential therefore require the coordinator's later leased Cargo verification. No modifying Git command was used.
