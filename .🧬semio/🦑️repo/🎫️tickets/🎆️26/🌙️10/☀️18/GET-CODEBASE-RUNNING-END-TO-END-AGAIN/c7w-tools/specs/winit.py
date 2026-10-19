from specs.lib import *
F = RW + '/🪟️winit-app/🦀️.rs'
c = Cut(F, 'winit.rs')
c.child('🎞', 'frame-generation-and-os-host', 'frame_generation_and_os_host', [(29, 380), (997, 1007)],
        'Frame generation and os host: the render frame operation id, the frame generation and scheduler redraw helpers, the host event enqueue, the `OsHost` window delegate and impls, the cursor and ime mapping and `run_native`.')
c.child('🚀', 'native-winit-app', 'native_winit_app', [(384, 992)],
        'Native winit app: the native `native` module with the winit application handler, the event normalisation and the native event loop.')
SPEC = c.spec()
