"""🔁 Stages code-preserving font changes and checks editable text encodings."""

from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/text-font-edit"
output.mkdir(exist_ok=True)
base = Path(sys.argv[1]).resolve()
page = base / "✏️editor/🖼️page"
font_overlay = ticket / "🗑️generated/simple-cff-stage/codec.rs"

def replace(source, old, new):
    assert source.count(old) == 1, old
    return source.replace(old, new)

source = font_overlay.read_text()
source = replace(source, "PdfFontProgram, PdfSimpleEncoding, PdfToUnicode", "PdfFontProgram, PdfOp, PdfSimpleEncoding, PdfTextArrayItem, PdfTextString, PdfToUnicode")
start = source.index("        let glyph_id = if !self.composite {")
end = source.index("        DecodedGlyph {", start)
glyph_id = source[start:end].strip().removeprefix("let glyph_id = ").removesuffix(";")
source = source[:start] + "        let glyph_id = self.glyph_id(code, cid);\n" + source[end:]
where = source.index("    fn text_for_code(")
source = source[:where] + "    fn glyph_id(&self, code: u32, cid: u32) -> Option<u16> {\n        " + glyph_id + "\n    }\n\n" + source[where:]
anchor = "    /// 📏 Width of a code in 1/1000 text space."
source = replace(source, anchor, '''    /// 🖋️ Checks that the encoded run has known glyphs when the font includes a program.
    pub fn shows_text(&self, text: &str) -> bool {
        let Some(bytes) = self.encode(text) else { return false; };
        if self.program.is_none() && self.raw_cff.is_none() { return true; }
        let mut codes = cmap::codes(self.cid_codec.as_ref(), &bytes);
        codes.all(|(code, _)| {
            self.glyph_id(code, self.cid_for_code(code)).is_some_and(|glyph|
                glyph != 0 && self.program.as_ref().is_none_or(|font| glyph < font.num_glyphs))
        })
    }

    /// 🔄️ Rebinds a text-show operator through Unicode while preserving its spacing operands.
    pub fn rebind_show(&self, original: &PdfOp, former: Option<&Self>) -> Option<PdfOp> {
        let resolve = |codes: &[u32]| {
            let former = former?;
            let mut text = String::new();
            for &code in codes { text.push_str(&former.text_for_code(code)?); }
            Some(text)
        };
        let mut operator = original.clone();
        match &mut operator {
            PdfOp::ShowText { text } | PdfOp::NextLineShowText { text }
            | PdfOp::NextLineShowTextSpaced { text, .. } => {
                if let PdfTextString::Codes { codes } = text { *text = PdfTextString::text(resolve(codes)?); }
                let PdfTextString::Text { text } = text else { return None; };
                self.shows_text(text).then_some(())?;
            }
            PdfOp::ShowTextArray { items } => {
                for item in items {
                    if let PdfTextArrayItem::Codes { codes } = item { *item = PdfTextArrayItem::Text { text: resolve(codes)? }; }
                    if let PdfTextArrayItem::Text { text } = item { self.shows_text(text).then_some(())?; }
                }
            }
            _ => return None,
        }
        Some(operator)
    }

''' + anchor)
(output / "codec.rs").write_text(source)
(output / "cff.rs").write_bytes((font_overlay.parent / "cff.rs").read_bytes())

source = (page / "🔤️metrics/🦀️.rs").read_text()
source = replace(source, "use super::PdfSnapshot;", "use super::{PdfFont, PdfSnapshot};")
source = replace(source, "FontCodec::new(font, &self.resources).ok());", "self.font_codec(font));")
anchor = "    pub(super) fn encoded_codes("
source = replace(source, anchor, '''    pub(super) fn font_codec(&self, font: &PdfFont) -> Option<FontCodec> {
        FontCodec::new(font, &self.resources).ok()
    }

''' + anchor)
(output / "metrics.rs").write_text(source)

source = (page / "🧮️content-ops/🦀️.rs").read_text()
start = source.index("pub(super) fn ensure_font(")
end = source.index("pub(super) fn replace_shown_text(", start)
source = source[:start] + r'''pub(super) fn ensure_font(snapshot: &PdfSnapshot, text: &str) -> Result<(String, Option<PdfMutation>), Fault> {
    let mut fonts = FontMeasurements::new(snapshot);
    let encodes = |codec: &_| text.split('\n').all(|line| codec.shows_text(line.strip_suffix('\r').unwrap_or(line)));
    for font in &snapshot.fonts {
        if fonts.codec(&font.id).is_some_and(encodes) { return Ok((font.id.clone(), None)); }
    }
    let font = new_standard_font(snapshot, "Helvetica");
    if !fonts.font_codec(&font).as_ref().is_some_and(encodes) { return Err(fault("no admitted PDF font can show the requested text")); }
    Ok((font.id.clone(), Some(PdfMutation::SetFont(SetFont { font, index: None }))))
}

pub(super) fn new_standard_font(snapshot: &PdfSnapshot, base_font: &str) -> PdfFont {
    let occupied: std::collections::HashSet<_> = snapshot.fonts.iter().map(|font| font.id.as_str()).collect();
    let id = (1..).map(|index| format!("F{index}")).find(|id| !occupied.contains(id.as_str())).expect("available PDF font id");
    PdfFont::standard(id, base_font)
}

''' + source[end:]
source = source.replace("let encodes = |codec: &_|", "let encodes = |codec: &semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::FontCodec|")
(output / "content-ops.rs").write_text(source)

source = (page / "🖱️object-edits/🦀️.rs").read_text()
source = replace(source, '''        "set-text" if object.kind == ObjectKind::Text => replace_shown_text(&mut content, &object, text)?,''', '''        "set-text" if object.kind == ObjectKind::Text => {
            let mut fonts = FontMeasurements::new(snapshot);
            let valid = fonts.codec(&object.font).map_or_else(|| text.bytes().all(|byte| (0x20..=0x7E).contains(&byte)), |font| font.shows_text(text));
            if !valid { return Err(fault("the active PDF font cannot show the requested text")); }
            replace_shown_text(&mut content, &object, text)?;
        }''')
source = replace(source, "ensure_font(snapshot);", "ensure_font(snapshot, text)?;")
source = replace(source, "    let (font_id, font_mutation) = font_for_base(snapshot, base_font);", '''    let font = font_for_base(snapshot, base_font)?;
    if font.id == object.font { return Ok(Vec::new()); }
    let mut fonts = FontMeasurements::new(snapshot);
    let target = fonts.font_codec(&font).ok_or_else(|| fault("the selected PDF font is unavailable"))?;''')
start = source.index("    let restore = text_cursor_after", source.index("pub(super) fn set_font_of_text"))
source = source[:start] + '''    content[object.end] = target.rebind_show(&content[object.end], fonts.codec(&object.font))
        .ok_or_else(|| fault("the selected PDF font cannot preserve the shown text"))?;
''' + source[start:]
source = replace(source, "    content.insert(object.end, PdfOp::SetFont { name: font_id, size: object.font_size });", "    content.insert(object.end, PdfOp::SetFont { name: font.id.clone(), size: object.font_size });")
source = replace(source, '''    if let Some(mutation) = font_mutation {
        mutations.push(mutation);''', '''    if let std::borrow::Cow::Owned(font) = font {
        mutations.push(PdfMutation::SetFont(SetFont { font, index: None }));''')
start = source.index("fn font_for_base(")
end = source.index("//#endregion", start)
source = source[:start] + '''fn font_for_base<'a>(snapshot: &'a PdfSnapshot, name: &str) -> Result<std::borrow::Cow<'a, PdfFont>, Fault> {
    use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::standard_font;
    if let Some(font) = snapshot.fonts.iter().find(|font| font.base_font() == name) {
        return Ok(std::borrow::Cow::Borrowed(font));
    }
    standard_font(name).filter(|font| font.name == name).ok_or_else(|| fault("the PDF font is not admitted"))?;
    Ok(std::borrow::Cow::Owned(new_standard_font(snapshot, name)))
}
''' + source[end:]
(output / "object-edits.rs").write_text(source)
