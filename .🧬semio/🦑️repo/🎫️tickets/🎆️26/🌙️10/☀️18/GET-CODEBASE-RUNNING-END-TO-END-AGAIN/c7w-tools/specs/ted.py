from specs.lib import *
F = R + '/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs'
c = Cut(F, 'ted.rs')
c.child('📄', 'scene-document-and-host-sync', 'scene_document_and_host_sync', [(17, 254)],
        'Scene document and host sync: the editor scene range, token, diagnostic, placeholder, span, settings and document records, their field parsers and the editor host synchronisation.')
c.child('📤', 'draft-actions-and-outbox', 'draft_actions_and_outbox', [(259, 560)],
        'Draft actions and outbox: the explicit draft settings, the wheel, the action emission, the delivery bytes and receipt token, the typing commit and end and the outbox step.')
c.child('🧾', 'receipts-and-explicit-draft', 'receipts_and_explicit_draft', [(562, 874)],
        'Receipts and explicit draft: the action receipt settling, the selection and draft value writers, the retained action step, the explicit draft status, discard and history.')
c.child('🖋', 'editing-and-pointer-api', 'editing_and_pointer_api', [(876, 1140)],
        'Editing and pointer API: the draft queries, the selection replacement, the key, span, pointer and completion entry points, the caret and the rename preview.')
SPEC = c.spec(nest=True)
