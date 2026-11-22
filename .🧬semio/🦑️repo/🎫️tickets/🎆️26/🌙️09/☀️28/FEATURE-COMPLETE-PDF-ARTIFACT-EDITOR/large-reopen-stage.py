"""📦 Stage the paid memberless archive ingress without changing the shared checkout."""
from difflib import unified_diff
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if (parent / "Cargo.toml").exists())
CHANNEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel"
OUT = TICKET / "🗑️generated" / "large-reopen"
OUT.mkdir(parents=True, exist_ok=True)


def replace(original, old, new):
    assert original.count(old) == 1, old
    return original.replace(old, new)


module = CHANNEL / "🦀️.rs"
manifest = CHANNEL / "📦️packages/🦀️rust/Cargo.toml"
lock = ROOT / "Cargo.lock"
schema = CHANNEL / "📦️archive/🧬️schema/🦀️.rs"
ingress = CHANNEL / "📦️archive/🛬️ingress/🦀️.rs"
source = (TICKET / "large-reopen-ingress.rs").read_text()
before = {p: p.read_text() for p in (module, manifest, lock)}
original = 'pub use archive_decoding::{decode_document_archive_bytes_controlled,decode_document_archive_head_controlled};'
entry = replace(before[module], original, original + '''
#[path="📦️archive/🛬️ingress/🦀️.rs"]
pub mod archive_ingress;
''')
original = "[dependencies]\n"
manifest_text = replace(before[manifest], original, original + '''semio-framework-hash = { workspace = true }
semio-framework-value-persistent = { path = "../../../../../../../🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust" }
''')
package = before[lock].split('name = "semio-os-channel"\n')
assert len(package) == 2
body, rest = package[1].split('\n[[package]]', 1)
body = replace(body, ' "semio-framework-io-control",', ' "semio-framework-hash",\n "semio-framework-io-control",')
body = replace(body, ' "semio-framework-value-native-owned",', ' "semio-framework-value-native-owned",\n "semio-framework-value-persistent",')
lock_text = package[0] + 'name = "semio-os-channel"\n' + body + '\n[[package]]' + rest
changes = {module: entry, manifest: manifest_text, lock: lock_text, ingress: source}
patch = []
for path, after in changes.items():
    assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
    current = before.get(path, "")
    assert (path.read_text() if path.is_file() else "") == current, path
    name = path.relative_to(ROOT)
    patch.extend(unified_diff(current.splitlines(True), after.splitlines(True), f"a/{name}" if current else "/dev/null", f"b/{name}"))
(OUT / "reopen.patch").write_text("".join(patch))
(OUT / "ingress.rs").write_text(source)
(OUT / "schema.rs").write_text(schema.read_text())
(OUT / "probe.rs").write_text((TICKET / "large-reopen-test.rs").read_text())


snapshot = CHANNEL.parent.parent / "🏪️store/📦️codec/📸️native-snapshot/🫴️receiving/🦀️.rs"
old = snapshot.read_text()
typed = replace(old, "use protocol::PackLimits;", "use protocol::{OriginalWireSourceIdentity, PackLimits, RetainedWireByteSource};")
typed = replace(typed, " source:(usize,usize),control:", " source:OriginalWireSourceIdentity,control:")
typed = replace(typed, "pub fn admit(source:&[u8],envelope:", "pub fn admit<S:RetainedWireByteSource+?Sized>(source:&S,envelope:")
typed = replace(typed, "  if source.len()<prefix||source.len()as u64>limits.max_file_len", "  let source=source.original_source_identity();\n  if source.extent<prefix||source.extent as u64>limits.max_file_len")
typed = replace(typed, "source:(source.as_ptr()as usize,source.len()),control:", "source,control:")
typed = replace(typed, "fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len())", "fn validate<S:RetainedWireByteSource+?Sized>(&self,source:&S)->Result<(),ValueError>{if self.source!=source.original_source_identity()")
typed = replace(typed, "pub fn demands(&self,source:&[u8],body:", "pub fn demands<S:RetainedWireByteSource+?Sized>(&self,source:&S,body:")
typed = replace(typed, "pub fn step(&mut self,source:&[u8],native:", "pub fn step<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,native:")
typed = replace(typed, "fn step_action(&mut self,source:&[u8],native:", "fn step_action<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,native:")
typed = replace(typed, "source.get(self.position).copied()", "source.byte_at(self.position)")
typed = replace(typed, "(self.prefix,source.len()),T::RECORD", "(self.prefix,self.source.extent),T::RECORD")
assert snapshot.read_text() == old
relative = snapshot.relative_to(ROOT)
(OUT / "snapshot.patch").write_text("".join(unified_diff(old.splitlines(True), typed.splitlines(True), f"a/{relative}", f"b/{relative}")))
(OUT / "snapshot.rs").write_text(typed.split("\n#[cfg(test)]", 1)[0])


def item_change(path, changes, patch):
    original = path.read_text()
    revised = original
    for old, new in changes:
        revised = replace(revised, old, new)
    assert path.read_text() == original
    assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
    relative = path.relative_to(ROOT)
    patch.extend(unified_diff(original.splitlines(True), revised.splitlines(True), f"a/{relative}", f"b/{relative}"))
    return revised


store = CHANNEL.parent.parent / "🏪️store"
hydration = store / "🧾️document/📜️history/💧️hydration"
vcs = CHANNEL.parent.parent / "🌿️vcs"
hydration_patch = []
genesis_module = item_change(vcs / "🚪️io/💾️binary/🌱️genesis/🦀️.rs", [
    ("impl ArtifactGenesisPack {", """impl From<Vec<u8>> for ArtifactGenesisPack {
    fn from(bytes: Vec<u8>) -> Self { Self::Contiguous(bytes) }
}

impl From<ArtifactGenesisPackPages> for ArtifactGenesisPack {
    fn from(bytes: ArtifactGenesisPackPages) -> Self { Self::Paged(bytes) }
}

impl ArtifactGenesisPack {"""),
    ("""  let demand=match Self::verified_pack_birth_demand(){Ok(demand)=>demand,Err(error)=>return Err((error,snapshot,pack))};
  if let Err(error)=demand.admit(grant){return Err((error, snapshot, pack));}
  Ok((Self::from_verified_pack(snapshot, pack, digest),semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))""",
     """  Self::admit_verified_physical_pack(snapshot, ArtifactGenesisPack::Contiguous(pack), digest, grant)
    .map_err(|(error, snapshot, pack)| match pack {
      ArtifactGenesisPack::Contiguous(pack) => (error, snapshot, pack),
      ArtifactGenesisPack::Paged(_) => unreachable!("verified contiguous Pack admission changed representation"),
    })"""),
    ("""     let demand = match Self::verified_pack_birth_demand() {
         Ok(demand) => demand,
         Err(error) => return Err((error, snapshot, pack)),
     };""",
     """    Self::admit_verified_physical_pack(snapshot, ArtifactGenesisPack::Paged(pack), digest, grant)
        .map_err(|(error, snapshot, pack)| match pack {
            ArtifactGenesisPack::Paged(pack) => (error, snapshot, pack),
            ArtifactGenesisPack::Contiguous(_) => unreachable!("verified paged Pack admission changed representation"),
        })
 }

 /// 📥 Admits an already verified physical Pack without flattening its backing.
 pub fn admit_verified_physical_pack(snapshot: P, pack: ArtifactGenesisPack, digest: [u8; 32], grant: semio_framework_value::RetainedCloneGrant)
     -> Result<(Self, semio_framework_value::RetainedCloneProgress), (ValueError, P, ArtifactGenesisPack)>
 {
     let demand = match Self::verified_pack_birth_demand() {
         Ok(demand) => demand,
         Err(error) => return Err((error, snapshot, pack)),
     };"""),
    ("pack: Arc::new(ArtifactGenesisPack::Paged(pack))", "pack: Arc::new(pack)"),
], hydration_patch)
item_change(hydration / "🧬️hydration-types-constructors-and-drive/🦀️.rs", [
    ("use super::*;\n\n/// 🪪️", "use super::*;\nuse semio_os_vcs::io::binary::genesis::ArtifactGenesisPack;\n\n/// 🪪️"),
    ("pub pack: ManuallyDrop<Option<Vec<u8>>>", "pub pack: ManuallyDrop<Option<ArtifactGenesisPack>>"),
    ("        initial: P,\n        pack: Vec<u8>,", "        initial: P,\n        pack: impl Into<ArtifactGenesisPack>,"),
    ("Self::new(Some(pack), Some(initial),", "Self::new(Some(pack.into()), Some(initial),"),
    ("Self::new(Some(pack), None,", "Self::new(Some(pack.into()), None,"),
    ("        pack: Option<Vec<u8>>,", "        pack: Option<ArtifactGenesisPack>,"),
], hydration_patch)
item_change(hydration / "🚶️hydration-step/📦️pack-scan-and-fold/🦀️.rs", [
    ('let pack = self.pack.as_ref().expect("persisted Pack remains retained through scan");',
     'let Some(pack) = self.pack.as_ref().and_then(|pack| pack.contiguous()) else { return self.reject(MemberOpenDiagnostic::Decode); };'),
    ('&self.pack.as_ref().expect("retained Pack remains present")[self.pack_scanned..self.pack_scanned + scanned]',
     '&pack[self.pack_scanned..self.pack_scanned + scanned]'),
    ('let initial = match P::decode_pack(self.pack.as_ref().expect("scanned Pack remains retained")) {',
     'let Some(pack) = self.pack.as_ref().and_then(|pack| pack.contiguous()) else { return self.reject(MemberOpenDiagnostic::Decode); };\n        let initial = match P::decode_pack(pack) {'),
    ('::admit_verified_pack(initial, pack, initial_digest, child)', '::admit_verified_physical_pack(initial, pack, initial_digest, child)'),
], hydration_patch)
(OUT / "hydration.patch").write_text("".join(hydration_patch))
(OUT / "genesis.rs").write_text(genesis_module)
