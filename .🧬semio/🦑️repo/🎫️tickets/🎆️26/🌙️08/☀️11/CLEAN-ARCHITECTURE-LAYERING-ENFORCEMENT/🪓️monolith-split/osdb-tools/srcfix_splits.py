"""✂️ Source-law tests that scan more than one carved file, split by the file each assertion scans.

Every function maps the staged test text to {'replace': {(lib, name): text|None}, 'add': [(host lib, text)]}.
"""

import re

OWN = 'include_str!("../../🦀️.rs")'

RETRY_FAMILIES = [
    'db_engine', 'db_engine_base', 'db_engine_capability_open', 'db_engine_catalog_read', 'db_engine_catalog_bootstrap',
    'db_engine_catalog_create', 'db_engine_handle', 'db_artifact', 'db_artifact_contract', 'db_artifact_history', 'db_artifact_engine',
]

HANDLE_ONE_GRANT = '''#[test]
fn artifact_history_one_grant_advances_one_retained_phase_without_blocking() {
    let engine = include_str!("../../🦀️.rs");
    let history = &engine[engine.find("const ARTIFACT_HISTORY_OPERATION_SLOTS").unwrap()..engine.find("pub struct ArtifactHandle").unwrap()];
    for forbidden in ["block_on(", "ask_blocking", "submit_blocking", "loop {", "while "] {
        assert!(!history.contains(forbidden), "history outer authority retained {forbidden}");
    }
    assert!(history.contains("std::pin::Pin::new(future).poll(&mut context)"));
}'''

HISTORY_ONE_GRANT = '''#[test]
fn artifact_history_replay_cursor_one_grant_has_no_blocking_wait() {
    let artifact = include_str!("../../🦀️.rs");
    let replay = &artifact[artifact.find("//#region 🔖️HistoryReplay").unwrap()..artifact.find("//#endregion 🔖️HistoryReplay").unwrap()];
    for forbidden in ["block_on(", "ask_blocking", "submit_blocking", "loop {", "while "] {
        assert!(!replay.contains(forbidden), "history replay cursor retained {forbidden}");
    }
}'''

ARTIFACT_ONE_GRANT = '''#[test]
fn artifact_runner_polls_the_history_replay_future_once_per_grant() {
    assert!(include_str!("../../🦀️.rs").contains("future.as_mut().poll(&mut context)"));
}'''

WAL_CRC_AUTHENTICATION = '''#[test]
fn history_crc_faults_are_authenticated_by_the_wal_chain() {
    assert!(include_str!("../../🦀️.rs").contains("wal chain crc or frame length differs"), "history CRC faults are authenticated by the WAL chain");
}'''


def lines_with(body, needle):
    return [line for line in body.split('\n') if needle in line]


def splits(bodies):
    out = {'replace': {}, 'add': []}

    name = 'artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each'
    b = bodies[('db_artifact_history', name)]
    wal_line = '    assert!(include_str!("../../../../📝️wal/🦀️.rs").contains("wal chain crc or frame length differs"), "history CRC faults are authenticated by the WAL chain");'
    assert wal_line in b
    out['replace'][('db_artifact_history', name)] = b.replace(wal_line + '\n', '').replace('include_str!("../../../🦀️.rs")', OWN)
    out['add'].append(('db_wal_log', WAL_CRC_AUTHENTICATION))

    name = 'artifact_history_crc_and_frame_tokenizer_advance_one_page_per_grant'
    b = bodies[('db_engine', name)]
    art, wal = lines_with(b, 'artifact.contains'), lines_with(b, 'wal.contains')
    assert len(art) == 5 and len(wal) == 3, (len(art), len(wal))
    out['add'].append(('db_artifact_history', f'#[test]\nfn {name}() {{\n    let artifact = {OWN};\n' + '\n'.join(art) + '\n}'))
    out['add'].append(('db_wal_log', f'#[test]\nfn wal_crc_cursor_and_frame_tokenizer_advance_one_page_per_grant() {{\n    let wal = {OWN};\n' + '\n'.join(wal) + '\n}'))
    out['replace'][('db_engine', name)] = None

    name = 'retained_sync_hello_production_census_has_zero_blocking_waits_and_no_eager_follow_up'
    b = bodies[('db_sync', name)]
    i, j = b.index('    let engine = '), b.index('    let source = ')
    engine_part = b[i:j]
    assert 'production.contains("DatabaseSyncHelloFuture::try_submit")' in engine_part
    out['add'].append(('db_engine', '#[test]\nfn retained_sync_hello_engine_production_has_zero_blocking_waits_and_no_eager_follow_up() {\n' + engine_part.replace('include_str!("../../../⚙️engine/🦀️.rs")', OWN).rstrip('\n') + '\n}'))
    out['replace'][('db_sync', name)] = b.replace(engine_part, '')

    name = 'database_create_catalog_one_production_opportunity_is_under_eight_ms_and_native_wasm_share_source'
    b = bodies[('db_engine_catalog_create', name)]
    out['replace'][('db_engine_catalog_create', name)] = b.replace('include_str!("../../../🦀️.rs")', OWN)

    name = 'retained_retry_owners_release_their_guard_before_resubmission'
    b = bodies[('db_engine', name)]
    head = '    for (name, source) in [("engine", retained_submit_source()), ("artifact", include_str!("../../../🗿️artifact/🦀️.rs"))] {'
    assert head in b
    for lib in RETRY_FAMILIES:
        out['add'].append((lib, b.replace(head, f'    for (name, source) in [("{lib}", {OWN})] {{')))
    out['replace'][('db_engine', name)] = None

    name = 'artifact_history_construction_fault_is_public_and_admission_release_is_a_final_grant'
    b = bodies[('db_engine', name)]
    rows = b.split('\n')
    artifact_rows = [r for r in rows if 'assert!(artifact.contains' in r or 'assert!(!artifact.contains' in r]
    assert len(artifact_rows) == 9, len(artifact_rows)
    kept = [r for r in rows if r not in artifact_rows and 'let artifact = ' not in r]
    out['add'].append(('db_engine_handle', '\n'.join(kept).replace('retained_submit_source()', OWN)))
    out['add'].append(('db_artifact_history', f'#[test]\nfn artifact_history_replay_construction_fault_authority_is_retained() {{\n    let artifact = {OWN};\n' + '\n'.join(artifact_rows) + '\n}'))
    out['replace'][('db_engine', name)] = None

    name = 'artifact_history_one_grant_advances_one_retained_phase_without_blocking'
    out['add'] += [('db_engine_handle', HANDLE_ONE_GRANT), ('db_artifact_history', HISTORY_ONE_GRANT), ('db_artifact', ARTIFACT_ONE_GRANT)]
    out['replace'][('db_engine', name)] = None

    name = 'document_authority_durable_group_journal_commits_one_exact_fsync_event'
    b = bodies[('db_artifact', name)]
    i = b.index('    let source = include_str!(', b.index('witness.transaction_id()'))
    tail = b[i:].rstrip('}\n').rstrip()
    assert tail.count('witness_api') >= 4
    out['replace'][('db_artifact', name)] = b[:i].rstrip('\n') + '\n}'
    out['add'].append(('db_artifact_contract', '#[test]\nfn durable_group_witness_api_exposes_store_owned_recovery_without_record_access() {\n' + re.sub(r'include_str!\("[^"]*"\)', lambda m: OWN, tail, count=1) + '\n}'))

    name = 'preview_crate_never_references_wal_shaped_symbols'
    b = bodies[('db_preview', name)]
    manifest = 'include_str!("../../../📦️packages/🦀️rust/Cargo.toml")'
    forbidden = 'for forbidden_dependency in ["db_wal", "db_storage", "db_snapshot", "db_artifact", "db_engine"] {'
    assert manifest in b and forbidden in b
    out['replace'][('db_preview', name)] = b.replace(manifest, 'include_str!("../../📦️packages/🦀️rust/Cargo.toml")').replace(
        forbidden, 'for forbidden_dependency in ["db_wal", "db_storage", "db_snapshot", "db_artifact", "db_engine", "db-wal", "db-storage", "db-snapshot", "db-artifact", "db-engine", "db-io", "db-fault"] {')
    return out
