
from pathlib import Path
import json, shutil

TICKET = list(Path('.🦑️repo/🎫️tickets').rglob('STDIO-ARTIFACTS-AND-IO'))[0]
ROSTER = json.loads((TICKET / '🧪owner-table.json').read_text())['stdio_roster']
TOKENS = json.loads((TICKET / '🧪tokens.json').read_text())
ART = Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts')
ZIPA = ART / ROSTER['zip']['dir']
BIN_DIR = ROSTER['binary']['dir']
DEF_DIR = ROSTER['deflate']['dir']
DESER = TOKENS['deserializers']
SER = TOKENS['serializers']

def w(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding='utf-8')
    print('W', path.relative_to(ZIPA), len(text))

# ensure engine installed
src = TICKET / 'generators/codecs/zip_engine.rs'
dst = ZIPA / '⚙️engine' / '🦀️component.rs'
shutil.copyfile(src, dst)
print('engine', dst.stat().st_size)

# check snapshot has entries
snap = (ZIPA / '🧬️schema' / '📸️snapshot' / '🦀️component.rs').read_text(encoding='utf-8')
print('snapshot has entries', 'pub entries' in snap, 'ZipEntry' in snap)

# write deflate export IO (may be missing/corrupt)
w(ZIPA / '�, 'ZipEntry' in snap)

# write deflate export IO (may be missing/corrupt)
w(ZIPA / '🚪️io' / '📤️export' / SER / '🗿️artifacts' / DEF_DIR / '🦀️component.rs',
"""//! Serialize stdio.zip to stdio.deflate (encode ZIP then zlib-compress).

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::zip::ZipSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// Encode ZIP bytes then zlib-compress via deflate artifact.
pub fn serialize(from: &ZipSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let zip_bytes = crate::artifacts::zip::engine::encode_zip(from, true)
        .map_err(|e| store::PackError::Schema(e))?;
    let bytes = crate::artifacts::deflate::engine::zlib_compress(&zip_bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode as deflate pack bytes.
pub fn serialize_bytes(from: &ZipSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
""")

w(ZIPA / '🚪️io' / '🦀️component.rs',
"""//! IO stdio.zip
pub fn register() {
    crate::artifacts::zip::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::zip::io::import::deserializers::artifacts::deflate::register();
    crate::artifacts::zip::io::export::serializers::artifacts::binary::register();
    crate::artifacts::zip::io::export::serializers::artifacts::deflate::register();
}
""")

# TS stubs
for side, folder, peer in [
    ('📥️import', DESER, BIN_DIR),
    ('📤️export', SER, BIN_DIR),
    ('📥️import', DESER, DEF_DIR),
    ('📤️export', SER, DEF_DIR),
]:
    w(ZIPA / '🚪️io' / side / folder / '🗿️artifacts' / peer / '🟦️component.ts',
      '/** IO bridge stdio.zip */
export {};
')

# verify IO rust heads
for side, folder, peer in [
    ('�
export {};
')

# verify IO rust heads
for side, folder, peer in [
    ('📥️import', DESER, BIN_DIR),
    ('📤️export', SER, BIN_DIR),
    ('📥️import', DESER, DEF_DIR),
    ('�<|control37|>export', SER, DEF_DIR),
]:
    pass
for side, folder, peer in [
    ('📥️import', DESER, BIN_DIR),
    ('📤️export', SER, BIN_DIR),
    ('📥️import', DESER, DEF_DIR),
    ('📤️export', SER, DEF_DIR),
]:
    p = ZIPA / '🚪️io' / side / folder / '🗿️artifacts' / peer / '🦀️component.rs'
    t = p.read_text(encoding='utf-8')
    print('IO', peer[:8], side[:4], 'ok' if 'pub fn' in t and 'critters' not in t else 'BAD', 'lines', len(t.splitlines()))

print('zip files', sum(1 for x in ZIPA.rglob('*') if x.is_file()))
