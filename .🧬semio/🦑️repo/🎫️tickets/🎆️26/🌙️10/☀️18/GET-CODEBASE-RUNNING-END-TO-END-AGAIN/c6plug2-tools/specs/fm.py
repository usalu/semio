from specs.lib import *
F = FILES['fm']
CH = [
    ch('🎨', 'visual-state-and-quality', 'visual_state_and_quality', [(35, 187)],
       'Visual state and quality: the window constants, the region visual quality, the node live field, the fem visual state, the region quality slots and the live visual.'),
    ch('🗄', 'fixed-packet-pages-and-lease', 'fixed_packet_pages_and_lease', [(189, 359)],
       'Fixed packet pages and lease: the visual freshness, the fixed packet pages and order and the mounted visual lease.'),
    ch('🧵', 'visual-job', 'visual_job', [(362, 920)],
       'Visual job: the visual limits, the job stage and the visual job with its stepping.'),
    ch('🖼', 'structure-and-live-layers', 'structure_and_live_layers', [(923, 1244)],
       'Structure and live layers: the vector layers of the structure, the emphasis rings, the region triangles, the deformed shape and the render entries.'),
]
SPEC = dict(orig='fm.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='fm.rs'), keep_empty=('\U0001F9EA\ufe0fTests',))
