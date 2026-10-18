from specs.lib import *
IR = PL + '/🧠️interpreter/🏃️runtime'
CH = [
    ch('🧮', 'owned-constants-and-codec-origin', 'owned_constants_and_codec_origin', [(42, 159)],
       'Owned constants and codec origin: the result, checkpoint, fuel and deadline limits, the compiled guest and the codec origin with its cell, use and assembly.'),
    ch('🛰', 'owned-operation-and-guest-instance', 'owned_operation_and_guest_instance', [(161, 334)],
       'Owned operation and guest instance: the owned operation and stage, the pending record, the checkpoint metadata and view and the owned guest instance.'),
    ch('📥', 'snapshot-operation-and-input-records', 'snapshot_operation_and_input_records', [(336, 462)],
       'Snapshot operation and input records: the snapshot operation, the poll, job, restore, codec and snapshot inputs, the invocation, the engine identity and the owned runtime record.'),
    ch('🔄', 'owned-runtime', 'owned_runtime', [(464, 812)],
       'Owned runtime: the operations of the owned interpreter runtime over its guest instances.'),
    ch('⏯', 'operation-begin-resume-and-cancel', 'operation_begin_resume_and_cancel', [(814, 989)],
       'Operation begin, resume and cancel: starting an owned operation, the deadlines, the resume variants, the bound resume loop, the fault text and the cancellation.'),
    ch('📣', 'host-replies-wasi-and-channel-faults', 'host_replies_wasi_and_channel_faults', [(991, 1188)],
       'Host replies, wasi and channel faults: the wasi clock and entropy answers, the owned host reply, the guest memory helpers, the channel admission and result decoding and the turn and core faults.'),
]
_names = ['🦀️.rs'] + [c['folder'] + '/🦀️.rs' for c in CH]
_SRC_LINES = ['concat!('] + ['    include_str!("%s"),' % n for n in _names] + [')']
SRC = '\n'.join(_SRC_LINES)
SPEC = dict(source_concat=True, orig='ir.rs', file=IR + '/🦀️.rs', children=CH, wiring='block',
            external=mounted(IR + '/🦀️.rs', orig='ir.rs'),
            patch={'📥️snapshot-operation-and-input-records': [('        let host = include_str!("../🦀️.rs").as_bytes();', '        let host = OWNED_RUNTIME_SOURCE.as_bytes();')]},
            patch_parent=[('/// 🛂️ The neutral test caller admission', '/// 🪪️ The authored source of the owned runtime: the root and every child module, one text that the engine identity digests.\nconst OWNED_RUNTIME_SOURCE: &str = ' + SRC + ';\n\n/// 🛂️ The neutral test caller admission')],
            expect_missing=['let host = include_str!("🦀️.rs").as_bytes();'],
            expect_extra=['let host = OWNED_RUNTIME_SOURCE.as_bytes();', '/// 🪪️ The authored source of the owned runtime: the root and every child module, one text that the engine identity digests.', 'const OWNED_RUNTIME_SOURCE: &str = concat!('] + [l.strip() for l in _SRC_LINES[1:-1]] + [');'])
