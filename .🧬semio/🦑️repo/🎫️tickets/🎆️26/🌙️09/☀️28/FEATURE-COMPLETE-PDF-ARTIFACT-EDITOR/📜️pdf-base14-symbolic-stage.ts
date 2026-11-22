/** 🔠 Preserves the original Symbol and Dingbats codes in controlled PDF content. */
import assert from "node:assert/strict";
import { join } from "node:path";

const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const fontRoot = join(base,"🔨️modules/🔤️fonts");
const directory=join(import.meta.dir,"🗑️generated","pdf-base14");
const guard=async(path:string)=>await Bun.file(path).text();
const digest=(body:string)=>new Bun.CryptoHasher("sha256").update(body).digest("hex");
const edit=(text:string,old:string,next:string)=>{
  assert.equal(text.split(old).length,2,old.slice(0,110));
  return text.replace(old,next);
};
const base14: [string,string,string][]=await Bun.file(join(directory,"font-code-overlay.json")).json();
const mapping: {name:string;text:string}[]=await Bun.file(join(directory,"dingbats-map.json")).json();
assert(mapping.length>=190);
const names = mapping.map(({name})=>name);
assert.deepEqual(names,[...new Set(names)].sort());
const table=[
  "fn zapf_unicode(name: &str) -> Option<&'static str> {",
  "    const GLYPHS: &[(&str, &str)] = &[",
  ...mapping.map(({name,text})=>`        (${JSON.stringify(name)}, ${JSON.stringify(text)}),`),
  "    ];",
  "    GLYPHS.binary_search_by_key(&name, |(glyph, _)| *glyph).ok().map(|index| GLYPHS[index].1)",
  "}",
  "",
].join("\n");
const [[path,before,after],...scene] = base14;
assert(path===join(fontRoot,"🦀️.rs") && digest(await guard(path))===before);
const method = "    fn install_simple(&mut self, names:";
let updated=edit(after,"impl FontCodec {",table+"\nimpl FontCodec {");
const index=updated.indexOf(method);
assert(index>=0);
const next=edit(updated.slice(index),"        for (code, name) in names {\n            if let Some(text) = glyph_name_to_unicode(name) {",[
  '        let dingbats = standard.is_some_and(|font| font.name == "ZapfDingbats");',
  "        for (code, name) in names {",
  "            let text = dingbats.then(|| zapf_unicode(name)).flatten().map(str::to_owned).or_else(|| glyph_name_to_unicode(name));",
  "            if let Some(text) = text {",
].join("\n"));
updated=updated.slice(0,index)+next;
const resourcePath = join(fontRoot,"🎚️controlled-opentype/🦀️.rs");
const resource = await guard(resourcePath);
const qualified=edit(resource,"if font.symbolic || font.name != base_font { return None; }","if font.name != base_font { return None; }");
const liftPath = join(base,"🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs");
const lift=await guard(liftPath);
const coded=edit(edit(lift,
  "(self.0.id == name).then_some(encoding.base.unwrap_or(PdfBaseEncoding::Standard))",
  "(self.0.id == name && self.controlled_code_width(name).is_none()).then_some(encoding.base.unwrap_or(PdfBaseEncoding::Standard))"),
  "            PdfFontKind::Type1 { program: Some(PdfFontProgram::OpenType { .. }), .. } => Some(PdfFixedCodeWidth::One),",
  [
    '            PdfFontKind::Type1 { base_font, program: None, .. } if matches!(base_font.as_str(), "Symbol" | "ZapfDingbats") => Some(PdfFixedCodeWidth::One),',
    "            PdfFontKind::Type1 { program: Some(PdfFontProgram::OpenType { .. }), .. } => Some(PdfFixedCodeWidth::One),",
  ].join("\n"));
await Bun.write(join(directory,"symbolic-code-overlay.json"),JSON.stringify([
  [path,before,updated],...scene,
  [resourcePath,digest(resource),qualified],
  [liftPath,digest(lift),coded],
]));
console.log("[DEBUG] Staged symbolic Type 1 by original code width and Dingbats text mapping");
