"""🔤️ Generates the standard-14 AFM metrics, the Annex D encodings and the Adobe Glyph List as Rust
tables for `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🅰️tables/🦀️.rs`.

Sources (third-party, test/generation-time only — never a runtime dependency):
  * Adobe Core 14 AFM files as redistributed by matplotlib (`mpl-data/fonts/pdfcorefonts`).
  * fontTools: `fontTools.agl` (AGL + AGLFN), `fontTools.encodings.StandardEncoding`/`MacRoman`.
  * Python's cp1252 codec for WinAnsiEncoding (ISO 32000-1 Annex D.2 notes applied).

Usage: .venv/bin/python3 🐍️generate-font-tables.py <afm-dir> <out.rs>
"""
import sys, glob, os, codecs
from fontTools import agl
from fontTools.encodings import StandardEncoding, MacRoman

afm_dir, out_path = sys.argv[1], sys.argv[2]

FONTS = ["Courier", "Courier-Bold", "Courier-Oblique", "Courier-BoldOblique", "Helvetica", "Helvetica-Bold", "Helvetica-Oblique", "Helvetica-BoldOblique", "Times-Roman", "Times-Bold", "Times-Italic", "Times-BoldItalic", "Symbol", "ZapfDingbats"]

def parse_afm(path):
    info = {"widths": [], "codes": {}}
    with open(path, encoding="latin-1") as f:
        for line in f:
            parts = line.strip().split()
            if not parts:
                continue
            key = parts[0]
            if key == "FontBBox":
                info["bbox"] = [float(x) for x in parts[1:5]]
            elif key in ("Ascender", "Descender", "CapHeight", "XHeight", "ItalicAngle", "StdVW", "StdHW"):
                info[key] = float(parts[1])
            elif key == "IsFixedPitch":
                info[key] = parts[1] == "true"
            elif key == "Weight":
                info[key] = parts[1]
            elif key == "FamilyName":
                info[key] = " ".join(parts[1:])
            elif key == "C":
                fields = [p.strip() for p in line.split(";")]
                code = int(fields[0].split()[1])
                width = float([p for p in fields if p.startswith("WX")][0].split()[1])
                name = [p for p in fields if p.startswith("N ")][0].split()[1]
                info["widths"].append((name, width))
                if code >= 0:
                    info["codes"][code] = name
    return info

def rs_str(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'

out = []
out.append("//! 🅰️ Generated tables: Adobe Core 14 AFM metrics, ISO 32000-1 Annex D encodings and the Adobe")
out.append("//! Glyph List. Regenerate with the ticket script `🐍️generate-font-tables.py` (26/09/18/PDF-ARTIFACT-")
out.append("//! SPEC-COMPLETE); sources are the Adobe AFM files redistributed by matplotlib and fontTools' AGL.")
out.append("")
out.append("/// 📏 Metrics of one standard-14 font (AFM units, 1/1000 em).")
out.append("pub struct StandardFontMetrics {")
out.append("    pub name: &'static str,")
out.append("    pub family: &'static str,")
out.append("    pub bbox: [f64; 4],")
out.append("    pub ascender: f64,")
out.append("    pub descender: f64,")
out.append("    pub cap_height: f64,")
out.append("    pub x_height: f64,")
out.append("    pub italic_angle: f64,")
out.append("    pub stem_v: f64,")
out.append("    pub fixed_pitch: bool,")
out.append("    pub bold: bool,")
out.append("    pub symbolic: bool,")
out.append("    /// Glyph name → advance width, sorted by name for binary search.")
out.append("    pub widths: &'static [(&'static str, u16)],")
out.append("    /// Built-in encoding: character code → glyph name (Symbol/ZapfDingbats only carry their own).")
out.append("    pub builtin_encoding: &'static [(u8, &'static str)],")
out.append("}")
out.append("")
out.append("pub const STANDARD_FONTS: &[StandardFontMetrics] = &[")
for font in FONTS:
    info = parse_afm(os.path.join(afm_dir, font + ".afm"))
    widths = sorted(set(info["widths"]))
    codes = sorted(info["codes"].items())
    symbolic = font in ("Symbol", "ZapfDingbats")
    out.append("    StandardFontMetrics {")
    out.append(f"        name: {rs_str(font)},")
    out.append(f"        family: {rs_str(info.get('FamilyName', font))},")
    out.append(f"        bbox: [{info['bbox'][0]:.1f}, {info['bbox'][1]:.1f}, {info['bbox'][2]:.1f}, {info['bbox'][3]:.1f}],")
    out.append(f"        ascender: {info.get('Ascender', 0.0):.1f},")
    out.append(f"        descender: {info.get('Descender', 0.0):.1f},")
    out.append(f"        cap_height: {info.get('CapHeight', 0.0):.1f},")
    out.append(f"        x_height: {info.get('XHeight', 0.0):.1f},")
    out.append(f"        italic_angle: {info.get('ItalicAngle', 0.0):.1f},")
    out.append(f"        stem_v: {info.get('StdVW', 0.0):.1f},")
    out.append(f"        fixed_pitch: {'true' if info.get('IsFixedPitch') else 'false'},")
    out.append(f"        bold: {'true' if 'Bold' in font else 'false'},")
    out.append(f"        symbolic: {'true' if symbolic else 'false'},")
    out.append("        widths: &[" + ", ".join(f"({rs_str(n)}, {int(w)})" for n, w in widths) + "],")
    out.append("        builtin_encoding: &[" + ", ".join(f"({c}, {rs_str(n)})" for c, n in codes) + "],")
    out.append("    },")
out.append("];")
out.append("")

def encoding_table(names):
    return "&[" + ", ".join(f"({code}, {rs_str(name)})" for code, name in enumerate(names) if name and name != ".notdef") + "]"

out.append("/// 🔡 StandardEncoding (Annex D.2): code → glyph name.")
out.append("pub const STANDARD_ENCODING: &[(u8, &str)] = " + encoding_table(StandardEncoding.StandardEncoding) + ";")
out.append("")
out.append("/// 🔡 MacRomanEncoding (Annex D.2): code → glyph name.")
out.append("pub const MAC_ROMAN_ENCODING: &[(u8, &str)] = " + encoding_table(MacRoman.MacRoman) + ";")
out.append("")
# WinAnsi from cp1252 + AGL reverse; the spec notes: 0xA0 space (nbsp shown as space), 0xAD hyphen, undefined 0x80-0x9F codes → bullet.
uv2name = {}
for name, uv in sorted(agl.AGL2UV.items()):
    uv2name.setdefault(uv, name)
win = [None] * 256
for code in range(32, 256):
    try:
        ch = bytes([code]).decode("cp1252")
    except UnicodeDecodeError:
        win[code] = "bullet"
        continue
    uv = ord(ch)
    if code == 0xA0:
        win[code] = "space"
    elif code == 0xAD:
        win[code] = "hyphen"
    elif uv in uv2name:
        win[code] = uv2name[uv]
    else:
        win[code] = "bullet"
out.append("/// 🔡 WinAnsiEncoding (Annex D.2): code → glyph name.")
out.append("pub const WIN_ANSI_ENCODING: &[(u8, &str)] = " + encoding_table(win) + ";")
out.append("")
# PDFDocEncoding as unicode
pdfdoc = [None] * 256
for code in range(0x20, 0x7F):
    pdfdoc[code] = code
for code in range(0xA1, 0x100):
    if code != 0xAD:
        pdfdoc[code] = code
extra = {0x18: 0x02D8, 0x19: 0x02C7, 0x1A: 0x02C6, 0x1B: 0x02D9, 0x1C: 0x02DD, 0x1D: 0x02DB, 0x1E: 0x02DA, 0x1F: 0x02DC, 0x80: 0x2022, 0x81: 0x2020, 0x82: 0x2021, 0x83: 0x2026, 0x84: 0x2014, 0x85: 0x2013, 0x86: 0x0192, 0x87: 0x2044, 0x88: 0x2039, 0x89: 0x203A, 0x8A: 0x2212, 0x8B: 0x2030, 0x8C: 0x201E, 0x8D: 0x201C, 0x8E: 0x201D, 0x8F: 0x2018, 0x90: 0x2019, 0x91: 0x201A, 0x92: 0x2122, 0x93: 0xFB01, 0x94: 0xFB02, 0x95: 0x0141, 0x96: 0x0152, 0x97: 0x0160, 0x98: 0x0178, 0x99: 0x017D, 0x9A: 0x0131, 0x9B: 0x0142, 0x9C: 0x0153, 0x9D: 0x0161, 0x9E: 0x017E, 0xA0: 0x20AC}
for code, uv in extra.items():
    pdfdoc[code] = uv
out.append("/// 🔡 PDFDocEncoding (Annex D.2): code → Unicode scalar (text strings, not fonts).")
out.append("pub const PDF_DOC_ENCODING: &[(u8, u32)] = &[" + ", ".join(f"({c}, {uv})" for c, uv in enumerate(pdfdoc) if uv is not None) + "];")
out.append("")
full = {}
for name, uv in (agl.LEGACY_AGL2UV.items() if hasattr(agl, "LEGACY_AGL2UV") else []):
    full[name] = "".join(chr(c) for c in uv) if isinstance(uv, (list, tuple)) else chr(uv)
for name, uv in agl.AGL2UV.items():
    full[name] = chr(uv)
entries = sorted(full.items())
def rs_text(t):
    return '"' + "".join(f"\\u{{{ord(c):X}}}" for c in t) + '"'
out.append("/// 🔤 The Adobe Glyph List: glyph name → Unicode text (one scalar, or a sequence), sorted by name for binary search.")
out.append("pub const ADOBE_GLYPH_LIST: &[(&str, &str)] = &[" + ", ".join(f"({rs_str(n)}, {rs_text(uv)})" for n, uv in entries) + "];")
out.append("")
with open(out_path, "w", encoding="utf-8") as f:
    f.write("\n".join(out))
print(f"wrote {out_path}: {len(entries)} AGL entries, {len(FONTS)} fonts")
