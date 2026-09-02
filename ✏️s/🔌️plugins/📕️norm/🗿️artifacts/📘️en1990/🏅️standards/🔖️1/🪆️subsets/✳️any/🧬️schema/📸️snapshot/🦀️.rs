//! 🧬️ En1990 snapshot schema — artifact-lane fields only.

use crate::artifacts::en1990::En1990QkChild;
use crate::document::AnnexChoice;
use schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 📸️ Persisted En1990 document snapshot. Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2
/// (`norm→C:table` on `en1990.q_k`): the inline `Vec<En1990QkEntry>` variable-action table is
/// replaced by a fixed composed `s.stdio.semio.table` CHILD slot — see `🗿️artifacts/📘️en1990/🦀️.rs`'s
/// `🔖️Composition` region for the converters/working-scene cache. `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Snapshot {
    #[state(artifact)]
    pub g_k: f64,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub q_k: En1990QkChild,
    #[state(artifact)]
    pub resistance_kn: f64,
    #[state(artifact)]
    pub consequence_class: u8,
    #[state(artifact)]
    pub annex: AnnexChoice,
    /// 🌍️ Seismic accidental action A_Ed [kN] combined per Eq. 6.12b; 0.0 disables the seismic situation.
    #[state(artifact)]
    pub seismic_a_ed_kn: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct En1990QkEntry {
    pub category: String,
    pub value: f64,
}

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `➗️mathematical`'s/`📐️cad`'s own `enc_child`/
/// `dec_child`) — a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened
/// via `to_uri()`), never the child's own content.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
fn enc_child(c: &En1990QkChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child(s: &str) -> Result<En1990QkChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
fn enc_annex(a: AnnexChoice) -> &'static str {
    match a {
        AnnexChoice::En => "en",
        AnnexChoice::De => "de",
    }
}
fn dec_annex(s: &str) -> Result<AnnexChoice, String> {
    match s {
        "en" => Ok(AnnexChoice::En),
        "de" => Ok(AnnexChoice::De),
        other => Err(format!("bad annex {other:?}")),
    }
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_en1990_snapshot_body(s: &En1990Snapshot) -> String {
    format!("gK={}\nqK={}\nresistanceKn={}\nconsequenceClass={}\nannex={}\nseismicAEdKn={}", s.g_k, enc_child(&s.q_k), s.resistance_kn, s.consequence_class, enc_annex(s.annex), s.seismic_a_ed_kn,)
}
fn parse_en1990_snapshot_body(body: &str) -> Result<En1990Snapshot, String> {
    let mut g_k = None;
    let mut q_k = None;
    let mut resistance_kn = None;
    let mut consequence_class = None;
    let mut annex = None;
    let mut seismic_a_ed_kn = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("gK=") {
            g_k = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("qK=") {
            q_k = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("resistanceKn=") {
            resistance_kn = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("consequenceClass=") {
            consequence_class = Some(rest.parse::<u8>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("annex=") {
            annex = Some(dec_annex(rest)?);
        } else if let Some(rest) = line.strip_prefix("seismicAEdKn=") {
            seismic_a_ed_kn = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else {
            return Err(format!("en1990 snapshot: unknown line {line:?}"));
        }
    }
    Ok(En1990Snapshot {
        g_k: g_k.ok_or_else(|| "en1990 snapshot: missing gK line".to_string())?,
        q_k: q_k.ok_or_else(|| "en1990 snapshot: missing qK line".to_string())?,
        resistance_kn: resistance_kn.ok_or_else(|| "en1990 snapshot: missing resistanceKn line".to_string())?,
        consequence_class: consequence_class.ok_or_else(|| "en1990 snapshot: missing consequenceClass line".to_string())?,
        annex: annex.ok_or_else(|| "en1990 snapshot: missing annex line".to_string())?,
        seismic_a_ed_kn: seismic_a_ed_kn.ok_or_else(|| "en1990 snapshot: missing seismicAEdKn line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child(out: &mut Vec<u8>, c: &En1990QkChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child(reader: &mut store::ByteReader<'_>) -> Result<En1990QkChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn encode_en1990_snapshot_binary(s: &En1990Snapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    out.extend_from_slice(&s.g_k.to_le_bytes());
    write_child(&mut out, &s.q_k);
    out.extend_from_slice(&s.resistance_kn.to_le_bytes());
    out.push(s.consequence_class);
    write_str_lp(&mut out, enc_annex(s.annex));
    out.extend_from_slice(&s.seismic_a_ed_kn.to_le_bytes());
    out
}
fn decode_en1990_snapshot_binary(bytes: &[u8]) -> Result<En1990Snapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let g_k = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "short gK".to_string())?);
    let q_k = read_child(&mut reader)?;
    let resistance_kn = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "short resistanceKn".to_string())?);
    let consequence_class = reader.read_u8().map_err(|e| e.to_string())?;
    let annex = dec_annex(&read_str_lp(&mut reader)?)?;
    let seismic_a_ed_kn = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "short seismicAEdKn".to_string())?);
    Ok(En1990Snapshot { g_k, q_k, resistance_kn, consequence_class, annex, seismic_a_ed_kn })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack, real hex/bracket text + fixed-width/LEB128 binary
/// primitives — same upgrade `➗️mathematical`/`📐️cad`/`✒️writer` made once their snapshot gained a
/// real `ArtifactChild<S>` slot (the old `impl_norm_artifact_record!`/`dsl::DslRecord`-derive-driven
/// path cannot express a composed child slot, which has no `dsl::DslField` impl reachable from this
/// crate). The other fourteen norm families are unaffected — they have no composed child slot and
/// keep `impl_norm_artifact_record!` unchanged.
impl store::ArtifactDsl for En1990Snapshot {
    const EXTENSION: &'static str = "en1990";
    fn envelope_id() -> &'static str {
        "norm.en1990"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_en1990_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_en1990_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for En1990Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_en1990_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_en1990_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1990Snapshot {
    fn default() -> Self {
        let q_k = crate::artifacts::en1990::en1990_qk_child_from_entries(&[En1990QkEntry { category: "office".into(), value: 50.0 }, En1990QkEntry { category: "wind".into(), value: 30.0 }]);
        Self { g_k: 100.0, q_k, resistance_kn: 300.0, consequence_class: 2, annex: AnnexChoice::De, seismic_a_ed_kn: 40.0 }
    }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`En1990Snapshot`] — the surface
/// `../../../../../🧪️tests/mutate-en1990-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1990_snapshot_json(snapshot: &En1990Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1990_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1990Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1990_snapshot_json(text: &str) -> Result<En1990Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1990Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1990_dsl(text: &str) -> Result<En1990Snapshot, String> {
    <En1990Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1990Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1990_dsl(snapshot: &En1990Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`En1990Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1990_pack(bytes: &[u8]) -> Result<En1990Snapshot, String> {
    <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`En1990Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1990_pack(snapshot: &En1990Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
