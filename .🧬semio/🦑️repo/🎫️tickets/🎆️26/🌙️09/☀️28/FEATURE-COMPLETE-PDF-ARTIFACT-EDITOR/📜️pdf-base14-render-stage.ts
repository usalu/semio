/** 🎨️ Stages the verified Base 14 CFF programs behind the established PDF font codec. */
import assert from "node:assert/strict";
import { join, relative, resolve } from "node:path";

const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const fonts = join(base,"🔨️modules/🔤️fonts");
const programPath = "🖼️base14";
const testRoot = join(base,"✏️editor/🖼️page/🧪️tests/🔬️unit");
const directory = join(import.meta.dir,"🗑️generated","pdf-base14");
const fixture = await Bun.file(join(import.meta.dir,"🔣️pdf-base14-outlines.json")).json();
const read = (path: string) => Bun.file(path).text();
const sha = (body: string | Uint8Array) => new Bun.CryptoHasher("sha256").update(body).digest("hex");
const edit = (text: string, from: string, to: string) => {
  assert.equal(text.split(from).length,2,from.slice(0,100));
  return text.replace(from,to);
};
const crateSuffix = "📦️packages/🦀️rust";
const fontOwner = join(fonts,crateSuffix);
const workspaceDirectory = relative(process.cwd(),resolve(fontOwner,"../../../../../../../.."));
const families = [
  {id:"sans",folder:"🖼️sans",names:["Helvetica","Helvetica-Bold","Helvetica-Oblique","Helvetica-BoldOblique"]},
  {id:"serif",folder:"🖼️serif",names:["Times-Roman","Times-Bold"]},
  {id:"serif-italic",folder:"🖼️serif-italic",names:["Times-Italic","Times-BoldItalic"]},
  {id:"mono",folder:"🖼️mono",names:["Courier","Courier-Bold"]},
  {id:"mono-italic",folder:"🖼️mono-italic",names:["Courier-Oblique","Courier-BoldOblique"]},
  {id:"symbol",folder:"🖼️symbol",names:["Symbol","ZapfDingbats"]},
].map(group => ({...group, crate: "semio-s-artifact-stdio-pdf-base14-"+group.id, root:join(fonts,programPath,group.folder)}));
const assetDeclarations = fixture.faces.map(({font,program}: {font:string;program:string}) => {
  const group = families.find(group=>group.names.includes(font));
  assert(group,font);
  const entry = program.replaceAll("-","_").toUpperCase();
  return {font,program,group,entry};
});
const program = [
  "",
  "fn standard_outline_program(name: &str) -> Option<&'static [u8]> {",
  "    Some(match name {",
  ...assetDeclarations.map(({font,group,entry}) =>
    `        ${JSON.stringify(font)} => ${group.crate.replaceAll("-","_")}::${entry},`),
  "        _ => return None,",
  "    })",
  "}",
  "",
].join("\n");

const codePath = join(fonts,"🦀️.rs");
const code = await read(codePath);
let patched = edit(code, "    raw_cff: Option<Vec<u8>>,", "    raw_cff: Option<Cow<'static, [u8]>>,");
patched = edit(patched, "impl FontCodec {", program + "\nimpl FontCodec {");
patched = edit(patched, "            PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program } => {",
  "            PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program } => {\n                let standard = standard_font(base_font);");
patched = edit(patched,"                    Some(program @ PdfFontProgram::Cff { .. }) => Some(program_data(program, resources)?),",
  "                    Some(program @ PdfFontProgram::Cff { .. }) => Some(Cow::Owned(program_data(program, resources)?)),\n" +
  "                    None => standard.and_then(|font| standard_outline_program(font.name)).map(Cow::Borrowed),");
patched = edit(patched, "raw.is_none() && symbolic && !builtin.is_empty()",
  "(raw.is_none() || program.is_none()) && symbolic && !builtin.is_empty()");
patched = edit(patched, "widths, standard_font(base_font), descriptor.as_ref().and_then(|d| d.missing_width), symbolic || standard_font(base_font).is_some_and(|f| f.symbolic)",
  "widths, standard, descriptor.as_ref().and_then(|d| d.missing_width), symbolic || standard.is_some_and(|f| f.symbolic)");
patched = edit(patched,"                    Some(program @ PdfFontProgram::CidCff { .. }) => Some(program_data(program, resources)?),",
  "                    Some(program @ PdfFontProgram::CidCff { .. }) => Some(Cow::Owned(program_data(program, resources)?)),");

const testPath = join(testRoot,"🦀️.rs");
const test = await read(testPath);
let patchedTest = edit(test, "fn canvas_text_uses_a_catalog_face_and_pdf_baseline() {",
  "fn canvas_paints_base14_glyph_outlines_at_the_pdf_baseline() {");
patchedTest = edit(patchedTest, `    for key in ["content", "fontFamily"] {
        assert_eq!(layer["text"][key], expected["text"][key]);
    }
    assert_eq!(layer["text"]["size"].as_f64(), expected["text"]["size"].as_f64());
    for key in ["x", "y"] {
        assert_eq!(layer[key].as_f64(), expected[key].as_f64());
    }`, `    assert_eq!(layer["fillRule"], expected["fillRule"]);
    let segments = layer["segments"].as_array().expect("Base 14 outline paths");
    assert!(segments.iter().any(|segment| segment["kind"] == "cubic"));
    assert_eq!(segments.iter().filter(|segment| segment["kind"] == "close").count(), expected["contours"].as_u64().unwrap() as usize);`);
patchedTest = edit(patchedTest,"render_window(&snapshot).expect(\"catalog text scene admission\");",
  "render_window(&snapshot).expect(\"Base 14 outline scene admission\");");
const paintPath = join(testRoot,"🧪️text-paint.json");
const oldPaint = await read(paintPath);
const paint = JSON.parse(oldPaint);
assert.equal(paint.expected.text.fontFamily,"Anta");
paint.expected.transform[3] = -paint.expected.transform[3];
delete paint.expected.x;
delete paint.expected.y;
delete paint.expected.text.fontFamily;
paint.expected.fillRule = "nonzero";
paint.expected.contours = 7;
const codeRows = [
  [codePath,sha(code),patched],
  [testPath,sha(test),patchedTest],
  [paintPath,sha(oldPaint),JSON.stringify(paint,null,2)+"\n"],
];
const packageRows: [string, string|null, string][] = [];
const fontManifestPath = join(fontOwner,"Cargo.toml");
const fontManifest = await read(fontManifestPath);
const manifestAnchor = "[dependencies]\n";
const fontDependencies = families.map(family =>
  `${family.crate} = { path = "${relative(fontOwner,join(family.root,crateSuffix)).replaceAll("\\","/")}" }\n`).join("");
packageRows.push([fontManifestPath,sha(fontManifest),edit(fontManifest,manifestAnchor,manifestAnchor+fontDependencies)]);
const workspacePath = join(workspaceDirectory,"Cargo.toml");
const workspace = await read(workspacePath);
const workspaceAnchor = workspace.split("\n").find(line=>line.includes(relative(workspaceDirectory,fontOwner).replaceAll("\\","/")));
assert(workspaceAnchor);
const members = families.map(family =>
  "  " + JSON.stringify(relative(workspaceDirectory,join(family.root,crateSuffix)).replaceAll("\\","/")) + ",").join("\n");
packageRows.push([workspacePath,sha(workspace),edit(workspace,workspaceAnchor,workspaceAnchor+"\n"+members)]);
const lockedFonts = '[[package]]\nname = "semio-s-artifact-stdio-pdf-fonts"';
const version = (Bun.TOML.parse(workspace) as { workspace:{package:{version:string}} }).workspace.package.version;
const lockEntries = families.map(family =>
  `[[package]]\nname = "${family.crate}"\nversion = "${version}"\n\n`).sort().join("");
const lockDependencies = families.map(family => " " + JSON.stringify(family.crate) + ",\n").sort().join("");
const locks = [join(workspaceDirectory,"Cargo.lock")];
if (process.argv.includes("--all-locks")) {
  for (const root of ["✏️s","🧰️framework"]) {
    for await (const file of new Bun.Glob("**/Cargo.lock").scan({cwd:root,onlyFiles:true})) {
      const path = join(root,file);
      if (path !== locks[0] && (await read(path)).includes(lockedFonts)) locks.push(path);
    }
  }
}
for (const path of locks) {
  const before = await read(path);
  const first = before.indexOf(lockedFonts);
  assert(first >= 0,path);
  const stop = before.indexOf("\n\n[[package]]",first);
  const owner = before.slice(first,stop===-1?before.length:stop);
  const dependenciesAt = ' "semio-s-artifact-stdio-pdf-glyph-list",\n';
  const updated = edit(owner,dependenciesAt,lockDependencies+dependenciesAt);
  let after = edit(before,owner,updated);
  const firstFamilyLater = '[[package]]\nname = "semio-s-artifact-stdio-pdf-conformance-support"';
  const firstFamilyAfter = after.includes(firstFamilyLater) ? firstFamilyLater : lockedFonts;
  after = edit(after,firstFamilyAfter,lockEntries+firstFamilyAfter);
  packageRows.push([path,sha(before),after]);
}
for (const family of families) {
  const owner = join(family.root,crateSuffix);
  const manifest = [
    "[package]",`name = "${family.crate}"`,
    `workspace = "${relative(owner,workspaceDirectory).replaceAll("\\","/")}"`,
    "version.workspace = true","edition.workspace = true","rust-version.workspace = true",
    'description = "PDF standard Type 1 outline resources"', "",
    "[package.metadata.semio]",'role = "library"',"","[lints]","workspace = true","","[lib]",'path = "🦀️.rs"',"",
  ].join("\n");
  const lib = [
    "//! 🔤️ The pinned URW outline programs for this PDF standard font family.",
    ...assetDeclarations.filter(row=>row.group===family).map(row =>
      `pub const ${row.entry}: &[u8] = include_bytes!("../../${row.program}.cff");`),
    "",
  ].join("\n");
  for (const [name,body] of [["Cargo.toml",manifest],["🦀️.rs",lib]]) {
    const path = join(owner,name);
    assert(!await Bun.file(path).exists(),path);
    packageRows.push([path,null,body]);
  }
}
const artifacts = [
  ...["LICENSE","COPYING"].map(name=>({name,path:join(fonts,programPath,name)})),
  ...assetDeclarations.map(({program,group})=>({name:program+".cff",path:join(group.root,program+".cff")})),
];
const artifactRows = [];
for (const {name,path} of artifacts) {
  assert(Buffer.byteLength(path)<=256 && !await Bun.file(path).exists(),path);
  const data = await Bun.file(join(directory,name)).bytes();
  assert(data.byteLength>0);
  artifactRows.push({path,sha256:sha(data),base64:Buffer.from(data).toString("base64")});
}
const origin = [
  "# PDF Base 14 Outline Programs",
  "",
  "The unmodified CFF tables come from the URW Base35 OpenType fonts at release " + fixture.release + " (" + fixture.commit + ").",
  "The repository's PDF encoding, widths and original glyph names still determine text positioning and decoding.",
  "The upstream license and corresponding font programs are available with the same release.",
  "",
  "[Pinned complete corresponding font sources](https://github.com/ArtifexSoftware/urw-base35-fonts/tree/" + fixture.commit + ")",
  "",
].join("\n");
const originPath = join(fonts,programPath,"SOURCE.md");
artifactRows.push({path:originPath,sha256:sha(origin),base64:Buffer.from(origin).toString("base64")});
for (const [path] of [...codeRows,...packageRows]) assert(Buffer.byteLength(path)<=256,path);
await Bun.write(join(directory,"font-code-overlay.json"),JSON.stringify(codeRows));
await Bun.write(join(directory,"font-package-overlay.json"),JSON.stringify(packageRows));
await Bun.write(join(directory,"font-asset-overlay.json"),JSON.stringify(artifactRows));
console.log("[DEBUG] Staged Base 14 code, scene regression and pinned font assets");
