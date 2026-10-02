# Stdio Contribution Contract Actual Authority

Read-only current source; no jobs. Correction to prior bounded offender report: the package name semio-s-artifact-stdio-contract does not reflect actual artifact ownership. Its physical manifest is stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml and metadata role is s-module. Root S workspace215 points there. It is physically nested under plugin, but is not an artifact directory. Therefore package-name-based plugin→artifact violation was overstated; actual role/path policy must decide direction. Earlier report is superseded on that point.

## Actual API closure

Contract Rust101 ArtifactAssembly holds framework-plugin ArtifactDefinition or boxed ArtifactDeclaration; definition returns borrowed definition. Receipt140 has plugin/package/version/factory/descriptor/runtime-capability/artifact-kind/schema/extension strings, hash[32], and factory fn()->kernel::ArtifactCodec. instantiate verifies identity fields, schema/extension/hash equality and nonzero hash before returning codec. NativeCodecFactory131 holds id/artifact, kind fn()->ArtifactKindSpec and codec callback. ArtifactContribution174 holds identity/schema/optional constraint and definition/assembly/formats/native-codecs callbacks. Registry sibling65 stores Vec<ArtifactContribution>, validates roster cap4096, identities/directories/extensions/MIME/dialects/capabilities/dependencies, atomically register/remove, and checks callback identity before returning assemblies.

These types are catalog-independent and already admit caller-authored contributions; selected real artifact callbacks must remain higher. Stdio plugin Rust25 accepts &ContributionRegistry plus AssemblyOwner; it does not author a universal artifact catalog. Contract root18 mounts registry sibling and exports editing/part21/definition-hierarchy, including large unrelated UI and payload services. Extracting only three structs would strand validation/helpers; move coherent contribution authority closure, not a forwarding shim.

## Recommended canonical cut

A real lower S domain owner such as `✏️s/🔨️modules/🗄️stdio/📇️registry/🧬️contract` must first be admitted explicitly by taxonomy (no invented catchall). Move shared contribution structs, registry implementation, schema_summary/capability_ledger/native_codec_factory_receipts and their actual helper types as one source-owned module/package. Name it semio-s-stdio-contract, not artifact; rebind all import and Cargo/root workspace entries directly, with no alias/reexport old crate. Separate catalog-independent editing/Part21 into their own coherent leaves rather than retaining kernel/pack facade as a generic API dependency. ArtifactCodec and mutation contracts currently depend on OS kernel; a fully domain-neutral S service needs canonical owned lower contracts, not kernel forwarding.

Existing schema authority stdio/📇registry/🧬schema/🔣json and contributions schema must move with registry; scope s.stdio.registry remains domain identity unless schema-first contract intentionally changes it. Specific artifact schema/callbacks remain at each artifact; higher selected composition root receives the roster. Contract composition tests already model explicit selection/removal and parent-removal; preserve these rather than create a fixed lower catalog. Deleting any artifact must leave plugin/contract types intact; missing selected dependencies must refuse, while unselected removed artifacts must be irrelevant.

## Original proofs/routes

Existing permanent contract router7 uses runArtifactRustPackageMain and three portable twins: definition-hierarchy, contribution-removal, composition-selection. This artifact-oriented runner is itself classification debt for s-module. Preserve complete native collection and explicit details_arena_headroom/table_arena_headroom manifest targets. Contributions Rust tests24/64/82/117 own four original laws: independent serde dependency oracle, registration/output failure closure, registration/receipt vectors, and definition constraints. Fixtures include alpha/bravo/charlie + MIME/directory/extension conflicts and authored constraint; composition owns selection/native-export/parent-removal schemas. Rebind actual route/package/project/launch registration together; do not infer their current passing state here.

## Exact symbol occurrence roster

This is literal actual symbol occurrence evidence, not an exhaustive semantic alias-resolution claim. It includes all current source matches found outside generated owners:

- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1026:pub async fn preflight_composer_entry_refs_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1045:pub fn register_composer_entry_refs_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1301:pub async fn preflight_subset_validators_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1313:pub fn register_subset_validators_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1726:pub fn register_format_descriptors_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(), FormatRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1743:pub fn preflight_format_descriptors_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, rows: &[FormatDescriptor]) -> Result<(), FormatRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1884:pub struct ArtifactAssemblyRegistryPlan {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1893:impl ArtifactAssemblyRegistryPlan {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1902:pub enum ArtifactAssemblyRegistryError {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1903:    Transaction(store::ArtifactAssemblyTransactionError),`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1907:    Store(Box<store::ArtifactAssemblyStoreRegistryError>),`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1911:impl std::fmt::Display for ArtifactAssemblyRegistryError {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1924:impl std::error::Error for ArtifactAssemblyRegistryError {}`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1927:pub fn register_native_snapshot_codec(dialect: Dialect, codec: store::ArtifactCodec) -> Result<(), ArtifactAssemblyRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1928:    let assembly = store::begin_artifact_assembly().map_err(ArtifactAssemblyRegistryError::Transaction)?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1929:    let plan = ArtifactAssemblyRegistryPlan { document_codecs: vec![codec.clone()], native_snapshots: vec![io_mechanism::NativeSnapshotRegistration { dialect: dialect.into(), codec }], ..Default::default() };`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1934:pub fn register_native_document_codec(dialect: Dialect, codec: store::ArtifactCodec) -> Result<(), ArtifactAssemblyRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1935:    let assembly = store::begin_artifact_assembly().map_err(ArtifactAssemblyRegistryError::Transaction)?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1936:    let plan = ArtifactAssemblyRegistryPlan { document_codecs: vec![codec.clone()], native_snapshots: io_mechanism::NativeSnapshotRegistration::from_capability(dialect.into(), codec).into_iter().collect(), ..Default::default() };`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1942:pub fn commit_artifact_assembly_registry_plan(assembly: &store::ArtifactAssemblyTransaction, plan: ArtifactAssemblyRegistryPlan) -> Result<(), ArtifactAssemblyRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1943:    let mut store_guards = store::acquire_artifact_assembly_store_registry_guards(assembly).map_err(|error| ArtifactAssemblyRegistryError::Store(Box::new(error)))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1944:    let mut composers = io_registry().write().map_err(|_| ArtifactAssemblyRegistryError::Composer(Box::new(IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1945:    let mut subset_validators = subset_validator_registry().write().map_err(|_| ArtifactAssemblyRegistryError::SubsetValidator(SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" })))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1946:    let mut formats = format_catalog().write().map_err(|_| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" })))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1947:    let mut native_snapshots = io_mechanism::native_snapshot_registry().write().map_err(|_| ArtifactAssemblyRegistryError::NativeSnapshot(io_mechanism::IoRegistryError::Unavailable))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1948:    let proposed_snapshots = io_mechanism::propose_native_snapshots(&native_snapshots, &plan.native_snapshots).map_err(ArtifactAssemblyRegistryError::NativeSnapshot)?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1949:    let proposed_composers = composer_entries_by_key(plan.composer_entries.iter().copied()).map_err(|error| ArtifactAssemblyRegistryError::Composer(Box::new(error)))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1950:    validate_composer_entries(&composers, &proposed_composers).map_err(|error| ArtifactAssemblyRegistryError::Composer(Box::new(error)))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1951:    validate_subset_validators(&subset_validators, &plan.subset_validators).map_err(ArtifactAssemblyRegistryError::SubsetValidator)?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1952:    let (proposed_formats, proposed_formats_by_kind) = index_format_descriptors(plan.format_descriptors.iter().cloned()).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1953:    validate_format_descriptors(&formats, &proposed_formats, &proposed_formats_by_kind).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:1954:    store::preflight_artifact_assembly_store_registry_guards(&store_guards, &plan.document_codecs, &plan.dialect_migrations).map_err(|error| ArtifactAssemblyRegistryError::Store(Box::new(error)))?;`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:2154:    pub fn register_native_snapshots_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, registrations: &[NativeSnapshotRegistration]) -> Result<(), IoRegistryError> {`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs:2609:    /// this file already depends on store:: throughout (e.g. ArtifactAssemblyRegistryPlan`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:3138:    semio_framework::io::commit_artifact_assembly_registry_plan(&assembly, semio_framework::io::ArtifactAssemblyRegistryPlan { document_codecs: vec![codec.clone()], native_snapshots: vec![semio_framework::io::io_mechanism::NativeSnapshotRegistration { dialect: dialect.clone(), codec }], ..Default::default() }).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("registering a guest-backed semantic codec for {artifact_schema}: {error}")))?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🦀️.rs:76:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🦀️.rs:58:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🦀️.rs:91:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2456://#region 🔖️ArtifactAssembly`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2458:pub struct ArtifactAssemblyTransaction {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2464:pub enum ArtifactAssemblyTransactionError {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2468:impl std::fmt::Display for ArtifactAssemblyTransactionError {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2474:impl std::error::Error for ArtifactAssemblyTransactionError {}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2483:pub fn begin_artifact_assembly() -> Result<ArtifactAssemblyTransaction, ArtifactAssemblyTransactionError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2484:    Ok(ArtifactAssemblyTransaction { _guard: artifact_assembly_lock().lock().map_err(|_| ArtifactAssemblyTransactionError::Unavailable)? })`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2486://#endregion 🔖️ArtifactAssembly`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11454:pub fn preflight_document_codecs_in_assembly(_assembly: &ArtifactAssemblyTransaction, codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11475:pub fn register_document_codecs_in_assembly(_assembly: &ArtifactAssemblyTransaction, codecs: Vec<ArtifactCodec>) -> Result<(), DocumentCodecRegistryError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11663:pub async fn preflight_dialect_migrations_in_assembly(_assembly: &ArtifactAssemblyTransaction, migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11683:pub fn register_dialect_migrations_in_assembly(_assembly: &ArtifactAssemblyTransaction, migrations: Vec<DialectMigration>) -> Result<(), DialectMigrationRegistryError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11703:pub struct ArtifactAssemblyStoreRegistryGuards {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11710:pub enum ArtifactAssemblyStoreRegistryError {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11715:impl std::fmt::Display for ArtifactAssemblyStoreRegistryError {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11724:impl std::error::Error for ArtifactAssemblyStoreRegistryError {}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11728:pub fn acquire_artifact_assembly_store_registry_guards(_assembly: &ArtifactAssemblyTransaction) -> Result<ArtifactAssemblyStoreRegistryGuards, ArtifactAssemblyStoreRegistryError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11729:    let document_codecs = document_codec_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DocumentCodec(DocumentCodecRegistryError::Unavailable))?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11730:    let dialect_migrations = dialect_migration_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DialectMigration(DialectMigrationRegistryError::Unavailable))?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11731:    Ok(ArtifactAssemblyStoreRegistryGuards { document_codecs, dialect_migrations })`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11736:pub fn preflight_artifact_assembly_store_registry_guards(guards: &ArtifactAssemblyStoreRegistryGuards, document_codecs: &[ArtifactCodec], dialect_migrations: &[DialectMigration]) -> Result<(), ArtifactAssemblyStoreRegistryError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11737:    validate_document_codecs(&guards.document_codecs, document_codecs).map_err(ArtifactAssemblyStoreRegistryError::DocumentCodec)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11738:    validate_dialect_migrations(&guards.dialect_migrations, dialect_migrations).map_err(ArtifactAssemblyStoreRegistryError::DialectMigration)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11742:pub fn commit_artifact_assembly_store_registry_guards(guards: &mut ArtifactAssemblyStoreRegistryGuards, document_codecs: Vec<ArtifactCodec>, dialect_migrations: Vec<DialectMigration>) {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🦀️.rs:80:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🦀️.rs:97:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs:93:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️.rs:101:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🦀️.rs:58:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🦀️.rs:59:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs:105:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🦀️.rs:69:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🦀️.rs:52:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🦀️.rs:87:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:596:        semio_framework::io::commit_artifact_assembly_registry_plan(&assembly, semio_framework::io::ArtifactAssemblyRegistryPlan { document_codecs: vec![codec.clone()], native_snapshots: vec![NativeSnapshotRegistration { dialect: TYPED.into(), codec }], ..Default::default() }).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:620:        let schema="semio.testkit.missing-sqlite-owner/v1";let mut codec=store::ArtifactCodec::of::<Std1AnySnapshot,Std1AnyMutation>(schema);codec.snapshot_sqlite=None;let mut plan=semio_framework::io::ArtifactAssemblyRegistryPlan::new().await;plan.document_codecs.push(codec);let assembly=store::begin_artifact_assembly().unwrap();assert!(semio_framework::io::commit_artifact_assembly_registry_plan(&assembly,plan).is_err());drop(assembly);assert!(store::document_codec(schema).await.unwrap().is_none());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:625:        use semio_framework::io::{ArtifactAssemblyRegistryPlan, commit_artifact_assembly_registry_plan};`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:631:        let mut plan = ArtifactAssemblyRegistryPlan::new().await;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:645:        let mut rejected = ArtifactAssemblyRegistryPlan::new().await;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:4211:        pub(crate) fn into_runtime(self, definitions: ArtifactDefinitionRegistry) -> Result<(PluginRuntimeRegistry, semio_framework::io::ArtifactAssemblyRegistryPlan), PluginAssemblyError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:4237:                semio_framework::io::ArtifactAssemblyRegistryPlan { composer_entries: composers, subset_validators, format_descriptors: formats, document_codecs, dialect_migrations: migrations, native_snapshots },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:4374:    pub(crate) fn commit_artifact_registration_plan(assembly: &store::ArtifactAssemblyTransaction, plan: semio_framework::io::ArtifactAssemblyRegistryPlan) -> Result<(), PluginAssemblyError> {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:39196:                let plan = semio_framework::io::ArtifactAssemblyRegistryPlan { document_codecs: codecs, format_descriptors: format_rows, native_snapshots, ..Default::default() };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:42073:        let plan = semio_framework::io::ArtifactAssemblyRegistryPlan {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🦀️.rs:60:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🦀️.rs:67:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🦀️.rs:102:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🦀️.rs:78:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🦀️.rs:76:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🦀️.rs:96:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🦀️.rs:70:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🦀️.rs:106:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🦀️.rs:96:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🦀️.rs:104:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs:100:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🦀️.rs:84:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🦀️.rs:73:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:3:use crate::{capability_ledger, native_codec_factory_receipts, schema_summary, ArtifactAssembly, ArtifactContribution, CapabilityLedger, NativeCodecFactory, NativeCodecFactoryReceipt};`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:65:pub struct ContributionRegistry {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:69:impl ContributionRegistry {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:125:    pub fn artifact_assemblies(&self) -> Result<Vec<ArtifactAssembly>, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:160:    pub fn native_codec_factory_receipts(&self, plugin_id: &'static str, package_id: &str, package_version: &'static str) -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:101:pub enum ArtifactAssembly {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:106:impl ArtifactAssembly {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:140:pub struct NativeCodecFactoryReceipt {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:154:impl NativeCodecFactoryReceipt {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:179:    pub assembly: fn() -> Result<ArtifactAssembly, PluginAssemblyError>,`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:781:pub fn runtime_assembly(artifact: &'static str, definition: ArtifactDefinition, declaration: fn(ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError>) -> Result<ArtifactAssembly, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:786:    declaration(definition).map(|declaration| ArtifactAssembly::Runtime(Box::new(declaration))).map_err(PluginAssemblyError::definition)`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:790:pub fn definition_only_assembly(artifact: &'static str, definition: ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:794:    Ok(ArtifactAssembly::Definition(definition))`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:947:pub fn native_codec_factory_receipts(contribution: &ArtifactContribution, plugin_id: &'static str, package_id: impl Into<String>, package_version: &'static str) -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:967:        let receipt = NativeCodecFactoryReceipt {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:2:use crate::registry::ContributionRegistry;`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:12:    fn assembly(schema: &'static str) -> Result<ArtifactAssembly, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:13:        definition(schema).map(ArtifactAssembly::Definition)`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:41:        let mut registry = ContributionRegistry::new(selected.clone()).unwrap();`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:66:    let mut registry = ContributionRegistry::new(vec![contribution("alpha")]).unwrap();`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:76:    let registry = ContributionRegistry::new(vec![foreign]).unwrap();`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:94:        let mut registry = ContributionRegistry::new(vec![first]).unwrap();`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:108:        let registry = ContributionRegistry::new(vec![selected]).unwrap();`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/📇️contributions/🦀️.rs:133:        let result = ContributionRegistry::new(vec![selected]);`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/🔬️unit/🦀️.rs:26:    let bytes = size_of::<ArtifactAssembly>();`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:5:use semio_s_artifact_stdio_contract::registry::ContributionRegistry;`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:6:use semio_s_artifact_stdio_contract::{ArtifactAssembly, NativeCodecFactoryReceipt};`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:18:    assemblies: Vec<ArtifactAssembly>,`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:19:    receipts: Vec<NativeCodecFactoryReceipt>,`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:25:    pub fn new(registry: &ContributionRegistry, owner: AssemblyOwner<'_>) -> Result<Self, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:29:            ArtifactAssembly::Runtime(declaration) => Some(declaration.definition().identity().as_str()),`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:30:            ArtifactAssembly::Definition(_) => None,`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:47:    pub fn assemblies(&self) -> &[ArtifactAssembly] {`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:52:    pub fn receipts(&self) -> &[NativeCodecFactoryReceipt] {`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:60:                ArtifactAssembly::Definition(definition) => builder.artifact_definition(definition),`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:61:                ArtifactAssembly::Runtime(declaration) => builder.artifact(*declaration),`
- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:72:pub fn assemble<PA: PluginApp>(builder: PluginBuilder<Ready, PA>, registry: &ContributionRegistry, owner: AssemblyOwner<'_>) -> Result<PluginBuilder<Ready, PA>, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️assembly/🦀️.rs:1:use semio_s_artifact_stdio_contract::{ArtifactAssembly, ArtifactContribution};`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️assembly/🦀️.rs:2:use semio_s_artifact_stdio_contract::registry::ContributionRegistry;`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️assembly/🦀️.rs:13:    fn assembly(schema: &'static str) -> Result<ArtifactAssembly, PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️assembly/🦀️.rs:14:        definition(schema).map(ArtifactAssembly::Definition)`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️assembly/🦀️.rs:30:        let mut registry = ContributionRegistry::new(selected).unwrap();`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️assembly/🦀️.rs:37:        let actual = plan.assemblies().iter().map(|assembly| match assembly { ArtifactAssembly::Definition(definition) => definition.identity().as_str().strip_prefix("s.stdio.").unwrap(), ArtifactAssembly::Runtime(_) => unreachable!() }).collect::<Vec<_>>();`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🦀️.rs:71:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🦀️.rs:74:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🦀️.rs:76:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:53:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs:168:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🦀️.rs:76:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🦀️.rs:109:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🦀️.rs:70:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs:78:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🦀️.rs:109:pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {`

## Current policy decision

Cargo direction provider122–140 selects physical owner classification and semantic role, not package-name prefix. Current taxonomy29399 onward prohibits s-module→plugin/extension; moving this contract to S modules while retaining its framework-plugin dependency must be checked against actual target role (framework vs plugin), not guessed from crate name. No package rename requirement was established by these actual policy selectors: canonical rename above is an architectural naming recommendation, not a proved current admission failure. Parent requested preserving this distinction.
