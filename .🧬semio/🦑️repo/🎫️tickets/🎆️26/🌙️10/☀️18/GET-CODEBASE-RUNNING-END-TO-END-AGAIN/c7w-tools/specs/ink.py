from specs.lib import *
F = R + '/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs'
c = Cut(F, 'ink.rs').impl('impl InkInteractionJob {', [1803])
c.child('🧾', 'ink-document-model-and-edit-helpers', 'ink_document_model_and_edit_helpers', [(8, 433)],
        'Ink document model and edit helpers: the host id counter, the camera and document JSON, the item accessors, the scale, resize and snap helpers, the item creation, the stroke erasing and the table cell edit helpers.')
c.child('🔎', 'ink-interaction-document-and-block-scanning', 'ink_interaction_document_and_block_scanning', [(437, 645)],
        'Ink interaction document and block scanning: the interaction capacities, the interaction document and the JSON value, array and object block scanners.')
c.child('📤', 'ink-event-pages-and-edit-routing', 'ink_event_pages_and_edit_routing', [(646, 1062)],
        'Ink event pages and edit routing: the event JSON and raw pages, the event action writers, the gesture abort, the selection and hover writers and the edit key, blur and cancel routing.')
c.child('📋', 'ink-clipboard-camera-and-wheel', 'ink_clipboard_camera_and_wheel', [(1064, 1380)],
        'Ink clipboard, camera and wheel: the clipboard copy and paste, the pointer cancel, the document and value checks, the camera writers and the wheel handling.')
c.child('🔁', 'ink-interaction-job-scan', 'ink_interaction_job_scan', [(1381, 1802)],
        'Ink interaction job scan: the interaction event, stage and step types, the block cursor, the interaction job, the checked drag and the scan, eraser and hit steps.')
c.child('🏁', 'ink-interaction-job-publish-and-close', 'ink_interaction_job_publish_and_close', [(1803, 2047)],
        'Ink interaction job publish and close: the publish, down, move and close steps of the interaction job and the resize handle constant.')
SPEC = c.spec()
