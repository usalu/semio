"""🪶 Stages native rendering for raw CID-keyed CFF font programs."""

from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/raw-cff-stage"
output.mkdir(exist_ok=True)
base = Path(sys.argv[1]).resolve()
codec = base / "🔨️modules/🔤️fonts/🦀️.rs"
cff = codec.parent / "✒️cff/🦀️.rs"
painter = base / "✏️editor/🖼️page/🗺️scene/🔠️truetype-text/🦀️.rs"

def replace(source, old, new):
    assert source.count(old) == 1, old
    return source.replace(old, new)

source = cff.read_text()
first = source.index("        30 if dictionary => {")
last = source.index("        32..=246", first)
source = source[:first] + "        30 if dictionary => read_real(input)?,\n" + source[last:]
first = source.index("#[derive(Default)]")
source = source[:first] + '''fn read_real(input: &mut &[u8]) -> Option<f64> {
    let mut text = [0; 64];
    let mut length = 0;
    while let Some((&byte, rest)) = input.split_first() {
        *input = rest;
        for nibble in [byte >> 4, byte & 15] {
            let character = match nibble {
                0..=9 => b'0' + nibble,
                10 => b'.',
                11 | 12 => b'e',
                14 => b'-',
                15 => return std::str::from_utf8(&text[..length]).ok()?.parse::<f64>().ok().filter(|value| value.is_finite()),
                _ => return None,
            };
            *text.get_mut(length)? = character;
            length += 1;
            if nibble == 12 {
                *text.get_mut(length)? = b'-';
                length += 1;
            }
        }
    }
    None
}

type FontMatrix = [f64; 6];
const DEFAULT_MATRIX: FontMatrix = [0.001, 0.0, 0.0, 0.001, 0.0, 0.0];

fn matrix_point([a, b, c, d, e, f]: FontMatrix, [x, y]: [f64; 2]) -> [f64; 2] {
    [a * x + c * y + e, b * x + d * y + f]
}

''' + source[first:]
source = replace(source, "    cid: bool,", "    cid: bool,\n    matrix: Option<FontMatrix>,")
source = replace(source, "                (12, 30) => dict.cid = true,", "                (12, 7) => dict.matrix = Some(args.as_slice().try_into().ok()?),\n                (12, 30) => dict.cid = true,")
source = replace(source, "    cid_charset: Option<&'a [u8]>,", "    cid_charset: Option<&'a [u8]>,\n    matrix: Option<FontMatrix>,")
source = replace(source, "Self { glyphs, global, local, cid_charset }", "Self { glyphs, global, local, cid_charset, matrix: dict.matrix }")
first = source.index("        let local = match &self.local", source.index("pub fn outline"))
last = source.index("\n}\n", first)
source = source[:first] + '''        self.read_outline(glyph, false)
    }

    /// 🧭️ Interprets a raw CFF glyph in normalized 1000-unit PDF font space.
    pub fn raw_outline(&self, glyph: u16) -> Option<Vec<CffOutlineSegment>> {
        self.read_outline(glyph, true)
    }

    fn read_outline(&self, glyph: u16, normalize: bool) -> Option<Vec<CffOutlineSegment>> {
        let (local, sub_matrix) = match &self.local {
            LocalIndexes::Shared(index) => (*index, None),
            LocalIndexes::PerFont { source, dictionaries, selection } => {
                let dictionary = Dictionary::read(dictionaries.get(selection.dictionary(glyph)?)?)?;
                (local_subroutines(source, &dictionary)?, dictionary.matrix)
            }
        };
        let mut execution = Interpreter::new(self.global, local);
        if execution.run(self.glyphs.get(usize::from(glyph))?, 0)? != Flow::End { return None; }
        let mut segments = execution.geometry.segments;
        if normalize {
            let map = |point| {
                let [x, y] = match (self.matrix, sub_matrix) {
                    (Some(top), Some(sub)) => matrix_point(top, matrix_point(sub, point)),
                    (top, sub) => matrix_point(sub.or(top).unwrap_or(DEFAULT_MATRIX), point),
                };
                let result = [x * 1000.0, y * 1000.0];
                result.iter().all(|value| value.is_finite()).then_some(result)
            };
            use CffOutlineSegment::*;
            for segment in &mut segments {
                match segment {
                    MoveTo(to) | LineTo(to) => *to = map(*to)?,
                    QuadraticTo { control, to } => { *control = map(*control)?; *to = map(*to)?; }
                    CubicTo { first, second, to } => { *first = map(*first)?; *second = map(*second)?; *to = map(*to)?; }
                    Close => {}
                }
            }
        }
        Some(segments)
    }''' + source[last:]
(output / "cff.rs").write_text(source)

source = codec.read_text()
source = replace(source, "    program: Option<TrueTypeFont>,", "    program: Option<TrueTypeFont>,\n    raw_cff: Option<Vec<u8>>,")
source = replace(source, "program: None, code_to_gid:", "program: None, raw_cff: None, code_to_gid:")
first = source.index("                if descendant.cid_to_gid.is_none()")
last = source.index("\n            }", first)
source = source[:first] + '''                let raw = match &descendant.program {
                    Some(program @ PdfFontProgram::CidCff { .. }) => Some(program_data(program, resources)?),
                    _ => None,
                };
                if descendant.cid_to_gid.is_none() || raw.is_some() {
                    let cff = raw.as_deref().or_else(|| codec.program.as_ref()?.table("CFF "));
                    if let Some(cff) = cff.and_then(CffOutlineFont::new) {
                        let mut map = BTreeMap::new();
                        if cff.visit_cids(|cid, glyph| { map.insert(u32::from(cid), glyph); }).is_some() {
                            if descendant.cid_to_gid.is_none() {
                                codec.cid_to_gid_identity = false;
                                codec.cid_to_gid = map;
                            }
                            codec.raw_cff = raw;
                        }
                    }
                }''' + source[last:]
source = replace(source, "    /// ✒️ Interprets the glyph at its original ID", '''    /// 📏️ Returns the font units per em of the horizontal outline source.
    pub fn outline_units(&self) -> Option<u16> {
        if self.cid_codec.as_ref().is_some_and(|codec| codec.vertical) { return None; }
        if self.raw_cff.is_some() { return Some(1000); }
        self.outline_program().map(|font| font.units_per_em)
    }

    /// ✒️ Interprets the glyph at its original ID''')
source = replace(source, "        let font = self.outline_program()?;", '''        if self.cid_codec.as_ref().is_some_and(|codec| codec.vertical) { return None; }
        if let Some(data) = &self.raw_cff { return CffOutlineFont::new(data)?.raw_outline(glyph); }
        let font = self.outline_program()?;''')
(output / "codec.rs").write_text(source)

source = painter.read_text()
source = replace(source, "let font = codec.outline_program()?;", "let units_per_em = codec.outline_units()?;")
source = replace(source, "size / f64::from(font.units_per_em)", "size / f64::from(units_per_em)")
(output / "painter.rs").write_text(source)
