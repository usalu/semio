#!/usr/bin/env python3
# 🗂️ The WP2 carve model: crate table (root, mounted modules, lift modules), module -> crate map, root-name table, upstream crates.
import json, os, re, sys
from common import *
import modmap

CRATES_JSON = json.load(open(GEN + '/pwp0-crates.json'))
UNITS = json.load(open(GEN + '/pwp2-units.json'))
WP2 = ['decl-views', 'decl-emission', 'decl-app', 'app-builder', 'decl-surface', 'decl-registry', 'plugin-program']

def libname(key):
    return PFX + key.replace('-', '_')

# crate -> root file (relative to PLUG) and mounted modules: name -> (file, kind). `file` is the existing module file (stays where it is).
ROOT = {k: CRATES_JSON[k]['root'] + '/🦀️.rs' for k in CRATES_JSON}
MODS_RAW = modmap.build()
MOD_FILE = {m['name']: m['file'] for m in MODS_RAW}
import plan_tools as pt
MOD_CRATE = {n: pt.file_crate(f) for n, f in MOD_FILE.items()}
MOD_CRATE['reactor'] = 'reactor-core'

# per-crate children: module name -> file (mounted from the crate root with #[path], relative to PLUG)
CHILDREN = {
    'decl-views': {
        'child_view': '📜️declaration/🪆️child/🦀️.rs', 'gesture_contract': '📜️declaration/🛠️tool-machine/🦀️.rs', 'history_view': '📜️declaration/🎮️command/🦀️.rs',
        'media_contract': '📜️declaration/🎞️media/🦀️.rs', 'tool_run_contract': '📜️declaration/⏯️tool-run/🦀️.rs', 'presence_retirement': '👥️presence/♻️retirement/🦀️.rs',
        'no_config_retirement': '🎚️config/🚫️none/♻️retirement/🦀️.rs', 'no_draft_retirement': '📝️draft/🚫️none/♻️retirement/🦀️.rs', 'transient_retirement': '🫧️transient/♻️retirement/🦀️.rs',
    },
    'decl-emission': {},
    'decl-app': {'tool_contract': '📜️declaration/👷️job/🦀️.rs'},
    'app-builder': {},
    'decl-surface': {'natural_file_import': '📜️declaration/📥️natural-file/🦀️.rs'},
    'decl-registry': {'artifact_registration': '📜️declaration/📇️registration/🦀️.rs', 'declarations': '📜️declaration/🌳️tree/🦀️.rs'},
    'plugin-program': {'surface_context': '⚛️reactor/🪟️surfaces/🦀️.rs'},
}

for _k, _ch in CHILDREN.items():
    for _n, _f in _ch.items():
        MOD_FILE.setdefault(_n, _f)
        MOD_CRATE.setdefault(_n, _k)
MOD_FILE.setdefault('surface_context', '⚛️reactor/🪟️surfaces/🦀️.rs')
MOD_CRATE['surface_context'] = 'plugin-program'

def crate_of_module(name):
    c = MOD_CRATE.get(name)
    return c

def module_in_crate(name):
    c = MOD_CRATE.get(name)
    if c in WP2 and MOD_FILE[name] == ROOT[c]:
        return (c, None)
    return (c, name)

# lift modules a crate mounts (incoming lifts and intra moves): crate -> [(module name, lift file relative to PLUG, state)] state: 'incoming' (stub filled by another package) | 'intra' (filled by this tool)
LIFTS_IN = {
    'decl-views': [('tool_contract', '📜️declaration/👁️view/🛬️lifted/👷️job/🦀️.rs', 'intra')],
    'decl-emission': [
        ('child_emit_preparation', '📜️declaration/📨️emission/🛬️lifted/📦️preparation/🦀️.rs', 'incoming'),
        ('surface_contract', '📜️declaration/📨️emission/🛬️lifted/🪟️surface/🦀️.rs', 'intra'),
    ],
    'decl-app': [
        ('completion_retirement', '📜️declaration/🧩️app/🛬️lifted/♻️retirement/🦀️.rs', 'incoming'),
        ('completion_fault', '📜️declaration/🧩️app/🛬️lifted/⚠️fault/🦀️.rs', 'incoming'),
        ('completion_refusal_retirement', '📜️declaration/🧩️app/🛬️lifted/🚫️refusal/🦀️.rs', 'incoming'),
        ('owned_context', '📜️declaration/🧩️app/🛬️lifted/🧬️context/🦀️.rs', 'incoming'),
        ('media_operation_refusal', '📜️declaration/🧩️app/🛬️lifted/🎞️media-turn/🦀️.rs', 'incoming'),
        ('media_intrinsic', '📜️declaration/🧩️app/🛬️lifted/🛫️intrinsic/🦀️.rs', 'incoming'),
        ('operation_progress', '📜️declaration/🧩️app/🛬️lifted/⏳️operation-progress/🦀️.rs', 'incoming'),
        ('app_emission', '📜️declaration/🧩️app/🛬️lifted/📨️emission/🦀️.rs', 'intra'),
        ('tool_run_contract', '📜️declaration/🧩️app/🛬️lifted/⏯️tool-run/🦀️.rs', 'intra'),
        ('media_contract', '📜️declaration/🧩️app/🛬️lifted/🎬️media-export/🦀️.rs', 'intra'),
    ],
    'decl-registry': [
        ('artifact_inference', '📜️declaration/🛬️lifted/💡️inference/🦀️.rs', 'incoming'),
        ('plugin_program', '📜️declaration/🛬️lifted/🔌️program/🦀️.rs', 'intra'),
    ],
    'plugin-program': [
        ('plugin_runtime', '🏗️builder/🔌️program/🛬️lifted/🏃️runtime/🦀️.rs', 'incoming'),
        ('artifact_inference', '🏗️builder/🔌️program/🛬️lifted/💡️inference/🦀️.rs', 'incoming'),
        ('inference_gateway', '🏗️builder/🔌️program/🛬️lifted/🚪️gateway/🦀️.rs', 'incoming'),
    ],
}

# items cut by this package: (source file, [item names]) -> (destination crate, destination module, destination lift file or an existing foreign lift stub)
# every name is the defining name of a plan_tools unit; bound inherent impls travel with the unit; trait impls named in IMPLS travel with their type (orphan rule)
CUTS = [
    dict(file='📜️declaration/👷️job/🦀️.rs', to='decl-views', module='tool_contract', names=['ArtifactInstanceOperationOwner', 'OriginalInferenceSessionOwner', 'OriginalInferenceSession', 'InstanceInferenceSessions', 'original_inference_close_demands', 'original_inference_close_step', 'ArtifactInstanceOperationOwnerHandle', 'ArtifactInferenceSessionAccess', 'original_inference_metadata', 'ArtifactInstanceOperationOwnerAliasRetirement', 'EmptyArtifactInstanceOperationOwner'], include=('♻️alias/🦀️.rs', '📜️declaration/👷️job/♻️alias/🦀️.rs', '../../../👷️job/♻️alias/🦀️.rs'),
         tests=[('#[cfg(test)]\n#[path="🧪️inference-close/🦀️.rs"]\nmod original_inference_close_tests;\n', '📜️declaration/👷️job/🧪️inference-close/🦀️.rs', '../../../👷️job/🧪️inference-close/🦀️.rs', 'original_inference_close_tests'),
                ('#[cfg(test)]\n#[path="🧪️inference-admission/🦀️.rs"]\nmod original_inference_admission_tests;\n', '📜️declaration/👷️job/🧪️inference-admission/🦀️.rs', '../../../👷️job/🧪️inference-admission/🦀️.rs', 'original_inference_admission_tests')]),
    dict(file='📜️declaration/📨️emission/🦀️.rs', to='decl-app', module='app_emission', names=['EphemeralEmit']),
    dict(file='📜️declaration/⏯️tool-run/🦀️.rs', to='decl-app', module='tool_run_contract', names=['ToolRunJobRequest']),
    dict(file='📜️declaration/🎞️media/🦀️.rs', to='decl-app', module='media_contract', names=['ArtifactMediaExportJobRequest', 'ArtifactMediaExportRequestRetirement']),
    dict(file='📜️declaration/🪟️surface/🦀️.rs', to='decl-emission', module='surface_contract', names=['ViewEmit']),
    dict(file='🏗️builder/🔌️program/🦀️.rs', to='decl-registry', module='plugin_program', names=['AppFactory']),
    dict(file='📜️declaration/👁️view/🦀️.rs', to='operation', module='view', names=['ArtifactOperationProgress', 'CANCEL_TYPED_OPERATION_ACTION_ID'], lift='♻️cancellation/🛬️lifted/👁️view/🦀️.rs'),
    dict(file='📜️declaration/🌳️tree/🦀️.rs', to='extension', module='tree', names=['preflight_artifact_declarations', 'commit_artifact_declarations'], lift='🧩️extension/🛬️lifted/🌳️tree/🦀️.rs'),
    dict(file='📜️declaration/🌳️tree/🦀️.rs', to='instance-app', module='tree', names=['editor_surface', 'viewer_surface', 'EditorSurfaceApp', 'ViewerSurfaceApp'], lift='🧵️instance/🛬️lifted/🌳️tree/🦀️.rs'),
    dict(file='📜️declaration/🧩️app/🦀️.rs', to='document-media', module='app', names=['MediaArtifact', 'whole_document_media'], lift='🗃️document/🛬️lifted/🧩️app/🦀️.rs',
         after=[('🎞️media/🛫️intrinsic/🦀️.rs', '../../../🎞️media/🛫️intrinsic/🦀️.rs'), ('🎞️media/💥️fault/🦀️.rs', '../../../🎞️media/💥️fault/🦀️.rs')]),
]


# test files that name facade-only items (higher crates, facade macros, test fixtures of the facade) leave their module and run as test host files below hub 2 (`🪝️mounts/📜️wp2-declaration/🧪️hosts/<emoji>️<slug>/🦀️.rs`)
# kind include: the host file `include!`s the test (the test had no inner docs); kind path: the host file mounts it with `#[path]` (the test starts with `//!` and `use super::*`)
HOSTS_DIR = '🪝️mounts/📜️wp2-declaration/🧪️hosts'
TEST_HOSTS = [
    dict(emoji='🔬️', slug='declarations-fixture', file='📜️declaration/🌳️tree/🦀️.rs', module='declarations', host='declarations_fixture_tests', test='🧪️tests/🔬️app-declarations-fixture/🦀️.rs', kind='include', cfg='test', reexport=['fixture'], remove='#[cfg(test)]\ninclude!("../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs");\n'),
    dict(emoji='🏗️', slug='app-builder', file='🏗️builder/🧩️app/🦀️.rs', module='app_builder', host='app_builder_tests_host', test='🧪️tests/🔬️app-app-builder/🦀️.rs', kind='include', cfg='test', remove='#[cfg(test)]\ninclude!("../../🧪️tests/🔬️app-app-builder/🦀️.rs");\n'),
    dict(emoji='🪪️', slug='codec-binding', file='📜️declaration/📇️registration/🦀️.rs', module='artifact_registration', host='original_codec_binding_laws', test='🧪️tests/🪪️codec-binding/🦀️.rs', kind='include', cfg='test', remove='#[cfg(test)]\nmod original_codec_binding_laws {\n    use super::*;\n    include!("../../🧪️tests/🪪️codec-binding/🦀️.rs");\n}\n'),
    dict(emoji='🎯️', slug='app-commands', file='📜️declaration/📨️emission/🦀️.rs', module='app_emission', host='app_commands_tests_host', test='🧪️tests/🔬️app-app-commands/🦀️.rs', kind='include', cfg='test', remove='#[cfg(test)]\ninclude!("../../🧪️tests/🔬️app-app-commands/🦀️.rs");\n'),
    dict(emoji='🎞️', slug='owned-media-error', file='📜️declaration/🧩️app/🦀️.rs', module='app_contract', host='owned_media_error_tests_host', test='🧪️tests/🔬️app-owned-media-error/🦀️.rs', kind='include', cfg='test', remove='#[cfg(test)]\ninclude!("../../🧪️tests/🔬️app-owned-media-error/🦀️.rs");\n'),
    dict(emoji='📤️', slug='export-credit', file='📜️declaration/🎞️media/🦀️.rs', module='media_contract', host='media_export_credit_tests_host', test='🧪️tests/🔬️app-artifact-media-export-credit/🦀️.rs', kind='include', cfg='test', remove='#[cfg(test)]\ninclude!("../../🧪️tests/🔬️app-artifact-media-export-credit/🦀️.rs");\n'),
    dict(emoji='📬️', slug='media-publication', file='📜️declaration/🎞️media/🦀️.rs', module='media_contract', host='media_publication_tests_host', test='📜️declaration/🎞️media/🧪️tests/🎟️publication/🦀️.rs', kind='path', cfg='any(test, feature = "media-turn-testing")', reexport=['original_completion_result_receiving_and_physical_close', 'original_handle_receiving_and_physical_close', 'original_download_receiving_chunk_and_physical_close'],
         remove='#[cfg(any(test,feature="media-turn-testing"))]\n#[path="🧪️tests/🎟️publication/🦀️.rs"]\nmod original_publication_tests;\n#[cfg(any(test,feature="media-turn-testing"))]\npub(crate) use original_publication_tests::original_completion_result_receiving_and_physical_close;\n\n#[cfg(any(test,feature="media-turn-testing"))]\npub(crate) use original_publication_tests::original_handle_receiving_and_physical_close;\n\n#[cfg(any(test,feature="media-turn-testing"))]\npub(crate) use original_publication_tests::original_download_receiving_chunk_and_physical_close;\n'),
    dict(emoji='🎟️', slug='child-receipt', lift='📜️declaration/📨️emission/🛬️lifted/📦️preparation/🦀️.rs', module='child_emit_preparation', host='receipt_tests_host', test='🧩️composition/📨️emission/📦️preparation/🎟️receipt/🧪️tests/🦀️.rs', kind='path', cfg='test',
         remove='#[cfg(test)]\n#[path="../../../../🧩️composition/📨️emission/📦️preparation/🎟️receipt/🧪️tests/🦀️.rs"]\nmod original_receipt_tests;\n'),
    dict(emoji='📡️', slug='contributed-mutation-wire', lift='🏗️builder/🔌️program/🛬️lifted/🏃️runtime/🦀️.rs', uses_from='⚛️reactor/🏃️runtime/🦀️.rs', module='plugin_runtime', host='contributed_mutation_wire_tests_host', test='🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs', kind='include', cfg='test',
         remove='#[cfg(test)]\ninclude!("../../../../🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs");\n'),
]


# D4: test-only helpers that tests outside their crate use become `#[doc(hidden)] pub` (never a test feature): (file, name)
UNGATE = [
    ('📜️declaration/🎞️media/🦀️.rs', 'handle_encoding'),
    ('📜️declaration/👷️job/🦀️.rs', 'test_take_download'),
    ('📜️declaration/👷️job/🦀️.rs', 'test_with_name'),
    ('📜️declaration/👷️job/🦀️.rs', 'with_busy_test_lock'),
    ('📜️declaration/📇️registration/🦀️.rs', 'encode_contributed_wire'),
]

# private definitions a file outside the crate names (defscan, reviewed by hand): widened to `pub`
WIDEN_EXTRA = {
    '📜️declaration/🌳️tree/🦀️.rs': ['format_descriptor_of', 'capability_rows_for', 'check_surface_document_schema', 'check_surface_id', 'subset_binding', 'preflight_io_entries', 'register_boot_scope_schema_exports'],
    '🏗️builder/🧩️app/🦀️.rs': ['artifact_kinds', 'panel_tabs', 'window_kinds', 'modes', 'dialogs', 'hosted_artifact_kinds', 'apps'],
    '📜️declaration/🎞️media/🦀️.rs': ['media_export_handle', 'media_export_metadata', 'is_sealed'],
    '📜️declaration/📨️emission/🛬️lifted/📦️preparation/🦀️.rs': ['TypedChildEmitPreparation', 'owned_source', 'operation_count'],
    '📜️declaration/🧩️app/🛬️lifted/♻️retirement/🦀️.rs': ['return_alias'],
}


# include! lines of document-media code inside decl-app (WP3 text for MediaArtifact): the code belongs to document-media with MediaArtifact, so decl-app drops the lines and the MediaArtifact lift includes the two files at the same module level (Appendix K edge, done as the cut moves)
DROP_INCLUDES = {
    '📜️declaration/🧩️app/🦀️.rs': ['include!("../../🎞️media/🛫️intrinsic/🦀️.rs");\n\ninclude!("../../🎞️media/💥️fault/🦀️.rs");\n\n'],
}
