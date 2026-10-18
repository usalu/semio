from specs.lib import *
F = STORE + '/📦️codec/🧬️owned-schema/🦀️.rs'
CH = [
    ch('🧮', 'decode-owner-bundle-and-bounded-array-authority', 'decode_owner_bundle_and_bounded_array_authority', [(140, 464)],
       'Decode owner bundle and bounded array authority: the envelope decode owner bundle, the bounded array state and the credit bounded array authority with its drop.'),
    ch('📄', 'decode-page-state-and-authority', 'decode_page_state_and_authority', [(470, 987)],
       'Decode page, state and authority: the decode limits, the decode page and record, the decode state and the envelope decode authority with its interactive job and drop.'),
    ch('🚫', 'decode-rejections-and-erased-retirement', 'decode_rejections_and_erased_retirement', [(989, 1209)],
       'Decode rejections and erased retirement: the admitted and unadmitted decode rejections with their erased snapshot retirement and drop.'),
    ch('🎫', 'completed-record-registry-and-completion', 'completed_record_registry_and_completion', [(1211, 1536)],
       'Completed record registry and completion: the completed record ticket, fault, target and owner, the registry state and registry and the decode completion.'),
    ch('🌱', 'fresh-snapshot-target-and-vcs-authority', 'fresh_snapshot_target_and_vcs_authority', [(1538, 1923)],
       'Fresh snapshot target and VCS authority: the fresh VCS fields, the fresh snapshot target and its field target, the active state and the fresh VCS authority.'),
    ch('🔬', 'fresh-vcs-active-and-verifier', 'fresh_vcs_active_and_verifier', [(1925, 2140)],
       'Fresh VCS active and verifier: the active fresh VCS operations, the verifier and the authority drop.'),
    ch('🧷', 'fresh-record-target-and-field-decoder', 'fresh_record_target_and_field_decoder', [(2142, 2605)],
       'Fresh record target and field decoder: the fresh record target, its decoded record target, the active record state and the fresh field decoder with its registries.'),
    ch('🛑', 'whole-buffer-ingress-refusal', 'whole_buffer_ingress_refusal', [(2607, 2621)],
       'Whole buffer ingress refusal: the whole buffer artifact envelope ingress error and its rejection.'),
]
SPEC = dict(orig='cos.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='cos.rs'))
