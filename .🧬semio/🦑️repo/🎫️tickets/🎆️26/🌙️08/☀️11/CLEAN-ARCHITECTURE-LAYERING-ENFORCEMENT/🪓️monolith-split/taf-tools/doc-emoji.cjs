// usage: node doc-emoji.cjs <file>...   prints: file \t first-docstring-emoji \t folder-emoji \t count of docstrings in file starting with same emoji
const fs = require("fs");
const path = require("path");
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const emojiRe = /^\p{Extended_Pictographic}/u;
function firstGrapheme(s) { for (const { segment } of seg.segment(s)) return segment; return ""; }
function docstrings(text) {
  const out = [];
  const re = /\/\*\*\s*([^\s*][\s\S]{0,12})/g;
  let m;
  while ((m = re.exec(text))) { const g = firstGrapheme(m[1]); if (emojiRe.test(g)) out.push(g); else out.push(null); }
  return out;
}
module.exports = { firstGrapheme, docstrings, emojiRe };
if (require.main === module) {
  for (const f of process.argv.slice(2)) {
    const text = fs.readFileSync(f, "utf8");
    const m = text.match(/^\s*(?:\/\/[^\n]*\n\s*)*\/\*\*\s*([\s\S]{0,12})/);
    const first = m ? firstGrapheme(m[1].trimStart()) : "";
    const folder = path.basename(path.dirname(f));
    const fe = firstGrapheme(folder);
    const norm = (g) => g.replace(/️/g, "");
    const all = docstrings(text).filter(Boolean).map(norm);
    console.log([f, first, fe, all.filter((g) => g === norm(first)).length, all.filter((g) => g === norm(fe)).length].join("\t"));
  }
}
