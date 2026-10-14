#!/usr/bin/env python3
"""🛰️ Wave 0: splits the 36 committed synthetic-orbit views into four orbit-quarter modules of nine views each
(`🌑️quarter-1` … `🌗️quarter-4`, 90° of the orbit apiece) so no compilation unit carries more than nine PNGs; the example
module assembles them back into capture order through `QUARTERS`/`frames()` (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
ART = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling")
EX = os.path.join(ART, "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit")
QUARTERS = [("🌑️quarter-1", "quarter_1", "first"), ("🌓️quarter-2", "quarter_2", "second"), ("🌕️quarter-3", "quarter_3", "third"), ("🌗️quarter-4", "quarter_4", "fourth")]
example = os.path.join(EX, "🦀️.rs")
text = open(example, encoding="utf-8").read()
rows = re.findall(r'\("(synthetic-orbit-frame-\d+)", include_bytes!\("🖼️assets/(🎞️frame-\d+\.png)"\)\),', text)
assert len(rows) == 36, len(rows)
for index, (dirname, module, ordinal) in enumerate(QUARTERS):
    chunk = rows[index * 9:(index + 1) * 9]
    first, last = index * 90, index * 90 + 80
    body = "".join(f'    ("{asset}", include_bytes!("../🖼️assets/{png}")),\n' for asset, png in chunk)
    os.makedirs(os.path.join(EX, dirname), exist_ok=True)
    open(os.path.join(EX, dirname, "🦀️.rs"), "w", encoding="utf-8").write(
        f"//! 🛰️ The {ordinal} quarter of the synthetic orbit: views {index * 9}–{index * 9 + 8} ({first}°–{last}° azimuth), in capture order.\n\n"
        f"/// 🎞️ `(asset id, PNG bytes)` of this quarter's nine views.\n"
        f"pub const FRAMES: [(&str, &[u8]); 9] = [\n{body}];\n")
start = text.index("/// 🎞️ `(asset id, PNG bytes)` for every view, in capture order.\n")
end = text.index("];\n", start) + 3
text = text[:start] + (
    "/// 🎞️ The four orbit quarters, nine views each, in capture order.\n"
    "pub const QUARTERS: [&[(&str, &[u8])]; 4] = [&crate::examples::synthetic_orbit::quarter_1::FRAMES, &crate::examples::synthetic_orbit::quarter_2::FRAMES, &crate::examples::synthetic_orbit::quarter_3::FRAMES, &crate::examples::synthetic_orbit::quarter_4::FRAMES];\n\n"
    "/// 🎞️ `(asset id, PNG bytes)` for every view, in capture order.\n"
    "pub fn frames() -> impl Iterator<Item = &'static (&'static str, &'static [u8])> {\n"
    "    QUARTERS.iter().flat_map(|quarter| quarter.iter())\n"
    "}\n") + text[end:]
open(example, "w", encoding="utf-8").write(text)
root = os.path.join(ART, "🦀️.rs")
r = open(root, encoding="utf-8").read()
anchor = '        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🦀️.rs"]\n        mod component;\n        pub use component::*;\n'
assert anchor in r
mounts = "".join(f'        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/{d}/🦀️.rs"]\n        pub mod {m};\n' for d, m, _ in QUARTERS)
r = r.replace(anchor, anchor + mounts, 1)
open(root, "w", encoding="utf-8").write(r)
print("ok")
