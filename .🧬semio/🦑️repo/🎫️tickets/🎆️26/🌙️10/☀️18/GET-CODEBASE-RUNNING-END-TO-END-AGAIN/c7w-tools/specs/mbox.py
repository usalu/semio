from specs.lib import *
F = RW + '/🧊️renderer/📮️mailbox-impl/🦀️.rs'
H = 'impl<R: MailboxRuntime> RuntimeMailbox<R> {'
c = Cut(F, 'mbox.rs').impl(H, [243, 493, 930, 1248])
c.child('🏗', 'mailbox-construction-and-authorities', 'mailbox_construction_and_authorities', [(5, 242)],
        'Mailbox construction and authorities: the mailbox constructor, the interaction submission, the hub document status, the presentation and wake authorities, the engine surface close and the input close step.')
c.child('📥', 'renderer-asset-steps', 'renderer_asset_steps', [(243, 492)],
        'Renderer asset steps: the renderer asset take, reserve, finish and rejection steps, the shared asset bytes, the decode boundary and the asset decode session steps.')
c.child('🖼', 'asset-decode-pump-and-reference-images', 'asset_decode_pump_and_reference_images', [(493, 929)],
        'Asset decode pump and reference images: the renderer asset decode pump, the owner return and seal, the close steps, the currency checks, the rejection and the staged reference images.')
c.child('🌐', 'native-asset-pumps', 'native_asset_pumps', [(930, 1247)],
        'Native asset pumps: the native reference decode pump, the native asset handoff and transport and the native asset pump.')
c.child('⏳', 'pending-work-deadlines-and-spawns', 'pending_work_deadlines_and_spawns', [(1248, 1598)],
        'Pending work, deadlines and spawns: the waker, the apply and dispatch queues, the pending work queries, the fault takes, the control deadlines, the future reservations and the frame spawns.')
SPEC = c.spec()
