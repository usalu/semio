# First Shared Assembly Synchronization Cut

Read-only source/design receipt. No Rust/Cargo/production edits or native runs. `CURRENT-ASSEMBLY-BARRIER-EPOCHS.md` confirms physical IO, Store and the whole app-declarations native fixture are byte-identical to their preserved 113-source capture; the full fixture includes the currently ignored law. No historical restore of High's current Pack JSON changes is authorized by this plan.

## Exact Existing Barrier and Lock Scope

Store `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` lines 2458–2485 define `ArtifactAssemblyTransaction { _guard: MutexGuard<'static, ()> }`, `ArtifactAssemblyTransactionError::Unavailable`, one OnceLock-owned mutex and synchronous blocking `begin_artifact_assembly`. The guard is private, process-wide, non-Send through std guard ownership; it releases at Drop. Poison maps to Unavailable and never clears. There is no explicit cancellation, progress, timeout, recursion detection or fairness promise. A writer panic while holding the barrier poisons it. Existing async registration/preflight signatures still execute blocking lock acquisition and do not create an asynchronous acquisition operation.

| Lock acquisition path | Current order and release behavior |
|---|---|
| IO composer preflight 1022–1030 / register 1041–1053 | barrier; read/write composer lock; validate full proposal; register inserts only after validation; wrapper keeps barrier until callee returns |
| IO subset preflight 1297–1304 / register 1309–1321 | barrier; read/write subset lock; full validation; insert after validation |
| IO format register 1722–1734 / preflight 1739–1748 | barrier; write/read format catalog; index and full conflict validation before mutation |
| IO native snapshot register 2155–2160 | caller-provided barrier; snapshot write lock; propose all rows then extend. `preflight_native_snapshots` 2163–2166 instead only takes a snapshot read lock, without barrier |
| IO mechanism register 2320–2329 | barrier; build proposed map; mechanism write lock; validate then insert. This separate seventh registry is not part of the aggregate six-registry plan |
| Store document preflight 11477–11484 / register 11494–11510 | barrier; document read/write lock; full descriptor/executable conflict validation before insertion |
| Store migration preflight 11683–11693 / register 11703–11716 | barrier; migration read/write lock; exact pairs/kind/function conflicts before insertion |
| Aggregate IO commit 1943–1968 + Store guard acquisition 11755–11759 | caller owns barrier; document write; migration write; composer write; subset write; format write; snapshot write. Every guard survives all validations and all six registry mutations; Rust drops acquired locals on early Result error and releases barrier in its outer caller |
| Store component-codec registration 11578–11582 | component registry write only, replaces its own schema entry, no assembly barrier. Separate mounted-component ownership operation, not a member of native aggregate atomicity |

Aggregate preflight order is snapshot proposal, composer proposal/conflict, subset conflict, format indexing/conflict, Store document+ migration conflict. Mutation order is composer, subset, format, Store document+migration, snapshot. There is no Result-producing operation after the first mutation. This does not establish panic/OOM rollback or allocation-free commit. Preserve the exact source law rather than describing it as more powerful.

Barrier serializes participating writers. Normal readers generally acquire only their own registry lock. There is no current multi-registry reader snapshot protocol, so a reader performing separate reads across two publication epochs is not guaranteed one atomic snapshot. Aggregate success releases locks only after all its mutations have completed; a failed preflight mutates none.

## Lifetime Proof Gap and Lower API

`ArtifactAssemblyStoreRegistryGuards` at Store 11730 holds private write guards with `'static` lifetime. Its acquisition function accepts an ignored transaction reference and returns guards with no lifetime tied to that reference. Current aggregate correctly keeps the transaction alive, but the public Store API permits releasing it while those guards survive. Fix the lifetime in the real cut: `StoreRegistryGuards<'assembly>` and lower locked batches must borrow the opaque transaction for their entire lifetime, or use a closure-scoped owning batch that cannot escape it. This is an actual type-contract improvement, not a currently passing compile-fail law.

Canonical lower owner proposal: IO assembly synchronization, std-only and free of OS/store vocabulary. Public names `AssemblyTransaction`, `AssemblyUnavailable`, `begin_assembly` (or a controlled operation below) are repo-owned; the private implementation contains the one std mutex guard. Never export MutexGuard or another library's guard/type. Do not use public deref/get-inner APIs, Clone, Send coercions, arbitrary token constructors, a second lock, or cross-product forwarding aliases. Acquire exactly once in upper assembly; lower and Store methods accept a reference to the same canonical transaction.

The literal first ownership move can preserve existing blocking begin behavior and add structural transaction-borrowed registry guard lifetimes. It cannot claim cancellation/progress has thereby been implemented. If acquisition must satisfy the repo's interaction rule in that same feature, define a separate explicit schema-first control contract: a repo-owned `AssemblyAcquire` cursor, bounded `step` result `Pending | Acquired(transaction) | Cancelled | Unavailable`, and caller-observed progress/cancel hooks. Each busy step tries once and returns; it must not poll indefinitely inside an async function or call user progress while holding partial registry write locks. A pending complete-batch acquisition releases partial guards before retry. Cancel before acquire or before first publish releases all acquired locks without mutations. Once all validation and the final cancellation checkpoint succeed, publication executes indivisibly without callbacks or await; cancellation never cuts through six registry insertions.

This controlled contract is proposed new behavior. Current source proves none of those control-extension scenarios. Same-thread recursive acquisition currently blocks; proposed try-step returns Pending instead. Do not silently claim behavioral identity on that point. Poisoned global-lock tests must use isolated process/owner instances; poisoning a shared global in a normal parallel cohort contaminates every later test.

## Schema-First Owned Corpus

`ASSEMBLY-BARRIER-PROPOSED-SCHEMA.json` and `ASSEMBLY-BARRIER-PROPOSED-CORPUS.json` contain 26 finite scenarios, explicitly distinguished as preservation, provider-policy or control-extension. Ownership/order is fixed as barrier → document → migration → composer → subset → format → snapshot. Rows cover two-writer serialization/resume, exact duplicates, conflicts and poison at each aggregate registry, barrier poison, preflight-only no publication, reader blocking, pack-only construction, missing assembled provider, cancellation before acquire and before commit, busy progress, partial-lock release and indivisible commit.

This is a proposal under the ticket, not a production fixture or executed concurrency test. The schema was actually checked with the first-party `validateJsonSchemaSubset` and independent Ajv; both accepted all 26 rows and rejected a foreign-owner property mutation. IDs were checked unique. That verifies fixture closure only. Future native harness must observe actual owned transaction/registry changes and deterministic channel/barrier scheduling; no timer sleep oracle, text-only equivalent model, or pure BTreeMap stand-in proves the synchronization cut. Keep the existing pure `registration_is_all_or_nothing` map law as an original law, and supplement it with real contention tests.

## Exact Ignored Native Law and Enablement

Whole original native fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs`, captured completely. Its line 711 ignore reason is exactly `activate mandatory document capability enforcement after the semantic provider roster is complete`. The next law sets an otherwise real codec's `snapshot_sqlite = None`, commits a plan containing it as document codec, requires `is_err()`, drops the assembly and requires `document_codec(schema).is_none()`. Retain all assertions unchanged when it is enabled.

Current Store `validate_document_codecs` only validates duplicate descriptor/executable identity; IO aggregate does not universally reject a document codec's missing snapshot provider. Its snapshot validation refuses missing capability only when that codec appears in `native_snapshots`. Thus this unchanged law is presently a policy gap, not a lock extraction problem. Simply unignoring it would not install the missing admission rule.

The original capability corpus and `native_document_codec_requires_only_declared_snapshot_capabilities` deliberately support pack-only bare construction. Keep its pack-only and pack-relational rows and every original assertion. `ArtifactCodec::of` explicitly requires ArtifactSqliteSnapshot, while `bare` uses the owner's optional `ArtifactPack::sqlite_snapshot_codec`. Requiring all bare construction to have relational capability would destroy this separate original law.

Enablement requires the real upper assembled-document provider roster: each actual declaration's native document codec, schema/extension/dialect, concrete snapshot type, handwritten domain SQL, TypeId, controlled native decode/encode/preflight, subset policy, retirement and `ArtifactPack::sqlite_snapshot_codec` exposure. Validate actual parents/leaf owners and adapters, not merely a trait impl somewhere. Every assembled document lacking those capabilities needs the actual provider implemented and mounted; then aggregate document admission must reject missing declared capability while holding all locks and before any registry mutation. Bare pack-only construction remains valid outside that mandated assembled-document admission. Keep existing headless atomic refusal, typed bypass/identity, diagnostics, cancellation and all native retirement/encoding/decoding cohorts.

`CURRENT-ASSEMBLY-DOCUMENT-CODEC-CONSTRUCTOR-ROSTER.md` records 258 actual lexical constructor sites in 174 source files with source SHA-256, including tests. It is a complete rg-listed constructor inventory for framework/S/Hub, not the closed actual registration roster: generic constructors, test-only bare specimens, macro-generated contributions and runtime component ownership must be resolved through the actual declaration routes. No missing-provider count is inferred from source names alone. A mere count of `ArtifactSqliteSnapshot` implementations cannot enable the ignored law safely.

## SourceReady and Registered Queue

SourceReady for first barrier ownership extraction: current High Record/JSON source stable, full source snapshots and import inverses admitted, one canonical barrier ownership/mount and no duplicate lock/alias, explicit lower API without product public types, structural guard lifetime, complete participating writer/reader roster, and registered corpus route. No existing Rust mount or native job should precede that authority.

SourceReady for aggregate provider enforcement is stronger: complete actual assembled document capability roster, real controlled owner implementations and unchanged pack-only construction/ignored-law assertions, independent schema/SQLite oracles and actual provider mounting; lower snapshot/IO batch contracts and upper aggregate dependencies must be acyclic. Existing IO is still physically mounted under Kernel, so moving only the barrier cannot establish general Framework product absence.

Actual registered targets inspected: `@semio-tech/framework-rs:test` executes the whole semio-framework Cargo cohort plus Vitest; `test-snapshot-sqlite-io` executes OS kernel integration `sqlite_snapshot_native_admission` plus plugin `sqlite_snapshot_` lib laws; `test-snapshot-sqlite-native` executes the neutral SQLite package whole lib cohort and builds its oracle; `@semio-tech/framework-os-kernel:test-native` executes the whole OS kernel lib with sync,ureq; `test-snapshot-native-admission` first executes its portable ownership/schema cases then the integration test. The IO target is already in launch.json at 25277, snapshot-native-admission at 21713 in the observed epoch. A new isolated assembly owner requires its own registered Nx → existing directory script → launch route before any new command is executable. Original app fixture is included by plugin root at 39290; IO unit/fidelity/mechanism and Store snapshot/native-retirement mounts must stay real.

Queue after root admission: High's real Record/Pack/JSON/native control originals → lower assembly isolated corpus and independent schema/oracle → lifetime compile rejection and actual lower/upper registry contention → complete original IO/Store/plugin cohorts and actual provider roster enforcement → facade/source cut and final physical absence proof. This audit ran only portable schema validation. It reports no native pass and no whole-framework absence.

## Exact Participating Registry Lock Acquisition Census

All explicit read/write acquisition lines in the two current source owners for composer/subset/format/native-snapshot/mechanism/document/component-document/migration registry access. Unrelated store/history/scene locks are outside this shared publication cut.

| Source | Line | Actual Acquisition |
|---|---:|---|
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1029 | `let registry = io_registry().read().map_err(\|_\| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1048 | `let mut reg = io_registry().write().map_err(\|_\| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1067 | `let reg = io_registry().read().map_err(\|_\| IoResolveError { message: "io composer registry unavailable".to_string(), candidates: Vec::new(), unavailable: Some(IoRegistryUnavailable { registry: "io-composer" }) })?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1077 | `let reg = io_registry().read().map_err(\|_\| IoRegistryUnavailable { registry: "io-composer" })?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1089 | `let reg = io_registry().read().map_err(\|_\| IoRegistryUnavailable { registry: "io-composer" })?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1097 | `let reg = io_registry().read().map_err(\|_\| IoRegistryUnavailable { registry: "io-composer" })?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1303 | `let registry = subset_validator_registry().read().map_err(\|_\| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1315 | `let mut reg = subset_validator_registry().write().map_err(\|_\| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1326 | `let registry = subset_validator_registry().read().map_err(\|_\| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1353 | `let reg = subset_validator_registry().read().map_err(\|_\| SubsetValidationError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1729 | `let mut registry = format_catalog().write().map_err(\|_\| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1746 | `let registry = format_catalog().read().map_err(\|_\| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1862 | `let registry = format_catalog().read().map_err(\|_\| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1874 | `let registry = format_catalog().read().map_err(\|_\| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1945 | `let mut composers = io_registry().write().map_err(\|_\| ArtifactAssemblyRegistryError::Composer(Box::new(IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1946 | `let mut subset_validators = subset_validator_registry().write().map_err(\|_\| ArtifactAssemblyRegistryError::SubsetValidator(SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" })))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1947 | `let mut formats = format_catalog().write().map_err(\|_\| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" })))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1948 | `let mut native_snapshots = io_mechanism::native_snapshot_registry().write().map_err(\|_\| ArtifactAssemblyRegistryError::NativeSnapshot(io_mechanism::IoRegistryError::Unavailable))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 1974 | `let reg = format_catalog().read().map_err(\|_\| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2156 | `let mut registry = native_snapshot_registry().write().map_err(\|_\| IoRegistryError::Unavailable)?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2164 | `let registry = native_snapshot_registry().read().map_err(\|_\| IoRegistryError::Unavailable)?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2170 | `let registry = native_snapshot_registry().read().map_err(\|_\| IoRegistryError::Unavailable)?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2323 | `let mut registry = io_mechanism_registry().write().map_err(\|_\| IoRegistryError::Unavailable)?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2414 | `let snapshots = native_snapshot_registry().read().map_err(\|_\| IoError { message: "native snapshot registry unavailable".to_string(), diagnostics: Vec::new() })?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2419 | `let registry = io_mechanism_registry().read().map_err(\|_\| IoError { message: "io mechanism registry unavailable".to_string(), diagnostics: Vec::new() })?.clone();` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2424 | `let codec = native_snapshot_registry().read().map_err(\|_\| IoError::from("native snapshot registry unavailable".to_string()))?.get(dialect).cloned().ok_or_else(\|\| IoError::from(format!("unregistered typed snapshot dialect {}", dialect.to_coordinate())))?;` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2531 | `let codec = native_snapshot_registry().read().map_err(\|_\| IoError { message: "native snapshot registry unavailable".to_string(), diagnostics: Vec::new() })?.get(native).cloned();` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2538 | `let registry = io_mechanism_registry().read().map_err(\|_\| IoError { message: "io mechanism registry unavailable".to_string(), diagnostics: Vec::new() })?.clone();` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2576 | `let registry = io_mechanism_registry().read().map_err(\|_\| IoError { message: "io mechanism registry unavailable".to_string(), diagnostics: Vec::new() })?.clone();` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2585 | `match io_mechanism_registry().read() {` |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2591 | `if let Ok(snapshots) = native_snapshot_registry().read() {` |

Current source SHA-256 for `🧰️framework/🔨️modules/🚪️io/🦀️.rs`: `99c58efb8292c39c17a68c2c5b827479d753400b56a83bdfd8ce3401cfe6000e`.

| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11482 | `let registry = document_codec_registry().read().map_err(\|_\| DocumentCodecRegistryError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11503 | `let mut registry = document_codec_registry().write().map_err(\|_\| DocumentCodecRegistryError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11516 | `let registry = document_codec_registry().read().map_err(\|_\| DocumentCodecRegistryError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11579 | `let mut registry = component_document_codec_registry().write().map_err(\|_\| DocumentCodecRegistryError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11590 | `let registry = component_document_codec_registry().read().map_err(\|_\| DocumentCodecRegistryError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11691 | `let registry = dialect_migration_registry().read().map_err(\|_\| DialectMigrationRegistryError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11711 | `let mut registry = dialect_migration_registry().write().map_err(\|_\| DialectMigrationRegistryError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11724 | `let registry = dialect_migration_registry().read().map_err(\|_\| DialectMigrationError::Unavailable)?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11756 | `let document_codecs = document_codec_registry().write().map_err(\|_\| ArtifactAssemblyStoreRegistryError::DocumentCodec(DocumentCodecRegistryError::Unavailable))?;` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | 11757 | `let dialect_migrations = dialect_migration_registry().write().map_err(\|_\| ArtifactAssemblyStoreRegistryError::DialectMigration(DialectMigrationRegistryError::Unavailable))?;` |

Current source SHA-256 for `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: `823f795f9f4eb30914d533202948c052821fedeb421539330bbe4269b21f7da7`.

