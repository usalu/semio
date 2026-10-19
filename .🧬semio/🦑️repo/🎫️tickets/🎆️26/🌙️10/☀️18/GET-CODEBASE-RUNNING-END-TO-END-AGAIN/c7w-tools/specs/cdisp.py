from specs.lib import *
F = R + '/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs'
c = Cut(F, 'cdisp.rs').impl("impl<'ctx> ActionsCtx<'ctx> {", [77, 763])
c.child('📨', 'dispatch-app-switch', 'dispatch_and_app_switch', [(8, 76), (763, 1172)],
        'Dispatch and app switch: the command and session command dispatch, the mutation and op application, the managed app and session role switch, the extension install, enable and uninstall and the shell uri.')
c.child('📇', 'directory-and-hub', 'directory_and_hub', [(77, 762)],
        'Directory and hub: the replay shell command, the directory events pump, the hub workspace actions and artifact creation, the open artifact relay and the document opening.')
SPEC = c.spec()
