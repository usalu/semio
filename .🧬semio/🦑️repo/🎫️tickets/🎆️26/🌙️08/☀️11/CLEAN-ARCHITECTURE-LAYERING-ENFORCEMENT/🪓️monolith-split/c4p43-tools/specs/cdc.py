from specs.lib import *

CH = [
    ch('📦', 'archive-decode', 'archive_decode', [(7, 273)],
       'Archive decode of the paged command decoders: the field limits, the document archive decode phases and members and the paged document archive decode.'),
    ch('🛣', 'route-and-media-decode', 'route_and_media_decode', [(275, 454)],
       'Route and media decode of the paged command decoders: the route field plan and the paged route field and media export handle decodes.'),
    ch('🧾', 'command-owner', 'command_owner', [(456, 680)],
       'Command owner of the paged command decoders: the decode state, the cursor and owner records, the owner implementation and the cursor shape.'),
    ch('🧭', 'command-cursor', 'command_cursor', [(682, 1108)],
       'Command cursor of the paged command decoders: the resumable paged AppCommand decode cursor implementation.'),
]

SPEC = dict(
    orig='cdc.rs',
    file=CDC + '/🦀️.rs',
    children=CH,
    nest=True,
    external=mounted(M + '/📡️spr/🧵️channel/🦀️.rs'),
)
