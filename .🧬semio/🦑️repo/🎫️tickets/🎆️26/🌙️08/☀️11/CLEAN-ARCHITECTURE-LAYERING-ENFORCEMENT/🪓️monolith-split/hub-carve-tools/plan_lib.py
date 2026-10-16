#!/usr/bin/env python3
"""📐️ Hub lib carve plan (directory + artifact-authority families): structural edits, then rewrite passes, then the union oracle."""
import os, sys, re, json, time, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ws as wsm, hcmod, carve, union
from carve import Edits, Carver, Model, REPO
from helpers import *

CRATES = collections.OrderedDict()


def crate(ident, root, name=None, order=None):
    CRATES[ident] = dict(root=P(root), name=name or ident.replace('_', '-'))


crate('semio_hub_testing', '🧪️tests/🗂️artifact-root/🦀️.rs')
crate('semio_hub_refusal', '🚧️refusal/🦀️.rs')
crate('semio_hub_directory_contract', '📇️directory/📜️contract/🦀️.rs')
crate('semio_hub_artifact_authority', '🗿️artifact-authority/🦀️.rs')
crate('semio_hub_artifact_creation', '🗿️artifact-authority/🌱️creation/🦀️.rs')
crate('semio_hub_directory', '📇️directory/🦀️.rs')
crate('semio_hub_directory_sqlite', '📇️directory/🪶️sqlite/🦀️.rs')
crate('semio_hub_directory_postgres', '📇️directory/🐘️postgres/🦀️.rs')
crate('semio_hub_directory_neo4j_rows', '📇️directory/🌐️neo4j/🧰️rows/🦀️.rs')
crate('semio_hub_directory_neo4j', '📇️directory/🌐️neo4j/🦀️.rs')
crate('semio_hub_directory_dispatch', '📇️directory/🔀️dispatch/🦀️.rs')
crate('semio_hub_directory_service', '📇️directory/🛠️service/🦀️.rs')
crate('semio_hub_directory_laws', '📇️directory/🧪️tests/🏛️laws/🦀️.rs')
crate('semio_hub_trusted_catalog_contract', '🗿️artifact-authority/🔏️trusted-catalog/📜️contract/🦀️.rs')
crate('semio_hub_trusted_catalog_descriptor', '🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs')
crate('semio_hub_trusted_catalog', '🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs')
crate('semio_hub_trusted_catalog_laws', '🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🏛️laws/🦀️.rs')
crate('semio_hub_artifact_creation_service', '🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs')
crate('semio_hub_artifact_checkin', '🗿️artifact-authority/📌️check-in/🦀️.rs')
crate('semio_hub_native_openable', '🗿️artifact-authority/📇️native-openable-provider/🦀️.rs')
crate('semio_hub_auth', '🔐️auth/🦀️.rs')
crate('semio_hub_stores', '🗄️stores/🦀️.rs')
crate('semio_hub_lag_rebootstrap', '🛰️lag-rebootstrap/🦀️.rs')
crate('semio_hub_local_bootstrap', '🚀️local-bootstrap/🦀️.rs')
crate('semio_hub_observability', '📊️observability/🦀️.rs')
crate('semio_hub_trusted_catalog_fixtures', '🧪️tests/🔏️trusted-catalog-profile/🦀️.rs')
crate('semio_hub_component_codec_laws', '🧪️tests/🧩️component-codecs/🏛️laws/🦀️.rs')
crate('semio_hub_inference', '💡️inference/🦀️.rs')
crate('semio_hub_inference_ledger', '💡️inference/🪶️sqlite/🦀️.rs')
crate('semio_hub_inference_catalog', '💡️inference/📇️catalog/🦀️.rs')
crate('semio_hub_inference_runtime_contract', '💡️inference/🏃️runtime/📜️contract/🦀️.rs')
crate('semio_hub_inference_committer', '💡️inference/🏃️runtime/🔐️committer/🦀️.rs')
crate('semio_hub_inference_runtime', '💡️inference/🏃️runtime/🦀️.rs')
crate('semio_hub_inference_laws', '💡️inference/🏃️runtime/🧪️tests/🏛️laws/🦀️.rs')
crate('semio_hub_inference_contract_laws', '💡️inference/🧪️tests/🏛️laws/🦀️.rs')
crate('semio_hub_trusted_catalog_contract_laws', '🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🏛️contract-laws/🦀️.rs')
crate('semio_hub_directory_backend_laws', '📇️directory/🧪️tests/🏛️backend-laws/🦀️.rs')


import plan_bin, plan_bintests
for _ident, _root in plan_bin.ROOTS.items():
    CRATES[_ident] = dict(root=_root, name=_ident.replace('_', '-'))
for _ident, _root in plan_bintests.ROOTS.items():
    CRATES[_ident] = dict(root=_root, name=_ident.replace('_', '-'))
CRATES['os_hub'] = dict(root=P('🏗️bootstrap/🦀️.rs'), name='semio-hub')


def structural(ws, E):
    D = P('📇️directory/')
    A = P('🗿️artifact-authority/')
    append(ws, P('🧪️tests/🗂️artifact-root/🦀️.rs'), '''/// 📖️ A file of the hub workspace read when the law runs instead of being embedded into the test binary; `relative` is relative to the hub workspace root. The text lives as long as the process, like an embedded constant.
pub fn hub_source(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..").join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("hub source {relative}: {error}")).into_boxed_str())
}''')
    # ---------------------------------------------------------------- directory contract
    contract = D + '📜️contract/🦀️.rs'
    create(ws, contract, """//! 📜️ Directory contract: the error vocabulary, bounded constants, events, records and capability vocabulary every directory backend, the writer and the HTTP surface share. It depends on no other hub crate.

#[path = "../🧾️model/🦀️.rs"]
pub mod model;

#[path = "../🎟️capabilities/🦀️.rs"]
mod capabilities;
use capabilities::*;

#[path = "../🕰️events/🦀️.rs"]
mod events;
use events::*;
""")
    root = D + '🦀️.rs'
    consts = ['ARTIFACT_CHECKPOINT_LINEAGE_MAX', 'DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS', 'DIRECTORY_EVENT_READ_MAX', 'ACTIVE_SYNC_SESSION_READ_MAX', 'ADMIN_INTENT_REQUEST_MAX_BYTES', 'ADMIN_PAGE_MAX', 'ADMIN_PAGE_FETCH_MAX', 'ADMIN_RESPONSE_MAX_BYTES', 'SPACE_ADMINISTRATION_PAGE_MAX', 'SPACE_ADMINISTRATION_PAGE_FETCH_MAX', 'DIRECTORY_WIRE_INTEGER_MAX', 'ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES', 'ARTIFACT_CAS_SWEEP_PAGE_MAX', 'ARTIFACT_CAS_SWEEP_OBJECT_MAX', 'ARTIFACT_CAS_RESERVATION_MAX_TTL_MS', 'ARTIFACT_CAS_DELETE_LEASE_TTL_MS']
    E.move(root, ['error'] + consts, contract)
    E.move(D + '🔤️wire/🦀️.rs', ['role_to_wire', 'role_from_wire', 'kind_to_str', 'visibility_to_str'], contract)
    E.move(D + '🧭️decider/🦀️.rs', ['actor_user_id'], contract)
    # ---------------------------------------------------------------- directory (port) root
    E.move(D + '🛠️service/🦀️.rs', ['DirectoryProjectionRejectionV1', 'directory_projection_space_v1', 'directory_projection_rejection_v1', 'DirectoryAppendOutcomeV1', 'capped_socket_session_binding'], D + '🔤️wire/🦀️.rs')
    service = D + '🛠️service/🦀️.rs'
    E.move(root, ['ARTIFACT_CAS_SWEEP_CONTINUATION_DOMAIN_V1', 'ARTIFACT_CAS_DELETE_LEASE_DOMAIN_V1', 'ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES', 'ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES', 'ArtifactCasSweepContinuation', 'ArtifactCasSweepRequest', 'ArtifactCasSweepResult'], service, region_wrap=True)
    drop_mounts(ws, root, ['model', 'capabilities', 'events', 'decider', 'service', 'sqlite', 'postgres', 'neo4j', 'dispatch', 'tests'])
    privatize_glob_reexports(ws, root, ['wire', 'port'])
    remove_items(ws, root, lambda it, toks: it.kind == 'mod' and it.name == 'backend_corpus')
    append(ws, root, """#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🔮️backend-corpus/🦀️.rs"]
pub mod backend_corpus;""")
    # ---------------------------------------------------------------- backends: tests that need the writer move to the service crate
    for b, nm in (('🪶️sqlite', 'sqlite'), ('🐘️postgres', 'postgres'), ('🌐️neo4j', 'neo4j')):
        f = D + b + '/🦀️.rs'
        remove_items(ws, f, lambda it, toks: it.kind == 'mod' and it.name == 'creation_tests')
        if nm in ('postgres', 'neo4j'):
            fx = {'postgres': 'PostgresFixture', 'neo4j': 'Neo4jFixture'}[nm]
            tk = D + b + '/🧪️tests/🧰️testkit/🦀️.rs'
            create(ws, tk, f'//! 🧰️ Live-lane fixture of the {nm} directory backend: the claimed shared server and the directory over an emptied store.\n')
            E.move(D + b + '/🧪️tests/🔬️unit/🦀️.rs', [fx, 'test_directory'] + (['NEXT_DATABASE'] if nm == 'postgres' else []), tk)
            append(ws, f, '#[cfg(any(test, feature = "testkit"))]\n#[path = "🧪️tests/🧰️testkit/🦀️.rs"]\npub mod testkit;')
            E.proxy[tk] = D + b + '/🧪️tests/🔬️unit/🦀️.rs'
        append_tests = f'''#[cfg(test)]
#[path = "../../{b}/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs"]
mod {nm}_creation_tests;'''
        E_service_tests.append(append_tests)
        if nm == 'neo4j':
            rows = D + b + '/🧰️rows/🦀️.rs'
            its, _ = carve.parse_items(ws.read(f))
            movable = [i.name for i in its if i.kind not in ('use', 'impl', 'mod', 'none', 'extern_crate', 'include', 'macro') and i.name and i.name not in ('Neo4jDirectory', 'ArtifactGenesisTestControlV1', 'testkit')]
            create(ws, rows, '//! 🧰️ Neo4j directory rows: the Cypher row decoders and writers, the retry classification and the effect-commit guards every `Neo4jDirectory` operation is built from.\n')
            E.move(f, movable, rows, impl_idents=['AdminEffectCommitV1'])
            E.proxy[rows] = f
        drop_mounts(ws, f, ['tests'], glob_reexports=False)
        BACKEND_TESTS.append(f'#[cfg(test)]\n#[path = "../../{b}/🧪️tests/🔬️unit/🦀️.rs"]\nmod {nm}_tests;')
    # ---------------------------------------------------------------- service root: decider as child, tests
    replace_once(ws, service, 'use super::*;\n', '')
    append(ws, service, '''#[path = "../🧭️decider/🦀️.rs"]
mod decider;
use decider::*;''')
    laws = D + '🧪️tests/🏛️laws/🦀️.rs'
    create(ws, laws, '//! 🏛️ Directory laws: the writer and decider unit laws and the per-backend artifact-creation laws, run over the real backends.\n\n#[cfg(test)]\n#[path = "../🔬️unit/🦀️.rs"]\nmod unit;\n')
    for t in E_service_tests:
        append(ws, laws, t)
    E.proxy[laws] = D + '🦀️.rs'
    blaws = D + '🧪️tests/🏛️backend-laws/🦀️.rs'
    create(ws, blaws, '//! 🏛️ Directory backend laws: the per-backend unit laws of the sqlite, postgres and neo4j directories over their real stores.\n\n' + '\n\n'.join(BACKEND_TESTS) + '\n')
    E.move(A + '🌱️creation/🧪️tests/🔬️unit/🦀️.rs', ['directory_document_index_is_ordered_idempotent_and_backend_descriptor_bound'], D + '🧪️tests/🔬️unit/🦀️.rs')
    # ---------------------------------------------------------------- artifact authority
    aaroot = A + '🦀️.rs'
    replace_once(ws, aaroot, '''
    /// 📈️ Where a trusted-catalog load reports its per-package progress (`TrustedCatalogLoadProgressV1`), which the
    /// loaded catalog keeps for its background verification; `None` keeps both private to the catalog.
    fn catalog_progress(&self) -> Option<std::sync::Arc<trusted_catalog::TrustedCatalogLoadProgressCellV1>> {
        None
    }
''', '')
    replace_once(ws, aaroot, '''    /// 📈️ The catalog progress cell the host reads, when it reads one.
    pub(crate) fn catalog_progress(&self) -> Option<std::sync::Arc<trusted_catalog::TrustedCatalogLoadProgressCellV1>> {
        self.control.catalog_progress()
    }

''', '')
    drop_mounts(ws, A + '🦀️.rs', ['creation', 'check_in', 'trusted_catalog', 'native_openable_provider'], glob_reexports=False)
    # trusted catalog: the Arc delegation becomes a blanket impl owned by the trait's crate
    tcroot0 = A + '🔏️trusted-catalog/🦀️.rs'
    replace_once(ws, tcroot0, '''impl TrustedArtifactCatalog for Arc<VerifiedTrustedCatalog> {
    type Codec = VerifiedNativeArtifactCodec;

    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.as_ref().resolve(required).await
    }
}
''', '')
    append(ws, aaroot, '''impl<C: TrustedArtifactCatalog> TrustedArtifactCatalog for std::sync::Arc<C> {
    type Codec = C::Codec;

    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.as_ref().resolve(required).await
    }
}''')
    # creation: the catalog-bound genesis becomes a free function of the service crate
    cre = A + '🌱️creation/🦀️.rs'
    csrc = ws.read(cre)
    cits, _ = parse_items(csrc)
    cimpl = [i for i in cits if i.kind == 'impl' and carve.impl_self_ident(i) == 'ValidatingCanonicalArtifactAuthority'][0]
    body = csrc[cimpl.a:cimpl.b]
    ws.write(cre, csrc[:carve.attached_start(csrc, cimpl)] + csrc[cimpl.b:].lstrip('\n'))
    start = body.index('        context.checkpoint()?;')
    end = body.rindex('    }\n}')
    fnbody = body[start:end].replace('self.catalog.as_ref()', 'catalog').replace('self.catalog.', 'catalog.')
    fnbody = '\n'.join(l[4:] if l.startswith('    ') else l for l in fnbody.split('\n'))
    svc = A + '🌱️creation/🧑‍🏭️service-v1/🦀️.rs'
    append(ws, svc, '/// 🌱️ Builds one private zero-history candidate from the catalog\'s unambiguous creation selection without publishing or applying a domain edit.\npub async fn materialize_genesis(catalog: &VerifiedTrustedCatalog, request: ArtifactGenesisRequest, context: &OperationContext<\'_>) -> Result<ArtifactGenesisCandidate, AuthorityError> {\n' + fnbody.rstrip() + '\n}')
    replace_once(ws, svc, 'match ValidatingCanonicalArtifactAuthority::new(self.catalog.clone()).materialize_genesis(ArtifactGenesisRequest', 'match materialize_genesis(self.catalog.as_ref(), ArtifactGenesisRequest')
    drop_mounts(ws, cre, ['service'], glob_reexports=False)
    remove_items(ws, cre, lambda it, toks: it.kind == 'use' and 'ArtifactCreationServiceV1' in ''.join(t.x for t in it.toks_all))
    # trusted catalog: fixtures need the inference crate and move up with the testing crate
    tc = A + '🔏️trusted-catalog/🦀️.rs'
    remove_items(ws, tc, lambda it, toks: it.kind == 'mod' and it.name in ('trusted_catalog_fixture', 'count_trusted_catalog_fixture'))
    TC = A + '🔏️trusted-catalog/'
    replace_once(ws, tc, '''    pub async fn load_current(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
''', '''    pub async fn load_current(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
        Self::load_current_observed(data_path, providers, &Arc::default(), context).await
    }

    /// 📈️ Opens the current generation and reports every package's load progress into the host's `progress` cell.
    pub async fn load_current_observed(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, progress: &Arc<TrustedCatalogLoadProgressCellV1>, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
''')
    replace_once(ws, tc, 'providers, &verifications, context).await?;\n        if verified.generation_id()', 'providers, &verifications, progress, context).await?;\n        if verified.generation_id()')
    replace_once(ws, tc, '''    pub(crate) async fn load_fixture(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
''', '''    pub(crate) async fn load_fixture(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        Self::load_fixture_observed(bundle_path, profile_id, providers, &Arc::default(), context).await
    }

    #[cfg(any(test, feature = "integration-fixtures"))]
    pub(crate) async fn load_fixture_observed(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, progress: &Arc<TrustedCatalogLoadProgressCellV1>, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
''')
    replace_once(ws, tc, '&GuestCodecVerificationCacheV1::disabled(), context).await', '&GuestCodecVerificationCacheV1::disabled(), progress, context).await')
    replace_once(ws, tc, '''        verifications: &GuestCodecVerificationCacheV1,
        context: &OperationContext<'_>,
    ) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let (catalog, registration) = Self::verify_selected(root, bundle_path, bundle_bytes, profile_id, providers, verifications, context).await?;''', '''        verifications: &GuestCodecVerificationCacheV1,
        progress: &Arc<TrustedCatalogLoadProgressCellV1>,
        context: &OperationContext<'_>,
    ) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let (catalog, registration) = Self::verify_selected(root, bundle_path, bundle_bytes, profile_id, providers, verifications, progress, context).await?;''')
    replace_once(ws, tc, '''        verifications: &GuestCodecVerificationCacheV1,
        context: &OperationContext<'_>,
    ) -> Result<(VerifiedTrustedCatalog, ArtifactAssemblyRegistryPlan), AuthorityError> {''', '''        verifications: &GuestCodecVerificationCacheV1,
        progress: &Arc<TrustedCatalogLoadProgressCellV1>,
        context: &OperationContext<'_>,
    ) -> Result<(VerifiedTrustedCatalog, ArtifactAssemblyRegistryPlan), AuthorityError> {''')
    replace_once(ws, tc, 'let progress = context.catalog_progress().unwrap_or_default();', 'let progress = Arc::clone(progress);')
    tcut = TC + '🧪️tests/🔬️unit/🦀️.rs'
    replace_once(ws, tcut, '''
        fn catalog_progress(&self) -> Option<Arc<TrustedCatalogLoadProgressCellV1>> {
            Some(Arc::clone(&self.0))
        }
''', '')
    replace_once(ws, tcut, '''    let catalog = load_fixture(&fixture, &[fixture.binding()], &context).await.expect("fixture catalog");
    let observed = control.0.snapshot();''', '''    let catalog = TrustedCatalogLoader::load_fixture_observed(&fixture.bundle_path, "fixture", &FixtureProviderSource::new(vec![fixture.binding()]), &control.0, &context).await.expect("fixture catalog");
    let observed = control.0.snapshot();''')
    tcc = TC + '📜️contract/🦀️.rs'
    tcd = TC + '🧾️descriptor/🦀️.rs'
    create(ws, tcc, '''//! 📜️ Trusted-catalog contract: the bundle schema, the retained-file roots, the browser-actor and plugin-module sources and the bounded helpers every trusted-catalog crate shares.

#[path = "../🧬️schema/🦀️.rs"]
pub mod schema;

#[path = "../🌐️browser-actor/🦀️.rs"]
mod browser_actor;

#[path = "../🧩️plugin-module/🦀️.rs"]
pub mod plugin_module;

#[path = "../🛡️opened-root/🦀️.rs"]
mod opened_root;
''')
    create(ws, tcd, '//! 🧾️ Trusted-catalog descriptor: bundle and descriptor validation, document-open target derivation, native codec provider vocabulary and the retained asset handles of a verified package.\n')
    contract_names = ['TRUSTED_BUNDLE_MAX_BYTES', 'TRUSTED_DESCRIPTOR_MAX_BYTES', 'TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION', 'TRUSTED_COMPONENT_MAX_BYTES', 'TRUSTED_IDENTITY_MAX_BYTES', 'TRUSTED_RELATIVE_PATH_MAX_BYTES', 'TRUSTED_PACKAGE_MAX_DEPENDENCIES', 'TRUSTED_BUNDLE_MAX_PROFILES', 'TRUSTED_CATALOG_MAX_OPEN_TARGETS', 'catalog', 'catalog_error', 'valid_identity', 'valid_open_identity', 'valid_package_id', 'decode_digest', 'sha256', 'dual_hash', 'verify_length', 'verify_digest', 'append_document_open_catalog_field', 'ensure_count', 'validate_identity', 'validate_file']
    desc_names = ['decode_package_descriptor', 'reject_duplicate_descriptor_fields', 'validate_descriptor', 'validate_bundle', 'validate_native_bindings', 'SelectedTrustedBundleV1', 'CodecKey', 'NativeCodecBinding', 'NativeCodecProviderPackageV1', 'NativeCodecProviderRequirementV1', 'NativeCodecProviderSourceV1', 'descriptor_open_targets', 'app_opens_kind', 'validate_descriptor_open_target', 'hosted_codec_targets', 'open_target_codec_package', 'owner_preferred_creation', 'MostGeneralDialect', 'most_general_dialect', 'dialect_covers', 'declared_kind', 'document_open_target_sort_key', 'sort_open_targets', 'descriptor_open_targets_answer', 'selected_closure_digest', 'package_actor_renderer', 'append_trusted_profile_dependencies', 'trusted_profile_generation', 'trusted_profile_digests_json', 'VerifiedDocumentOpenSelectionV1', 'TrustedRetainedFile', 'TrustedCatalogAsset', 'TrustedCatalogAssetSource', 'TrustedCatalogAssetStream', 'TrustedCatalogAssetStreamSource', 'VerifiedPluginModule', 'VerifiedTrustedPackage', 'VerifiedExecutionTargetAssets', 'verify_plugin_module', 'report_package_progress']
    E.move(tc, contract_names, tcc)
    E.move(tc, desc_names, tcd)
    E.proxy[tcc] = tc
    E.proxy[tcd] = tc
    drop_mounts(ws, tc, ['schema', 'browser_actor', 'plugin_module', 'opened_root', 'tests'], glob_reexports=False)
    pubfile = TC + '🧪️tests/📤️publication/🦀️.rs'
    tcut0 = TC + '🧪️tests/🔬️unit/🦀️.rs'
    replace_once(ws, tcut0, 'include!("../📤️publication/🦀️.rs");', ws.read(pubfile).rstrip('\n'))
    ws.delete(pubfile)
    tccl = TC + '🧪️tests/🏛️contract-laws/🦀️.rs'
    drop_mounts(ws, TC + '🧩️plugin-module/🦀️.rs', ['tests'], glob_reexports=False)
    drop_mounts(ws, TC + '🛡️opened-root/🦀️.rs', ['publication_tests'], glob_reexports=False)
    drop_mounts(ws, TC + '🌐️browser-actor/🦀️.rs', ['tests'], glob_reexports=False)
    create(ws, tccl, '''//! 🏛️ Trusted-catalog contract laws: the plugin-module index, the opened-root publication and the browser-actor laws over the contract.

#[cfg(test)]
#[path = "../../🧩️plugin-module/🧪️tests/🔬️unit/🦀️.rs"]
mod plugin_module_tests;

#[cfg(test)]
#[path = "../../🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs"]
mod publication_tests;

#[cfg(test)]
#[path = "../../🌐️browser-actor/🧪️tests/🔬️unit/🦀️.rs"]
mod browser_actor_tests;
''')
    tclaws = TC + '🧪️tests/🏛️laws/🦀️.rs'
    create(ws, tclaws, '//! 🏛️ Trusted-catalog laws: the loader, guest residency, execution-target and document-open laws over the verified catalog.\n\n#[cfg(test)]\n#[path = "../🔬️unit/🦀️.rs"]\nmod unit;\n')
    E.proxy[tclaws] = tc
    E.move(TC + '🧪️tests/🔬️unit/🦀️.rs', [], tc, with_impls=False, impl_idents=['Vec'])
    replace_once(ws, tc, 'impl GuestResidentFootprintV1 for Vec<u8> {', '#[cfg(any(test, feature = "testkit"))]\nimpl GuestResidentFootprintV1 for Vec<u8> {')
    libroot = P('📦️packages/🦀️rust/🦀️.rs')
    drop_mounts(ws, libroot, ['directory', 'artifact_authority', 'refusal', 'test_artifact_root', 'auth', 'stores', 'lag_rebootstrap', 'local_bootstrap', 'observability', 'credential_source_order_oracle', 'socket_grant_oracle', 'inference'], glob_reexports=False)
    inference_structural(ws, E)
    drop_mounts(ws, libroot, ['component_codec_tests'], glob_reexports=False)
    append(ws, P('🧪️tests/🔏️trusted-catalog-profile/🦀️.rs'), '#[path = "🪶️count/🦀️.rs"]\npub mod count_trusted_catalog_fixture;')
    E.proxy[P('🧪️tests/🔏️trusted-catalog-profile/🦀️.rs')] = P('🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs')
    create(ws, P('🧪️tests/🧩️component-codecs/🏛️laws/🦀️.rs'), '//! 🏛️ Component codec laws: the real hub-owned component codecs over caller-selected runtimes.\n\n#[cfg(feature = "integration-fixtures")]\n#[path = "../🦀️.rs"]\nmod component_codecs;\n')
    append(ws, P('🔐️auth/🦀️.rs'), '#[cfg(test)]\n#[path = "🧪️tests/🧭️credential-source-order/🦀️.rs"]\nmod credential_source_order_oracle;')
    append(ws, D + '🦀️.rs', '#[cfg(test)]\n#[path = "🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs"]\nmod socket_grant_oracle;')
    return contract


def inference_structural(ws, E):
    I = P('💡️inference/')
    RT = I + '🏃️runtime/'
    iroot = I + '🦀️.rs'
    drop_mounts(ws, iroot, ['catalog', 'sqlite', 'authorization', 'runtime'], glob_reexports=False)
    remove_items(ws, iroot, lambda it, toks: it.kind == 'use' and ('verified_gis_map_binding' in ''.join(t.x for t in it.toks_all) or 'semio_hub_inference_binding' in ''.join(t.x for t in it.toks_all)))
    replace_once(ws, iroot, '#[path = "✉️command/🦀️.rs"]\npub(crate) mod command;', '#[path = "✉️command/🦀️.rs"]\npub mod command;')
    catroot = I + '📇️catalog/🦀️.rs'
    append(ws, catroot, '#[path = "../🛂️authorization/🦀️.rs"]\npub mod authorization;')
    rtc = RT + '📜️contract/🦀️.rs'
    create(ws, rtc, '''//! 📜️ Inference runtime contract: the approval contract and committer port, the neutral composed-map binding, the job vocabulary and the bounded helpers every runtime crate shares.

#[path = "../🧾️approval-contract/🦀️.rs"]
mod approval_contract;
use approval_contract::*;

#[path = "../🔗️binding/🦀️.rs"]
mod binding;
use binding::*;

#[path = "../🧭️job-types-and-port/🦀️.rs"]
mod job_types_and_port;
use job_types_and_port::*;
''')
    helpers = ['OPERATION_CAPACITY', 'DOCUMENT_GATE_CAPACITY', 'WORK_UNIT_LIMIT', 'ALLOCATION_BYTES', 'RECURSION_DEPTH', 'APPROVAL_MAX_RECORDS', 'GIS_MAP_COMMITTER_CAPACITY', 'approval_stamp_v1', 'document_key', 'fresh_now_ms', 'parse_frontier_chain_hash', 'approval_actor', 'approval_mutation_id', 'reader', 'inference_budget']
    rtroot = RT + '🦀️.rs'
    E.move(rtroot, helpers, rtc)
    E.proxy[rtc] = rtroot
    E.move(RT + '🧭️job-types-and-port/🦀️.rs', ['RetainedInferenceRunOwnerV1'], RT + '🏭️inference-runtime/🦀️.rs')
    E.move(RT + '🧭️job-types-and-port/🦀️.rs', [], RT + '🔐️committer/🦀️.rs', with_impls=False, impl_idents=['RetainedGisMapApprovalCommitterV1'])
    E.move(RT + '🚧️checkpoint-test-gate/🦀️.rs', [], I + '🧬️schema/🦀️.rs', with_impls=False, impl_idents=['GisInferenceCheckpointControlFrameV1'])
    drop_mounts(ws, rtroot, ['approval_contract', 'binding', 'committer', 'document_preparation', 'commit_turns', 'checkpoint_verify', 'commit_and_close', 'job_types_and_port', 'tests'])
    icl = I + '🧪️tests/🏛️laws/🦀️.rs'
    drop_mounts(ws, I + '🧾️wal/🦀️.rs', ['tests'], glob_reexports=False)
    drop_mounts(ws, I + '🧬️schema/🦀️.rs', ['tests', 'scope_schema_export_law'], glob_reexports=False)
    drop_mounts(ws, I + '🧬️schema/✅️approval/🦀️.rs', ['tests'], glob_reexports=False)
    drop_mounts(ws, I + '✉️command/🦀️.rs', ['tests'], glob_reexports=False)
    create(ws, icl, '''//! 🏛️ Inference contract laws: the write-ahead proof, the schema, approval and canonical command laws of the hub-private inference lifecycle.

#[cfg(test)]
#[path = "../../🧾️wal/🧪️tests/🔬️unit/🦀️.rs"]
mod wal_tests;

#[cfg(test)]
#[path = "../../🧬️schema/🧪️tests/🔬️unit/🦀️.rs"]
mod schema_tests;

#[cfg(test)]
#[path = "../../🧬️schema/🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs"]
mod scope_schema_export_law;

#[cfg(test)]
#[path = "../../🧬️schema/✅️approval/🧪️tests/🔬️unit/🦀️.rs"]
mod approval_tests;

#[cfg(test)]
#[path = "../../✉️command/🧪️tests/🔬️unit/🦀️.rs"]
mod command_tests;
''')
    ilaws = RT + '🧪️tests/🏛️laws/🦀️.rs'
    create(ws, ilaws, '//! 🏛️ Inference laws: the owner-private proposal runtime and the retained approval committer over the real ledger.\n\n#[cfg(test)]\n#[path = "../🔬️unit/🦀️.rs"]\nmod unit;\n')
    E.proxy[ilaws] = rtroot
    cm = RT + '🔐️committer/🦀️.rs'
    for nm, folder in (('document_preparation', '🧱️document-preparation'), ('commit_turns', '🔁️commit-turns'), ('checkpoint_verify', '🔎️checkpoint-verify'), ('commit_and_close', '📕️commit-and-close')):
        append(ws, cm, f'#[path = "../{folder}/🦀️.rs"]\nmod {nm};')
    privatize_glob_reexports(ws, rtroot, ['inference_runtime', 'routes'])
    privatize_glob_reexports(ws, rtroot, ['checkpoint_test_gate'])


E_service_tests = []
BACKEND_TESTS = []


def main(apply=False):
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    base = Model(base_ws, collections.OrderedDict([('semio_hub', P('📦️packages/🦀️rust/🦀️.rs')), ('os_hub', P('🏗️bootstrap/🦀️.rs'))]), allow_dup=True)
    ws = wsm.WS()
    E = Edits(ws)
    contract = structural(ws, E)
    E.proxy[contract] = P('📇️directory/🦀️.rs')
    plan_bin.bin_structural(ws, E, base_ws)
    plan_bintests.bin_tests(ws, E, base_ws)
    plan_bin.bin_root_cleanup(ws, E)
    return ws, E, base, base_ws


if __name__ == '__main__':
    ws, E, base, base_ws = main()
    roots = collections.OrderedDict((k, v['root']) for k, v in CRATES.items())
    C = Carver(ws, E, base)
    C.build_new(roots)
    print('missing mounts:', [m.path for m in C.new.missing])
    print('pass1 files changed:', C.pass1())
    C.build_new(roots)
    print('pass2 files changed:', C.pass2())
    C.report.dump()
    print('changed files', len(ws.changed()))


def debug_show(paths, maxlines=80):
    import showdiff
    ws, E, base, base_ws = main()
    roots = collections.OrderedDict((k, v['root']) for k, v in CRATES.items())
    C = Carver(ws, E, base)
    C.build_new(roots)
    C.pass1()
    C.build_new(roots)
    C.pass2()
    for p in paths:
        print('=' * 20, p)
        showdiff.show(ws, p, maxlines)


def run_all(variants=(('native', None, ()),), baseline=True, tag='lib'):
    import pickle, hashlib
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    results = {}
    if baseline:
        for vn, under, cfgs in variants:
            pk = f'/tmp/hubw/base-{tag}-{vn}.pkl'
            if os.path.exists(pk):
                results[('base', vn)] = pickle.load(open(pk, 'rb'))
                continue
            u = union.Union(base_ws, {'semio_hub': P('📦️packages/🦀️rust/🦀️.rs')}, tag='base-' + vn)
            diags, err = u.run('semio_hub' if under else None, cfgs)
            c = union.multiset(diags)
            pickle.dump(c, open(pk, 'wb'))
            results[('base', vn)] = c
    ws, E, base, _ = main()
    roots = collections.OrderedDict((k, v['root']) for k, v in CRATES.items())
    C = Carver(ws, E, base)
    C.build_new(roots)
    C.pass1()
    C.build_new(roots)
    C.pass2()
    C.build_new(roots)
    C.pass3(roots)
    C.build_new(roots)
    C.finalize_exports()
    C.build_new(roots)
    C.report.dump()
    for vn, under, cfgs in variants:
        u = union.Union(ws, roots, tag='split-' + vn)
        diags, err = u.run(under, cfgs)
        results[('split', vn)] = (union.multiset(diags), diags)
    return ws, C, results


def compare(results, vn='native', limit=80):
    b = results[('base', vn)]
    s, diags = results[('split', vn)]
    crates = set(CRATES)
    def clean(c):
        return collections.Counter({k: n for k, n in c.items() if not union.is_extern_noise(k[0], k[1], crates)})
    b0, s0 = b, s
    b, s = clean(b), clean(s)
    print(f'extern noise: base {sum(b0.values()) - sum(b.values())} split {sum(s0.values()) - sum(s.values())}')
    only_s, only_b = union.diff(s, b)
    print(f'[{vn}] base {sum(b.values())} errors / split {sum(s.values())}; only-split {sum(only_s.values())} only-base {sum(only_b.values())}')
    print('--- only in split')
    for (code, msg, f), n in sorted(only_s.items(), key=lambda x: (x[0][2], x[0][1]))[:limit]:
        print(f'   {n}x {code} {msg[:150]}   @ {f[-60:]}')
    print('--- only in base')
    for (code, msg, f), n in sorted(only_b.items(), key=lambda x: (x[0][2], x[0][1]))[:limit]:
        print(f'   {n}x {code} {msg[:150]}   @ {f[-60:]}')
    return only_s, only_b


ALLFEAT = ('feature="integration-fixtures"', 'feature="testkit"')


def prepare():
    ws, E, base, base_ws = main()
    roots = collections.OrderedDict((k, v['root']) for k, v in CRATES.items())
    C = Carver(ws, E, base)
    C.test_true_crates = {plan_bintests.TESTKIT[0]}
    C.build_new(roots)
    C.pass1()
    C.build_new(roots)
    C.pass2()
    plan_bintests.postprocess(ws)
    plan_bintests.postprocess_files(ws, [P('🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs'), P('📇️directory/🧪️tests/🔬️unit/🦀️.rs')])
    C.build_new(roots)
    C.pass3(roots)
    C.build_new(roots)
    C.finalize_exports()
    C.build_new(roots)
    C.open_test_imports()
    C.build_new(roots)
    C.flip_not_test_partners()
    C.build_new(roots)
    import scan
    found = scan.member_scan(C.new)
    C.widen_members(found)
    C.build_new(roots)
    C.member_findings = found
    return ws, C, roots, base_ws


def baseline_variant(label, under, cfgs, base_ws):
    import pickle
    pk = f'/tmp/hubw/res/base2-{label}.pkl'
    os.makedirs('/tmp/hubw/res', exist_ok=True)
    if os.path.exists(pk):
        return pickle.load(open(pk, 'rb'))
    hcmod.FS = base_ws
    u = union.Union(base_ws, collections.OrderedDict([('semio_hub', P('📦️packages/🦀️rust/🦀️.rs')), ('os_hub', P('🏗️bootstrap/🦀️.rs'))]), tag='base-' + label)
    diags, err = u.run(under, tuple(cfgs) + ALLFEAT)
    c = union.multiset(diags)
    pickle.dump(c, open(pk, 'wb'))
    return c


def split_variant(label, ws, roots, under, cfgs):
    import pickle
    os.makedirs('/tmp/hubw/res', exist_ok=True)
    u = union.Union(ws, roots, tag='split-' + label.replace(':', '_'))
    diags, err = u.run(under, tuple(cfgs) + ALLFEAT)
    c = union.multiset(diags)
    pickle.dump(c, open(f'/tmp/hubw/res/split-{label.replace(":", "_")}.pkl', 'wb'))
    return c


def diff_report(label, b, s, crates, limit=60):
    def clean(c):
        return collections.Counter({k: n for k, n in c.items() if not union.is_extern_noise(k[0], k[1], crates)})
    b, s = clean(b), clean(s)
    only_s, only_b = union.diff(s, b)
    print(f'[{label}] base {sum(b.values())} / split {sum(s.values())}; only-split {sum(only_s.values())} only-base {sum(only_b.values())}')
    for (code, msg, f), n in sorted(only_s.items(), key=lambda x: (x[0][2], x[0][1]))[:limit]:
        print(f'   SPLIT+ {n}x {code} {msg[:150]}   @ {f[-70:]}')
    return only_s, only_b


def final_edges(model):
    import regs
    names = set(CRATES)
    normal, testonly = {}, {}
    for ident in CRATES:
        n, t = regs.scan_usage(model, ident, names)
        n.discard(ident)
        t.discard(ident)
        normal[ident] = n
        testonly[ident] = t - n
    return normal, testonly


def find_cycles(normal):
    index, low, onstack, stack, out = {}, {}, set(), [], []
    counter = [0]
    sys.setrecursionlimit(10000)

    def strong(v):
        index[v] = low[v] = counter[0]
        counter[0] += 1
        stack.append(v)
        onstack.add(v)
        for w in normal.get(v, ()):
            if w not in index:
                strong(w)
                low[v] = min(low[v], low[w])
            elif w in onstack:
                low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop()
                onstack.discard(w)
                comp.append(w)
                if w == v:
                    break
            if len(comp) > 1:
                out.append(comp)

    for v in list(normal):
        if v not in index:
            strong(v)
    return out


def structure_report(C):
    import scan
    normal, testonly = final_edges(C.new)
    print('CRATE DAG (normal edges):')
    for f, ts in sorted(normal.items()):
        if ts:
            print('   ', f, '->', ', '.join(sorted(ts)))
    cyc = find_cycles(normal)
    print('CYCLES:', cyc if cyc else 'none')
    print('TEST-ONLY EDGES:')
    for f, ts in sorted(testonly.items()):
        if ts:
            print('   ', f, '->', ', '.join(sorted(ts)))
    both = find_cycles({f: set(normal[f]) | set(testonly[f]) for f in normal})
    print('CYCLES INCLUDING TEST EDGES:', both if both else 'none')
    print('ORPHAN/INHERENT findings:')
    for r in scan.orphan_scan(C.new):
        print('   ', r)
    print('MEMBER widenings applied:', len(getattr(C, 'member_findings', [])))
    sys.stdout.flush()


def batch(test_crates, with_fixtures=False):
    ws, C, roots, base_ws = prepare()
    structure_report(C)
    crates = set(CRATES)
    out = {}
    b = baseline_variant('native', None, (), base_ws)
    s = split_variant('native', ws, roots, None, ())
    diff_report('native', b, s, crates)
    bt = baseline_variant('test', 'semio_hub', (), base_ws)
    for c in test_crates:
        st = split_variant('test:' + c, ws, roots, c, ())
        diff_report('test:' + c, bt, st, crates)
    if with_fixtures:
        bf = baseline_variant('fix', None, ('feature="integration-fixtures"',), base_ws)
        sf = split_variant('fix', ws, roots, None, ('feature="integration-fixtures"',))
        diff_report('fix', bf, sf, crates)


def census(ws, roots, label='lib'):
    import json, subprocess, tempfile
    overlay = {p: ws.cur[p] for p in ws.cur}
    ov = f'/tmp/hubw/overlay-{label}.json'
    json.dump(overlay, open(ov, 'w'))
    dl = f'/tmp/hubw/deleted-{label}.json'
    json.dump(sorted(ws.deleted), open(dl, 'w'))
    specs = [dict(name=k, root=v, pkg=os.path.join(os.path.dirname(v), '📦️packages', '🦀️rust')) for k, v in roots.items()]
    sp = f'/tmp/hubw/specs-{label}.json'
    json.dump(specs, open(sp, 'w'))
    r = subprocess.run(['bun', os.path.join(os.path.dirname(os.path.abspath(__file__)), 'census-overlay.ts')], capture_output=True, text=True, cwd=REPO, env=dict(os.environ, OVERLAY=ov, DELETED=dl, SPECS=sp))
    print(r.stdout)
    if r.stderr.strip():
        print(r.stderr[-800:])
    return r.stdout


def save_state(ws, C, path='/tmp/hubw/state.pkl'):
    import pickle
    pickle.dump(dict(cur=ws.cur, created=ws.created, deleted=ws.deleted, orig=ws.orig, moved=C.edits.moved), open(path, 'wb'))


def load_state(path='/tmp/hubw/state.pkl'):
    import pickle
    st = pickle.load(open(path, 'rb'))
    ws = wsm.WS()
    ws.cur, ws.created, ws.deleted, ws.orig = st['cur'], st['created'], st['deleted'], st['orig']
    roots = collections.OrderedDict((k, v['root']) for k, v in CRATES.items())
    hcmod.FS = ws
    model = Model(ws, roots)
    return ws, model, roots
