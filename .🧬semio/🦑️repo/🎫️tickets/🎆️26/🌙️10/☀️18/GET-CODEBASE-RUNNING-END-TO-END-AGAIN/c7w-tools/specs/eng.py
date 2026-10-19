from specs.lib import *
F = W + '/⚙️engine/🦀️.rs'
H = 'impl Ui {'
c = Cut(F, 'eng.rs').impl(H, [518, 990, 1428, 1777, 2137], at=82)
c.child('🏁', 'ui-state-and-presented-input', 'ui_state_and_presented_input', [(40, 517)],
        'Ui state and presented input: the `Ui` state, the retained input draft, the constructor, the theme and driver settings, the presented input candidates and the surface close.')
c.child('📄', 'viewport-and-document-ingress', 'viewport_and_document_ingress', [(518, 989)],
        'Viewport and document ingress: the viewport and window flow, the presence notes, the tree application and the document begin, page, finish, publish, reconcile and close steps.')
c.child('📐', 'layout-stepping', 'layout_stepping', [(990, 1427)],
        'Layout stepping: the surface lane, the layout steps, the theme propagation, the layout queue and the shell settings.')
c.child('🎞', 'frame-step', 'frame_step', [(1428, 1776)],
        'Frame step: the `frame_step` that paints one surface frame.')
c.child('🖥', 'frame-into-step-and-frame', 'frame_into_step_and_frame', [(1777, 2136)],
        'Frame into step and frame: the `frame_into_step` that paints into a caller buffer and the `frame` entry point.')
c.child('📣', 'dispatch-and-overlay-api', 'dispatch_and_overlay_api', [(2137, 2592)],
        'Dispatch and overlay API: the layout queries, the paint census, the event, pointer and accessibility dispatch, the overlay API, the clock and the commands.')
c.child('🔬', 'introspection-queries', 'introspection_queries', [(2596, 2851)],
        'Introspection queries: the window and surface queries, the retained input draft, the presented tree sources and the scene lookups.')
SPEC = c.spec(keep_empty=('RetainedDocumentHostileFixtures',))
