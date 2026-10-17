//! 🗂️ Immutable trusted-catalog bundle verification for headless hub authority startup.

use semio_hub_artifact_authority::adapters::{bounded_message, AUTHORITY_MAX_CODEC_TEXT_BYTES, TRUSTED_CATALOG_MAX_CODECS};
use semio_hub_artifact_authority::{AcceptedArtifactOperation, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactGenesisCodec, TrustedArtifactIdentity, TrustedArtifactReplayCodec};
use directory::os_directory::{DocumentDescriptor, DocumentExecutionProtocolV1, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1};
use directory::os_directory::io::binary::artifact_hash::hex_lower;
use directory::os_store::{self, serialization::ArtifactCodec};
use directory::io::{ArtifactAssemblyRegistryPlan,ArtifactCodecBinding,ArtifactCatalogBinding,ArtifactCatalogCapability,ArtifactCatalogTarget};
use semio_framework::{DslValue, PackageDescriptor, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::{PackageHash, PackageId, PackageRef};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

use semio_hub_trusted_catalog_contract::schema::{publication_revision, TrustedBundleFileV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageV1, TrustedBundleRendererTarget, TrustedBundleV1, TrustedCatalogCurrentPointerV1, TrustedCatalogPublicationCommandV1, TrustedCatalogPublicationReceiptV1, TRUSTED_CATALOG_PUBLICATION_MAX_BYTES, TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE, TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED, TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA, TRUSTED_CATALOG_PUBLICATION_SCHEMA, TRUSTED_CATALOG_GUEST_RESIDENCY, TrustedCatalogGuestResidencyStateV1, TrustedCatalogGuestResidencyV1, TrustedCatalogLoadProgressV1, TrustedCatalogPackagePhaseV1, TrustedCatalogPackageProgressV1, GuestCodecVerificationV1, GUEST_CODEC_VERIFICATION_SCHEMA};
use directory::os_component_receipt::ComponentReceiptV1;
use directory::os_plugin_module_schema::{TrustedPluginModuleBundleV1, TrustedPluginModuleFileV1, TrustedPluginModuleIndexEntryV1, TrustedPluginModuleIndexV1, TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA};

use semio_hub_trusted_catalog_contract::plugin_module::{plugin_module_blob_path, TrustedPluginModuleSourceV1};
use semio_hub_trusted_catalog_contract::{TrustedCatalogDataRoot, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1, TRUSTED_READ_CHUNK_BYTES};
#[cfg(all(any(test, feature = "testkit"),unix))]
use semio_hub_trusted_catalog_contract::create_fifo_fixture;
use directory::os_directory::schema::{DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1, DOCUMENT_BROWSER_ACTOR_MAX_BYTES};
use semio_hub_trusted_catalog_descriptor::{CodecKey, NativeCodecProviderPackageV1, NativeCodecProviderRequirementV1, NativeCodecProviderSourceV1, SelectedTrustedBundleV1, TrustedCatalogAsset, TrustedRetainedFile, VerifiedDocumentOpenSelectionV1, VerifiedExecutionTargetAssets, VerifiedPluginModule, VerifiedTrustedPackage, declared_kind, decode_package_descriptor, document_open_target_sort_key, hosted_codec_targets, most_general_dialect, open_target_codec_package, owner_preferred_creation, package_actor_renderer, report_package_progress, sort_open_targets, trusted_profile_generation, validate_bundle, validate_descriptor, validate_descriptor_open_target, validate_native_bindings, verify_plugin_module};
use semio_hub_trusted_catalog_contract::{TRUSTED_BUNDLE_MAX_BYTES, TRUSTED_CATALOG_MAX_OPEN_TARGETS, TRUSTED_COMPONENT_MAX_BYTES, TRUSTED_DESCRIPTOR_MAX_BYTES, TRUSTED_IDENTITY_MAX_BYTES, catalog, catalog_error, decode_digest, dual_hash, sha256, verify_digest, verify_length};

/// 🔐️ Publishes only a fully verified selection while holding the cooperating-writer OS fence.
pub struct TrustedCatalogPublisher;

/// 🚦️ Requires callers to distinguish durable selection from reconciliation-only visibility.
pub enum TrustedCatalogPublicationOutcome {
    Durable(Vec<u8>),
    Unconfirmed(Vec<u8>),
}

impl TrustedCatalogPublisher {
    /// 📤️ Consumes the closed command schema under a private server-owned catalog namespace.
    pub async fn publish_current(data_path: &Path, command_bytes: &[u8], providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<TrustedCatalogPublicationOutcome, AuthorityError> {
        context.checkpoint()?;
        if command_bytes.is_empty() || command_bytes.len() > TRUSTED_CATALOG_PUBLICATION_MAX_BYTES { return Err(catalog("trusted publication command exceeds its bound")); }
        let fields: serde_json::Value = serde_json::from_slice(command_bytes).map_err(catalog_error)?;
        if !fields.as_object().is_some_and(|object| object.contains_key("expectedCurrentSha256")) { return Err(catalog("trusted publication expected current token is required")); }
        let command: TrustedCatalogPublicationCommandV1 = serde_json::from_slice(command_bytes).map_err(catalog_error)?;
        if command.schema != TRUSTED_CATALOG_PUBLICATION_SCHEMA || command.request_id.len() != 32 || !command.request_id.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) || command.profile_id.is_empty() || command.profile_id.len() > TRUSTED_IDENTITY_MAX_BYTES || command.profile_id.chars().any(char::is_control) {
            return Err(catalog("trusted publication command has invalid identity"));
        }
        decode_digest(&command.generation_id, "trusted generation id")?;
        let bundle_digest = decode_digest(&command.bundle_sha256, "trusted bundle sha256")?;
        if let Some(expected) = &command.expected_current_sha256 { decode_digest(expected, "trusted expected current sha256")?; }
        let data = TrustedCatalogDataRoot::open_server_owned(data_path)?;
        let owner = data.acquire_publication(context).await?;
        let (observed, revision) = match owner.open_current()? {
            Some(file) => {
                let bytes = file.read_bounded(65_536, context).await?;
                let current = TrustedCatalogCurrentPointerV1::decode(&bytes)?;
                (Some(hex_lower(&sha256(&bytes, context).await?)), publication_revision(&current.publication_revision)?)
            }
            None => (None, 0),
        };
        if observed != command.expected_current_sha256 { return Err(catalog("trusted publication current token is stale")); }
        let revision = revision.checked_add(1).ok_or_else(|| catalog("trusted publication revision exhausted"))?;
        let generation = Arc::new(owner.open_generation(&command.generation_id)?);
        let relative = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation.read_regular(&relative, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != bundle_digest { return Err(catalog("trusted publication candidate bundle differs from its digest")); }
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let (verified, _) = TrustedCatalogLoader::verify_selected(&generation, relative, bundle_bytes, &command.profile_id, providers, &GuestCodecVerificationCacheV1::beside(data_path), context).await?;
        if verified.generation_id() != command.generation_id { return Err(catalog("trusted publication candidate generation differs from the verified profile")); }
        let mut published_module_files = BTreeSet::new();
        for package in &verified.packages {
            let record = bundle.packages.iter().find(|record| record.plugin_id == package.plugin_id && record.package_id == package.package.package.0 && record.version == package.version).ok_or_else(|| catalog("verified publication package lost its bundle record"))?;
            let mut files = vec![TrustedBundleFileV1 { path: record.component.path.clone(), byte_length: record.component.byte_length, sha256: record.component.sha256.clone() }, record.descriptor.clone()];
            if let Some(actor) = record.browser_actor.file() { files.push(actor); }
            files.push(TrustedBundleFileV1 { path: record.plugin_module.path.clone(), byte_length: record.plugin_module.byte_length, sha256: record.plugin_module.sha256.clone() });
            for file in &package.plugin_module.bundle.files {
                if published_module_files.insert(file.sha256.clone()) { files.push(TrustedBundleFileV1 { path: plugin_module_blob_path(&file.sha256), byte_length: file.byte_length, sha256: file.sha256.clone() }); }
            }
            for file in files {
                let bytes = generation.read_regular(&TrustedCatalogRelativePathV1::parse(&file.path)?, file.byte_length, context).await?;
                if bytes.len() as u64 != file.byte_length || sha256(&bytes, context).await? != decode_digest(&file.sha256, "trusted publication leaf sha256")? { return Err(catalog("trusted publication leaf changed after candidate verification")); }
            }
        }
        let final_bundle = generation.read_regular(&TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&final_bundle, context).await? != bundle_digest { return Err(catalog("trusted publication bundle changed after candidate verification")); }
        let pointer = TrustedCatalogCurrentPointerV1 { profile_id: command.profile_id, generation_id: command.generation_id, bundle_sha256: command.bundle_sha256, publication_revision: revision.to_string() };
        let current_bytes = pointer.encode()?;
        let current_sha256 = hex_lower(&sha256(&current_bytes, context).await?);
        let sync = owner.replace_current(&command.request_id, &current_bytes, context)?;
        let receipt = TrustedCatalogPublicationReceiptV1 { schema: TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA, request_id: command.request_id, profile_id: pointer.profile_id, generation_id: pointer.generation_id, bundle_sha256: pointer.bundle_sha256, publication_revision: pointer.publication_revision, current_sha256, outcome: match sync { semio_hub_trusted_catalog_contract::TrustedPublicationSync::Durable => TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE, semio_hub_trusted_catalog_contract::TrustedPublicationSync::Unconfirmed => TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED } };
        let bytes = serde_json::to_vec(&receipt).map_err(catalog_error)?;
        Ok(match sync { semio_hub_trusted_catalog_contract::TrustedPublicationSync::Durable => TrustedCatalogPublicationOutcome::Durable(bytes), semio_hub_trusted_catalog_contract::TrustedPublicationSync::Unconfirmed => TrustedCatalogPublicationOutcome::Unconfirmed(bytes) })
    }
}

/// ⛽️ Fuel and wall-clock ceiling for ONE guest codec call. A `codec` function is pure and bounded
/// (encode a snapshot, print a history log, reduce an op batch), so a call that needs more than this
/// is refused rather than allowed to hold a request thread. The fuel figure is the same order the
/// build-time `describe()` cap sits at, measured on a real debug-built component.
pub const GUEST_CODEC_BUDGET: semio_framework::kernel::Budget =
    semio_framework::kernel::Budget { fuel: 4_000_000_000, deadline_ms: 30_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 };

/// 🧊️ Compiled values held within an operator-configured memory budget (`TrustedCatalogGuestResidencyV1`):
/// every value is charged its source's byte length, and every operation ([`OperationContext`]) that uses a
/// value counts as one use of it however many calls it makes. A newly compiled value that fits stays resident.
/// One that does not fit stays only when its uses before this operation outnumber those of every value it
/// would release — the least recently used values no call holds, taken in that order — and otherwise serves
/// the operation that compiled it (and any other that reaches it meanwhile) and is dropped with it. Use counts
/// are capped and halve together after a declared number of uses per registered value, so a round-robin over
/// more values than fit keeps a stable resident set instead of recompiling every value on every use, while a
/// workload that moved on displaces the values it left. Nothing is released for idleness, a value a running
/// call holds is never released, and a released value compiles again on its next use.
pub struct GuestResidencyLedgerV1<T> {
    pub budget_bytes: std::sync::atomic::AtomicU64,
    pub access_count_ceiling: u32,
    pub access_count_aging_per_guest: u64,
    clock: std::sync::atomic::AtomicU64,
    accesses_since_aging: std::sync::atomic::AtomicU64,
    pub hits: std::sync::atomic::AtomicU64,
    pub compiles: std::sync::atomic::AtomicU64,
    pub admitted: std::sync::atomic::AtomicU64,
    pub bypassed: std::sync::atomic::AtomicU64,
    pub released: std::sync::atomic::AtomicU64,
    pub compile_micros: std::sync::atomic::AtomicU64,
    slots: std::sync::Mutex<Vec<Arc<GuestResidencySlotV1<T>>>>,
}

/// 📏️ What a resident value holds beyond the bytes it was registered with, measured on the value itself once it exists
/// and again after every codec call that used it — a guest's state can grow with its first call.
pub trait GuestResidentFootprintV1 {
    /// 📐️ The bytes the value holds beyond its registration charge.
    fn footprint_bytes(&self) -> u64;
}

impl GuestResidentFootprintV1 for semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest {
    /// 🧊️ Beyond its parsed component (which its registration charges) a compiled guest holds the owned interpreter's codec
    /// origin once a codec call assembled it: the guest's linear memory right after its plugin bundle assembled.
    fn footprint_bytes(&self) -> u64 {
        self.codec_origin_bytes()
    }
}

/// 🧩️ What one slot holds: nothing, a resident value, or a value that serves only the calls holding it.
enum GuestResidentValueV1<T> {
    Absent,
    Resident(Arc<T>),
    Held(std::sync::Weak<T>),
}

struct GuestResidencySlotV1<T> {
    value: tokio::sync::Mutex<GuestResidentValueV1<T>>,
    charge_bytes: u64,
    footprint_bytes: std::sync::atomic::AtomicU64,
    last_used: std::sync::atomic::AtomicU64,
    access_count: std::sync::atomic::AtomicU32,
    last_operation: std::sync::atomic::AtomicU64,
    prior_uses: std::sync::atomic::AtomicU32,
}

impl<T> GuestResidencySlotV1<T> {
    fn counts_as_resident(&self) -> bool {
        self.value.try_lock().map_or(true, |value| match &*value {
            GuestResidentValueV1::Resident(_) => true,
            GuestResidentValueV1::Held(held) => held.strong_count() > 0,
            GuestResidentValueV1::Absent => false,
        })
    }

    fn charged(&self) -> bool {
        self.value.try_lock().map_or(true, |value| matches!(&*value, GuestResidentValueV1::Resident(_)))
    }

    /// 📏️ What the slot's value is charged: its registered bytes and its measured footprint.
    fn charge(&self) -> u64 {
        self.charge_bytes.saturating_add(self.footprint_bytes.load(std::sync::atomic::Ordering::Acquire))
    }
}

impl<T> GuestResidencyLedgerV1<T> {
    pub fn new(residency: TrustedCatalogGuestResidencyV1) -> Arc<Self> {
        let counter = || std::sync::atomic::AtomicU64::new(0);
        Arc::new(Self {
            budget_bytes: std::sync::atomic::AtomicU64::new(residency.resident_component_bytes),
            access_count_ceiling: residency.access_count_ceiling,
            access_count_aging_per_guest: residency.access_count_aging_per_guest.max(1),
            clock: counter(),
            accesses_since_aging: counter(),
            hits: counter(),
            compiles: counter(),
            admitted: counter(),
            bypassed: counter(),
            released: counter(),
            compile_micros: counter(),
            slots: std::sync::Mutex::new(Vec::new()),
        })
    }

    /// 🪪️ Registers one value charged `charge_bytes` once resident; answers its residency handle.
    pub fn register(self: &Arc<Self>, charge_bytes: u64) -> GuestResidencyV1<T> {
        let slot = Arc::new(GuestResidencySlotV1 {
            value: tokio::sync::Mutex::new(GuestResidentValueV1::Absent),
            charge_bytes,
            footprint_bytes: std::sync::atomic::AtomicU64::new(0),
            last_used: std::sync::atomic::AtomicU64::new(0),
            access_count: std::sync::atomic::AtomicU32::new(0),
            last_operation: std::sync::atomic::AtomicU64::new(0),
            prior_uses: std::sync::atomic::AtomicU32::new(0),
        });
        self.slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(Arc::clone(&slot));
        GuestResidencyV1 { ledger: Arc::clone(self), slot }
    }

    fn snapshot(&self) -> Vec<Arc<GuestResidencySlotV1<T>>> {
        self.slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    /// 🎚️ Applies a new budget and releases least recently used values no call holds until the resident
    /// charge fits it.
    pub(crate) fn configure(&self, budget_bytes: u64) {
        self.budget_bytes.store(budget_bytes, std::sync::atomic::Ordering::Release);
        let mut slots = self.snapshot();
        slots.sort_by_key(|slot| slot.last_used.load(std::sync::atomic::Ordering::Acquire));
        let mut charged: u64 = slots.iter().filter(|slot| slot.charged()).map(|slot| slot.charge()).sum();
        for slot in slots {
            if charged <= budget_bytes {
                break;
            }
            let Ok(mut value) = slot.value.try_lock() else { continue };
            if matches!(&*value, GuestResidentValueV1::Resident(resident) if Arc::strong_count(resident) == 1) {
                *value = GuestResidentValueV1::Absent;
                charged = charged.saturating_sub(slot.charge());
                self.released.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            }
        }
    }

    /// 🕰️ Records one call of `operation`: the slot becomes the most recently used, and the operation's first
    /// call counts one use (capped); every `accessCountAgingPerGuest × registered` uses all counts halve.
    /// Answers the slot's uses before this operation and whether this call was the operation's first.
    fn touch(&self, slot: &GuestResidencySlotV1<T>, operation: u64) -> (u32, bool) {
        let tick = self.clock.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;
        slot.last_used.store(tick, std::sync::atomic::Ordering::Release);
        if slot.last_operation.swap(operation, std::sync::atomic::Ordering::AcqRel) == operation {
            return (slot.prior_uses.load(std::sync::atomic::Ordering::Acquire), false);
        }
        let slots = self.snapshot();
        let window = self.access_count_aging_per_guest.saturating_mul(u64::try_from(slots.len()).unwrap_or(u64::MAX).max(1));
        let aged = self.accesses_since_aging.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |uses| Some(if uses + 1 >= window { 0 } else { uses + 1 })).is_ok_and(|uses| uses + 1 >= window);
        if aged {
            for slot in slots {
                let _ = slot.access_count.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| Some(count / 2));
            }
        }
        let ceiling = self.access_count_ceiling;
        let prior = slot.access_count.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| (count < ceiling).then_some(count + 1)).unwrap_or_else(|count| count);
        slot.prior_uses.store(prior, std::sync::atomic::Ordering::Release);
        (prior, true)
    }

    /// ⚖️ Whether `keep`'s compiled value stays resident: it fits, or its `prior` uses outnumber those of every
    /// least recently used unheld value it would release, which are then released. Never admits a value larger
    /// than the whole budget. A slot another call is compiling or acquiring right now counts as charged.
    fn admit(&self, keep: &Arc<GuestResidencySlotV1<T>>, prior: u32) -> bool {
        let budget = self.budget_bytes.load(std::sync::atomic::Ordering::Acquire);
        if keep.charge() > budget {
            return false;
        }
        let slots = self.snapshot();
        let others: Vec<&Arc<GuestResidencySlotV1<T>>> = slots.iter().filter(|slot| !Arc::ptr_eq(slot, keep)).collect();
        let charged: u64 = others.iter().filter(|slot| slot.charged()).map(|slot| slot.charge()).sum();
        let excess = charged.saturating_add(keep.charge()).saturating_sub(budget);
        if excess == 0 {
            return true;
        }
        let mut candidates: Vec<(&Arc<GuestResidencySlotV1<T>>, tokio::sync::MutexGuard<'_, GuestResidentValueV1<T>>)> = others
            .iter()
            .filter_map(|slot| slot.value.try_lock().ok().filter(|value| matches!(&**value, GuestResidentValueV1::Resident(resident) if Arc::strong_count(resident) == 1)).map(|value| (*slot, value)))
            .collect();
        candidates.sort_by_key(|(slot, _)| slot.last_used.load(std::sync::atomic::Ordering::Acquire));
        let (mut freed, mut victim_count, mut hottest_victim) = (0u64, 0usize, 0u32);
        for (slot, _) in &candidates {
            if freed >= excess {
                break;
            }
            freed = freed.saturating_add(slot.charge());
            victim_count += 1;
            hottest_victim = hottest_victim.max(slot.access_count.load(std::sync::atomic::Ordering::Acquire));
        }
        if freed < excess || prior <= hottest_victim {
            return false;
        }
        for (_, value) in candidates.iter_mut().take(victim_count) {
            **value = GuestResidentValueV1::Absent;
        }
        self.released.fetch_add(u64::try_from(victim_count).unwrap_or(u64::MAX), std::sync::atomic::Ordering::AcqRel);
        true
    }

    /// 📏️ The ledger's state now.
    pub fn state(&self) -> TrustedCatalogGuestResidencyStateV1 {
        let slots = self.snapshot();
        let resident: Vec<_> = slots.iter().filter(|slot| slot.counts_as_resident()).collect();
        let load = |counter: &std::sync::atomic::AtomicU64| counter.load(std::sync::atomic::Ordering::Acquire);
        TrustedCatalogGuestResidencyStateV1 {
            budget_bytes: load(&self.budget_bytes),
            registered_guests: u64::try_from(slots.len()).unwrap_or(u64::MAX),
            resident_guests: u64::try_from(resident.len()).unwrap_or(u64::MAX),
            resident_bytes: resident.iter().map(|slot| slot.charge()).sum(),
            footprint_bytes: resident.iter().map(|slot| slot.footprint_bytes.load(std::sync::atomic::Ordering::Acquire)).sum(),
            hits: load(&self.hits),
            compiles: load(&self.compiles),
            admitted: load(&self.admitted),
            bypassed: load(&self.bypassed),
            released: load(&self.released),
            compile_micros: load(&self.compile_micros),
        }
    }
}

/// 🔑️ One value's place in a [`GuestResidencyLedgerV1`].
pub struct GuestResidencyV1<T> {
    ledger: Arc<GuestResidencyLedgerV1<T>>,
    pub slot: Arc<GuestResidencySlotV1<T>>,
}

impl<T: GuestResidentFootprintV1 + Send + Sync + 'static> GuestResidencyV1<T> {
    /// 🔑️ The resident or held value for one call of `context`'s operation, compiled by `compile` when there is
    /// none (one compile at a time). A value that is not admitted stays with the operation that compiled it — and
    /// with any other operation that reaches it meanwhile — and is dropped with the last of them; a held value
    /// is admitted as soon as its uses outnumber the values it would release.
    pub async fn acquire<F, Fut>(&self, context: &OperationContext<'_>, compile: F) -> Result<Arc<T>, AuthorityError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AuthorityError>>,
    {
        let mut value = self.slot.value.lock().await;
        let (prior, first_call) = self.ledger.touch(&self.slot, context.serial());
        let present = match &*value {
            GuestResidentValueV1::Resident(resident) => Some((Arc::clone(resident), true)),
            GuestResidentValueV1::Held(held) => held.upgrade().map(|held| (held, false)),
            GuestResidentValueV1::Absent => None,
        };
        if let Some((present, resident)) = present {
            self.ledger.hits.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            if !resident {
                if self.ledger.admit(&self.slot, prior) {
                    self.ledger.admitted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    *value = GuestResidentValueV1::Resident(Arc::clone(&present));
                } else if first_call {
                    context.retain(Arc::clone(&present) as Arc<dyn std::any::Any + Send + Sync>);
                }
            }
            return Ok(present);
        }
        let started = std::time::Instant::now();
        let compiled = Arc::new(compile().await?);
        self.ledger.compile_micros.fetch_add(u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX), std::sync::atomic::Ordering::AcqRel);
        self.ledger.compiles.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.slot.footprint_bytes.store(compiled.footprint_bytes(), std::sync::atomic::Ordering::Release);
        *value = if self.ledger.admit(&self.slot, prior) {
            self.ledger.admitted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            GuestResidentValueV1::Resident(Arc::clone(&compiled))
        } else {
            self.ledger.bypassed.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            context.retain(Arc::clone(&compiled) as Arc<dyn std::any::Any + Send + Sync>);
            GuestResidentValueV1::Held(Arc::downgrade(&compiled))
        };
        Ok(compiled)
    }

    /// 📏️ Charges `value` what it holds now ([`GuestResidentFootprintV1`]) after a call of `context`'s operation used it.
    /// A resident value that grew past what the budget has left stays resident only when its uses before this operation
    /// outnumber those of every least recently used unheld value it would release — which are then released — and
    /// otherwise serves the operations holding it (this one among them) and is dropped with the last of them. A held value
    /// is charged its growth when it is next admitted; a value the slot no longer holds changes nothing.
    pub async fn remeasure(&self, value: &Arc<T>, context: &OperationContext<'_>) {
        let footprint = value.footprint_bytes();
        if self.slot.footprint_bytes.load(std::sync::atomic::Ordering::Acquire) == footprint {
            return;
        }
        let mut held = self.slot.value.lock().await;
        let resident = match &*held {
            GuestResidentValueV1::Resident(resident) if Arc::ptr_eq(resident, value) => true,
            GuestResidentValueV1::Held(weak) if std::ptr::eq(weak.as_ptr(), Arc::as_ptr(value)) => false,
            GuestResidentValueV1::Resident(_) | GuestResidentValueV1::Held(_) | GuestResidentValueV1::Absent => return,
        };
        self.slot.footprint_bytes.store(footprint, std::sync::atomic::Ordering::Release);
        if !resident || self.ledger.admit(&self.slot, self.slot.prior_uses.load(std::sync::atomic::Ordering::Acquire)) {
            return;
        }
        *held = GuestResidentValueV1::Held(Arc::downgrade(value));
        context.retain(Arc::clone(value) as Arc<dyn std::any::Any + Send + Sync>);
        self.ledger.released.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

impl<T> GuestResidencyV1<T> {
    /// 📏️ Whether a compiled value stays resident now (not merely held by a running operation).
    #[cfg(test)]
    pub fn is_resident(&self) -> bool {
        self.slot.charged()
    }
}

/// 📈️ The live [`TrustedCatalogLoadProgressV1`] of one catalog: its load and its background verification
/// report into it, and the hub reads it for `/readyz` while it starts and for its admin observability route.
#[derive(Default)]
pub struct TrustedCatalogLoadProgressCellV1 {
    pub progress: std::sync::Mutex<TrustedCatalogLoadProgressV1>,
}

impl TrustedCatalogLoadProgressCellV1 {
    /// 📸️ The progress now.
    pub fn snapshot(&self) -> TrustedCatalogLoadProgressV1 {
        self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    pub fn update(&self, change: impl FnOnce(&mut TrustedCatalogLoadProgressV1)) {
        let mut progress = self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        change(&mut progress);
        progress.packages_total = u64::try_from(progress.packages.len()).unwrap_or(u64::MAX);
        progress.packages_ready = u64::try_from(progress.packages.iter().filter(|package| package.phase == TrustedCatalogPackagePhaseV1::Ready).count()).unwrap_or(u64::MAX);
        progress.packages_refused = u64::try_from(progress.packages.iter().filter(|package| package.phase == TrustedCatalogPackagePhaseV1::Refused).count()).unwrap_or(u64::MAX);
        progress.component_bytes_total = progress.packages.iter().map(|package| package.component_bytes).sum();
        progress.rows_total = progress.packages.iter().map(|package| package.rows).sum();
        progress.rows_pinned = progress.packages.iter().map(|package| package.rows_pinned).sum();
        progress.rows_verified = progress.packages.iter().map(|package| package.rows_verified).sum();
    }

    pub fn select(&self, packages: Vec<TrustedCatalogPackageProgressV1>) {
        self.update(|progress| *progress = TrustedCatalogLoadProgressV1 { packages, ..TrustedCatalogLoadProgressV1::default() });
    }

    pub fn package(&self, position: usize, change: impl FnOnce(&mut TrustedCatalogPackageProgressV1)) {
        self.update(|progress| {
            if let Some(package) = progress.packages.get_mut(position) {
                change(package);
            }
        });
    }

    pub fn phase(&self, position: usize, phase: TrustedCatalogPackagePhaseV1) {
        self.package(position, |package| package.phase = phase);
    }

    fn component_read(&self, position: usize) {
        self.update(|progress| {
            if let Some(bytes) = progress.packages.get(position).map(|package| package.component_bytes) {
                progress.component_bytes_read = progress.component_bytes_read.saturating_add(bytes);
            }
        });
    }
}

/// 🗜️ One verified package's actor, compiled when a document operation needs it and kept within the
/// catalog's residency budget ([`GuestResidencyLedgerV1`]). Its codec rows that neither a linked native
/// codec nor this engine's verification memory pinned at load (`rows`) are pinned against the component's
/// own `pack-schema-hash` once, before its first codec call — by the catalog's background verification or
/// by that call, whichever comes first — so a hub serves without interpreting its whole catalog first and
/// no codec call ever runs on a row its component did not answer as the trust record says.
struct GuestArtifactComponent {
    runtime: Arc<semio_framework_plugin_interpreter_runtime::OwnedRuntime>,
    package: PackageRef,
    component: TrustedCatalogAsset,
    compiled: GuestResidencyV1<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>,
    position: usize,
    component_sha256: [u8; 32],
    rows: Vec<(String, [u8; 32])>,
    verified: tokio::sync::OnceCell<Result<(), String>>,
    verifications: Arc<GuestCodecVerificationCacheV1>,
    progress: Arc<TrustedCatalogLoadProgressCellV1>,
}

impl GuestArtifactComponent {
    async fn compiled(&self, context: &OperationContext<'_>) -> Result<Arc<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>, AuthorityError> {
        self.compiled
            .acquire(context, || async {
                context.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCompiling, completed_units: 0, total_units: 1 })?;
                let bytes = self.component.read(context).await?;
                let runtime = Arc::clone(&self.runtime);
                let compiled = interpret_off_worker(context, move |_handle, _progress, _cancellation| runtime.compile_component(&bytes).map_err(semio_framework_plugin_guest_contract::TurnFault::Host)).await?.map_err(|error| catalog_error(format!("{}: {error}", self.package.package.0)))?;
                context.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCompiling, completed_units: 1, total_units: 1 })?;
                Ok(compiled)
            })
            .await
    }

    /// 🔐️ Every row of `rows` pinned against this component's own answer, once for the catalog's lifetime. A
    /// caller that cancels or stalls leaves the rows for the next caller; a component that answers a row
    /// differently from its trust record, or does not compile, refuses every codec call of its package.
    async fn verified(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        if self.rows.is_empty() {
            return Ok(());
        }
        let outcome = self.verified.get_or_try_init(|| self.verify_once(context)).await?;
        outcome.clone().map_err(AuthorityError::Catalog)
    }

    /// 🔐️ One verification attempt: its phase in the catalog progress, and its terminal outcome — `Ok(Err(..))`
    /// is a refusal the package keeps, `Err` a cancellation or stall the next caller retries.
    async fn verify_once(&self, context: &OperationContext<'_>) -> Result<Result<(), String>, AuthorityError> {
        self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Verifying);
        match self.verify_rows(context).await {
            Ok(()) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Ready);
                Ok(Ok(()))
            }
            Err(error @ (AuthorityError::Cancelled | AuthorityError::Stalled | AuthorityError::DeadlineExceeded)) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Staged);
                Err(error)
            }
            Err(AuthorityError::Catalog(refusal)) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Refused);
                Ok(Err(refusal))
            }
            Err(error) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Refused);
                Ok(Err(error.to_string()))
            }
        }
    }

    /// 🔐️ Interprets every pending row on one compile, several at once ([`guest_verification_concurrency`]).
    async fn verify_rows(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        use futures::StreamExt;
        let schemas = self.rows.iter().map(|(schema, _)| schema.as_str()).collect::<Vec<_>>().join(", ");
        let compiled = self.compiled(context).await.map_err(|error| catalog_error(format!("{schemas}: {error}")))?;
        let pending: Vec<_> = self.rows.iter().map(|(schema, expected)| self.verify_row(Arc::clone(&compiled), schema, *expected, context)).collect();
        let mut running = futures::stream::iter(pending).buffer_unordered(guest_verification_concurrency());
        let mut verified = Ok(());
        while let Some(row) = running.next().await {
            if let Err(error) = row {
                verified = Err(error);
                break;
            }
        }
        drop(running);
        self.compiled.remeasure(&compiled, context).await;
        verified
    }

    /// 🔐️ One row: the component's own `pack-schema-hash`, compared with the trust record and remembered for this engine.
    async fn verify_row(&self, compiled: Arc<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>, schema: &str, expected: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let (runtime, row_schema) = (Arc::clone(&self.runtime), schema.to_string());
        let observed = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_pack_schema_hash_observed(&compiled, &row_schema, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation)))
            .await?
            .map_err(|error| catalog_error(format!("{schema}: {error}")))?;
        context.checkpoint()?;
        if observed != expected {
            return Err(catalog(&format!("{schema}: guest artifact codec schema hash differs from its trust record")));
        }
        self.verifications.remember(&self.component_sha256, schema, &observed).await;
        self.progress.package(self.position, |package| package.rows_verified = package.rows_verified.saturating_add(1));
        Ok(())
    }
}

/// 🧩️ The component half of one artifact identity: the package's own actor plus the document
/// schema that selects the owning app inside it. Every verified codec carries one, because `genesis`
/// now comes from the component for EVERY package (ticket 26/09/18 slice TC3b) — the compiled-in
/// stdio/gis/vcs genesis table it replaced could only ever answer for three packages.
pub struct GuestArtifactCodecBinding {
    component: Arc<GuestArtifactComponent>,
    artifact_schema: String,
}

impl GuestArtifactCodecBinding {
    /// 🌱️ The guest's canonical empty document. The call is the longest single thing a creation
    /// does — the interpreter walks a ≈ 48 MB component's whole app bundle — so the guest's own
    /// fuel progress is reported into the caller's context as it happens: under a stall bound a
    /// checkpoint is what says "still moving", and without these an honest interpreter would look
    /// exactly like a wedged one. A caller that stops waiting ends the interpretation at its next
    /// step ([`interpret_off_worker`]).
    async fn genesis(&self, document_id: &str, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, document_id, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), document_id.to_string(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_genesis_observed(&guest, &schema, &document_id, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))).await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let pair = answer.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: pair.pack, spr: pair.spr })
    }

    async fn print_mirror(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        let (runtime, schema, pack, spr, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), pair.pack.clone(), pair.spr.clone(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_print_mirror_observed(&guest, &schema, &pack, &spr, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))).await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let mirror = answer.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        if mirror.dsl.len().checked_add(mirror.ops.len()).is_none_or(|length| length > AUTHORITY_MAX_CODEC_TEXT_BYTES) {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        Ok(())
    }

    async fn apply_ops(&self, pair: ArtifactPair, encoded: Vec<u8>, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_apply_ops_observed(&guest, &schema, &pair.pack, &pair.spr, &encoded, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))).await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let next = answer.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: next.pack, spr: next.spr })
    }

    /// 📜️ The guest's own replica fold of a ledger stream; fuel progress reaches the caller's stall
    /// bound exactly as [`Self::genesis`]'s does.
    async fn replay_envelopes(&self, pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, envelopes, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), envelopes.to_vec(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| {
            handle.block_on(runtime.codec_replay_envelopes_observed(&guest, &schema, &pair.pack, &pair.spr, &envelopes, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))
        })
        .await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let next = answer.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: next.pack, spr: next.spr })
    }
}

/// 🧵️ Interprets one guest codec call on the hub runtime's blocking pool, never on the async worker
/// that awaits it: a call interprets for seconds to minutes, and on a worker it would stall every
/// request, socket and timer queued behind it (a sign-in, a presence heartbeat, an edit's ack). The
/// call's fuel observations reach the caller's context while it runs — each one a checkpoint of its
/// stall bound — and the call observes its caller: once the caller is released — refused at an
/// observation, cancelled (seen within [`GUEST_CALL_CANCELLATION_POLL`] between observations), or
/// dropped mid-await like the background verification a hub shutdown aborts — the interpreter ends
/// the call at its next step, so no interpretation outlives the operation that asked for it and a
/// runtime being dropped waits for none. The outer result is the caller's own outcome, the inner one
/// the guest's answer. Guest codec calls belong to the hub's runtime; outside one they are refused.
pub async fn interpret_off_worker<T, F>(context: &OperationContext<'_>, call: F) -> Result<Result<T, semio_framework_plugin_guest_contract::TurnFault>, AuthorityError>
where
    T: Send + 'static,
    F: FnOnce(&tokio::runtime::Handle, &mut dyn FnMut(u64), &semio_framework_plugin_guest_contract::GuestCallCancellation) -> Result<T, semio_framework_plugin_guest_contract::TurnFault> + Send + 'static,
{
    let handle = tokio::runtime::Handle::try_current().map_err(|_| catalog("guest codec calls run on the hub runtime"))?;
    let (sender, mut observations) = tokio::sync::mpsc::unbounded_channel::<u64>();
    let released = GuestCallRelease(semio_framework_plugin_guest_contract::GuestCallCancellation::default());
    let (blocking, cancellation) = (handle.clone(), released.0.clone());
    let joined = handle.spawn_blocking(move || {
        call(
            &blocking,
            &mut |fuel| {
                let _ = sender.send(fuel);
            },
            &cancellation,
        )
    });
    loop {
        match tokio::time::timeout(GUEST_CALL_CANCELLATION_POLL, observations.recv()).await {
            Ok(Some(fuel)) => context.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: fuel.min(GUEST_CODEC_BUDGET.fuel), total_units: GUEST_CODEC_BUDGET.fuel })?,
            Ok(None) => break,
            Err(_) if context.is_cancelled() => return Err(AuthorityError::Cancelled),
            Err(_) => {}
        }
    }
    Ok(joined.await.unwrap_or_else(|error| Err(semio_framework_plugin_guest_contract::TurnFault::Trapped(format!("guest codec call ended abnormally: {error}")))))
}

/// ⏲️ How often a caller awaiting a guest call between two fuel observations (one per 25 M fuel or 5 s)
/// looks at its own cancellation.
pub const GUEST_CALL_CANCELLATION_POLL: std::time::Duration = std::time::Duration::from_millis(50);

/// 🧯️ The awaiting side of one guest call: however its caller is released — answered, refused, cancelled or
/// dropped mid-await — the call is cancelled, which ends its interpretation at the next step (a call that already
/// returned is unaffected).
struct GuestCallRelease(semio_framework_plugin_guest_contract::GuestCallCancellation);

impl Drop for GuestCallRelease {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

/// 🧪️ One immutable authority identity bound to its executable. `codec` is `Some` only for a kind
/// this binary links a Rust codec for (stdio import/export and GIS inference still run natively, TC2
/// §9) — its owner's, or a host's for a kind it hosts from that owner ([`hosted_codec_targets`]);
/// every other identity validates and applies through its package's own component. `guest` is never
/// optional: it is the only creation authority there is, always the identity's own package.
pub struct VerifiedNativeArtifactCodec {
    identity: TrustedArtifactIdentity,
    codec: Option<ArtifactCodec>,
    guest: GuestArtifactCodecBinding,
    owner_declaration: ArtifactCatalogBinding,
}

impl VerifiedNativeArtifactCodec {
    /// 🏛️ Returns the exact original codec-owner declaration, including for hosted aliases.
    pub fn owner_declaration(&self)->&ArtifactCatalogBinding{&self.owner_declaration}
}

impl TrustedArtifactCodec for VerifiedNativeArtifactCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let Some(codec) = &self.codec else {
            self.guest.print_mirror(pair, stage, context).await?;
            return context.checkpoint();
        };
        let mirror = (codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        if mirror.dsl.len().checked_add(mirror.ops.len()).is_none_or(|length| length > AUTHORITY_MAX_CODEC_TEXT_BYTES) {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()
    }

    async fn apply_operation(&self, pair: ArtifactPair, operation: &AcceptedArtifactOperation, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let maximum=usize::try_from(context.limits().max_pair_bytes).ok().and_then(|bytes|bytes.checked_mul(16)).and_then(|bytes|bytes.checked_add(operation.encoded.len().checked_mul(16)?)).ok_or(AuthorityError::ResourceLimit("identity source capacity"))?;
        let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|{context.report(semio_hub_artifact_authority::AuthorityProgress{stage:semio_hub_artifact_authority::AuthorityProgressStage::ApplyingOperations,completed_units:operation.sequence,total_units:context.limits().max_operations as u64}).is_ok()};
        let mut identity=directory::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum,&mut observer).map_err(|error|AuthorityError::Codec{stage:ArtifactValidationStage::Input,message:bounded_message(error)})?;
        let encoded=identity.encode(|control|directory::os_spr::io::binary::operation_sequence::encode(std::slice::from_ref(&operation.encoded),control)).map_err(|error|AuthorityError::Codec{stage:ArtifactValidationStage::Input,message:bounded_message(error)})?;
        let Some(codec) = &self.codec else {
            let next = self.guest.apply_ops(pair, encoded, context).await?;
            context.checkpoint()?;
            return Ok(next);
        };
        let (pack, spr, ops) = (codec.apply_ops_binary)(&pair.pack, &pair.spr, &encoded, &mut identity).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

impl TrustedArtifactReplayCodec for VerifiedNativeArtifactCodec {
    async fn replay_envelopes(&self, pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let Some(codec) = &self.codec else {
            let next = self.guest.replay_envelopes(pair, envelopes, context).await?;
            context.checkpoint()?;
            return Ok(next);
        };
        let (pack, spr, ops) = (codec.replay_envelopes)(&pair.pack, &pair.spr, envelopes).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

impl TrustedArtifactGenesisCodec for VerifiedNativeArtifactCodec {
    /// 🌱️ Creation authority is the COMPONENT's, always. The `dialect` the caller carries is not
    /// passed to the guest: the guest stamps its own app's dialect, and a genesis whose dialect
    /// differs from the catalog's open target is refused here rather than silently accepted.
    async fn initial_pair(&self, document_id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let pair = self.guest.genesis(document_id, context).await?;
        context.checkpoint()?;
        let parsed = directory::os_spr::decode_history(&pair.spr, &directory::os_spr::DecodeOptions::default()).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: bounded_message(error) })?;
        if parsed.doc_id != document_id || !parsed.edits.is_empty() || !parsed.transitions.is_empty() || !parsed.conflicts.is_empty() {
            return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: "guest genesis is not a zero-history document of the requested identity".into() });
        }
        let _ = dialect;
        context.checkpoint()?;
        Ok(pair)
    }
}

/// 🗂️ Process-lifetime snapshot produced only after complete bundle verification and codec activation.
pub struct VerifiedTrustedCatalog {
    pub residency: Arc<GuestResidencyLedgerV1<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>>,
    guests: Box<[Arc<GuestArtifactComponent>]>,
    pub progress: Arc<TrustedCatalogLoadProgressCellV1>,
    pub packages: Box<[VerifiedTrustedPackage]>,
    pub codecs: Box<[VerifiedNativeArtifactCodec]>,
    artifact_kind_count: usize,
    pub open_targets: Box<[VerifiedDocumentOpenSelectionV1]>,
    pub generation_id: String,
}

impl VerifiedTrustedCatalog {
    /// 📦️ Returns selected packages in deterministic dependency-first order.
    pub fn packages(&self) -> &[VerifiedTrustedPackage] {
        &self.packages
    }

    /// 📏️ What the compiled-guest residency holds now and has done since the catalog loaded.
    pub fn guest_residency(&self) -> TrustedCatalogGuestResidencyStateV1 {
        self.residency.state()
    }

    /// 🎚️ Applies the operator's residency budget (`OS_HUB_GUEST_RESIDENCY_BYTES`), releasing least recently
    /// used unheld guests until the resident charge fits it.
    pub fn configure_guest_residency(&self, residency: TrustedCatalogGuestResidencyV1) {
        self.residency.configure(residency.resident_component_bytes);
    }

    /// 📈️ How far this catalog's load and codec-row verification have come.
    pub fn load_progress(&self) -> TrustedCatalogLoadProgressV1 {
        self.progress.snapshot()
    }

    /// 🔐️ Pins every package's rows that still wait for their component's answer, smallest component first, so the
    /// most packages are ready soonest; a codec call that reaches a package first verifies it itself and this
    /// pass waits for it. A refused package is recorded in [`Self::load_progress`] and the pass moves on; the
    /// pass stops at cancellation or a stall of `context`.
    pub async fn verify_pending_guests(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        let mut pending: Vec<&Arc<GuestArtifactComponent>> = self.guests.iter().filter(|guest| !guest.rows.is_empty() && guest.verified.get().is_none()).collect();
        pending.sort_by_key(|guest| guest.component.byte_length());
        for guest in pending {
            context.checkpoint()?;
            match guest.verified(context).await {
                Ok(()) | Err(AuthorityError::Catalog(_)) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }

    /// 🧪️ Returns the exact number of activated artifact identities.
    pub fn codec_count(&self) -> usize {
        self.codecs.len()
    }

    /// 🧮️ Returns the distinct artifact kinds across all activated schema-qualified identities.
    pub fn artifact_kind_count(&self) -> usize {
        self.artifact_kind_count
    }

    /// 🪪 Returns the number of exact catalog-backed open choices in this immutable generation.
    pub fn open_target_count(&self) -> usize {
        self.open_targets.len()
    }

    /// 🧬 Returns the neutral SHA-256 identity of the sorted immutable open-target projection.
    pub fn generation_id(&self) -> &str {
        &self.generation_id
    }

    /// 📇️ Every plugin module of this generation in ascending plugin order: what
    /// `GET /trusted-catalog/plugin-modules` answers.
    pub fn plugin_module_index(&self) -> TrustedPluginModuleIndexV1 {
        let mut modules = self
            .packages
            .iter()
            .map(|package| TrustedPluginModuleIndexEntryV1 {
                plugin_id: package.plugin_id.clone(),
                package_id: package.package.package.0.clone(),
                version: package.version.clone(),
                component_sha256: hex_lower(&package.component_sha256),
                descriptor_byte_sha256: hex_lower(&package.descriptor_sha256),
                receipt_sha256: hex_lower(&Sha256::digest(&package.receipt.encode_canonical())),
                dependencies: package.dependencies.clone(),
                dialect_artifact_kinds: package.descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).collect::<BTreeSet<_>>().into_iter().collect(),
                extends_plugin_id: (package.descriptor.role == semio_framework::PackageRole::Extension).then(|| package.descriptor.manifest.dependencies.first().map(|dependency| dependency.plugin_id.clone())).flatten(),
                bundle_sha256: package.plugin_module.bundle_sha256.clone(),
                bundle_byte_length: package.plugin_module.manifest_bytes.len() as u64,
                entry: package.plugin_module.bundle.entry.clone(),
            })
            .collect::<Vec<_>>();
        modules.sort_by(|left, right| left.plugin_id.as_bytes().cmp(right.plugin_id.as_bytes()));
        TrustedPluginModuleIndexV1 { schema: TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA.into(), generation_id: self.generation_id.clone(), modules }
    }

    /// 🧩️ The verified plugin module whose manifest has this SHA-256, when this generation carries it.
    pub fn plugin_module(&self, bundle_sha256: &str) -> Option<&VerifiedPluginModule> {
        self.packages.iter().map(|package| &package.plugin_module).find(|module| module.bundle_sha256 == bundle_sha256)
    }

    /// 🎯 Returns the profile's sole completely verified document-open choice without reconstructing it from public plan bytes.
    pub fn selected_document_open(&self) -> Option<&VerifiedDocumentOpenSelectionV1> {
        (self.open_targets.len() == 1).then(|| &self.open_targets[0])
    }

    /// 🌱️ Creation resolves one writable target in this exact admitted generation, OWNER-PREFERRED ([`owner_preferred_creation`]):
    /// the editors of packages that declare the kind decide; only a kind no owner's editor creates falls to the editors of packages
    /// that HOST it (`PluginManifest::hosted_artifact_kinds`). Hosting opens documents, it never re-owns their creation
    /// (demonstrator's embedded cad/gis/procedural/process editors made every creation of those kinds ambiguous, hub 7800 p34,
    /// 2026-09-29). Within the deciding tier the most general dialect creates ([`most_general_dialect`]): a kind whose subset
    /// editors share it with their whole-standard editor (stdio json, xml; the families' jpg, svg, tiff, step, docx, zip, ifc)
    /// is created by the whole-standard editor — the same app the guest answers `codec.genesis` with.
    pub fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {
        let (hosted, owned): (Vec<_>, Vec<_>) = self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor).partition(|selection| self.hosted_row(selection).is_some());
        owner_preferred_creation(owned, hosted, |selection| &selection.parent_dialect).one()
    }

    /// 📦️ The verified package `selection` opens its documents in.
    fn selection_package(&self, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&VerifiedTrustedPackage> {
        self.packages.iter().find(|package| {
            package.plugin_id == selection.package.plugin_id
                && package.package.package.0 == selection.package.package_id
                && package.version == selection.package.version
                && hex_lower(&package.component_sha256) == selection.package.component_sha256
        })
    }

    /// 🏠️ The row by which `selection`'s own package HOSTS its kind (`PluginManifest::hosted_artifact_kinds`); `None` for a
    /// kind the package declares itself.
    fn hosted_row(&self, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&semio_framework::HostedArtifactKind> {
        self.selection_package(selection)?.descriptor.manifest.hosted_artifact_kinds.iter().find(|kind| kind.id == selection.artifact.kind && kind.schema == selection.artifact.schema)
    }

    /// 🏠️ The package that OWNS `selection`'s hosted kind: the hosted row's explicit `owner`, a declared dependency the load
    /// verified before its host.
    fn hosted_kind_owner(&self, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&VerifiedTrustedPackage> {
        let hosted = self.hosted_row(selection)?;
        self.packages.iter().find(|package| package.plugin_id == hosted.owner)
    }

    /// 🗣️ Projects only unambiguous factory-backed choices from retained compiled descriptors. A hosted kind is labelled by its
    /// owner's declaration (a host carries only `{ id, schema, owner }`); a kind no declaration labels is not offered.
    pub fn artifact_creation_catalog(&self, space_id: &str) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1> {
        use directory::os_directory::schema::space_artifact_creation::{
            SpaceArtifactCreationCatalogV1, SpaceArtifactCreationDialectV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationLabelV1,
        };
        let kind_ids = self.open_targets.iter().map(|selection| selection.artifact.kind.as_str()).collect::<BTreeSet<_>>();
        let mut kinds = Vec::new();
        for kind_id in kind_ids {
            let Some(selection) = self.artifact_creation_selection(kind_id) else { continue };
            let retained = self.selection_package(selection)?;
            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;
            let owner = || self.hosted_kind_owner(selection).and_then(|owner| declared_kind(owner.descriptor.manifest.artifact_kinds.iter().chain(owner.descriptor.manifest.apps.iter().flat_map(|app| app.artifact_kinds.iter())), selection));
            let Some(kind) = declared_kind(app.artifact_kinds.iter().chain(retained.descriptor.manifest.artifact_kinds.iter()), selection).or_else(owner) else { continue };
            kinds.push(SpaceArtifactCreationKindV1 {
                kind_id: selection.artifact.kind.clone(),
                schema: selection.artifact.schema.clone(),
                dialect: SpaceArtifactCreationDialectV1 {
                    artifact_kind: selection.parent_dialect.artifact_kind.clone(),
                    standard: selection.parent_dialect.standard.clone(),
                    subset: selection.parent_dialect.subset.clone(),
                },
                label: SpaceArtifactCreationLabelV1 {
                    en: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::En).to_string(),
                    de: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),
                },
            });
        }
        kinds.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));
        let catalog = SpaceArtifactCreationCatalogV1 { schema: "semio.hub.space-artifact-creation-catalog/v1".into(), space_id: space_id.into(), catalog_generation_id: self.generation_id.clone(), kinds };
        catalog.validate().then_some(catalog)
    }

    /// 🧱 Returns the verified bytes of the current selection only. It is deliberately not a package
    /// lookup: it accepts no package id, digest, path or generation selector from a caller, resolves
    /// the selection from the durable descriptor and subject role alone, and answers only while the
    /// caller-observed generation is still this immutable catalog's own.
    pub fn assets_for_current_selection(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool, current_generation: &str) -> Option<VerifiedExecutionTargetAssets> {
        if current_generation != self.generation_id {
            return None;
        }
        let selection = self.resolve_document_open(descriptor, requested_surface_id, writable)?;
        let package = self.packages.iter().find(|retained| {
            retained.plugin_id == selection.package.plugin_id
                && retained.package.package.0 == selection.package.package_id
                && retained.version == selection.package.version
                && hex_lower(&retained.component_sha256) == selection.package.component_sha256
                && hex_lower(&retained.package.hash.0) == selection.package.component_blake3
                && hex_lower(&retained.descriptor_sha256) == selection.package.descriptor_byte_sha256
        })?;
        if package.component.byte_length() == 0 || package.descriptor_bytes.is_empty() || package.component.byte_length() > TRUSTED_COMPONENT_MAX_BYTES || package.descriptor_bytes.len() as u64 > TRUSTED_DESCRIPTOR_MAX_BYTES {
            return None;
        }
        let browser_actor = match (&selection.browser_actor, &package.browser_actor, &package.browser_actor_asset) {
            (DocumentOpenBrowserActorV1::None, _, _) if !matches!(selection.surface.renderer_target, DocumentOpenRendererTargetV1::Wasm) => None,
            (selected, retained, Some(asset)) if selected == retained && matches!(selected, DocumentOpenBrowserActorV1::ClosedBrowserActor { .. }) && asset.byte_length() != 0 && asset.byte_length() <= DOCUMENT_BROWSER_ACTOR_MAX_BYTES => {
                Some(asset.clone())
            }
            _ => return None,
        };
        Some(VerifiedExecutionTargetAssets { selection, component: package.component.clone(), descriptor: Arc::clone(&package.descriptor_bytes), browser_actor })
    }

    /// 🎯 Resolves one exact descriptor, subject role, and optional surface preference without fallback. Without a preference
    /// the role's most general surface opens the document ([`most_general_dialect`]): a json document opens in the
    /// `rfc8259/*` editor, never refused because an `i-json` editor opens it too (the semio MCP workspace names no surface).
    pub fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        let role = if writable { DocumentOpenSurfaceRoleV1::Editor } else { DocumentOpenSurfaceRoleV1::Viewer };
        let matches = self.open_targets.iter().filter(|selection| {
            selection.package.plugin_id == descriptor.owner.plugin_id
                && selection.package.package_id == descriptor.owner.package_id
                && selection.package.version == descriptor.owner.version
                && selection.package.component_sha256 == descriptor.owner.package_hash
                && selection.artifact.kind == descriptor.artifact_kind
                && selection.artifact.schema == descriptor.artifact_schema
                && selection.artifact.pack_schema_hash == descriptor.pack_schema_hash
                && selection.surface.role == role
                && requested_surface_id.is_none_or(|requested| selection.surface.surface_id == requested)
        });
        most_general_dialect(matches.collect(), |selection| &selection.parent_dialect).one().cloned()
    }
}

impl TrustedArtifactCatalog for VerifiedTrustedCatalog {
    type Codec = VerifiedNativeArtifactCodec;

    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.codecs.iter().find(|entry| &entry.identity == required).ok_or_else(|| AuthorityError::Catalog("descriptor identity is absent from the verified trusted catalog".to_string()))
    }
}

/// 🪪️ What a remembered verification was computed by: the owned engine's semantic identity and the
/// fuel ceiling every guest codec call runs under.
pub fn guest_codec_engine_identity() -> String {
    format!("{}:fuel-{}", semio_framework_plugin_interpreter_runtime::owned_engine_identity(), GUEST_CODEC_BUDGET.fuel)
}

/// 🗃️ The hub's content-addressed memory of guest codec verifications (`GuestCodecVerificationV1`),
/// one file per verification under `<hub data>/trusted-catalog/guest-codec-verifications/`, named by
/// the SHA-256 of its canonical key. A record is used only when its component SHA-256 — taken from the
/// bytes this very load re-read and re-hashed — its artifact schema and its engine all match. The
/// engine is the owned interpreter's semantic identity plus the codec budget, never the executable's
/// path, inode or signature, so a copied, re-signed or container build of the same engine boots warm
/// on a catalog another build verified or published, and a changed engine verifies afresh. The records
/// live inside `trusted-catalog/` so they travel with a copied or restored catalog. Reading or writing a
/// record never decides a verification: an unreadable record is a miss, and a record that cannot be
/// written is recomputed on the next boot.
#[derive(Clone)]
pub struct GuestCodecVerificationCacheV1 {
    pub root: Option<std::path::PathBuf>,
    engine: String,
}

impl GuestCodecVerificationCacheV1 {
    /// 📂️ The cache inside a hub's trusted catalog, keyed to the owned engine that interprets its guests.
    pub fn beside(data_path: &Path) -> Self {
        Self { root: Some(data_path.join("trusted-catalog").join("guest-codec-verifications")), engine: guest_codec_engine_identity() }
    }

    /// 🚫️ No memory at all: every verification interprets its component.
    pub fn disabled() -> Self {
        Self { root: None, engine: String::new() }
    }

    #[cfg(test)]
    pub fn at(root: std::path::PathBuf, engine: &str) -> Self {
        Self { root: Some(root), engine: engine.into() }
    }

    pub fn record(&self, component_sha256: &[u8; 32], artifact_schema: &str, pack_schema_hash: &[u8; 32]) -> GuestCodecVerificationV1 {
        GuestCodecVerificationV1 { schema: GUEST_CODEC_VERIFICATION_SCHEMA.into(), component_sha256: hex_lower(component_sha256), artifact_schema: artifact_schema.into(), engine: self.engine.clone(), pack_schema_hash: hex_lower(pack_schema_hash) }
    }

    pub fn path(&self, component_sha256: &[u8; 32], artifact_schema: &str) -> Option<std::path::PathBuf> {
        let root = self.root.as_ref()?;
        let key = serde_json::to_vec(&(GUEST_CODEC_VERIFICATION_SCHEMA, hex_lower(component_sha256), artifact_schema, &self.engine)).ok()?;
        let mut hash = Sha256::new();
        hash.update(&key);
        Some(root.join(format!("{}.json", hex_lower(&hash.finalize()))))
    }

    /// 🔎️ The pack-schema hash remembered for exactly this component, schema and engine.
    pub async fn recall(&self, component_sha256: &[u8; 32], artifact_schema: &str) -> Option<[u8; 32]> {
        let bytes = tokio::fs::read(self.path(component_sha256, artifact_schema)?).await.ok()?;
        if bytes.len() > 4096 {
            return None;
        }
        let record: GuestCodecVerificationV1 = serde_json::from_slice(&bytes).ok()?;
        let hash = decode_digest(&record.pack_schema_hash, "remembered pack schema hash").ok()?;
        (record == self.record(component_sha256, artifact_schema, &hash) && hash != [0; 32]).then_some(hash)
    }

    /// 💾️ Remembers one verification: written to a sibling temporary file, then renamed into place.
    pub async fn remember(&self, component_sha256: &[u8; 32], artifact_schema: &str, pack_schema_hash: &[u8; 32]) -> bool {
        let Some(path) = self.path(component_sha256, artifact_schema) else { return false };
        let Ok(bytes) = serde_json::to_vec(&self.record(component_sha256, artifact_schema, pack_schema_hash)) else { return false };
        let staged = path.with_extension(format!("{}.tmp", std::process::id()));
        let written = async {
            tokio::fs::create_dir_all(path.parent().ok_or_else(|| std::io::Error::other("cache path has no parent"))?).await?;
            tokio::fs::write(&staged, &bytes).await?;
            tokio::fs::rename(&staged, &path).await
        }
        .await;
        if written.is_err() {
            let _ = tokio::fs::remove_file(&staged).await;
        }
        written.is_ok()
    }
}

/// 🧵️ How many codec rows of one package are interpreted at once: the declared residency bound, never
/// more than half the cores this hub may use, never fewer than one.
pub fn guest_verification_concurrency() -> usize {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    TRUSTED_CATALOG_GUEST_RESIDENCY.concurrent_verifications.min(cores / 2).max(1)
}

/// 🏗️ Stateless verifier for one explicitly selected immutable trust bundle.
pub struct TrustedCatalogLoader;

impl TrustedCatalogLoader {
    /// 🛡️ Opens the current immutable generation beneath one server-owned Hub data root.
    pub async fn load_current(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
        Self::load_current_observed(data_path, providers, &Arc::default(), context).await
    }

    /// 📈️ Opens the current generation and reports every package's load progress into the host's `progress` cell.
    pub async fn load_current_observed(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, progress: &Arc<TrustedCatalogLoadProgressCellV1>, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
        if !data_path.try_exists().map_err(catalog_error)? {
            return Ok(None);
        }
        let data_root = TrustedCatalogDataRoot::open_server_owned(data_path)?;
        let Some(current_file) = data_root.open_current()? else {
            return Ok(None);
        };
        let current_bytes = current_file.read_bounded(64 * 1024, context).await?;
        let current = TrustedCatalogCurrentPointerV1::decode(&current_bytes)?;
        let expected_bundle_sha256 = decode_digest(&current.bundle_sha256, "trusted bundle sha256")?;
        let generation_root = Arc::new(data_root.open_generation(&current.generation_id)?);
        let bundle_path = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != expected_bundle_sha256 {
            return Err(catalog("trusted bundle differs from the current pointer digest"));
        }
        let verifications = GuestCodecVerificationCacheV1::beside(data_path);
        let verified = Self::load_selected(&generation_root, bundle_path, bundle_bytes, &current.profile_id, providers, &verifications, progress, context).await?;
        if verified.generation_id() != current.generation_id {
            return Err(catalog("trusted current pointer generation differs from the selected profile"));
        }
        Ok(Some(verified))
    }

    #[cfg(any(test, feature = "integration-fixtures"))]
    pub async fn load_fixture(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        Self::load_fixture_observed(bundle_path, profile_id, providers, &Arc::default(), context).await
    }

    #[cfg(any(test, feature = "integration-fixtures"))]
    pub async fn load_fixture_observed(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, progress: &Arc<TrustedCatalogLoadProgressCellV1>, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let path = std::fs::canonicalize(bundle_path).map_err(catalog_error)?;
        let fixture_root = path.parent().ok_or_else(|| catalog("bundle has no containing directory"))?;
        let generation_root = Arc::new(TrustedCatalogGenerationRoot::open_fixture_owned(fixture_root)?);
        let bundle_path = TrustedCatalogRelativePathV1::parse(path.file_name().and_then(|name| name.to_str()).ok_or_else(|| catalog("fixture bundle name is not UTF-8"))?)?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        Self::load_selected(&generation_root, bundle_path, bundle_bytes, profile_id, providers, &GuestCodecVerificationCacheV1::disabled(), progress, context).await
    }

    async fn load_selected(
        root: &Arc<TrustedCatalogGenerationRoot>,
        bundle_path: TrustedCatalogRelativePathV1,
        bundle_bytes: Vec<u8>,
        profile_id: &str,
        providers: &dyn NativeCodecProviderSourceV1,
        verifications: &GuestCodecVerificationCacheV1,
        progress: &Arc<TrustedCatalogLoadProgressCellV1>,
        context: &OperationContext<'_>,
    ) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let (catalog, registration) = Self::verify_selected(root, bundle_path, bundle_bytes, profile_id, providers, verifications, progress, context).await?;
        let assembly = semio_framework_schema_registry::assembly::begin().map_err(catalog_error)?;
        os_store::codec_registry::preflight_document_codecs_in_assembly(&assembly, &registration.document_codecs).map_err(catalog_error)?;
        directory::io::preflight_artifact_codec_bindings_in_assembly(&assembly,&registration.document_bindings,&registration.document_codecs).map_err(catalog_error)?;
        directory::io::preflight_artifact_catalog_bindings_in_assembly(&assembly,&registration.catalog_bindings,&registration.document_codecs).map_err(catalog_error)?;
        context.checkpoint()?;
        directory::io::commit_artifact_assembly_registry_plan(&assembly, registration).map_err(catalog_error)?;
        Ok(catalog)
    }

    /// 📦️ One selected package whose files, digests, descriptor and browser actor are already
    /// verified, held until the whole closure is proven so no provider sees a package belonging to a
    /// closure that is still able to be refused. Component and actor bytes are not held: each was
    /// read, hashed and dropped, and is retained by identity.
    async fn verify_selected(
        root: &Arc<TrustedCatalogGenerationRoot>,
        bundle_path: TrustedCatalogRelativePathV1,
        bundle_bytes: Vec<u8>,
        profile_id: &str,
        providers: &dyn NativeCodecProviderSourceV1,
        verifications: &GuestCodecVerificationCacheV1,
        progress: &Arc<TrustedCatalogLoadProgressCellV1>,
        context: &OperationContext<'_>,
    ) -> Result<(VerifiedTrustedCatalog, ArtifactAssemblyRegistryPlan), AuthorityError> {
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 0, total_units: 1 })?;
        let guest_runtime = Arc::new(semio_framework_plugin_interpreter_runtime::OwnedRuntime::new());
        let residency = GuestResidencyLedgerV1::new(TRUSTED_CATALOG_GUEST_RESIDENCY);
        let progress = Arc::clone(progress);
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let SelectedTrustedBundleV1 { package_indices: order, profile } = validate_bundle(&bundle, profile_id)?;
        progress.select(
            order
                .iter()
                .map(|index| {
                    let record = &bundle.packages[*index];
                    TrustedCatalogPackageProgressV1 { plugin_id: record.plugin_id.clone(), component_bytes: record.component.byte_length, phase: TrustedCatalogPackagePhaseV1::Pending, rows: u64::try_from(record.native_codecs.len()).unwrap_or(u64::MAX), rows_pinned: 0, rows_verified: 0 }
                })
                .collect(),
        );
        let order_len = u64::try_from(order.len()).map_err(|error| catalog_error(error))?;
        let total_units = order_len.checked_mul(4).and_then(|units| units.checked_add(1)).ok_or_else(|| catalog("catalog progress total overflow"))?;
        let requirements = order
            .iter()
            .flat_map(|index| {
                let record = &bundle.packages[*index];
                record.native_codecs.iter().map(move |codec| NativeCodecProviderRequirementV1 {
                    package: NativeCodecProviderPackageV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version },
                    artifact_kind: &codec.artifact_kind,
                    artifact_schema: &codec.artifact_schema,
                    pack_schema_hash: &codec.pack_schema_hash,
                })
            })
            .collect::<Vec<_>>();
        providers.preflight_selection(&requirements)?;
        drop(requirements);
        let mut packages = Vec::with_capacity(order.len());
        let mut codecs = Vec::new();
        let mut open_targets = Vec::new();
        let mut registration = ArtifactAssemblyRegistryPlan::default();
        let mut resolved_paths = BTreeSet::from([bundle_path]);

        struct StagedTrustedPackage<'a> {
            position: usize,
            record: &'a TrustedBundlePackageV1,
            component: TrustedCatalogAsset,
            component_sha256: [u8; 32],
            component_blake3: [u8; 32],
            descriptor_bytes: Vec<u8>,
            descriptor_sha256: [u8; 32],
            descriptor: PackageDescriptor,
            browser_actor: DocumentOpenBrowserActorV1,
            browser_actor_asset: Option<TrustedCatalogAsset>,
            plugin_module: VerifiedPluginModule,
        }

        let mut staged = Vec::with_capacity(order.len());
        let mut plugin_module_files = BTreeMap::new();
        for (position, index) in order.into_iter().enumerate() {
            context.checkpoint()?;
            progress.phase(position, TrustedCatalogPackagePhaseV1::Reading);
            let record = &bundle.packages[index];
            let component_path = TrustedCatalogRelativePathV1::parse(&record.component.path)?;
            if !resolved_paths.insert(component_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let component_bytes = root.read_regular(&component_path, TRUSTED_COMPONENT_MAX_BYTES, context).await?;
            verify_length(record.component.byte_length, component_bytes.len())?;
            let (component_sha256, component_blake3) = dual_hash(&component_bytes, context).await?;
            drop(component_bytes);
            verify_digest(&record.component.sha256, component_sha256, "component sha256")?;
            verify_digest(&record.component.blake3, component_blake3, "component blake3")?;
            let component = TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), component_path, record.component.byte_length, component_sha256, Some(component_blake3)));
            progress.component_read(position);
            report_package_progress(context, position, 1, total_units)?;

            let descriptor_path = TrustedCatalogRelativePathV1::parse(&record.descriptor.path)?;
            if !resolved_paths.insert(descriptor_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let descriptor_bytes = root.read_regular(&descriptor_path, TRUSTED_DESCRIPTOR_MAX_BYTES, context).await?;
            verify_length(record.descriptor.byte_length, descriptor_bytes.len())?;
            let descriptor_sha256 = sha256(&descriptor_bytes, context).await?;
            verify_digest(&record.descriptor.sha256, descriptor_sha256, "descriptor sha256")?;
            let descriptor = decode_package_descriptor(&descriptor_bytes)?;
            validate_descriptor(record, &descriptor, &bundle.packages)?;
            report_package_progress(context, position, 2, total_units)?;

            let browser_actor = record.browser_actor.identity();
            record.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &hex_lower(&component_sha256), descriptor_byte_sha256: &hex_lower(&descriptor_sha256) }, package_actor_renderer(record))?;
            let browser_actor_asset = if let Some(file) = record.browser_actor.file() {
                let actor_path = TrustedCatalogRelativePathV1::parse(&file.path)?;
                if !resolved_paths.insert(actor_path.clone()) {
                    return Err(catalog("trusted browser actor path is already used by the selected closure"));
                }
                let bytes = root.read_regular(&actor_path, file.byte_length, context).await?;
                verify_length(file.byte_length, bytes.len())?;
                let actor_sha256 = sha256(&bytes, context).await?;
                drop(bytes);
                verify_digest(&file.sha256, actor_sha256, "browser actor sha256")?;
                Some(TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), actor_path, file.byte_length, actor_sha256, None)))
            } else {
                None
            };
            let plugin_module = verify_plugin_module(root, record, &hex_lower(&component_sha256), &hex_lower(&descriptor_sha256), &mut resolved_paths, &mut plugin_module_files, context).await?;
            report_package_progress(context, position, 3, total_units)?;
            progress.phase(position, TrustedCatalogPackagePhaseV1::Staged);
            staged.push(StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module });
        }

        let selected = |record: &TrustedBundlePackageV1, target: &TrustedBundleOpenTargetV1| {
            profile.open_targets.iter().any(|selected| selected.package.plugin_id == record.plugin_id && selected.package.package_id == record.package_id && selected.package.version == record.version && selected.target == *target)
        };
        let mut previews = Vec::with_capacity(staged.len());
        let mut pending_rows = BTreeMap::new();
        let mut natively_bound = BTreeSet::new();
        for stage in &staged {
            context.checkpoint()?;
            let native_bindings = providers.preview(NativeCodecProviderPackageV1 { plugin_id: &stage.record.plugin_id, package_id: &stage.record.package_id, version: &stage.record.version }, &stage.descriptor, context)?;
            context.checkpoint()?;
            let bound = validate_native_bindings(&native_bindings)?.into_keys().collect::<BTreeSet<_>>();
            let hosted = hosted_codec_targets(&bundle.packages, stage.record, |target| selected(stage.record, target))?;
            let own_rows = stage.record.native_codecs.iter().filter(|expected| !bound.contains(&CodecKey::from_parts(&stage.record.plugin_id, &stage.record.package_id, &expected.artifact_kind, &expected.artifact_schema))).map(|expected| (&expected.artifact_schema, &expected.pack_schema_hash));
            let hosted_rows = hosted.iter().filter(|(owner, target)| !natively_bound.contains(&CodecKey::from_parts(&owner.plugin_id, &owner.package_id, &target.artifact_kind, &target.artifact_schema))).map(|(_, target)| (&target.artifact_schema, &target.pack_schema_hash));
            let mut rows: Vec<(String, [u8; 32])> = Vec::new();
            for (schema, hash) in own_rows.chain(hosted_rows) {
                let expected_hash = decode_digest(hash, "pack schema hash")?;
                if expected_hash == [0; 32] {
                    return Err(catalog("artifact codec schema hash is zero"));
                }
                if rows.iter().any(|(row, row_hash)| row == schema && *row_hash == expected_hash) || verifications.recall(&stage.component_sha256, schema).await == Some(expected_hash) {
                    continue;
                }
                rows.push((schema.clone(), expected_hash));
            }
            let total = stage.record.native_codecs.len() + hosted.len();
            let (total, pinned) = (u64::try_from(total).unwrap_or(u64::MAX), u64::try_from(total - rows.len()).unwrap_or(u64::MAX));
            natively_bound.extend(bound);
            progress.package(stage.position, |package| {
                package.rows = total;
                package.rows_pinned = pinned;
                if rows.is_empty() {
                    package.phase = TrustedCatalogPackagePhaseV1::Ready;
                }
            });
            pending_rows.insert(stage.position, rows);
            previews.push(native_bindings);
        }
        let verifications = Arc::new(verifications.clone());
        let mut guests = Vec::with_capacity(staged.len());

        for (stage, native_bindings) in staged.into_iter().zip(previews) {
            let StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module } = stage;
            let catalog_package=DocumentOpenPackageV1{plugin_id:record.plugin_id.clone(),package_id:record.package_id.clone(),version:record.version.clone(),component_sha256:hex_lower(&component_sha256),component_blake3:hex_lower(&component_blake3),descriptor_byte_sha256:hex_lower(&descriptor_sha256),execution_protocol:DocumentExecutionProtocolV1{app_channel_version:descriptor.execution_protocol.app_channel_version},component_receipt:record.receipt.clone()};
            context.checkpoint()?;
            let binding_map = validate_native_bindings(&native_bindings)?;
            let mut consumed_bindings = BTreeSet::new();
            let package_ref = PackageRef { package: PackageId(record.package_id.clone()), hash: PackageHash(component_blake3) };
            let guest = Arc::new(GuestArtifactComponent {
                runtime: Arc::clone(&guest_runtime),
                package: package_ref.clone(),
                component: component.clone(),
                compiled: residency.register(record.component.byte_length),
                position,
                component_sha256,
                rows: pending_rows.remove(&position).unwrap_or_default(),
                verified: tokio::sync::OnceCell::new(),
                verifications: Arc::clone(&verifications),
                progress: Arc::clone(&progress),
            });
            guests.push(Arc::clone(&guest));

            for expected in &record.native_codecs {
                if codecs.len() >= TRUSTED_CATALOG_MAX_CODECS {
                    return Err(AuthorityError::ResourceLimit("trusted codec count"));
                }
                let key = CodecKey::from_parts(&record.plugin_id, &record.package_id, &expected.artifact_kind, &expected.artifact_schema);
                let expected_hash = decode_digest(&expected.pack_schema_hash, "pack schema hash")?;
                if expected_hash == [0; 32] {
                    return Err(catalog("artifact codec schema hash is zero"));
                }
                let binding = binding_map.get(&key);
                if let Some(binding) = binding {
                    if binding.codec.pack_schema_hash == [0; 32] || binding.codec.pack_schema_hash != expected_hash || binding.codec.schema != expected.artifact_schema {
                        return Err(catalog("native codec schema hash is zero or mismatched"));
                    }
                    consumed_bindings.insert(key);
                }
                let identity = TrustedArtifactIdentity {
                    plugin_id: record.plugin_id.clone(),
                    package_id: record.package_id.clone(),
                    version: record.version.clone(),
                    package_hash: hex_lower(&component_sha256),
                    artifact_kind: expected.artifact_kind.clone(),
                    artifact_schema: expected.artifact_schema.clone(),
                    pack_schema_hash: expected.pack_schema_hash.clone(),
                };
                if codecs.iter().any(|entry: &VerifiedNativeArtifactCodec| entry.identity == identity) {
                    return Err(catalog("duplicate exact trusted artifact identity"));
                }
                if let Some(binding) = binding {
                    registration.document_codecs.push(binding.codec.clone());
                }
                let owner_declaration=ArtifactCatalogBinding{contributor:catalog_package.clone(),owner:catalog_package.clone(),artifact:DocumentOpenArtifactV1{kind:expected.artifact_kind.clone(),schema:expected.artifact_schema.clone(),pack_schema_hash:expected.pack_schema_hash.clone()},target:None,capability:binding.map_or(ArtifactCatalogCapability::UnlinkedGuest,|binding|ArtifactCatalogCapability::linked(&binding.codec,binding.factory_id.clone()))};
                registration.catalog_bindings.push(owner_declaration.clone());
                codecs.push(VerifiedNativeArtifactCodec {
                    identity,
                    codec: binding.map(|binding| binding.codec.clone()),
                    guest: GuestArtifactCodecBinding { component: Arc::clone(&guest), artifact_schema: expected.artifact_schema.clone() },
                    owner_declaration,
                });
            }
            if consumed_bindings.len() != binding_map.len() {
                return Err(catalog("selected provider returned a binding outside its exact declared package closure"));
            }
            for target in &record.open_targets {
                if open_targets.len() >= TRUSTED_CATALOG_MAX_OPEN_TARGETS {
                    return Err(AuthorityError::ResourceLimit("trusted document-open target count"));
                }
                let parent_dialect = validate_descriptor_open_target(&descriptor, target)?;
                if !selected(record, target) {
                    continue;
                }
                let bound = open_target_codec_package(&bundle.packages, record, target)?;
                if bound.plugin_id != record.plugin_id && !descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == target.artifact_kind && kind.schema == target.artifact_schema && kind.owner == bound.plugin_id) {
                    return Err(catalog("document-open target binds a dependency's codec its descriptor does not host from that owner"));
                }
                let role = match target.role {
                    TrustedBundleOpenRole::Viewer => DocumentOpenSurfaceRoleV1::Viewer,
                    TrustedBundleOpenRole::Editor => DocumentOpenSurfaceRoleV1::Editor,
                };
                let renderer_target = match target.renderer_target {
                    TrustedBundleRendererTarget::React => DocumentOpenRendererTargetV1::React,
                    TrustedBundleRendererTarget::Wgpu => DocumentOpenRendererTargetV1::Wgpu,
                    TrustedBundleRendererTarget::Wasm => DocumentOpenRendererTargetV1::Wasm,
                };
                let selection = VerifiedDocumentOpenSelectionV1 {
                    parent_dialect,
                    package: DocumentOpenPackageV1 {
                        plugin_id: record.plugin_id.clone(),
                        package_id: record.package_id.clone(),
                        version: record.version.clone(),
                        component_sha256: hex_lower(&component_sha256),
                        component_blake3: hex_lower(&component_blake3),
                        descriptor_byte_sha256: hex_lower(&descriptor_sha256),
                        execution_protocol: DocumentExecutionProtocolV1 { app_channel_version: descriptor.execution_protocol.app_channel_version },
                        component_receipt: record.receipt.clone(),
                    },
                    artifact: DocumentOpenArtifactV1 { kind: target.artifact_kind.clone(), schema: target.artifact_schema.clone(), pack_schema_hash: target.pack_schema_hash.clone() },
                    surface: DocumentOpenSurfaceV1 { surface_id: target.surface_id.clone(), app_id: target.app_id.clone(), window_kind_id: target.window_kind_id.clone(), role, renderer_target },
                    grant: DocumentOpenGrantV1 { read: target.grant.read, write: target.grant.write, observe: target.grant.observe },
                    browser_actor: if matches!(renderer_target, DocumentOpenRendererTargetV1::Wasm) { browser_actor.clone() } else { DocumentOpenBrowserActorV1::None },
                };
                if open_targets.iter().any(|existing| document_open_target_sort_key(existing) == document_open_target_sort_key(&selection)) {
                    return Err(catalog("document-open target identity is duplicated"));
                }
                let owned=codecs.iter().find(|entry|entry.identity.plugin_id==bound.plugin_id&&entry.identity.package_id==bound.package_id&&entry.identity.version==bound.version&&entry.identity.artifact_kind==target.artifact_kind&&entry.identity.artifact_schema==target.artifact_schema&&entry.identity.pack_schema_hash==target.pack_schema_hash).ok_or_else(||catalog("document-open target's exact codec owner is not verified before selection"))?;
                registration.catalog_bindings.push(ArtifactCatalogBinding{contributor:selection.package.clone(),owner:owned.owner_declaration.owner.clone(),artifact:selection.artifact.clone(),target:Some(ArtifactCatalogTarget{parent_dialect:selection.parent_dialect.clone(),surface:selection.surface.clone(),grant:selection.grant,browser_actor:selection.browser_actor.clone()}),capability:owned.owner_declaration.capability.clone()});
                open_targets.push(selection);
            }
            for (owner, target) in hosted_codec_targets(&bundle.packages, record, |target| selected(record, target))? {
                if codecs.len() >= TRUSTED_CATALOG_MAX_CODECS {
                    return Err(AuthorityError::ResourceLimit("trusted codec count"));
                }
                let identity = TrustedArtifactIdentity {
                    plugin_id: record.plugin_id.clone(),
                    package_id: record.package_id.clone(),
                    version: record.version.clone(),
                    package_hash: hex_lower(&component_sha256),
                    artifact_kind: target.artifact_kind.clone(),
                    artifact_schema: target.artifact_schema.clone(),
                    pack_schema_hash: target.pack_schema_hash.clone(),
                };
                let owned = codecs
                    .iter()
                    .find(|entry| entry.identity.plugin_id == owner.plugin_id && entry.identity.package_id == owner.package_id && entry.identity.version == owner.version && entry.identity.artifact_kind == identity.artifact_kind && entry.identity.artifact_schema == identity.artifact_schema && entry.identity.pack_schema_hash == identity.pack_schema_hash)
                    .ok_or_else(|| catalog("hosted document-open target's owner codec is not verified before its host"))?;
                let codec = owned.codec.clone();
                let owner_declaration=owned.owner_declaration.clone();
                if codecs.iter().any(|entry| entry.identity == identity) {
                    return Err(catalog("duplicate exact trusted artifact identity"));
                }
                codecs.push(VerifiedNativeArtifactCodec { identity, codec, guest: GuestArtifactCodecBinding { component: Arc::clone(&guest), artifact_schema: target.artifact_schema.clone() },owner_declaration });
            }
            report_package_progress(context, position, 4, total_units)?;
            packages.push(VerifiedTrustedPackage {
                plugin_id: record.plugin_id.clone(),
                package: package_ref,
                version: record.version.clone(),
                dependencies: record.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).collect(),
                plugin_module,
                component_sha256,
                descriptor_sha256,
                component,
                descriptor_bytes: descriptor_bytes.into(),
                receipt: record.receipt.clone(),
                browser_actor,
                browser_actor_asset,
                descriptor: Arc::new(descriptor),
            });
        }
        if codecs.is_empty() {
            return Err(catalog("selected profile exposes no executable artifact codec"));
        }
        sort_open_targets(&mut open_targets);
        if open_targets.len() != profile.open_targets.len() {
            return Err(catalog("selected profile resolved a different number of document-open targets than it declares"));
        }
        if open_targets.is_empty() {
            return Err(catalog("selected profile must resolve at least one document-open target"));
        }
        let generation_id = trusted_profile_generation(&bundle, &profile)?;
        if generation_id != profile.generation_id {
            return Err(catalog("trusted profile generation differs from the completely verified package, codec, and target closure"));
        }
        let artifact_kind_count = {
            let mut kinds = BTreeSet::new();
            for codec in &codecs {
                context.checkpoint()?;
                kinds.insert(codec.identity.artifact_kind.as_str());
            }
            kinds.len()
        };
        let catalog = VerifiedTrustedCatalog { residency, guests: guests.into_boxed_slice(), progress, packages: packages.into_boxed_slice(), codecs: codecs.into_boxed_slice(), artifact_kind_count, open_targets: open_targets.into_boxed_slice(), generation_id };
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogResolved, completed_units: total_units, total_units })?;
        for codec in &registration.document_codecs{let binding=ArtifactCodecBinding::schema_only(codec);if !registration.document_bindings.contains(&binding){registration.document_bindings.push(binding);}}
        Ok((catalog, registration))
    }
}

/// 🧫️ The files of a never-executed fixture plugin module: its entry, both descriptor forms (the packed one
/// the package's own descriptor), the package's canonical receipt and one vendored import shared by every fixture module.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn fixture_plugin_module_files(plugin_id: &str, descriptor_bytes: &[u8], receipt: &ComponentReceiptV1) -> Vec<(String, Vec<u8>)> {
    vec![
        (format!("{plugin_id}/🌉️bridge.js"), b"export async function createActorApi() {}\n".to_vec()),
        (format!("{plugin_id}/🔣️.json"), serde_json::json!({ "manifest": { "pluginId": plugin_id } }).to_string().into_bytes()),
        (format!("{plugin_id}/🛂️.descriptor.semio"), descriptor_bytes.to_vec()),
        (format!("{plugin_id}/{}", semio_hub_trusted_catalog_contract::plugin_module::TRUSTED_PLUGIN_MODULE_RECEIPT_FILE), receipt.encode_canonical()),
        ("🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js".to_owned(), b"export const streams = {};\n".to_vec()),
    ]
}

/// 🧫️ The shipped receipt a never-executed fixture component carries: it names exactly the component bytes with this digest
/// and length and this semantic descriptor.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn fixture_component_receipt(plugin_id: &str, package_id: &str, wasm_sha256: &str, wasm_bytes: u64, descriptor_bytes: &[u8]) -> ComponentReceiptV1 {
    ComponentReceiptV1 {
        schema: directory::os_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
        plugin_id: plugin_id.into(),
        package_id: package_id.into(),
        profile: directory::os_component_receipt::ComponentProfileV1::Release,
        target: directory::os_component_receipt::COMPONENT_TARGET.into(),
        wasm_sha256: wasm_sha256.into(),
        wasm_bytes,
        semantic_descriptor_sha256: hex_lower(&Sha256::digest(descriptor_bytes)),
        toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
        source_tree_digest: "3".repeat(64),
        provenance_sha256: "4".repeat(64),
    }
}

/// 🧫️ Writes one package's fixture plugin module beneath `root` and answers the `pluginModule` record naming it.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn write_fixture_plugin_module(root: &Path, plugin_id: &str, package_id: &str, version: &str, receipt: &ComponentReceiptV1, descriptor_bytes: &[u8]) -> Result<serde_json::Value, AuthorityError> {
    let descriptor_byte_sha256 = hex_lower(&Sha256::digest(descriptor_bytes));
    let receipt_sha256 = hex_lower(&Sha256::digest(&receipt.encode_canonical()));
    let source = TrustedPluginModuleSourceV1 { plugin_id, package_id, version, component_sha256: &receipt.wasm_sha256, descriptor_byte_sha256: &descriptor_byte_sha256, receipt_sha256: &receipt_sha256 };
    let record = semio_hub_trusted_catalog_contract::plugin_module::write_plugin_module_bundle(root, &format!("plugin-module-{plugin_id}.json"), source, plugin_id, &fixture_plugin_module_files(plugin_id, descriptor_bytes, receipt))?;
    serde_json::to_value(record).map_err(catalog_error)
}

/// 🧫️ Shares headless Stdio metadata between native GIS fixtures; synthetic bytes are never executed.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn headless_stdio_fixture_package(root: &Path) -> Result<(serde_json::Value, serde_json::Value), AuthorityError> {
    let dependency = semio_hub_stdio_catalog::native_artifact_catalog_dependency().map_err(catalog_error)?;
    let version = dependency.version.0.to_string();
    let receipts = semio_hub_stdio_catalog::native_codec_factory_receipts().map_err(catalog_error)?;
    let mut builder = semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("stdio").label("Stdio Fixture").version(version.clone()).package_id("semio:stdio");
    for kind in semio_hub_stdio_catalog::native_codec_artifact_kinds() {
        builder = builder.artifact_kind(kind);
    }
    let plugin = builder.contributes_topic(semio_hub_stdio_catalog::native_artifact_catalog_contribution().map_err(catalog_error)?).try_library().map_err(catalog_error)?;
    let component = b"synthetic-stdio-component-for-linked-catalog-test";
    let component_sha256 = hex_lower(&Sha256::digest(component));
    let mut component_blake3 = Hasher::new();
    component_blake3.update(component);
    let descriptor = semio_framework::PackageDescriptor {
        descriptor_version: 1, package_id: "semio:stdio".into(), role: semio_framework::PackageRole::Plugin,
        manifest: plugin.manifest, activation_events: Vec::new(), capability_requests: Vec::new(), extension_points: Vec::new(),
        execution: semio_framework::ExecutionMode::Isolated,
        execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        quotas: semio_framework::kernel::QuotaSchema::default(), contributions: semio_framework::ContributionSet::default(), assets: Vec::new(),
    };
    let bytes = os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor));
    decode_package_descriptor(&bytes)?;
    std::fs::write(root.join("stdio-component.wasm"), component).map_err(catalog_error)?;
    std::fs::write(root.join("stdio-descriptor.semio"), &bytes).map_err(catalog_error)?;
    let identity = serde_json::json!({ "pluginId": "stdio", "packageId": "semio:stdio", "version": version });
    let codecs: Vec<_> = receipts.into_iter().map(|receipt| serde_json::json!({ "artifactKind": receipt.artifact_kind, "artifactSchema": receipt.schema, "packSchemaHash": hex_lower(&receipt.pack_schema_hash) })).collect();
    let receipt = fixture_component_receipt("stdio", "semio:stdio", &component_sha256, component.len() as u64, &bytes);
    let plugin_module = write_fixture_plugin_module(root, "stdio", "semio:stdio", &version, &receipt, &bytes)?;
    let record = serde_json::json!({
        "pluginId": "stdio", "packageId": "semio:stdio", "version": version, "role": "plugin", "dependencies": [],
        "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
        "component": { "path": "stdio-component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
        "descriptor": { "path": "stdio-descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
        "receipt": receipt,
        "browserActor": { "kind": "none" }, "pluginModule": plugin_module, "nativeCodecs": codecs, "openTargets": [],
    });
    serde_json::from_value::<TrustedBundlePackageV1>(record.clone()).map_err(catalog_error)?;
    Ok((identity, record))
}

#[cfg(any(test, feature = "testkit"))]
impl GuestResidentFootprintV1 for Vec<u8> {
    /// 📏️ A byte-charged test value holds only the bytes it was registered with.
    fn footprint_bytes(&self) -> u64 {
        0
    }
}
