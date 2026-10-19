from specs.lib import *
F = R + '/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs'
c = Cut(F, 'ptr.rs').impl("impl<'ctx> PointerCtx<'ctx> {", [326, 527], at=57)
c.child('🔑', 'key-and-move', 'key_and_move', [(7, 325)],
        'Key and move: the key action to ui event mapping, the tree chevron and host blur checks and the pointer cancel, move and dock drag finish handlers.')
c.child('🎯', 'shell-hit', 'shell_hit', [(527, 943)],
        'Shell hit: the long `handle_shell_hit` that routes one pointer hit through the shell.')
c.child('♿', 'access-and-routing', 'accessibility_and_routing', [(326, 526), (945, 1118)],
        'Accessibility and routing: the accessibility event handler, the chrome press ownership, the pointer hit owner, the drag capture and the retained pointer routing.')
SPEC = c.spec()
