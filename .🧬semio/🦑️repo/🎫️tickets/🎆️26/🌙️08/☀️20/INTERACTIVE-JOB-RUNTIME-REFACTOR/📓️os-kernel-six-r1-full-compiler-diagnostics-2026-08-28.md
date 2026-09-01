# OS-Kernel Six-Law R1 — Full Rendered Compiler Diagnostics

Actual retained fingerprint JSON copied without rerunning Cargo. Original and ticket JSONL copy are both818114bytes/SHA256654962ed8040bcc4fb3f693e5c827faca180e2f4a332f3532aa900476140f16e. Rendering below removes ANSI color escapes only; the original JSON retains them, spans, children and expansion details.

[Exact JSONL](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-compiler-diagnostics-2026-08-28.jsonl)

There are92 source error diagnostics,66 warnings, one abort summary at error level and two failure notes:161 JSON records total. No test executed.

```text
error[E0432]: unresolved import `semio_framework_async::TokioHostRuntime`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/📇️directory/🔌️client/🦀️component.rs:481:94
    |
481 |     use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle, TokioHostRuntime};
    |                                                                                              ^^^^^^^^^^^^^^^^ no `TokioHostRuntime` in the root
    |
    = help: consider importing this struct instead:
            semio_framework_os_services::TokioHostRuntime


error[E0603]: function `fixture_runner_handle` is private
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4169:60
     |
4169 |         let runner = native_actor::retained_turn_fixtures::fixture_runner_handle(host.pool.clone(), 1, cmd_rx.close_handle());
     |                                                            ^^^^^^^^^^^^^^^^^^^^^ private function
     |
note: the function `fixture_runner_handle` is defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2538:9
     |
2538 | ...   pub(super) fn fixture_runner_handle(pool: Arc<semio_framework_async::WorkerPool>, generation: u64, mailbox: ArtifactMailboxClose) -> ArtifactActorRunnerHandle {
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:1765:13
     |
1765 |     marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
     |
1765 -     marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
1765 +     marker: PhantomData<fn() -> (P, Mutation)>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:1770:119
     |
1770 | ...active: std::mem::ManuallyDrop::new(None), marker: std::marker::PhantomData }
     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1770 -         Self { phase: ArtifactStoreCursorDisposerPhase::Displaced, active: std::mem::ManuallyDrop::new(None), marker: std::marker::PhantomData }
1770 +         Self { phase: ArtifactStoreCursorDisposerPhase::Displaced, active: std::mem::ManuallyDrop::new(None), marker: PhantomData }
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2267:16
     |
2267 |     pub lanes: std::collections::BTreeMap<String, HistoryLane>,
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
2267 -     pub lanes: std::collections::BTreeMap<String, HistoryLane>,
2267 +     pub lanes: BTreeMap<String, HistoryLane>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2292:16
     |
2292 |     lanes: &'a std::collections::BTreeMap<String, HistoryLane>,
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
2292 -     lanes: &'a std::collections::BTreeMap<String, HistoryLane>,
2292 +     lanes: &'a BTreeMap<String, HistoryLane>,
     |


warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️component.rs:167:43
    |
167 | impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
    |                                           ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
167 - impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
167 + impl<P: Clone + Send + Sync + 'static, M: Mutation<P>> PresenceStore<P, M> {
    |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5905:49
     |
5905 | struct ArtifactRepositoryHistoryEntryDecoder<T>(std::marker::PhantomData<T>);
     |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5905 - struct ArtifactRepositoryHistoryEntryDecoder<T>(std::marker::PhantomData<T>);
5905 + struct ArtifactRepositoryHistoryEntryDecoder<T>(PhantomData<T>);
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5909:14
     |
5909 |         Self(std::marker::PhantomData)
     |              ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5909 -         Self(std::marker::PhantomData)
5909 +         Self(PhantomData)
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5994:14
     |
5994 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5994 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
5994 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5995:23
     |
5995 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5995 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
5995 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6013:18
     |
6013 |         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6013 -         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6013 +         catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6014:27
     |
6014 |         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6014 -         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6014 +         mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6208:14
     |
6208 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6208 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6208 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6209:23
     |
6209 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6209 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6209 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6210:25
     |
6210 |     retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6210 -     retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6210 +     retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6228:18
     |
6228 |         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6228 -         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6228 +         catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6229:27
     |
6229 |         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6229 -         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6229 +         mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6230:29
     |
6230 |         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6230 -         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6230 +         retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6448:14
     |
6448 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6448 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6448 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6449:23
     |
6449 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6449 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6449 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6458:29
     |
6458 |         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6458 -         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6458 +         retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6674:12
     |
6674 |     state: std::sync::Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6674 -     state: std::sync::Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
6674 +     state: Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6692:20
     |
6692 | ...   state: std::sync::Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECO...
     |              ^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6692 -             state: std::sync::Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY }),
6692 +             state: Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY }),
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:8064:12
     |
8064 |     state: std::sync::Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
8064 -     state: std::sync::Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
8064 +     state: Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:8074:32
     |
8074 | ...   Arc::new(Self { state: std::sync::Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_...
     |                              ^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
8074 -         Arc::new(Self { state: std::sync::Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, live: 0, occupied: 0, closing: 0 }) })
8074 +         Arc::new(Self { state: Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, live: 0, occupied: 0, closing: 0 }) })
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:8858:24
     |
8858 |                 lanes: std::collections::BTreeMap::new(),
     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
8858 -                 lanes: std::collections::BTreeMap::new(),
8858 +                 lanes: BTreeMap::new(),
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9228:71
     |
9228 | static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::BTreeMap<String, ArtifactCodec>>> = std::s...
     |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9228 - static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::BTreeMap<String, ArtifactCodec>>> = std::sync::OnceLock::new();
9228 + static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<BTreeMap<String, ArtifactCodec>>> = std::sync::OnceLock::new();
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9230:60
     |
9230 | fn document_codec_registry() -> &'static std::sync::RwLock<std::collections::BTreeMap<String, ArtifactCodec>> {
     |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9230 - fn document_codec_registry() -> &'static std::sync::RwLock<std::collections::BTreeMap<String, ArtifactCodec>> {
9230 + fn document_codec_registry() -> &'static std::sync::RwLock<BTreeMap<String, ArtifactCodec>> {
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9231:67
     |
9231 |     DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::BTreeMap::new()))
     |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9231 -     DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::BTreeMap::new()))
9231 +     DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(BTreeMap::new()))
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9268:40
     |
9268 | fn validate_document_codecs(registry: &std::collections::BTreeMap<String, ArtifactCodec>, codecs: &[ArtifactCodec]) -> Result<(), ...
     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9268 - fn validate_document_codecs(registry: &std::collections::BTreeMap<String, ArtifactCodec>, codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
9268 + fn validate_document_codecs(registry: &BTreeMap<String, ArtifactCodec>, codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9269:23
     |
9269 |     let mut proposed: std::collections::BTreeMap<&str, &ArtifactCodec> = std::collections::BTreeMap::new();
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9269 -     let mut proposed: std::collections::BTreeMap<&str, &ArtifactCodec> = std::collections::BTreeMap::new();
9269 +     let mut proposed: BTreeMap<&str, &ArtifactCodec> = std::collections::BTreeMap::new();
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9269:74
     |
9269 |     let mut proposed: std::collections::BTreeMap<&str, &ArtifactCodec> = std::collections::BTreeMap::new();
     |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9269 -     let mut proposed: std::collections::BTreeMap<&str, &ArtifactCodec> = std::collections::BTreeMap::new();
9269 +     let mut proposed: std::collections::BTreeMap<&str, &ArtifactCodec> = BTreeMap::new();
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9362:74
     |
9362 | ...::RwLock<std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>> = std::...
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9362 - static DIALECT_MIGRATION_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>> = std::sync::OnceLock::new();
9362 + static DIALECT_MIGRATION_REGISTRY: std::sync::OnceLock<std::sync::RwLock<BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>> = std::sync::OnceLock::new();
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9364:63
     |
9364 | ...::RwLock<std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>> {
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9364 - fn dialect_migration_registry() -> &'static std::sync::RwLock<std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>> {
9364 + fn dialect_migration_registry() -> &'static std::sync::RwLock<BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>> {
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9365:70
     |
9365 |     DIALECT_MIGRATION_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::BTreeMap::new()))
     |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9365 -     DIALECT_MIGRATION_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::BTreeMap::new()))
9365 +     DIALECT_MIGRATION_REGISTRY.get_or_init(|| std::sync::RwLock::new(BTreeMap::new()))
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9399:43
     |
9399 | ...gistry: &std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>, migratio...
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9399 - fn validate_dialect_migrations(registry: &std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>, migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
9399 + fn validate_dialect_migrations(registry: &BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>, migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9400:23
     |
9400 | ...roposed: std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), &DialectMigration> = std::c...
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9400 -     let mut proposed: std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), &DialectMigration> = std::collections::BTreeMap::new();
9400 +     let mut proposed: BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), &DialectMigration> = std::collections::BTreeMap::new();
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9400:135
     |
9400 | ...::os_io::ArtifactDialect), &DialectMigration> = std::collections::BTreeMap::new();
     |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9400 -     let mut proposed: std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), &DialectMigration> = std::collections::BTreeMap::new();
9400 +     let mut proposed: std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), &DialectMigration> = BTreeMap::new();
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9474:59
     |
9474 |     document_codecs: std::sync::RwLockWriteGuard<'static, std::collections::BTreeMap<String, ArtifactCodec>>,
     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9474 -     document_codecs: std::sync::RwLockWriteGuard<'static, std::collections::BTreeMap<String, ArtifactCodec>>,
9474 +     document_codecs: std::sync::RwLockWriteGuard<'static, BTreeMap<String, ArtifactCodec>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9475:62
     |
9475 | ...'static, std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>,
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9475 -     dialect_migrations: std::sync::RwLockWriteGuard<'static, std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>,
9475 +     dialect_migrations: std::sync::RwLockWriteGuard<'static, BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>,
     |


warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9643:16
     |
9643 |         lanes: std::collections::BTreeMap::new(),
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9643 -         lanes: std::collections::BTreeMap::new(),
9643 +         lanes: BTreeMap::new(),
     |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:10601:30
      |
10601 |     let mut message_ledger = std::collections::BTreeMap::new();
      |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10601 -     let mut message_ledger = std::collections::BTreeMap::new();
10601 +     let mut message_ledger = BTreeMap::new();
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:10771:16
      |
10771 |         lanes: std::collections::BTreeMap::new(),
      |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10771 -         lanes: std::collections::BTreeMap::new(),
10771 +         lanes: BTreeMap::new(),
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:10991:16
      |
10991 |         lanes: std::collections::BTreeMap::new(),
      |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10991 -         lanes: std::collections::BTreeMap::new(),
10991 +         lanes: BTreeMap::new(),
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12001:26
      |
12001 |             .checked_mul(std::mem::size_of::<String>())?
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12001 -             .checked_mul(std::mem::size_of::<String>())?
12001 +             .checked_mul(size_of::<String>())?
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12002:68
      |
12002 |             .checked_add(self.redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
      |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12002 -             .checked_add(self.redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
12002 +             .checked_add(self.redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12003:78
      |
12003 |             .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
      |                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12003 -             .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
12003 +             .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(size_of::<String>())?)?
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12004:75
      |
12004 |             .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
      |                                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12004 -             .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
12004 +             .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12005:71
      |
12005 |             .checked_add(self.applied_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)?
      |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12005 -             .checked_add(self.applied_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)?
12005 +             .checked_add(self.applied_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)?
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12006:68
      |
12006 |             .checked_add(self.redo_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)
      |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12006 -             .checked_add(self.redo_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)
12006 +             .checked_add(self.redo_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12584:6
      |
12584 | impl serde::Serialize for ArtifactEditMessageLedger {
      |      ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12584 - impl serde::Serialize for ArtifactEditMessageLedger {
12584 + impl Serialize for ArtifactEditMessageLedger {
      |


warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🧵️borrowed/🧪️component.rs:128:6
    |
128 |     (super::tests::authority().begin_one_item_seal(edit, Arc::new(17), Arc::new(MapRetirementFactory), Arc::new(super::tests::Fixtu...
    |      ^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
128 -     (super::tests::authority().begin_one_item_seal(edit, Arc::new(17), Arc::new(MapRetirementFactory), Arc::new(super::tests::FixtureSnapshotRetirement)), fixture, lifetime)
128 +     (tests::authority().begin_one_item_seal(edit, Arc::new(17), Arc::new(MapRetirementFactory), Arc::new(super::tests::FixtureSnapshotRetirement)), fixture, lifetime)
    |


warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🧵️borrowed/🧪️component.rs:128:113
    |
128 | ...), Arc::new(MapRetirementFactory), Arc::new(super::tests::FixtureSnapshotRetirement)), fixture, lifetime)
    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
128 -     (super::tests::authority().begin_one_item_seal(edit, Arc::new(17), Arc::new(MapRetirementFactory), Arc::new(super::tests::FixtureSnapshotRetirement)), fixture, lifetime)
128 +     (super::tests::authority().begin_one_item_seal(edit, Arc::new(17), Arc::new(MapRetirementFactory), Arc::new(tests::FixtureSnapshotRetirement)), fixture, lifetime)
    |


warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️component.rs:851:17
    |
851 |         assert!(super::tests::authority().validate_prepared(prepared).is_err());
    |                 ^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
851 -         assert!(super::tests::authority().validate_prepared(prepared).is_err());
851 +         assert!(tests::authority().validate_prepared(prepared).is_err());
    |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:20091:33
      |
20091 |         let mut first_by_slot = std::collections::BTreeMap::new();
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
20091 -         let mut first_by_slot = std::collections::BTreeMap::new();
20091 +         let mut first_by_slot = BTreeMap::new();
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:21833:29
      |
21833 |         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
21833 -         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
21833 +         published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:21839:126
      |
21839 | ... 2, retained_bytes: 512 }, published_root: Arc::new(std::sync::Mutex::new(None)), forge_digest: false }
      |                                                        ^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
21839 -             Self { footprint: ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 512 }, published_root: Arc::new(std::sync::Mutex::new(None)), forge_digest: false }
21839 +             Self { footprint: ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 512 }, published_root: Arc::new(Mutex::new(None)), forge_digest: false }
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:21854:29
      |
21854 |         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
21854 -         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
21854 +         published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:22001:29
      |
22001 |         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22001 -         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
22001 +         published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:22006:125
      |
22006 | ...: 1, retained_bytes: 64 }, published_root: Arc::new(std::sync::Mutex::new(None)) }
      |                                                        ^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22006 -             Self { footprint: ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 64 }, published_root: Arc::new(std::sync::Mutex::new(None)) }
22006 +             Self { footprint: ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 64 }, published_root: Arc::new(Mutex::new(None)) }
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:22014:29
      |
22014 |         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22014 -         published_root: Arc<std::sync::Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
22014 +         published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:22301:38
      |
22301 |             published_root: Arc::new(std::sync::Mutex::new(None)),
      |                                      ^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22301 -             published_root: Arc::new(std::sync::Mutex::new(None)),
22301 +             published_root: Arc::new(Mutex::new(None)),
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:22335:28
      |
22335 |         commit: Option<Arc<std::sync::Mutex<crate::os_vcs::ArtifactGroupVisibilityOwner>>>,
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22335 -         commit: Option<Arc<std::sync::Mutex<crate::os_vcs::ArtifactGroupVisibilityOwner>>>,
22335 +         commit: Option<Arc<Mutex<crate::os_vcs::ArtifactGroupVisibilityOwner>>>,
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:22356:20
      |
22356 |             lanes: std::collections::BTreeMap::new(), edit_messages: ArtifactEditMessageLedger::new(), conflicts: Vec::new(),
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22356 -             lanes: std::collections::BTreeMap::new(), edit_messages: ArtifactEditMessageLedger::new(), conflicts: Vec::new(),
22356 +             lanes: BTreeMap::new(), edit_messages: ArtifactEditMessageLedger::new(), conflicts: Vec::new(),
      |


warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:22376:34
      |
22376 |             let owner = Arc::new(std::sync::Mutex::new(crate::os_vcs::ArtifactGroupVisibilityOwner::new()));
      |                                  ^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22376 -             let owner = Arc::new(std::sync::Mutex::new(crate::os_vcs::ArtifactGroupVisibilityOwner::new()));
22376 +             let owner = Arc::new(Mutex::new(crate::os_vcs::ArtifactGroupVisibilityOwner::new()));
      |


error[E0053]: method `envelope_id` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3727:35
     |
3727 |         async fn envelope_id() -> &'static str {
     |                                   ^^^^^^^^^^^^ expected `&'static str`, found future
     |
note: type in trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:4547:25
     |
4547 |     fn envelope_id() -> &'static str {
     |                         ^^^^^^^^^^^^
     = note: expected signature `fn() -> &'static str`
                found signature `fn() -> impl futures::Future<Output = &'static str>`


error[E0053]: method `parse_dsl` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3730:43
     |
3730 |         async fn parse_dsl(text: &str) -> Result<Self, crate::os_dsl::TextError> {
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<DemoSnapshot, TextError>`, found future
     |
note: type in trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:4544:33
     |
4544 |     fn parse_dsl(text: &str) -> Result<Self, TextError>;
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^
     = note: expected signature `fn(&_) -> Result<os_store::sync::tests::DemoSnapshot, protocol::TextError>`
                found signature `fn(&_) -> impl futures::Future<Output = Result<os_store::sync::tests::DemoSnapshot, protocol::TextError>>`


error[E0053]: method `print_dsl` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3738:38
     |
3738 |         async fn print_dsl(&self) -> String {
     |                                      ^^^^^^ expected `std::string::String`, found future
     |
note: type in trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:4545:28
     |
4545 |     fn print_dsl(&self) -> String;
     |                            ^^^^^^
     = note: expected signature `fn(&os_store::sync::tests::DemoSnapshot) -> std::string::String`
                found signature `fn(&os_store::sync::tests::DemoSnapshot) -> impl futures::Future<Output = std::string::String>`


error[E0053]: method `encode_pack_with` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3746:74
     |
3746 |         async fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
     |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Vec<u8>, protocol::PackError>`, found future
     |
note: type in trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:8980:64
     |
8980 |     fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError>;
     |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: expected signature `fn(&os_store::sync::tests::DemoSnapshot, &os_pack::value::EncodeOptions) -> Result<Vec<u8>, protocol::PackError>`
                found signature `fn(&os_store::sync::tests::DemoSnapshot, &os_pack::value::EncodeOptions) -> impl futures::Future<Output = Result<Vec<u8>, protocol::PackError>>`


error[E0053]: method `decode_pack_with` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3751:81
     |
3751 |         async fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
     |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<DemoSnapshot, PackError>`, found future
     |
note: type in trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:8981:71
     |
8981 |     fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError>;
     |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^
     = note: expected signature `fn(&_, &os_pack::value::DecodeOptions) -> Result<os_store::sync::tests::DemoSnapshot, protocol::PackError>`
                found signature `fn(&_, &os_pack::value::DecodeOptions) -> impl futures::Future<Output = Result<os_store::sync::tests::DemoSnapshot, protocol::PackError>>`


error[E0053]: method `record_spec` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3759:35
     |
3759 |         async fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<RecordSpec>`, found future
     |
note: type in trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9001:25
     |
9001 |     fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: expected signature `fn() -> std::option::Option<os_dsl::schema::RecordSpec>`
                found signature `fn() -> impl futures::Future<Output = std::option::Option<os_dsl::schema::RecordSpec>>`


error[E0053]: method `apply` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3770:59
     |
3770 |         async fn apply(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<DemoSnapshot, MutationApplyError>`, found future
     |
     = note: expected signature `fn(&os_store::sync::tests::DemoDiff, &os_store::sync::tests::DemoSnapshot) -> Result<os_store::sync::tests::DemoSnapshot, protocol::MutationApplyError>`
                found signature `fn(&os_store::sync::tests::DemoDiff, &os_store::sync::tests::DemoSnapshot) -> impl futures::Future<Output = Result<os_store::sync::tests::DemoSnapshot, protocol::MutationApplyError>>`


error[E0053]: method `absorb` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3774:48
     |
3774 |         async fn absorb(&mut self, other: Self) {
     |                                                ^ expected `()`, found future
     |
     = note: expected signature `fn(&mut os_store::sync::tests::DemoDiff, os_store::sync::tests::DemoDiff) -> ()`
                found signature `fn(&mut os_store::sync::tests::DemoDiff, os_store::sync::tests::DemoDiff) -> impl futures::Future<Output = ()>`


error[E0053]: method `parse_op` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3789:42
     |
3789 |         async fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<DemoMutation, TextError>`, found future
     |
     = note: expected signature `fn(&_) -> Result<os_store::sync::tests::DemoMutation, protocol::TextError>`
                found signature `fn(&_) -> impl futures::Future<Output = Result<os_store::sync::tests::DemoMutation, protocol::TextError>>`


error[E0053]: method `print_op` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3800:37
     |
3800 |         async fn print_op(&self) -> String {
     |                                     ^^^^^^ expected `std::string::String`, found future
     |
     = note: expected signature `fn(&os_store::sync::tests::DemoMutation) -> std::string::String`
                found signature `fn(&os_store::sync::tests::DemoMutation) -> impl futures::Future<Output = std::string::String>`


error[E0053]: method `encode_op` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3809:38
     |
3809 |         async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Vec<u8>, protocol::ProtocolError>`, found future
     |
     = note: expected signature `fn(&os_store::sync::tests::DemoMutation) -> Result<Vec<u8>, protocol::ProtocolError>`
                found signature `fn(&os_store::sync::tests::DemoMutation) -> impl futures::Future<Output = Result<Vec<u8>, protocol::ProtocolError>>`


error[E0053]: method `decode_op` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3820:45
     |
3820 |         async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
     |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<DemoMutation, ProtocolError>`, found future
     |
     = note: expected signature `fn(&_) -> Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError>`
                found signature `fn(&_) -> impl futures::Future<Output = Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError>>`


error[E0053]: method `diff` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3840:59
     |
3840 |         async fn diff(&self, _snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `MutationOutcome<DemoDiff>`, found future
     |
     = note: expected signature `fn(&os_store::sync::tests::DemoMutation, &os_store::sync::tests::DemoSnapshot) -> protocol::MutationOutcome<os_store::sync::tests::DemoDiff>`
                found signature `fn(&os_store::sync::tests::DemoMutation, &os_store::sync::tests::DemoSnapshot) -> impl futures::Future<Output = protocol::MutationOutcome<os_store::sync::tests::DemoDiff>>`


error[E0053]: method `inverse` has an incompatible type for trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3847:61
     |
3847 |         async fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
     |                                                             ^^^^^^^^^ expected `Vec<os_store::sync::tests::DemoMutation>`, found future
     |
     = note: expected signature `fn(&os_store::sync::tests::DemoMutation, &os_store::sync::tests::DemoSnapshot) -> Vec<os_store::sync::tests::DemoMutation>`
                found signature `fn(&os_store::sync::tests::DemoMutation, &os_store::sync::tests::DemoSnapshot) -> impl futures::Future<Output = Vec<os_store::sync::tests::DemoMutation>>`


error[E0277]: `Result<std::option::Option<os_store::component::Backbones>, os_vcs::VcsError>` is not a future
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:900:38
    |
900 |         self.store.detach_backbone().await;
    |                                      ^^^^^ `Result<std::option::Option<os_store::component::Backbones>, os_vcs::VcsError>` is not a future
    |
    = help: the trait `futures::Future` is not implemented for `Result<std::option::Option<os_store::component::Backbones>, os_vcs::VcsError>`
    = note: Result<std::option::Option<os_store::component::Backbones>, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
    = note: required for `Result<std::option::Option<os_store::component::Backbones>, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
    |
900 -         self.store.detach_backbone().await;
900 +         self.store.detach_backbone();
    |


error[E0277]: `&str` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3740:111
     |
3740 | ...emioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id().await, semio_format::Component::Dsl, 1).expect("valid envel...
     |                                   ------------------------------------ ^^^^^ `&str` is not a future
     |                                   |
     |                                   this call returns `&str`
     |
     = help: the trait `futures::Future` is not implemented for `&str`
     = note: &str must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&str` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3740 -             let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id().await, semio_format::Component::Dsl, 1).expect("valid envelope_id");
3740 +             let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Dsl, 1).expect("valid envelope_id");
     |


error[E0277]: `Result<Vec<u8>, protocol::PackError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3747:105
     |
3747 |             let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
     |                         ------------------------------------------------------------------------------- ^^^^^ `Result<Vec<u8>, protocol::PackError>` is not a future
     |                         |
     |                         this call returns `Result<Vec<u8>, protocol::PackError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::PackError>`
     = note: Result<Vec<u8>, protocol::PackError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::PackError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3747 -             let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
3747 +             let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
     |
help: alternatively, consider making `fn encode_document` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:4585:8
     |
4585 |     pub async fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
     |         +++++


error[E0277]: `&str` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3748:111
     |
3748 | ...emioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id().await, semio_format::Component::Pack, 1).map_err(|e| PackEr...
     |                                   ------------------------------------ ^^^^^ `&str` is not a future
     |                                   |
     |                                   this call returns `&str`
     |
     = help: the trait `futures::Future` is not implemented for `&str`
     = note: &str must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&str` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3748 -             let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id().await, semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
3748 +             let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
     |


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3747:17
     |
3747 |             let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
     |                 ^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3747:25
     |
3747 |             let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3747:110
     |
3747 |             let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
     |                                                                                                              ^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
note: required by an implicit `Sized` bound in `ControlFlow`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/control_flow.rs:89:25
     |
  89 | pub enum ControlFlow<B, C = ()> {
     |                         ^^^^^^ required by the implicit `Sized` requirement on this type parameter in `ControlFlow`


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3747:25
     |
3747 |             let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `[u8]`
note: required by an implicit `Sized` bound in `ControlFlow`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/control_flow.rs:89:25
     |
  89 | pub enum ControlFlow<B, C = ()> {
     |                         ^^^^^^ required by the implicit `Sized` requirement on this type parameter in `ControlFlow`


error[E0277]: `&str` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3753:79
     |
3753 |             if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id().await {
     |                                          ------------------------------------ ^^^^^ `&str` is not a future
     |                                          |
     |                                          this call returns `&str`
     |
     = help: the trait `futures::Future` is not implemented for `&str`
     = note: &str must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&str` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3753 -             if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id().await {
3753 +             if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
     |


error[E0277]: `&str` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3754:138
     |
3754 | ...ismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id().await, envelope.envelope_id())));
     |                                   ------------------------------------ ^^^^^ `&str` is not a future
     |                                   |
     |                                   this call returns `&str`
     |
     = help: the trait `futures::Future` is not implemented for `&str`
     = note: &str must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&str` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3754 -                 return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id().await, envelope.envelope_id())));
3754 +                 return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.envelope_id())));
     |


error[E0277]: `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3756:100
     |
3756 |             let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options).await?;
     |                                     -------------------------------------------------------------- ^^^^^ `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>` is not a future
     |                                     |
     |                                     this call returns `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>`
     = note: Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3756 -             let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options).await?;
3756 +             let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
     |
help: alternatively, consider making `fn decode_document` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:4590:8
     |
4590 |     pub async fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::os_pack::DecodeReport), PackError> {
     |         +++++


error[E0277]: `Result<Vec<u8>, protocol::PackError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3813:111
     |
3813 | ...  let body = crate::os_pack::encode_record_body(&spec_fn(), &record, &PackEncodeOptions::default()).await.map_err(|e| crate::os...
     |                 -------------------------------------------------------------------------------------- ^^^^^ `Result<Vec<u8>, protocol::PackError>` is not a future
     |                 |
     |                 this call returns `Result<Vec<u8>, protocol::PackError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::PackError>`
     = note: Result<Vec<u8>, protocol::PackError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::PackError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3813 -             let body = crate::os_pack::encode_record_body(&spec_fn(), &record, &PackEncodeOptions::default()).await.map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op pack", offset: 0, detail: e.to_string() })?;
3813 +             let body = crate::os_pack::encode_record_body(&spec_fn(), &record, &PackEncodeOptions::default()).map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op pack", offset: 0, detail: e.to_string() })?;
     |
help: alternatively, consider making `fn encode_record_body` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🎒️pack/🦀️component.rs:39:4
     |
  39 | pub async fn encode_record_body(spec: &crate::os_dsl::schema::RecordSpec, record: &crate::os_dsl::schema::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
     |     +++++


error[E0277]: `protocol::ByteReader<'_>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3821:69
     |
3821 |             let mut reader = crate::os_pack::ByteReader::new(bytes).await;
     |                              -------------------------------------- ^^^^^ `protocol::ByteReader<'_>` is not a future
     |                              |
     |                              this call returns `protocol::ByteReader<'_>`
     |
     = help: the trait `futures::Future` is not implemented for `protocol::ByteReader<'_>`
     = note: protocol::ByteReader<'_> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `protocol::ByteReader<'_>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3821 -             let mut reader = crate::os_pack::ByteReader::new(bytes).await;
3821 +             let mut reader = crate::os_pack::ByteReader::new(bytes);
     |


error[E0277]: `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3831:116
     |
3831 | ...cord, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).await.map_err(crate::os_spr::Pr...
     |                     ------------------------------------------------------------------------------ ^^^^^ `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>` is not a future
     |                     |
     |                     this call returns `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>`
     = note: Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<(os_dsl::schema::RecordValue, os_pack::value::DecodeReport), protocol::PackError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3831 -             let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).await.map_err(crate::os_spr::ProtocolError::from)?;
3831 +             let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
     |
help: alternatively, consider making `fn decode_record_body` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🎒️pack/🦀️component.rs:45:4
     |
  45 | pub async fn decode_record_body(bytes: &[u8], spec: &crate::os_dsl::schema::RecordSpec, options: &DecodeOptions) -> Result<(crate::os_dsl::schema::RecordValue, DecodeReport), PackError> {
     |     +++++


error[E0277]: `protocol::MutationOutcome<os_store::sync::tests::DemoDiff>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3844:14
     |
3841 | /             crate::os_spr::MutationOutcome::new(match self {
3842 | |                 DemoMutation::SetN { n } => DemoDiff { n: Some(*n) },
3843 | |             })
     | |______________- this call returns `protocol::MutationOutcome<os_store::sync::tests::DemoDiff>`
3844 |               .await
     |                ^^^^^ `protocol::MutationOutcome<os_store::sync::tests::DemoDiff>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `protocol::MutationOutcome<os_store::sync::tests::DemoDiff>`
     = note: protocol::MutationOutcome<os_store::sync::tests::DemoDiff> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `protocol::MutationOutcome<os_store::sync::tests::DemoDiff>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3844 -             .await
     |


error[E0277]: `os_store::component::ArtifactCodec` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3859:104
     |
3859 | ...odec(ArtifactCodec::of::<DemoSnapshot, DemoMutation>("demo/v1").await).await.expect("register demo codec");
     |         ---------------------------------------------------------- ^^^^^ `os_store::component::ArtifactCodec` is not a future
     |         |
     |         this call returns `os_store::component::ArtifactCodec`
     |
help: the trait `futures::Future` is not implemented for `os_store::component::ArtifactCodec`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9085:1
     |
9085 | pub struct ArtifactCodec {
     | ^^^^^^^^^^^^^^^^^^^^^^^^
     = note: os_store::component::ArtifactCodec must be a future or must implement `IntoFuture` to be awaited
     = note: required for `os_store::component::ArtifactCodec` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3859 -             let _ = register_document_codec(ArtifactCodec::of::<DemoSnapshot, DemoMutation>("demo/v1").await).await.expect("register demo codec");
3859 +             let _ = register_document_codec(ArtifactCodec::of::<DemoSnapshot, DemoMutation>("demo/v1")).await.expect("register demo codec");
     |


error[E0277]: `Result<Vec<protocol::MutationEnvelope>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3878:132
     |
3878 | ...opes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).await.expect("ope...
     |           ------------------------------------------------------------------------------------------------------ ^^^^^ `Result<Vec<protocol::MutationEnvelope>, protocol::ProtocolError>` is not a future
     |           |
     |           this call returns `Result<Vec<protocol::MutationEnvelope>, protocol::ProtocolError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<protocol::MutationEnvelope>, protocol::ProtocolError>`
     = note: Result<Vec<protocol::MutationEnvelope>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<protocol::MutationEnvelope>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3878 -         let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).await.expect("operation envelope");
3878 +         let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
     |


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4093:164
     |
4093 | ...load: OpBinary::encode_op(&DemoMutation::SetN { n: 6 }).await.expect("encode demo op") },
     |          ------------------------------------------------- ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |          |
     |          this call returns `Result<Vec<u8>, protocol::ProtocolError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4093 -             diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 6 }).await.expect("encode demo op") },
4093 +             diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 6 }).expect("encode demo op") },
     |


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4094:170
     |
4094 | ...load: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).await.expect("encode demo op") },
     |          ------------------------------------------------- ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |          |
     |          this call returns `Result<Vec<u8>, protocol::ProtocolError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4094 -             inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).await.expect("encode demo op") },
4094 +             inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
     |


error[E0277]: `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4222:91
     |
4222 |             create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None).await
     |             ----------------------------------------------------------------------------- ^^^^^ `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
     |             |
     |             this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
     |
help: the trait `futures::Future` is not implemented for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4222 -             create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None).await
4222 +             create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None)
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0277]: `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4699:171
     |
4699 | ...ew(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None).await).a...
     |       -------------------------------------------------------------------------------------------------------------------------- ^^^^^ `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |       |
     |       this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>`
     |
help: the trait `futures::Future` is not implemented for `ArtifactEnvelope<DemoSnapshot, DemoMutation>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-3024850257278704069.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4699 -             let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None).await).await.expect("valid fixture store");
4699 +             let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None)).await.expect("valid fixture store");
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0277]: `Result<os_store::sync::tests::DemoMutation, protocol::TextError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4718:77
     |
4718 | ...te = DemoMutation::parse_op(text).await.unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
     |         ---------------------------- ^^^^^ `Result<os_store::sync::tests::DemoMutation, protocol::TextError>` is not a future
     |         |
     |         this call returns `Result<os_store::sync::tests::DemoMutation, protocol::TextError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoMutation, protocol::TextError>`
     = note: Result<os_store::sync::tests::DemoMutation, protocol::TextError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoMutation, protocol::TextError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4718 -                                 let concrete = DemoMutation::parse_op(text).await.unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
4718 +                                 let concrete = DemoMutation::parse_op(text).unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
     |


error[E0277]: `&os_store::component::ArtifactEnvelope<_, _>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4741:62
     |
4741 |             let timeline_ids: Vec<String> = store.envelope().await.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
     |                                                              ^^^^^ `&os_store::component::ArtifactEnvelope<_, _>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&os_store::component::ArtifactEnvelope<_, _>`
     = note: &os_store::component::ArtifactEnvelope<_, _> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&os_store::component::ArtifactEnvelope<_, _>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4741 -             let timeline_ids: Vec<String> = store.envelope().await.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
4741 +             let timeline_ids: Vec<String> = store.envelope().vcs.edits.iter().map(|edit| edit.id.clone()).collect();
     |


error[E0277]: `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3885:160
     |
3885 | ...t, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await;
     |                       ------------------------------------------------------------------------ ^^^^^ `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
     |                       |
     |                       this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
     |
help: the trait `futures::Future` is not implemented for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3885 -         let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await;
3885 +         let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3889:45
     |
3889 |         assert_eq!(session.store.snapshot().await.expect("snapshot").n, 5);
     |                                             ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3889 -         assert_eq!(session.store.snapshot().await.expect("snapshot").n, 5);
3889 +         assert_eq!(session.store.snapshot().expect("snapshot").n, 5);
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3890:45
     |
3890 |         assert_eq!(session.store.envelope().await.vcs.edits.len(), 1);
     |                                             ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-5306113092493874522.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
3890 -         assert_eq!(session.store.envelope().await.vcs.edits.len(), 1);
3890 +         assert_eq!(session.store.envelope().vcs.edits.len(), 1);
     |


error[E0277]: `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3895:160
     |
3895 | ...t, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await;
     |                       ------------------------------------------------------------------------ ^^^^^ `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
     |                       |
     |                       this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
     |
help: the trait `futures::Future` is not implemented for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3895 -         let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await;
3895 +         let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0277]: `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4799:148
     |
4799 | ...e::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "doc-a", DemoSnapshot { n: 0 }, None).await).await.exp...
     |           ------------------------------------------------------------------------------------------------------- ^^^^^ `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |           |
     |           this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>`
     |
help: the trait `futures::Future` is not implemented for `ArtifactEnvelope<DemoSnapshot, DemoMutation>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-3024850257278704069.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4799 -         let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "doc-a", DemoSnapshot { n: 0 }, None).await).await.expect("valid folder store");
4799 +         let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "doc-a", DemoSnapshot { n: 0 }, None)).await.expect("valid folder store");
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3902:45
     |
3902 |         assert_eq!(session.store.envelope().await.vcs.edits.len(), 0, "buffered until edit-1 arrives");
     |                                             ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-6394317836226535183.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
3902 -         assert_eq!(session.store.envelope().await.vcs.edits.len(), 0, "buffered until edit-1 arrives");
3902 +         assert_eq!(session.store.envelope().vcs.edits.len(), 0, "buffered until edit-1 arrives");
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4801:40
     |
4801 |         let post_e1 = store.snapshot().await.expect("post-e1");
     |                                        ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4801 -         let post_e1 = store.snapshot().await.expect("post-e1");
4801 +         let post_e1 = store.snapshot().expect("post-e1");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3904:45
     |
3904 |         assert_eq!(session.store.envelope().await.vcs.edits.len(), 2, "both edits now applied");
     |                                             ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-5820538990444794744.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
3904 -         assert_eq!(session.store.envelope().await.vcs.edits.len(), 2, "both edits now applied");
3904 +         assert_eq!(session.store.envelope().vcs.edits.len(), 2, "both edits now applied");
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4804:37
     |
4804 |         assert_eq!(store.snapshot().await.expect("live"), post_e1, "precondition: live store is back at post-e1");
     |                                     ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4804 -         assert_eq!(store.snapshot().await.expect("live"), post_e1, "precondition: live store is back at post-e1");
4804 +         assert_eq!(store.snapshot().expect("live"), post_e1, "precondition: live store is back at post-e1");
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3905:45
     |
3905 |         assert_eq!(session.store.snapshot().await.expect("snapshot").n, 9);
     |                                             ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3905 -         assert_eq!(session.store.snapshot().await.expect("snapshot").n, 9);
3905 +         assert_eq!(session.store.snapshot().expect("snapshot").n, 9);
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4806:58
     |
4806 |         let files = print_document_pack(store.envelope().await).await.expect("print document pack");
     |                                                          ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-3553798859700403735.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4806 -         let files = print_document_pack(store.envelope().await).await.expect("print document pack");
4806 +         let files = print_document_pack(store.envelope()).await.expect("print document pack");
     |


error[E0599]: no method named `send` found for struct `os_store::component::ChannelBackbone` in the current scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3708:17
      |
 3708 |         channel.send(BackboneMessage::Ack { op_ids: vec!["first".into()] }).await.expect("first backbone owner");
      |                 ^^^^ method not found in `os_store::component::ChannelBackbone`
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16601:14
      |
16601 |     async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
      |              ---- the method is available for `os_store::component::ChannelBackbone` here
...
16826 | pub struct ChannelBackbone {
      | -------------------------- method `send` not found for this struct
      |
      = help: items from traits can only be used if the trait is in scope
help: trait `Backbone` which provides `send` is implemented but not in scope; perhaps you want to import it
      |
 3610 +     use crate::os_store::component::Backbone;
      |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4813:40
     |
4813 |         assert_eq!(reloaded.snapshot().await.expect("reloaded"), post_e1);
     |                                        ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4813 -         assert_eq!(reloaded.snapshot().await.expect("reloaded"), post_e1);
4813 +         assert_eq!(reloaded.snapshot().expect("reloaded"), post_e1);
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4816:40
     |
4816 |         assert_eq!(reloaded.snapshot().await.expect("post-redo"), DemoSnapshot { n: 2 });
     |                                        ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4816 -         assert_eq!(reloaded.snapshot().await.expect("post-redo"), DemoSnapshot { n: 2 });
4816 +         assert_eq!(reloaded.snapshot().expect("post-redo"), DemoSnapshot { n: 2 });
     |


error[E0277]: `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4832:142
     |
4832 | ...DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expect("valid zero-edi...
     |                       ------------------------------------------------------------------------ ^^^^^ `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
     |                       |
     |                       this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
     |
help: the trait `futures::Future` is not implemented for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4832 -         let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expect("valid zero-edit text fixture");
4832 +         let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid zero-edit text fixture");
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0599]: no method named `send` found for struct `os_store::component::ChannelBackbone` in the current scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3709:17
      |
 3709 |         channel.send(BackboneMessage::Ack { op_ids: vec!["second".into()] }).await.expect("second backbone owner");
      |                 ^^^^ method not found in `os_store::component::ChannelBackbone`
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16601:14
      |
16601 |     async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
      |              ---- the method is available for `os_store::component::ChannelBackbone` here
...
16826 | pub struct ChannelBackbone {
      | -------------------------- method `send` not found for this struct
      |
      = help: items from traits can only be used if the trait is in scope
help: trait `Backbone` which provides `send` is implemented but not in scope; perhaps you want to import it
      |
 3610 +     use crate::os_store::component::Backbone;
      |


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4014:164
     |
4014 | ...load: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).await.expect("encode demo op") },
     |          ------------------------------------------------- ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |          |
     |          this call returns `Result<Vec<u8>, protocol::ProtocolError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4014 -             diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).await.expect("encode demo op") },
4014 +             diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4833:57
     |
4833 |         let files = print_document_text(seed.envelope().await).await.expect("print document text");
     |                                                         ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-8936967023904543530.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4833 -         let files = print_document_text(seed.envelope().await).await.expect("print document text");
4833 +         let files = print_document_text(seed.envelope()).await.expect("print document text");
     |


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4015:170
     |
4015 | ...load: OpBinary::encode_op(&DemoMutation::SetN { n: 0 }).await.expect("encode demo op") },
     |          ------------------------------------------------- ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |          |
     |          this call returns `Result<Vec<u8>, protocol::ProtocolError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4015 -             inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 0 }).await.expect("encode demo op") },
4015 +             inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 0 }).expect("encode demo op") },
     |


error[E0277]: `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4836:147
     |
4836 | ...e::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expe...
     |           ------------------------------------------------------------------------------------------------------ ^^^^^ `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |           |
     |           this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>`
     |
help: the trait `futures::Future` is not implemented for `ArtifactEnvelope<DemoSnapshot, DemoMutation>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-3024850257278704069.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4836 -         let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expect("valid text fixture");
4836 +         let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid text fixture");
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4838:43
     |
4838 |         let first_edit = store.envelope().await.vcs.edits.last().expect("first edit");
     |                                           ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-1714057728461919931.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4838 -         let first_edit = store.envelope().await.vcs.edits.last().expect("first edit");
4838 +         let first_edit = store.envelope().vcs.edits.last().expect("first edit");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4842:44
     |
4842 |         let second_edit = store.envelope().await.vcs.edits.last().expect("second edit");
     |                                            ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-883039986785237996.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4842 -         let second_edit = store.envelope().await.vcs.edits.last().expect("second edit");
4842 +         let second_edit = store.envelope().vcs.edits.last().expect("second edit");
     |


error[E0277]: `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4868:142
     |
4868 | ...DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expect("valid pack fix...
     |                       ------------------------------------------------------------------------ ^^^^^ `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` is not a future
     |                       |
     |                       this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
     |
help: the trait `futures::Future` is not implemented for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, _>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4868 -         let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expect("valid pack fixture");
4868 +         let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid pack fixture");
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2070:42
     |
2070 | ...   self.runner.external_tickets.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count|...
     |                                    ^^^^^^^^^^^^
     |
     = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
     |
2070 -             self.runner.external_tickets.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| count.checked_add(1)).expect("artifact actor ticket capacity exhausted");
2070 +             self.runner.external_tickets.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| count.checked_add(1)).expect("artifact actor ticket capacity exhausted");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4869:57
     |
4869 |         let files = print_document_pack(seed.envelope().await).await.expect("print document pack");
     |                                                         ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-8936967023904543530.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4869 -         let files = print_document_pack(seed.envelope().await).await.expect("print document pack");
4869 +         let files = print_document_pack(seed.envelope()).await.expect("print document pack");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4870:42
     |
4870 |         let dsl_mirror = seed.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |                                          ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-12171219270953751678.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4870 -         let dsl_mirror = seed.envelope().await.vcs.initial_snapshot.print_dsl().await;
4870 +         let dsl_mirror = seed.envelope().vcs.initial_snapshot.print_dsl().await;
     |


error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4870:13
     |
4870 |         let dsl_mirror = seed.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |             ^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `str`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4870:26
     |
4870 |         let dsl_mirror = seed.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `str`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4870:81
     |
4870 |         let dsl_mirror = seed.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |                                                                                 ^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `std::task::Poll`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/task/poll.rs:14:15
     |
  14 | pub enum Poll<T> {
     |               ^ required by the implicit `Sized` requirement on this type parameter in `Poll`


error[E0277]: `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4873:147
     |
4873 | ...e::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expe...
     |           ------------------------------------------------------------------------------------------------------ ^^^^^ `ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |           |
     |           this call returns `os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>`
     |
help: the trait `futures::Future` is not implemented for `ArtifactEnvelope<DemoSnapshot, DemoMutation>`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2320:1
     |
2320 | pub struct ArtifactEnvelope<P, Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-3024850257278704069.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4873 -         let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None).await).await.expect("valid pack append fixture");
4873 +         let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid pack append fixture");
     |
help: alternatively, consider making `fn create_document_envelope` asynchronous
    -->  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:9629:4
     |
9629 | pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
     |     +++++


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4875:43
     |
4875 |         let first_edit = store.envelope().await.vcs.edits.last().expect("first edit");
     |                                           ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-16677766725838892836.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4875 -         let first_edit = store.envelope().await.vcs.edits.last().expect("first edit");
4875 +         let first_edit = store.envelope().vcs.edits.last().expect("first edit");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4879:44
     |
4879 |         let second_edit = store.envelope().await.vcs.edits.last().expect("second edit");
     |                                            ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-10595025184784710395.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4879 -         let second_edit = store.envelope().await.vcs.edits.last().expect("second edit");
4879 +         let second_edit = store.envelope().vcs.edits.last().expect("second edit");
     |


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4198:116
     |
4198 | ...on::SetN { n: 42 }.encode_op().await.expect("encode")) }],
     |                                   ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4198 -             ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().await.expect("encode")) }],
4198 +             ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
     |


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4199:119
     |
4199 | ...ion::SetN { n: 0 }.encode_op().await.expect("encode")) }],
     |                                   ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4199 -             inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 0 }.encode_op().await.expect("encode")) }],
4199 +             inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 0 }.encode_op().expect("encode")) }],
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4895:59
     |
4895 |         let files2 = print_document_pack(store.envelope().await).await.expect("print document pack 2");
     |                                                           ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-10264380276007974151.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4895 -         let files2 = print_document_pack(store.envelope().await).await.expect("print document pack 2");
4895 +         let files2 = print_document_pack(store.envelope()).await.expect("print document pack 2");
     |


error[E0277]: `Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4205:91
     |
4205 |         let recovered = <DemoMutation as OpBinary>::decode_op(&envelopes[0].diff.payload).await.expect("decode op");
     |                         ----------------------------------------------------------------- ^^^^^ `Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError>` is not a future
     |                         |
     |                         this call returns `Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError>`
     = note: Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoMutation, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4205 -         let recovered = <DemoMutation as OpBinary>::decode_op(&envelopes[0].diff.payload).await.expect("decode op");
4205 +         let recovered = <DemoMutation as OpBinary>::decode_op(&envelopes[0].diff.payload).expect("decode op");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4896:44
     |
4896 |         let dsl_mirror2 = store.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |                                            ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-8491814673349273665.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4896 -         let dsl_mirror2 = store.envelope().await.vcs.initial_snapshot.print_dsl().await;
4896 +         let dsl_mirror2 = store.envelope().vcs.initial_snapshot.print_dsl().await;
     |


error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4896:13
     |
4896 |         let dsl_mirror2 = store.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |             ^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `str`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4896:27
     |
4896 |         let dsl_mirror2 = store.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `str`
     = note: all local variables must have a statically known size


error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4896:83
     |
4896 |         let dsl_mirror2 = store.envelope().await.vcs.initial_snapshot.print_dsl().await;
     |                                                                                   ^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `std::task::Poll`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/task/poll.rs:14:15
     |
  14 | pub enum Poll<T> {
     |               ^ required by the implicit `Sized` requirement on this type parameter in `Poll`


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4305:120
     |
4305 | ...on::SetN { n: 42 }.encode_op().await.expect("encode")) }],
     |                                   ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4305 -                 ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().await.expect("encode")) }],
4305 +                 ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
     |


error[E0277]: `Result<Vec<u8>, protocol::ProtocolError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4306:123
     |
4306 | ...ion::SetN { n: 1 }.encode_op().await.expect("encode")) }],
     |                                   ^^^^^ `Result<Vec<u8>, protocol::ProtocolError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<Vec<u8>, protocol::ProtocolError>`
     = note: Result<Vec<u8>, protocol::ProtocolError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<Vec<u8>, protocol::ProtocolError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4306 -                 inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 1 }.encode_op().await.expect("encode")) }],
4306 +                 inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 1 }.encode_op().expect("encode")) }],
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, protocol::TextError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4904:53
     |
4904 | ...q!(DemoSnapshot::parse_dsl(&mirror).await.expect("parse mirror").n, 0, "mirror captures the initial snapshot, not later edits");
     |       -------------------------------- ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, protocol::TextError>` is not a future
     |       |
     |       this call returns `Result<os_store::sync::tests::DemoSnapshot, protocol::TextError>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, protocol::TextError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, protocol::TextError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, protocol::TextError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4904 -         assert_eq!(DemoSnapshot::parse_dsl(&mirror).await.expect("parse mirror").n, 0, "mirror captures the initial snapshot, not later edits");
4904 +         assert_eq!(DemoSnapshot::parse_dsl(&mirror).expect("parse mirror").n, 0, "mirror captures the initial snapshot, not later edits");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4326:41
     |
4326 |             assert_eq!(store.envelope().await.vcs.edits.len(), 2, "external edit joined the timeline");
     |                                         ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-6861244876628826926.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4326 -             assert_eq!(store.envelope().await.vcs.edits.len(), 2, "external edit joined the timeline");
4326 +             assert_eq!(store.envelope().vcs.edits.len(), 2, "external edit joined the timeline");
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4327:41
     |
4327 |             assert_eq!(store.snapshot().await.expect("snapshot").n, 42);
     |                                         ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4327 -             assert_eq!(store.snapshot().await.expect("snapshot").n, 42);
4327 +             assert_eq!(store.snapshot().expect("snapshot").n, 42);
     |


error[E0277]: `WorkerSubmitError` doesn't implement `Debug`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2600:92
     |
2600 |                 pool.try_submit(semio_framework_async::Lane::UserVisible, Box::new(|| {})).expect("fill exact quiet queue slot");
     |                                                                                            ^^^^^^ the trait `Debug` is not implemented for `WorkerSubmitError`
     |
note: required by a bound in `Result::<T, E>::expect`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:1179:12
     |
1177 |     pub fn expect(self, msg: &str) -> T
     |            ------ required by a bound in this associated function
1178 |     where
1179 |         E: fmt::Debug,
     |            ^^^^^^^^^^ required by this bound in `Result::<T, E>::expect`


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4491:43
     |
4491 |             assert_eq!(store_b.snapshot().await.expect("snapshot b").n, 7, "B converged on A's operation");
     |                                           ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4491 -             assert_eq!(store_b.snapshot().await.expect("snapshot b").n, 7, "B converged on A's operation");
4491 +             assert_eq!(store_b.snapshot().expect("snapshot b").n, 7, "B converged on A's operation");
     |


error[E0277]: `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4545:43
     |
4545 |             assert_eq!(store_b.envelope().await.vcs.edits.len(), 2, "B caught up on the full backlog");
     |                                           ^^^^^ `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>`
     = note: &ArtifactEnvelope<DemoSnapshot, DemoMutation> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `&ArtifactEnvelope<DemoSnapshot, DemoMutation>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-18426215920064962981.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
4545 -             assert_eq!(store_b.envelope().await.vcs.edits.len(), 2, "B caught up on the full backlog");
4545 +             assert_eq!(store_b.envelope().vcs.edits.len(), 2, "B caught up on the full backlog");
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4546:43
     |
4546 |             assert_eq!(store_b.snapshot().await.expect("snapshot b").n, 4);
     |                                           ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4546 -             assert_eq!(store_b.snapshot().await.expect("snapshot b").n, 4);
4546 +             assert_eq!(store_b.snapshot().expect("snapshot b").n, 4);
     |


error[E0277]: `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4596:43
     |
4596 |             assert_eq!(store_b.snapshot().await.expect("snapshot b").n, 5);
     |                                           ^^^^^ `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>`
     = note: Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<os_store::sync::tests::DemoSnapshot, os_vcs::VcsError>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
4596 -             assert_eq!(store_b.snapshot().await.expect("snapshot b").n, 5);
4596 +             assert_eq!(store_b.snapshot().expect("snapshot b").n, 5);
     |


error[E0277]: `dyn Future<Output = Result<(ArtifactPackFiles, String), VcsError>>` cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:19:22
     |
  19 |         require_send(&future);
     |         ------------ ^^^^^^^ `dyn Future<Output = Result<(ArtifactPackFiles, String), VcsError>>` cannot be sent between threads safely
     |         |
     |         required by a bound introduced by this call
     |
     = help: the trait `std::marker::Send` is not implemented for `dyn Future<Output = Result<(ArtifactPackFiles, String), VcsError>>`
     = note: required for `Unique<dyn Future<Output = Result<(ArtifactPackFiles, String), ...>>>` to implement `std::marker::Send`
note: required because it appears within the type `Box<dyn Future<Output = Result<(ArtifactPackFiles, String), ...>>>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
     |
 236 | pub struct Box<
     |            ^^^
note: required because it appears within the type `Pin<Box<dyn Future<Output = Result<(ArtifactPackFiles, ...), ...>>>>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/pin.rs:1092:12
     |
1092 | pub struct Pin<Ptr> {
     |            ^^^
note: required by a bound in `require_send`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:4:42
     |
   4 | fn require_send<F: std::future::Future + Send>(_: &F) {}
     |                                          ^^^^ required by this bound in `require_send`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-10218491873560134621.txt'
     = note: consider using `--verbose` to print the full type name to the console


error[E0277]: `dyn Future<Output = Result<ArtifactTextFiles, VcsError>>` cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:36:22
     |
  36 |         require_send(&future);
     |         ------------ ^^^^^^^ `dyn Future<Output = Result<ArtifactTextFiles, VcsError>>` cannot be sent between threads safely
     |         |
     |         required by a bound introduced by this call
     |
     = help: the trait `std::marker::Send` is not implemented for `dyn Future<Output = Result<ArtifactTextFiles, VcsError>>`
     = note: required for `Unique<dyn Future<Output = Result<ArtifactTextFiles, VcsError>>>` to implement `std::marker::Send`
note: required because it appears within the type `Box<dyn Future<Output = Result<ArtifactTextFiles, VcsError>>>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
     |
 236 | pub struct Box<
     |            ^^^
note: required because it appears within the type `Pin<Box<dyn Future<Output = Result<ArtifactTextFiles, VcsError>>>>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/pin.rs:1092:12
     |
1092 | pub struct Pin<Ptr> {
     |            ^^^
note: required by a bound in `require_send`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:4:42
     |
   4 | fn require_send<F: std::future::Future + Send>(_: &F) {}
     |                                          ^^^^ required by this bound in `require_send`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-4199454142659372190.txt'
     = note: consider using `--verbose` to print the full type name to the console


warning: unused variable: `kind`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2186:21
     |
2186 | ...   kind @ (semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated) i...
     |       ^^^^ help: if this is intentional, prefix it with an underscore: `_kind`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default


error: future cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2245:21
     |
2245 | /                     Box::pin(async move {
2246 | |                         let outcome = actor.drive_one().await;
2247 | |                         (actor, outcome)
2248 | |                     }) as ActorTurnFuture
     | |______________________^ future created by async block is not `Send`
     |
     = help: the trait `std::marker::Send` is not implemented for `dyn Future<Output = Result<(ArtifactPackFiles, String), VcsError>>`
note: future is not `Send` as it awaits another future which is not `Send`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1263:53
     |
1263 | ... (pack_files, _dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).await.map_err(|error| error.to_string())?;
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here on type `Pin<Box<dyn futures::Future<Output = Result<(os_store::component::ArtifactPackFiles, std::string::String), os_vcs::VcsError>>>>`, which is not `Send`
     = note: required for the cast from `Pin<Box<...>>` to `Pin<Box<dyn Future<Output = (ArtifactActor, ArtifactDrive)> + Send>>`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-12094672542879477935.txt'
     = note: consider using `--verbose` to print the full type name to the console


error: future cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2245:21
     |
2245 | /                     Box::pin(async move {
2246 | |                         let outcome = actor.drive_one().await;
2247 | |                         (actor, outcome)
2248 | |                     }) as ActorTurnFuture
     | |______________________^ future created by async block is not `Send`
     |
     = help: the trait `std::marker::Send` is not implemented for `dyn Future<Output = Result<ArtifactTextFiles, VcsError>>`
note: future is not `Send` as it awaits another future which is not `Send`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1277:34
     |
1277 |                     let mirror = (codec.print_mirror)(pack, spr).await.map_err(|error| error.to_string())?;
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here on type `Pin<Box<dyn futures::Future<Output = Result<os_store::component::ArtifactTextFiles, os_vcs::VcsError>>>>`, which is not `Send`
     = note: required for the cast from `Pin<Box<...>>` to `Pin<Box<dyn Future<Output = (ArtifactActor, ArtifactDrive)> + Send>>`
     = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-7866165447935356549.txt'
     = note: consider using `--verbose` to print the full type name to the console


error: aborting due to 92 previous errors; 66 warnings emitted


Some errors have detailed explanations: E0053, E0277, E0432, E0599, E0603.

For more information about an error, try `rustc --explain E0053`.

```

## Referenced Full Type Names And Artifact Observation

The newest artifact directory is recorded as a filesystem observation; the terminal tool stream did not print an artifact path. No test stdout or completed binaries metadata is inferred from an empty directory.

```json
{
  "types": [
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-3024850257278704069.txt",
      "text": "os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-5306113092493874522.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-6394317836226535183.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-5820538990444794744.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-3553798859700403735.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-8936967023904543530.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-1714057728461919931.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-883039986785237996.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-12171219270953751678.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-16677766725838892836.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-10595025184784710395.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-10264380276007974151.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-8491814673349273665.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-6861244876628826926.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-18426215920064962981.txt",
      "text": "&os_store::component::ArtifactEnvelope<os_store::sync::tests::DemoSnapshot, os_store::sync::tests::DemoMutation>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-10218491873560134621.txt",
      "text": "dyn futures::Future<Output = Result<(os_store::component::ArtifactPackFiles, std::string::String), os_vcs::VcsError>>\nstd::ptr::Unique<dyn futures::Future<Output = Result<(os_store::component::ArtifactPackFiles, std::string::String), os_vcs::VcsError>>>\nBox<dyn futures::Future<Output = Result<(os_store::component::ArtifactPackFiles, std::string::String), os_vcs::VcsError>>>\nPin<Box<dyn futures::Future<Output = Result<(os_store::component::ArtifactPackFiles, std::string::String), os_vcs::VcsError>>>>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-4199454142659372190.txt",
      "text": "dyn futures::Future<Output = Result<os_store::component::ArtifactTextFiles, os_vcs::VcsError>>\nstd::ptr::Unique<dyn futures::Future<Output = Result<os_store::component::ArtifactTextFiles, os_vcs::VcsError>>>\nBox<dyn futures::Future<Output = Result<os_store::component::ArtifactTextFiles, os_vcs::VcsError>>>\nPin<Box<dyn futures::Future<Output = Result<os_store::component::ArtifactTextFiles, os_vcs::VcsError>>>>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-12094672542879477935.txt",
      "text": "dyn futures::Future<Output = Result<(os_store::component::ArtifactPackFiles, std::string::String), os_vcs::VcsError>>\nPin<Box<{async block@🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2245:30: 2245:40}>>\nPin<Box<dyn futures::Future<Output = (os_store::sync::native_actor::ArtifactActor, os_store::sync::native_actor::ArtifactDrive)> + std::marker::Send>>\n"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_os_kernel-3dbb555fdf919f7c.long-type-7866165447935356549.txt",
      "text": "dyn futures::Future<Output = Result<os_store::component::ArtifactTextFiles, os_vcs::VcsError>>\nPin<Box<{async block@🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2245:30: 2245:40}>>\nPin<Box<dyn futures::Future<Output = (os_store::sync::native_actor::ArtifactActor, os_store::sync::native_actor::ArtifactDrive)> + std::marker::Send>>\n"
    }
  ],
  "artifacts": [
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-eHzfQD",
      "mtime": "2026-08-28T02:25:35.426Z",
      "entries": []
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-Qctg94",
      "mtime": "2026-08-28T01:32:11.949Z",
      "entries": [
        "binaries-metadata.json"
      ]
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-ieZMlJ",
      "mtime": "2026-08-28T00:26:10.762Z",
      "entries": []
    }
  ]
}
```

