# Result value-repoint: `semio-framework-value` facade compat re-exports removed, importers re-pointed to the owning crates

Executor `value-repoint` (Sonnet 5.5, foreground, no sub-agents, no modifying git command). Tools in `value-repoint-tools/`; run state, backups (first-touch copy of every rewritten file) and logs under `🗑️generated/value-repoint*`.

## 1. What was removed from the facade

The facade lines added by rg-core (and the single-item siblings that predate them) are deleted from `🧰️framework/🔨️modules/🌱️value/🦀️.rs` and `…/📦️packages/🦀️rust/🦀️.rs`:

| owner crate | modules removed from the facade | flat item re-exports removed |
| --- | --- | --- |
| `semio-framework-value-registry` | `fixed_key_slots`, `fixed_request_owners`, `exact_key_lookup` | 17 + `ExactKeyCursor` |
| `semio-framework-value-persistent` | `list`, `original_vector_concat`, `slice_equality` | `OriginalVectorConcat`, `RetainedEqualityElement`, `RetainedSliceEquality`, `RetainedEqualityOwner` |
| `semio-framework-value-edit` | `sort`, `indexed_edit`, `indexed_collection_commit`, `named_edit` | 3 + 8 + `IndexedCollectionCommit` + 16 |
| `semio-framework-value-schema` | `graphql_int64`, `intrinsic_json`, fn `register_value_schema_exports` | none |
| `semio-framework-value-native-owned` | (see exception) | 11 `Native*` names, macro `native_flat_record_receive!` |

Table = `value-repoint-tools/moved.json` (66 flat items, 14 modules, 2 functions; snapshot of the facade lines before deletion).

### Exception kept on purpose (derive-expansion contract, flagged for the coordinator)
`#[derive(NativeReceive)]` and `#[derive(RetainedEquality)]` expand to `#value::native_decoding::…`, `#value::NativeDecodeControl`, `#value::retained_equality::equality_box`, `#v::RetainedEquality{,Cursor,Step,Fields}`, `#v::RetainedRecordEquality` where `#value` is the consumer's `semio_framework_value` path (proc-macro output cannot know which owner crates a consumer depends on). The facade therefore keeps exactly those names (modules `native_decoding`, `retained_equality` plus `NativeDecodeControl`, `NativeReceive`, `RetainedEquality`, `RetainedEqualityCursor`, `RetainedEqualityFields`, `RetainedEqualityStep`, `RetainedRecordEquality`), each with a `🧬` docstring naming the reason. Importers were nevertheless re-pointed to the owners (ruling), so the facade lines are used by derive output only. Removing them needs the derive expansion crates to emit owner paths behind a configurable root (value-fix/derive decision).

## 2. Codemod (`value-repoint-tools/`)

- `lex.ts` Rust lexer (comments, nested block comments, raw/byte strings, char vs lifetime, escapes) that never loses a byte. `transform.ts` = fail-closed rewriter: use-tree parser/flattener (groups, `self`, aliases, globs, `pub`/`pub(crate)`/attributes copied onto the new statement), non-`use` paths (types, expressions, macro token trees, macro invocation arguments), doc comments, `value::` prefix, alias crates (`pack::value::…`, `protocol::value::…`, `replication::value::…`), glob re-exporters (`protocol::…`), root-glob importers (explicit imports are added for names actually used), derive-name collision (`NativeReceive`/`RetainedEquality` exist as derive macro and trait: the facade import stays when the file derives, the trait import moves when the file implements it). Strings are never edited (noted only).
- Every rewrite is gated by `resolve-owner.ts`: the path is resolved from outside the owner crate (pub only) through the owner's module tree (the `rs-sweep-tools` resolver, globs and `pub use` followed). `missing`/`unknown` verdicts are not rewritten, only listed.
- `manifests.ts`: adds the owner dependency right after the facade dependency line in the same section, `{ workspace = true }` / dotted `.workspace = true` when the manifest inherits, relative `path` otherwise; the workspace declaration (`[workspace.dependencies.X]` table form or inline) is added to the workspace root when it does not declare the crate yet (49 declarations in 8 workspace roots incl. repo root and `🧰️framework`). `filemap.ts` maps every source file to the packages whose module trees mount it (so `#[path]`-mounted files get the right manifest).
- `internal.ts` re-points `crate::<moved>` inside the facade crate's own modules (10 files). `deps-from-ledger.ts` re-derives missing dependencies from the backups. `alias-sites.ts`, `caps.ts`, `audit-lex.ts` = audits.
- Test: `transform.test.ts` + `cases.json` (language-agnostic input/expected fixtures, 19 cases) and a third-party cross-check (every expected output must parse under `rustfmt --check` with `skip_children`).

## 3. Result

- Importers outside the value family: 1,009 files / 2,377 rewrites in 450 packages (first apply), +21 alias/glob sites, +4 later-found names (`OriginalVectorConcat`, `ExactKeyCursor`, `IndexedCollectionCommit`, macro `native_flat_record_receive!`); total 1,025 files touched by the tool (ledger `🗑️generated/value-repoint/backup/ledger.json`).
- Value family files that imported moved names (value-derive tests/equality, value-tests-2, dictionary, snapshot-read, 10 facade-crate `crate::` sites): 13 + 10 files, family manifests got their dependencies. These are inside value-fix's area; the edits are mechanical path swaps only.
- Manifests: about 456 edited (+49 workspace declarations). Every edited TOML parses (3,565 manifests re-parsed, 0 errors). Strings: 0 changed (`audit-lex.ts` compares all string tokens before/after for the 1,015 backed-up files).
- Hand edits: three macros built `semio_framework_value::$module::…` from an ident (`native_decoding`/`native_encoding`) in the stdio csv/tsv sqlite snapshot tests and the space native-caller; they now take `$root:ident,$module:ident`; the tsv receiving macro likewise.
- Law fixture: `crate-boundaries/🧬️cuts.json` row `semio-framework-value-persistent` now names the remaining facade line (`pub use semio_framework_value_persistent::retained_equality;`, definition `pub mod retained_equality;`).

## 4. Verification

- `bun test transform.test.ts` (value-repoint-tools): 19 pass, 0 fail.
- Static resolution: `bun rs-sweep-tools/main.ts --all --tag after2` (3,420 packages, 2,657,933 paths resolved): after the last codemod run no finding mentions a moved facade name or the five owner crates except the items in section 5 (unresolved list). The `dep-wrong-workspace` findings that mention the new crates (144) are the same checker rule that flags the facade itself 389 times (path dependencies into a workspace member), not new.
- `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts`: 125 pass, 2 fail; neither is caused by this task (see 5).
- Idempotence: a re-run of the codemod over all 19,812 files that mention the facade finds 0 rewrites.
- `cargo check` sample: NOT run to completion. A 10-crate `bun repo cargo … check --manifest-path 🧰️framework/Cargo.toml -p <10 rewritten crates> --lib --tests` (covering every owner-crate combination) was queued, then failed in `cargo fetch --locked`: `Cargo.lock` of the framework workspace needs the lock refresh for the new path dependencies (open item 6). The coordinator then paused all `bun repo cargo`/`run` until the dependency sync exits 0, and I stopped. Run after lockref: `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-time-travel -p semio-framework-tool-run-tick-wire -p semio-framework-pack-record-preflight -p semio-framework-pack-record-retained -p semio-framework-plugin-decl-views -p semio-framework-plugin-decl-registry -p semio-framework-pack -p semio-framework-editor -p semio-framework-math-rank -p semio-framework-diagnostic --lib --tests`.

## 5. Open items / unresolved list (listed, not guessed)

1. Value family compile status is value-fix's. `semio_framework_value::native_flat_record_receive!` is now `semio_framework_value_native_owned::native_flat_record_receive!` for importers; the two family test sites that call it with `crate::` (`value-edit` named index root receive test) still need the `native-owned` dev-dependency/path by value-fix. Pre-existing findings in the same sweep, not touched: `semio_framework_value::{FactorySharedRetirement, FactoryIssuerIdentity, SnapshotRetirementStep}` and `retirement::arc_custody::sealed::SealedShared` (7 sites), `crate::record::…` in the pack encode/format tests.
2. `semio-framework-replication` keeps `pub use semio_framework_value::*` (glob re-export) and 11 crates keep `pub use semio_framework_value as value|__value`; no consumer path of a removed name was found (checked lexically and by the sweep), but any new consumer must import from the owner crates.
3. Pre-existing failures in the crate-boundaries law, not from this change: `semio-framework-mutation-diff` forbids `semio-framework-value` but its manifest now carries `semio-framework-value = { workspace = true }` (staged line, not mine); `semio-framework-os-store-one-item-batch` test closure is 335,474 bytes (1.28 of the cap; this task added 1,497 bytes to it).
4. Packages that use the facade without declaring it (pre-existing, rewritten files inside them got no owner dependency because the facade is not a dependency either): `semio-s-artifact-stdio-xlsx` (and its test targets), `semio-s-artifact-stdio-pptx` test target, `semio-framework-os-mcp-capabilities` tests, `semio-framework-pack-value-intrinsic-media` tests template.
5. 13 rewritten files belong to no package tree (dead or macro-mounted sources): fem `📍node/🧪️tests`, bim `snapshot` cells/owner, cad `snapshot/🚦️owner`, trinity `🧵️capture`, pack-json `🔒️immutable/🧪️tests`, dsl-schema `producer/tests` and `📨️presence`, sqlite-snapshot `🫙️prefix`, store `🪆️child/🧵️owner`, plugin `child-operations` publication/registry. Their dependencies are not verified.
6. Every Cargo.lock of the 454 touched packages' workspaces needs a refresh (lockref executor): new path dependencies on `value-persistent`, `value-edit`, `value-registry`, `value-schema`, `value-native-owned`.
7. Crate source gate: path swaps lengthen import lines (about +11 bytes per `use`); only the one-item-batch closure was measured (+1,497 bytes). Re-measure crates at ≥0.97 (value-retained-clone 0.969, os-store-replay 0.981) after the family compiles.
8. Reproduce/extend: `bun value-repoint-tools/main.ts [--family] [--apply]`, `bun value-repoint-tools/internal.ts --apply`, `bun value-repoint-tools/deps-from-ledger.ts --apply`.
