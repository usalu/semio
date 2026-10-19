from specs.lib import *
F = R + '/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs'
c = Cut(F, 'shact.rs').impl("impl<'ctx> ActionsCtx<'ctx> {", [326, 588])
c.child('🔄', 'backbone-and-check-in', 'backbone_sync_and_check_in', [(7, 325)],
        'Backbone sync and check in: the shell turn alias and constructor, the native backbone sync, the invocation history, the auto check in, the checkpoints and the space index artifact.')
c.child('📎', 'sync-attach-and-open', 'sync_attach_and_document_open', [(326, 587)],
        'Sync attach and document open: the sync backbone attach, the document open preparation and finish and the sync card actions.')
c.child('🎯', 'dispatch-action', 'dispatch_action_child', [(588, 1354)],
        'Dispatch action: the single funnel that every shell action crosses, `ActionsCtx::dispatch_action`.')
SPEC = c.spec()
