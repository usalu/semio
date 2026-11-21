"""🔠 Stages named raw Type1C glyph bindings over the CID CFF implementation."""

import json
from pathlib import Path

from fontTools import cffLib

ticket = Path(__file__).resolve().parent
inputs = ticket / "🗑️generated/raw-cff-stage"
output = ticket / "🗑️generated/simple-cff-stage"
output.mkdir(exist_ok=True)

def replace(source, old, new):
    assert source.count(old) == 1, old
    return source.replace(old, new)

source = (inputs / "cff.rs").read_text()
source = replace(source, "    charset: Option<usize>,", "    charset: Option<usize>,\n    encoding: Option<usize>,")
source = replace(source, "                (15, 0) => dict.charset = Some(integer(0)?),", "                (15, 0) => dict.charset = Some(integer(0)?),\n                (16, 0) => dict.encoding = Some(integer(0)?),")
source = replace(source, "    cid_charset: Option<&'a [u8]>,", '''    source: &'a [u8],
    strings: Index<'a>,
    charset: usize,
    encoding: usize,
    cid: bool,''')
source = replace(source, "        let (_, global_start) = Index::read(source, string_start)?;", "        let (strings, global_start) = Index::read(source, string_start)?;")
start = source.index("        let cid_charset =")
end = source.index("        let mut glyph = 1;", start)
source = source[:start] + '''        let charset = dict.charset.unwrap_or(0);
        if dict.cid && charset < 3 { return None; }
        Some(Self { source, strings, charset, encoding: dict.encoding.unwrap_or(0), cid: dict.cid,
            glyphs, global, local, matrix: dict.matrix })
    }

    /// 📇️ Visits the specified CID for every glyph of a CID-keyed font.
    pub fn visit_cids(&self, mut visit: impl FnMut(u16, u16)) -> Option<()> {
        if !self.cid { return None; }
        self.visit_charset(|cid, glyph| { visit(cid, glyph); Some(()) })
    }

    /// 🏷️ Visits the original glyph name of a name-keyed CFF glyph.
    pub fn visit_glyph_names(&self, mut visit: impl FnMut(&'a str, u16)) -> Option<()> {
        if self.cid { return None; }
        self.visit_charset(|sid, glyph| { visit(self.glyph_name(sid)?, glyph); Some(()) })
    }

    fn glyph_name(&self, sid: u16) -> Option<&'a str> {
        let sid = usize::from(sid);
        if let Some(&name) = STANDARD_STRINGS.get(sid) { return Some(name); }
        std::str::from_utf8(self.strings.get(sid - STANDARD_STRINGS.len())?).ok().filter(|name| !name.is_empty())
    }

    fn visit_charset(&self, mut visit: impl FnMut(u16, u16) -> Option<()>) -> Option<()> {
        match self.charset {
            0 => {
                if self.glyphs.count > 229 { return None; }
                for glyph in 0..self.glyphs.count as u16 { visit(glyph, glyph)?; }
                return Some(());
            }
            1 | 2 => {
                let table = if self.charset == 1 { EXPERT_CHARSET } else { EXPERT_SUBSET_CHARSET };
                for (glyph, &sid) in table.get(..self.glyphs.count)?.iter().enumerate() { visit(sid, glyph as u16)?; }
                return Some(());
            }
            _ => {}
        }
        let (&format, mut bytes) = self.source.get(self.charset..)?.split_first()?;
''' + source[end:]
source = replace(source, "        visit(0, 0);", "        visit(0, 0)?;")
source = replace(source, "visit(first.checked_add(u16::try_from(offset).ok()?)?, u16::try_from(glyph).ok()?);", "visit(first.checked_add(u16::try_from(offset).ok()?)?, u16::try_from(glyph).ok()?)?;")
start = source.index("    /// 🖋️ Interprets one glyph")
source = source[:start] + '''    /// 📜️ Reads the name-keyed built-in code map, including custom supplements.
    pub fn builtin_encoding(&self) -> Option<Vec<(u8, String)>> {
        if self.cid { return None; }
        if self.encoding == 0 {
            return Some(super::STANDARD_ENCODING.iter().map(|&(code, name)| (code, name.into())).collect());
        }
        if self.encoding == 1 {
            return EXPERT_ENCODING.iter().enumerate().filter(|(_, sid)| **sid != 0)
                .map(|(code, &sid)| Some((code as u8, self.glyph_name(sid)?.into()))).collect();
        }
        let mut sids = Vec::with_capacity(self.glyphs.count);
        self.visit_charset(|sid, _| { sids.push(sid); Some(()) })?;
        let (&format, bytes) = self.source.get(self.encoding..)?.split_first()?;
        let kind = format & 127;
        if kind > 1 { return None; }
        let (&count, mut bytes) = bytes.split_first()?;
        let mut codes = [None; 256];
        let mut glyph = 1;
        for _ in 0..count {
            let (&first, rest) = bytes.split_first()?;
            bytes = rest;
            let end = match kind {
                0 => first,
                1 => {
                    let (&additional, rest) = bytes.split_first()?;
                    bytes = rest;
                    first.checked_add(additional)?
                }
                _ => return None,
            };
            for code in first..=end {
                if codes[usize::from(code)].replace(*sids.get(glyph)?).is_some() { return None; }
                glyph += 1;
            }
        }
        if format & 128 != 0 {
            let (&count, rest) = bytes.split_first()?;
            for row in rest.get(..usize::from(count) * 3)?.chunks_exact(3) {
                let sid = read_u16(row, 1)?;
                if !sids.contains(&sid) { return None; }
                codes[usize::from(row[0])] = Some(sid);
            }
        }
        codes.into_iter().enumerate().filter_map(|(code, sid)| sid.map(|sid| (code as u8, sid)))
            .map(|(code, sid)| Some((code, self.glyph_name(sid)?.into()))).collect()
    }

''' + source[start:]
source += "\n/// 🗂️ Standard CFF string IDs from Adobe Technical Note 5176 Appendix A.\n"
source += "const STANDARD_STRINGS: &[&str] = &[\n"
for start in range(0, len(cffLib.cffStandardStrings), 9):
    source += "    " + ", ".join(f'"{value}"' for value in cffLib.cffStandardStrings[start:start + 9]) + ",\n"
source += "];\n"
assert cffLib.cffISOAdobeStrings == cffLib.cffStandardStrings[:229]
for title, names in [("EXPERT_CHARSET", cffLib.cffIExpertStrings), ("EXPERT_SUBSET_CHARSET", cffLib.cffExpertSubsetStrings)]:
    ids = [cffLib.cffStandardStrings.index(name) for name in names]
    source += f"\nconst {title}: &[u16] = &[\n"
    for index in range(0, len(ids), 16):
        source += "    " + ", ".join(map(str, ids[index:index + 16])) + ",\n"
    source += "];\n"
ids = json.loads((ticket / "🧫️cff-expert-sids.json").read_text())["sids"]
assert len(ids) == 256 and ids[175] == 313
source += "\n/// 🔠 ExpertEncoding code-to-SID values from Adobe Technical Note 5176 Appendix B.\n"
source += "const EXPERT_ENCODING: &[u16; 256] = &[\n"
for start in range(0, len(ids), 16):
    source += "    " + ", ".join(map(str, ids[start:start + 16])) + ",\n"
source += "];\n"
(output / "cff.rs").write_text(source)

source = (inputs / "codec.rs").read_text()
start = source.index("                let builtin: Vec<")
end = source.index("                if !symbolic && encoding.differences.is_empty()", start)
source = source[:start] + '''                let raw = match program {
                    Some(program @ PdfFontProgram::Cff { .. }) => Some(program_data(program, resources)?),
                    _ => None,
                };
                let cff = raw.as_deref().and_then(CffOutlineFont::new);
                let builtin: Vec<(u8, String)> = match program {
                    Some(program @ PdfFontProgram::Type1 { .. }) => type1_builtin_encoding(&program_data(program,resources)?).unwrap_or_default(),
                    Some(PdfFontProgram::Cff { .. }) => cff.as_ref().and_then(CffOutlineFont::builtin_encoding).unwrap_or_default(),
                    _ => standard_font(base_font).filter(|f| f.symbolic).map(|f| f.builtin_encoding.iter().map(|(c, n)| (*c, n.to_string())).collect()).unwrap_or_default(),
                };
                let names = simple_names(encoding, &builtin, raw.is_none() && symbolic && !builtin.is_empty());
                if let Some(cff) = cff {
                    let mut glyphs = BTreeMap::new();
                    if cff.visit_glyph_names(|name, id| { glyphs.insert(name, id); }).is_some() {
                        for (&code, name) in &names {
                            if let Some(&glyph) = glyphs.get(name.as_str()) { codec.code_to_gid.insert(code, glyph); }
                        }
                        codec.raw_cff = raw;
                    }
                }

''' + source[end:]
(output / "codec.rs").write_text(source)
(output / "painter.rs").write_bytes((inputs / "painter.rs").read_bytes())
