from specs.lib import *
F = W + '/🖍️draw/🖼️raster/🦀️.rs'
H = 'impl RasterTextureTable {'
c = Cut(F, 'raster.rs').impl(H, [1339])
c.child('🔑', 'texture-keys-ledger-and-registry', 'texture_keys_ledger_and_registry', [(17, 349)],
        'Texture keys, ledger and registry: the raster texture, its witness and cleanup step, the keep set, the residency ledger, the texture key and the fixed texture registry.')
c.child('🧾', 'reservations-retirement-and-witnesses', 'reservations_retirement_and_witnesses', [(351, 772)],
        'Reservations, retirement and witnesses: the texture admission, the cancel and stage faults, the reservation and stage claim with their retirement and close cursors and the operation witness ledger.')
c.child('📤', 'upload-and-retirement-owners', 'upload_and_retirement_owners', [(774, 1095)],
        'Upload and retirement owners: the texture upload cursor, the upload pixels, the retirement owner, the upload close cursor, the retirement mode and cursor and the texture table state.')
c.child('🏗', 'texture-table-admission', 'texture_table_admission', [(1097, 1338)],
        'Texture table admission: the table constructor, the retained counts, the candidate ownership, the admission step, the engine texture reservation and the stage claims.')
c.child('🔁', 'texture-table-ensure-and-retirement', 'texture_table_ensure_and_retirement', [(1339, 1799)],
        'Texture table ensure and retirement: the ensure step, the bind group staging, the presenting, retirement and commit steps and the exact close steps.')
FIX = ['validate_engine_renderer_allocation', 'validate_engine_target_texture_allocation', 'validate_engine_target_view_allocation', 'retain_engine_allocation_fault']
_cut = c.pre()


def _pre(text):
    for n in FIX:
        assert text.count('pub(super) fn %s(' % n) == 1, n
        text = text.replace('pub(super) fn %s(' % n, 'pub fn %s(' % n)
    return _cut(text)


_orig = open(os.path.join(T, 'orig', 'raster.rs'), encoding='utf-8').read().split('\n')
_old = [l for l in _orig if any('pub(super) fn %s(' % n in l for n in FIX)]
_new = [l.replace('pub(super) fn', 'pub fn') for l in _old]
SPEC = c.spec(pre=_pre, expect_missing=_old, pre_extra=[l.strip() for l in _new])
