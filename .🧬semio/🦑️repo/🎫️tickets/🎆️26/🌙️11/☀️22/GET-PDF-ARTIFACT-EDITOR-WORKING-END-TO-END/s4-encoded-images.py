"""Slice 4, step 3b: large sampled images stay encoded (identity over their encoded body and declared chain, decoded when a consumer resolves pixels). Applies once, idempotently refuses a second application."""
import sys
from pathlib import Path

base = Path(__file__).resolve().parents[7] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"


def patch(relative, pairs):
    path = base / relative
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times:\n{old[:200]}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")


patch("🚪️io/📦️foreign-artifacts/🦀️.rs", [
    ("""pub fn qualified_image_dictionary(dict: &[PdfDictEntry]) -> bool {
    use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_name};
    dict.len() <= 9 && dict.iter().enumerate().all(|(index, entry)| {
        !dict[..index].iter().any(|previous| previous.key == entry.key) && match entry.key.as_str() {
            "Type" => entry.value.as_name() == Some("XObject"),
            "Subtype" => entry.value.as_name() == Some("Image"),
            "BitsPerComponent" => entry.value == PdfObject::Int(8),
            "ImageMask" => entry.value.as_bool() == Some(false),
            "Interpolate" => entry.value.as_bool().is_some(),
            "Width" | "Height" | "ColorSpace" => true,
            "SMask" => entry.value.as_ref().is_some(),
            _ => false,
        }
    }) && dict_name(dict, "Subtype") == Some("Image") && dict_get(dict, "BitsPerComponent").is_some()
}""", """pub fn qualified_image_dictionary(dict: &[PdfDictEntry]) -> bool { qualified_image_entries(dict, false) }

/// 🧾️ The qualified image keys, plus its declared `Filter` and `DecodeParms` when the body is still encoded.
fn qualified_image_entries(dict: &[PdfDictEntry], encoded: bool) -> bool {
    use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_name};
    dict.len() <= if encoded { 11 } else { 9 } && dict.iter().enumerate().all(|(index, entry)| {
        !dict[..index].iter().any(|previous| previous.key == entry.key) && match entry.key.as_str() {
            "Type" => entry.value.as_name() == Some("XObject"),
            "Subtype" => entry.value.as_name() == Some("Image"),
            "BitsPerComponent" => entry.value == PdfObject::Int(8),
            "ImageMask" => entry.value.as_bool() == Some(false),
            "Interpolate" => entry.value.as_bool().is_some(),
            "Width" | "Height" | "ColorSpace" => true,
            "SMask" => entry.value.as_ref().is_some(),
            "Filter" | "DecodeParms" => encoded,
            _ => false,
        }
    }) && dict_name(dict, "Subtype") == Some("Image") && dict_get(dict, "BitsPerComponent").is_some()
}

/// 🖼️ Whether an import keeps this image XObject as its encoded file body: a qualified 8-bit DeviceGray or DeviceRGB image whose declared chain is exactly one Flate filter and whose samples would exceed the inline sample ceiling, so it is typed as a sampled artifact identified by its encoded bytes and chain and decoded only when a consumer resolves its pixels.
pub fn encoded_sampled_image(dict: &[PdfDictEntry]) -> bool {
    use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_name};
    let flate = |value: &PdfObject| matches!(value.as_name(), Some("FlateDecode" | "Fl"));
    let single = match dict_get(dict, "Filter") { Some(PdfObject::Array(items)) => matches!(items.as_slice(), [item] if flate(item)), Some(value) => flate(value), None => false };
    let parameters = match dict_get(dict, "DecodeParms") {
        None | Some(PdfObject::Null | PdfObject::Dict(_)) => true,
        Some(PdfObject::Array(items)) => matches!(items.as_slice(), [PdfObject::Null | PdfObject::Dict(_)]),
        _ => false,
    };
    let components = match dict_name(dict, "ColorSpace") { Some("DeviceGray") => 1usize, Some("DeviceRGB") => 3, _ => return false };
    let dimension = |key| match dict_get(dict, key) { Some(PdfObject::Int(raw)) => usize::try_from(*raw).ok().filter(|size| *size != 0), _ => None };
    let (Some(width), Some(height)) = (dimension("Width"), dimension("Height")) else { return false; };
    single && parameters && qualified_image_entries(dict, true)
        && width.saturating_mul(height).saturating_mul(components).saturating_mul(std::mem::size_of::<u32>()) > INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES
}

/// 🧬️ The canonical spelling of an encoded stream's declared chain (its `Filter` then its `DecodeParms`), hashed into its identity ahead of its body.
fn encoded_chain(dict: &[PdfDictEntry]) -> String {
    let spell = |key: &str| semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&dict.iter().find(|entry| entry.key == key).map(|entry| &entry.value).unwrap_or(&PdfObject::Null).to_value()));
    spell("Filter") + &spell("DecodeParms")
}

/// 🗜️ Whether a retained stream still carries the encoded body of its declared logical chain.
fn encoded_stream(dict: &[PdfDictEntry], filters: &[PdfStreamFilter]) -> bool {
    filters.is_empty() && crate::standards::v1_7::subsets::base::modules::filters::declares_logical_codec(dict)
}

/// 🔓️ Decodes a resolved encoded stream into the form its consumers read (decoded body, typed chain, dictionary without its filter declaration); every other value is returned as it is.
fn decoded_resource(value: PdfObject) -> PResult<PdfObject> {
    let PdfObject::Stream { dict, data, filters } = value else { return Ok(value); };
    if !encoded_stream(&dict, &filters) { return Ok(PdfObject::Stream { dict, data, filters }); }
    let (body, chain) = crate::standards::v1_7::subsets::base::modules::filters::decode_stream_admitted(&dict, &data.to_vec_owner())?;
    let dict = dict.into_iter().filter(|entry| crate::standards::v1_7::subsets::base::modules::lexer::retain_stream_entry(entry, true)).collect();
    Ok(PdfObject::Stream { dict, data: stream_bytes(&body), filters: chain })
}"""),
    ("""    let PdfObject::Stream {dict,data,filters}=object else{return Err(PdfEngineError::Unsupported("foreign PDF artifact requires a native stream".into()));};
    let mut hash=semio_framework_hash::Sha256::new();hash.update(kind.as_bytes());{""",
     """    let PdfObject::Stream {dict,data,filters}=object else{return Err(PdfEngineError::Unsupported("foreign PDF artifact requires a native stream".into()));};
    let mut hash=semio_framework_hash::Sha256::new();hash.update(kind.as_bytes());if encoded_stream(dict,filters){hash.update(encoded_chain(dict).as_bytes());}{"""),
    ("""        if let Some(position)=self.identities.borrow().get(kind).filter(|_|native).and_then(|index|index.get(&reference.artifact_id)).copied(){return Ok(self.graph[position].value.clone());}""",
     """        if let Some(position)=self.identities.borrow().get(kind).filter(|_|native).and_then(|index|index.get(&reference.artifact_id)).copied(){return decoded_resource(self.graph[position].value.clone());}"""),
    ("""    /// ▶️ Hashes at most `work` pages of native stream bytes and reports page work consumed.""",
     """    /// 🗜️ Binds a large sampled image whose retained body is still its encoded file bytes: the identity hashes the declared chain ahead of the body and no typed filter after it.
    pub fn encoded(dict: &[PdfDictEntry]) -> Option<Self> {
        if !encoded_sampled_image(dict) { return None; }
        let mut filter_wire = [0u8; 160];
        filter_wire[..2].copy_from_slice(b"[]");
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(SAMPLED_IMAGE_KIND.as_bytes());
        hash.update(encoded_chain(dict).as_bytes());
        Some(Self { hash, cursor: 0, filter_wire, filter_len: 2 })
    }

    /// ▶️ Hashes at most `work` pages of native stream bytes and reports page work consumed."""),
])

patch("🚪️io/📥️import/📖️interpreted-streams/🦀️.rs", [
    ("""use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::{declares_logical_codec,""",
     """use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::encoded_sampled_image;
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::{declares_logical_codec,"""),
    ("""        let decodes = role.decodes() && matches!(value, PdfObject::Stream { dict, filters, .. } if filters.is_empty() && declares_logical_codec(dict));""",
     """        let decodes = role.decodes() && matches!(value, PdfObject::Stream { dict, filters, .. }
            if filters.is_empty() && declares_logical_codec(dict) && !(role == Role::XObject && encoded_sampled_image(dict)));"""),
    ("""//! 📖️ Decodes in place, in bounded units, exactly the retained streams the semantic lift interprets — page, form, glyph, pattern and appearance programs, font programs and character maps, colour, function and shading data, images, scripts, attachments and metadata — and leaves every other stream (private application data, structure, unreferenced objects) as its original encoded bytes with its declared filter chain, so an import costs what its interpreted content costs and Save carries the rest through byte for byte.""",
     """//! 📖️ Decodes in place, in bounded units, exactly the retained streams the semantic lift interprets — page, form, glyph, pattern and appearance programs, font programs and character maps, colour, function and shading data, small images, scripts, attachments and metadata — and leaves every other stream (private application data, structure, unreferenced objects, large sampled images typed as artifacts of their encoded body) as its original encoded bytes with its declared filter chain, so an import costs what its interpreted content costs and Save carries the rest through byte for byte."""),
])

patch("🔨️modules/🖼️images/🎞️controlled/🦀️.rs", [
    ("""    INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES, PdfDctResourceIdentity, PdfSampledResourceIdentity, qualified_image_dictionary,""",
     """    INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES, PdfDctResourceIdentity, PdfSampledResourceIdentity, encoded_sampled_image, qualified_image_dictionary,"""),
    ("""        let PdfObject::Stream { dict, data, filters } = value else { return Ok(None); };
        if !matches!(filters.as_slice(),""",
     """        let PdfObject::Stream { dict, data, filters } = value else { return Ok(None); };
        if filters.is_empty() && encoded_sampled_image(dict) {
            let dimension = |key| match dict_get(dict, key) { Some(PdfObject::Int(raw)) => u32::try_from(*raw).ok().filter(|size| *size != 0), _ => None };
            let (Some(width), Some(height), Some(identity)) = (dimension("Width"), dimension("Height"), PdfSampledResourceIdentity::encoded(dict)) else { return Ok(None); };
            return Ok(Some(Self {
                width, height, gray: dict_name(dict, "ColorSpace") == Some("DeviceGray"),
                interpolate: dict_get(dict, "Interpolate").and_then(PdfObject::as_bool).unwrap_or(false),
                soft_mask: dict_get(dict, "SMask").and_then(PdfObject::as_ref),
                backing: ImageBacking::SampledArtifact { identity, bytes: data.len() },
            }));
        }
        if !matches!(filters.as_slice(),"""),
])

patch("🔨️modules/🖼️images/🦀️.rs", [
    ("""    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{
        INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES, PdfSampledResourceIdentity, qualified_image_dictionary,
    };
    if !matches!(filters, [] | [PdfStreamFilter::Flate { .. }]) || !qualified_image_dictionary(dict) {""",
     """    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{
        INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES, PdfSampledResourceIdentity, encoded_sampled_image, qualified_image_dictionary,
    };
    if filters.is_empty() && encoded_sampled_image(dict) {
        let Some(mut identity) = PdfSampledResourceIdentity::encoded(dict) else { return Ok(None); };
        identity.append(data);
        return Ok(Some(identity.reference()));
    }
    if !matches!(filters, [] | [PdfStreamFilter::Flate { .. }]) || !qualified_image_dictionary(dict) {"""),
])
print("applied slice 4 encoded sampled images")
