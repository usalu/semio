from specs.lib import *
DD = PL + '/🗃️document'
_orig = open(os.path.join(T, 'orig', 'dd.rs'), encoding='utf-8').read().split('\n')
H = _orig[695]
HDR = _orig[695:701]
B = [895, 1151]
CH = [
    ch('📨', 'envelope-decode-and-ingress', 'envelope_decode_and_ingress', [(56, 386)],
       'Envelope decode and ingress of the document ingress: the decode operation handle and poll, the active decode state with its worker and rejection lifecycles, the active decode and the active ingress with their drop guards.'),
    ch('🏷', 'candidate-publication-and-composition-pins', 'candidate_publication_and_composition_pins', [(388, 482)],
       'Candidate publication and composition pins: the authoritative store candidate publication helpers and the composition pins retirement with its drop guard.'),
    ch('🧾', 'store-replacement-vocabulary', 'store_replacement_vocabulary', [(484, 694)],
       'Store replacement vocabulary: the replacement state, the refusal code and refusal, the replacement legs, the publication guard and the active store replacement record.'),
    ch('🔁', 'store-replacement-admission-and-member-drive', 'store_replacement_admission_and_member_drive', [(696, 1159)],
       'Store replacement admission and member drive: construction, refusal, member admission and sealing, the exact demands, and the member open, closure and candidate view drivers of the active store replacement.'),
    ch('🧰', 'store-replacement-initializer-and-retirement', 'store_replacement_initializer_and_retirement', [(1161, 1461)],
       'Store replacement initializer and retirement: the rejected member driver, the initializer driver, the retirement demands, the retained store driver, the drop guard, the admission target and the app projection cache.'),
    ch('🗄', 'document-archive-load', 'document_archive_load', [(1463, 1859)],
       'Document archive load: the archive load state and phase, the pending archive member, the member identity helpers and the active archive load with its drop guard.'),
]
SPEC = dict(orig='dd.rs', file=DD + '/🦀️.rs', children=CH, wiring='block', pre=split_impl(H, B, 6), expect_extra=split_extra(H, B, 6, HDR),
            external=mounted(DD + '/🦀️.rs', orig='dd.rs'))
