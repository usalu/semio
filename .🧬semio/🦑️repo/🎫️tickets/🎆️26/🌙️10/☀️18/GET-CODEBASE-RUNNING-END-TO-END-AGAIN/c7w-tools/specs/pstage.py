from specs.lib import *
F = W + '/🎟️prepared/📤️staging/🦀️.rs'
c = Cut(F, 'pstage.rs')
c.child('🧾', 'raster-ledger-and-atlas-permits', 'raster_ledger_and_atlas_permits', [(16, 339)],
        'Raster ledger and atlas permits: the raster key and item bytes, the raster ledger, credit and generation, the raster pages, the atlas limits, shifts and statics, the atlas permit, pages and abandonment.')
c.child('🗒', 'atlas-pages-and-raster-pages', 'atlas_pages_and_raster_pages', [(341, 631)],
        'Atlas pages and raster pages: the `PreparedAtlasPages` staging with its drop and the `PreparedRasterPages` staging.')
c.child('🎞', 'raster-reservation-and-producer', 'raster_reservation_and_producer', [(633, 980)],
        'Raster reservation and producer: the raster rejection, the reservation and claim and the paged raster producer with its step.')
c.child('🛫', 'render-upload-and-eviction', 'render_upload_and_eviction', [(982, 1151)],
        'Render upload and eviction: the render directive, the prepared render upload and the prepared render eviction.')
SPEC = c.spec()
