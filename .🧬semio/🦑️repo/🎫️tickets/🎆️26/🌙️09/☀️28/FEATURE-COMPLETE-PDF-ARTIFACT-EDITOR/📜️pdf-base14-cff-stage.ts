/** 🧪️ Extracts the installed CFF interpreter and neutral Base14 probe declarations. */
import assert from "node:assert/strict";
import { join } from "node:path";

const ticket = import.meta.dir;
const output = join(ticket,"🗑️generated","pdf-base14");
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts";
const parser = await Bun.file(join(root,"✒️cff/🦀️.rs")).text();
const outline = await Bun.file(join(root,"🔠️truetype/🧵️outline/🦀️.rs")).text();
const fixture = await Bun.file(join(ticket,"🔣️pdf-base14-outlines.json")).json();
const begin = outline.indexOf("#[derive(Clone, Copy, Debug, PartialEq)]\npub enum TrueTypeOutlineSegment");
const end = outline.indexOf("\n#[derive",begin+1);
assert(begin>=0&&end>begin);
const coordinates = outline.slice(begin,end);
const source = parser.replace(/^\/\/!.*\n/u,"");
const rust = (value: string) => JSON.stringify(value);
const rows = fixture.faces.map((row: { font:string; program:string; glyphs:{name:string;unicode:number}[] }) =>
  `(${rust(row.font)}, ${rust(row.program)}, &[${row.glyphs.map(({name,unicode})=>`(${rust(name)},${unicode})`).join(",")}]),`);
const law = await Bun.file(join(ticket,"📜️pdf-base14-cff-law.rs")).text();
await Bun.write(join(output,"base14.rs"),[
  "mod truetype {\n"+coordinates+"\n}",
  "const STANDARD_ENCODING: &[(u8,&str)] = &[];",
  "mod cff {\n"+source+"\n}",
  "const FACES: &[(&str,&str,&[(&str,u32)])] = &[\n"+rows.join("\n")+"\n];",
  law,
].join("\n"));
console.log("[DEBUG] Extracted exact CFF interpreter for the pinned Base 14 sources");
