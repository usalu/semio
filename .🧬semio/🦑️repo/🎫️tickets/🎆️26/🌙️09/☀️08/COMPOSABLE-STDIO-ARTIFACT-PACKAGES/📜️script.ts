const root = new URL("../../../../../../../", import.meta.url).pathname.replace(/\/$/, "");
const stdio = `${root}/✏️s/🔌️plugins/🗄️stdio`;
const facadePath = `${stdio}/📦️packages/🦀️rust/🦀️.rs`;

type ModuleBlock = { name: string; text: string; directory: string };

function matchingBrace(source: string, open: number): number {
  let depth = 0;
  let state: "code" | "line" | "block" | "string" | "char" = "code";
  let blockDepth = 0;
  for (let index = open; index < source.length; index += 1) {
    const char = source[index];
    const next = source[index + 1];
    if (state === "line") {
      if (char === "\n") state = "code";
      continue;
    }
    if (state === "block") {
      if (char === "/" && next === "*") {
        blockDepth += 1;
        index += 1;
      } else if (char === "*" && next === "/") {
        blockDepth -= 1;
        index += 1;
        if (blockDepth === 0) state = "code";
      }
      continue;
    }
    if (state === "string" || state === "char") {
      if (char === "\\") {
        index += 1;
      } else if ((state === "string" && char === '"') || (state === "char" && char === "'")) {
        state = "code";
      }
      continue;
    }
    if (char === "/" && next === "/") {
      state = "line";
      index += 1;
    } else if (char === "/" && next === "*") {
      state = "block";
      blockDepth = 1;
      index += 1;
    } else if (char === '"') {
      state = "string";
    } else if (char === "'") {
      const rest = source.slice(index, index + 16);
      if (/^'[A-Za-z0-9_]*\b/.test(rest) && !/^'.'/.test(rest)) continue;
      state = "char";
    } else if (char === "{") {
      depth += 1;
    } else if (char === "}") {
      depth -= 1;
      if (depth === 0) return index;
    }
  }
  throw new Error(`unclosed brace at ${open}`);
}

function outerModule(source: string, name: string): string {
  const match = new RegExp(`^pub mod ${name} \\{$`, "m").exec(source);
  if (!match) throw new Error(`missing outer module ${name}`);
  const open = source.indexOf("{", match.index);
  return source.slice(open + 1, matchingBrace(source, open));
}

function directModules(body: string): ModuleBlock[] {
  const modules: ModuleBlock[] = [];
  const pattern = /^    pub mod ([a-z0-9_]+) \{$/gm;
  for (let match = pattern.exec(body); match; match = pattern.exec(body)) {
    const open = body.indexOf("{", match.index);
    const close = matchingBrace(body, open);
    const text = body.slice(match.index, close + 1);
    const directory = text.match(/\.\.\/\.\.\/🗿️artifacts\/([^/]+)\//)?.[1] ?? "";
    modules.push({ name: match[1], text, directory });
    pattern.lastIndex = close + 1;
  }
  return modules;
}

function deindent(text: string, spaces: number): string {
  const prefix = " ".repeat(spaces);
  return text.split("\n").map((line) => line.startsWith(prefix) ? line.slice(spaces) : line).join("\n");
}

function localizePaths(text: string, directory: string): string {
  return text.replaceAll(`../../🗿️artifacts/${directory}/`, "");
}

function ensureDirectModulePaths(source: string, parent: string): string {
  const match = new RegExp(`^pub mod ${parent} \\{$`, "m").exec(source);
  if (!match) return source;
  const open = source.indexOf("{", match.index);
  const close = matchingBrace(source, open);
  let body = source.slice(open + 1, close);
  body = body.replace(/^(    )(pub mod [a-z0-9_]+ \{)$/gm, '$1#[path = "."]\n$1$2');
  return `${source.slice(0, open + 1)}${body}${source.slice(close)}`;
}

const command = Bun.argv[2] ?? "preview";
const artifacts: ModuleBlock[] = [
  { name: "binary", directory: "💾️binary", text: "" },
  { name: "txt", directory: "🔤️txt", text: "" },
  { name: "json", directory: "🧾️json", text: "" },
  { name: "xml", directory: "📰️xml", text: "" },
  { name: "csv", directory: "📊️csv", text: "" },
  { name: "md", directory: "📝️md", text: "" },
  { name: "deflate", directory: "🗜️deflate", text: "" },
  { name: "zip", directory: "🎒️zip", text: "" },
  { name: "step", directory: "📐️step", text: "" },
  { name: "ifc", directory: "🏗️ifc", text: "" },
  { name: "las", directory: "☁️las", text: "" },
  { name: "gltf", directory: "🧊️gltf", text: "" },
  { name: "obj", directory: "🗽️obj", text: "" },
  { name: "ply", directory: "🧱️ply", text: "" },
  { name: "dxf", directory: "🖋️dxf", text: "" },
  { name: "stl", directory: "🔺️stl", text: "" },
  { name: "svg", directory: "🎨️svg", text: "" },
  { name: "bmp", directory: "🪟️bmp", text: "" },
  { name: "dwg", directory: "🖊️dwg", text: "" },
  { name: "png", directory: "📷️png", text: "" },
  { name: "pdf", directory: "📖️pdf", text: "" },
  { name: "jpg", directory: "📸️jpg", text: "" },
  { name: "gif", directory: "🎞️gif", text: "" },
  { name: "tiff", directory: "🖼️tiff", text: "" },
  { name: "docx", directory: "📜️docx", text: "" },
  { name: "pptx", directory: "📽️pptx", text: "" },
  { name: "xlsx", directory: "📕️xlsx", text: "" },
  { name: "bcf", directory: "💬️bcf", text: "" },
  { name: "semio", directory: "🧿️semio", text: "" },
  { name: "mp4", directory: "🎥️mp4", text: "" },
  { name: "avi", directory: "📼️avi", text: "" },
  { name: "mp3", directory: "🎵️mp3", text: "" },
  { name: "wav", directory: "🔊️wav", text: "" },
  { name: "epw", directory: "🌦️epw", text: "" },
  { name: "tsv", directory: "📑️tsv", text: "" },
  { name: "html", directory: "🌐️html", text: "" },
];
const summary = artifacts.map((artifact) => ({ id: artifact.name, directory: artifact.directory }));

const codecSpecs: Record<string, { snapshot: string; mutation: string; schema: string; extension: string; protocol: string }> = {
  ply: { snapshot: "PlySnapshot", mutation: "PlyMutation", schema: "STDIO_PLY_DOCUMENT_SCHEMA", extension: "ply", protocol: "🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  stl: { snapshot: "StlSnapshot", mutation: "StlMutation", schema: "STDIO_STL_DOCUMENT_SCHEMA", extension: "stl", protocol: "🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  las: { snapshot: "LasSnapshot", mutation: "LasMutation", schema: "STDIO_LAS_DOCUMENT_SCHEMA", extension: "las", protocol: "🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  dxf: { snapshot: "DxfSnapshot", mutation: "DxfMutation", schema: "STDIO_DXF_DOCUMENT_SCHEMA", extension: "dxf", protocol: "🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  mp3: { snapshot: "Mp3Snapshot", mutation: "Mp3Mutation", schema: "STDIO_MP3_DOCUMENT_SCHEMA", extension: "mp3", protocol: "🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  xlsx: { snapshot: "XlsxSnapshot", mutation: "XlsxMutation", schema: "STDIO_XLSX_DOCUMENT_SCHEMA", extension: "xlsx", protocol: "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  tiff: { snapshot: "TiffSnapshot", mutation: "TiffMutation", schema: "STDIO_TIFF_DOCUMENT_SCHEMA", extension: "tiff", protocol: "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  jpg: { snapshot: "JpgSnapshot", mutation: "JpgMutation", schema: "STDIO_JPG_DOCUMENT_SCHEMA", extension: "jpg", protocol: "🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  avi: { snapshot: "AviSnapshot", mutation: "AviMutation", schema: "STDIO_AVI_DOCUMENT_SCHEMA", extension: "semio", protocol: "🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  png: { snapshot: "PngSnapshot", mutation: "PngMutation", schema: "STDIO_PNG_DOCUMENT_SCHEMA", extension: "png", protocol: "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  csv: { snapshot: "CsvSnapshot", mutation: "CsvMutation", schema: "STDIO_CSV_DOCUMENT_SCHEMA", extension: "csv", protocol: "🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  md: { snapshot: "MdSnapshot", mutation: "MdMutation", schema: "STDIO_MD_DOCUMENT_SCHEMA", extension: "md", protocol: "🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  docx: { snapshot: "DocxSnapshot", mutation: "DocxMutation", schema: "STDIO_DOCX_DOCUMENT_SCHEMA", extension: "docx", protocol: "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  mp4: { snapshot: "Mp4Snapshot", mutation: "Mp4Mutation", schema: "STDIO_MP4_DOCUMENT_SCHEMA", extension: "semio", protocol: "🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  json: { snapshot: "JsonSnapshot", mutation: "JsonMutation", schema: "STDIO_JSON_DOCUMENT_SCHEMA", extension: "json", protocol: "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  gltf: { snapshot: "GltfSnapshot", mutation: "GltfMutation", schema: "STDIO_GLTF_DOCUMENT_SCHEMA", extension: "gltf", protocol: "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  bcf: { snapshot: "BcfSnapshot", mutation: "BcfMutation", schema: "STDIO_BCF_DOCUMENT_SCHEMA", extension: "bcf", protocol: "🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  zip: { snapshot: "ZipSnapshot", mutation: "ZipMutation", schema: "STDIO_ZIP_DOCUMENT_SCHEMA", extension: "zip", protocol: "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  xml: { snapshot: "XmlSnapshot", mutation: "XmlMutation", schema: "STDIO_XML_DOCUMENT_SCHEMA", extension: "xml", protocol: "🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  deflate: { snapshot: "DeflateSnapshot", mutation: "DeflateMutation", schema: "STDIO_DEFLATE_DOCUMENT_SCHEMA", extension: "zz", protocol: "🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  obj: { snapshot: "ObjSnapshot", mutation: "ObjMutation", schema: "STDIO_OBJ_DOCUMENT_SCHEMA", extension: "obj", protocol: "🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  pdf: { snapshot: "crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot", mutation: "crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation", schema: "STDIO_PDF_DOCUMENT_SCHEMA", extension: "pdf", protocol: "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  pptx: { snapshot: "PptxSnapshot", mutation: "PptxMutation", schema: "STDIO_PPTX_DOCUMENT_SCHEMA", extension: "pptx", protocol: "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  step: { snapshot: "StepSnapshot", mutation: "StepMutation", schema: "STDIO_STEP_DOCUMENT_SCHEMA", extension: "step", protocol: "🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  dwg: { snapshot: "DwgSnapshot", mutation: "DwgMutation", schema: "STDIO_DWG_DOCUMENT_SCHEMA", extension: "dwg", protocol: "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
  svg: { snapshot: "SvgSnapshot", mutation: "SvgMutation", schema: "STDIO_SVG_DOCUMENT_SCHEMA", extension: "svg", protocol: "🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio" },
};

const artifactDependencies: Record<string, string[]> = {
  binary: [], txt: ["binary"], json: ["txt"], xml: ["txt"], csv: ["txt"], md: ["txt"],
  deflate: ["binary"], zip: ["binary", "deflate", "xml"], step: ["binary", "txt"],
  ifc: ["binary", "step", "txt"], las: ["binary"], gltf: ["json"], obj: ["txt"],
  ply: ["txt"], dxf: ["txt"], stl: ["binary", "txt"], svg: ["xml"], bmp: ["binary"],
  dwg: ["binary"], png: ["binary", "deflate", "zip"], pdf: ["binary", "deflate"],
  jpg: ["binary"], gif: ["binary"], tiff: ["binary"], docx: ["binary", "xml", "zip"],
  pptx: ["binary", "xml", "zip"], xlsx: ["binary", "xml", "zip"], bcf: ["binary", "xml", "zip"],
  semio: ["avi", "bcf", "bmp", "csv", "docx", "dwg", "dxf", "gif", "gltf", "ifc", "jpg", "json", "las", "md", "mp3", "mp4", "obj", "pdf", "ply", "png", "pptx", "step", "stl", "svg", "tiff", "txt", "wav", "xml", "zip"],
  mp4: [], avi: [], mp3: [], wav: [], epw: [], tsv: [], html: [],
};

async function artifactSources(directory: string): Promise<string> {
  let source = "";
  const glob = new Bun.Glob("**/🦀️.rs");
  for await (const relative of glob.scan({ cwd: `${stdio}/🗿️artifacts/${directory}`, onlyFiles: true })) {
    source += await Bun.file(`${stdio}/🗿️artifacts/${directory}/${relative}`).text();
  }
  return source;
}

if (command === "wire-facade") {
  let component = await Bun.file(`${stdio}/🦀️.rs`).text();
  for (const artifact of artifacts) {
    component = component.replaceAll(`crate::artifacts::${artifact.name}`, `semio_s_artifact_stdio_${artifact.name}`);
    for (const editor of editors.filter((item) => item.directory === artifact.directory)) {
      component = component.replaceAll(`crate::editor::${editor.name}`, `semio_s_artifact_stdio_${artifact.name}::editor::${editor.name}`);
    }
    for (const viewer of viewers.filter((item) => item.directory === artifact.directory)) {
      component = component.replaceAll(`crate::viewer::${viewer.name}`, `semio_s_artifact_stdio_${artifact.name}::viewer::${viewer.name}`);
    }
  }
  await Bun.write(`${stdio}/🦀️.rs`, component);
  const thin = `//! 🔌️ Stdio plugin composition over independently compiled artifact packages.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

#[cfg(feature = "component-app-assembly")]
#[path = "../../🦀️.rs"]
pub mod plugin;
#[cfg(feature = "component-app-assembly")]
pub use plugin::plugin;
#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, plugin::StdioApps);

#[path = "../../📇️registry/🦀️.rs"]
pub mod registry;

#[cfg(feature = "full-artifact-catalog")]
#[path = "../../🛂️manifest/🦀️.rs"]
pub mod manifest;
`;
  await Bun.write(facadePath, thin);

  const allFeatures = artifacts.map((artifact) => `"dep:semio-s-artifact-stdio-${artifact.name}"`).join(", ");
  const componentFeatures = artifacts.map((artifact) => `"semio-s-artifact-stdio-${artifact.name}/component-app-assembly"`).join(", ");
  const home = ["binary", "txt", "json", "xml", "csv", "deflate", "zip", "xlsx"].map((id) => `"dep:semio-s-artifact-stdio-${id}"`).join(", ");
  const artifactLines = artifacts.map((artifact) => `semio-s-artifact-stdio-${artifact.name} = { workspace = true, optional = true }`).join("\n");
  const manifest = `[package]
name = "semio-s-plugin-stdio"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Stdio plugin composition over independently compiled artifact packages"

[lints]
workspace = true

[package.metadata.component]
package = "semio:stdio"

[package.metadata.semio]
role = "plugin"

[lib]
crate-type = ["cdylib", "rlib"]
path = "🦀️.rs"

[features]
default = ["plugin-root"]
plugin-root = ["component-app-assembly", "semio-framework-plugin/component-guest"]
component-app-assembly = ["full-artifact-catalog", ${componentFeatures}]
full-artifact-catalog = [${allFeatures}]
home-io = [${home}]

[dependencies]
semio-framework = { workspace = true }
semio-framework-dispatch-macros = { workspace = true }
semio-framework-hash = { workspace = true }
semio-framework-os-kernel = { workspace = true }
semio-framework-plugin = { workspace = true }
semio-framework-value-derive = { workspace = true }
semio-s-artifact-stdio-contract = { workspace = true }
pack = { workspace = true }
${artifactLines}
serde = { workspace = true }
serde_json = { workspace = true }

[[test]]
name = "native_openable_provider"
path = "🧪️tests/📇️native-openable-provider/🦀️.rs"
`;
  await Bun.write(`${stdio}/📦️packages/🦀️rust/Cargo.toml`, manifest);
  console.log(JSON.stringify(summary, null, 2));
  process.exit(0);
}

if (command === "generate-cargo") {
  for (const artifact of artifacts) {
    const source = await artifactSources(artifact.directory);
    const dependencies = new Set<string>([
      "semio-s-artifact-stdio-contract",
      "semio-framework-os-kernel",
      "semio-framework-plugin",
      "semio-framework-schema",
      "semio-framework-value-derive",
    ]);
    const tokenDependencies: Array<[RegExp, string]> = [
      [/semio_framework::/, "semio-framework"],
      [/semio_framework_3d::/, "semio-framework-3d"],
      [/semio_framework_geometry::/, "semio-framework-geometry"],
      [/semio_framework_graph::|graph_core::/, "semio-framework-graph"],
      [/semio_framework_hash::/, "semio-framework-hash"],
      [/semio_framework_job::/, "semio-framework-job"],
      [/semio_framework_math::/, "semio-framework-math"],
      [/semio_framework_mesh_engine::/, "semio-framework-mesh-engine"],
      [/semio_framework_number::/, "semio-framework-number"],
      [/semio_framework_ui_contract::/, "semio-framework-ui-contract"],
      [/(^|[^A-Za-z0-9_])pack::/m, "pack"],
      [/(^|[^A-Za-z0-9_])serde::/m, "serde"],
      [/(^|[^A-Za-z0-9_])serde_json::/m, "serde_json"],
    ];
    for (const [pattern, dependency] of tokenDependencies) if (pattern.test(source)) dependencies.add(dependency);
    if (artifact.name === "binary" || artifact.name === "txt") dependencies.add("semio-framework-dispatch-macros");
    if (codecSpecs[artifact.name]) dependencies.add("semio-framework-hash");
    for (const dependency of artifactDependencies[artifact.name] ?? []) dependencies.add(`semio-s-artifact-stdio-${dependency}`);
    const featureDeps = dependencies.has("semio-framework-ui-contract") ? ["dep:semio-framework-ui-contract"] : [];
    const lines = [...dependencies].sort().map((dependency) => {
      if (dependency === "pack") return "pack = { workspace = true }";
      if (dependency === "semio-framework-ui-contract") return `${dependency} = { workspace = true, optional = true }`;
      return `${dependency} = { workspace = true }`;
    });
    const devDependencies: string[] = [];
    if (source.includes("semio_framework_async_macros::")) devDependencies.push("semio-framework-async-macros = { workspace = true }");
    if (source.includes("semio_framework_ui_scene::")) devDependencies.push("semio-framework-ui-scene = { workspace = true }");
    const manifest = `[package]\nname = "semio-s-artifact-stdio-${artifact.name}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "Stdio ${artifact.name} artifact"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[features]\ndefault = []\ncomponent-app-assembly = [${featureDeps.map((item) => `"${item}"`).join(", ")}]\n\n[dependencies]\n${lines.join("\n")}\n${devDependencies.length ? `\n[dev-dependencies]\n${devDependencies.join("\n")}\n` : ""}`;
    await Bun.write(`${stdio}/🗿️artifacts/${artifact.directory}/📦️packages/🦀️rust/Cargo.toml`, manifest);
  }
  console.log(JSON.stringify(summary, null, 2));
  process.exit(0);
}

if (command === "normalize-schema-alias") {
  for (const artifact of artifacts) {
    const directory = `${stdio}/🗿️artifacts/${artifact.directory}`;
    const glob = new Bun.Glob("**/🦀️.rs");
    for await (const relative of glob.scan({ cwd: directory, onlyFiles: true })) {
      const path = `${directory}/${relative}`;
      const current = await Bun.file(path).text();
      const next = current
        .replaceAll("extern crate semio_framework_schema as schema;\n", "")
        .replaceAll("extern crate semio_framework_schema as framework_schema;\n", "")
        .replaceAll("framework_schema::", "schema::")
        .replace(/::schema::(ArtifactSchema(?:Descriptor)?|ArtifactInferenceDescriptor|FacetLeaves|register_artifact_[a-z_]+)/g, "::framework_schema::$1")
        .replace(/(?<![A-Za-z0-9_:])schema::(ArtifactSchema(?:Descriptor)?|ArtifactInferenceDescriptor|FacetLeaves|register_artifact_[a-z_]+)/g, "framework_schema::$1");
      let rooted = relative === "🦀️.rs"
        ? next
            .replace("extern crate semio_framework_os_kernel as store;\n", "extern crate semio_framework_os_kernel as store;\nextern crate semio_framework_schema as framework_schema;\n")
        : next;
      if (relative === "🦀️.rs") {
        rooted = ensureDirectModulePaths(rooted, "editor");
        rooted = ensureDirectModulePaths(rooted, "viewer");
      }
      if (rooted !== current) await Bun.write(path, rooted);
    }
  }
  console.log(JSON.stringify(summary, null, 2));
  process.exit(0);
}

function codecCode(id: string): string {
  const spec = codecSpecs[id];
  if (!spec) return `pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {\n    Vec::new()\n}`;
  const snapshot = spec.snapshot.startsWith("crate::") ? spec.snapshot : `crate::${spec.snapshot}`;
  const mutation = spec.mutation.startsWith("crate::") ? spec.mutation : `crate::${spec.mutation}`;
  return `fn native_codec() -> store::ArtifactCodec {\n    let mut codec = store::ArtifactCodec::of::<${snapshot}, ${mutation}>(crate::${spec.schema});\n    codec.extension = "${spec.extension}";\n    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("${spec.protocol}"));\n    codec\n}\n\npub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {\n    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.${id}.v1", artifact: "${id}", kind: crate::artifact_kind, codec: native_codec }]\n}`;
}

if (command === "wire-contract") {
  for (const artifact of artifacts) {
    const artifactPath = `${stdio}/🗿️artifacts/${artifact.directory}/🦀️.rs`;
    let current = await Bun.file(artifactPath).text();
    const oldPreamble = `#![allow(async_fn_in_trait)]\n#![allow(long_running_const_eval)]\n\nextern crate semio_framework_os_kernel as dsl;\nextern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as store;\nextern crate semio_framework_schema as schema;\nextern crate semio_framework_value_derive as value_derive;\nextern crate semio_framework_graph as graph_core;\n\npub use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack};\npub use semio_s_artifact_stdio_contract::{base64_standard, impl_serde_op_codec, semantic_fingerprint, ArtifactAssembly};\n\n`;
    if (!current.startsWith(oldPreamble)) throw new Error(`unexpected preamble in ${artifact.name}`);
    current = current.slice(oldPreamble.length);
    const docMatch = /^(?:(?:\/\/!.*)\n)+\n/u.exec(current);
    if (!docMatch) throw new Error(`missing crate docs in ${artifact.name}`);
    const docs = docMatch[0];
    current = current.slice(docs.length);

    const sourceGlob = new Bun.Glob("**/🦀️.rs");
    let sources = current;
    for await (const relative of sourceGlob.scan({ cwd: `${stdio}/🗿️artifacts/${artifact.directory}`, onlyFiles: true })) {
      if (relative !== "🦀️.rs") sources += await Bun.file(`${stdio}/🗿️artifacts/${artifact.directory}/${relative}`).text();
    }
    const aliases: string[] = [];
    if (/(^|[^:\w])dsl::/m.test(sources)) aliases.push("extern crate semio_framework_os_kernel as dsl;");
    if (/(^|[^:\w])protocol::/m.test(sources)) aliases.push("extern crate semio_framework_os_kernel as protocol;");
    if (/(^|[^:\w])store::/m.test(sources) || codecSpecs[artifact.name]) aliases.push("extern crate semio_framework_os_kernel as store;");
    if (/(^|[^:\w])schema::/m.test(sources)) aliases.push("extern crate semio_framework_schema as schema;");
    if (/(^|[^:\w])value_derive::/m.test(sources)) aliases.push("extern crate semio_framework_value_derive as value_derive;");
    if (/(^|[^:\w])graph_core::/m.test(sources)) aliases.push("extern crate semio_framework_graph as graph_core;");
    const helpers: string[] = [];
    if (sources.includes("crate::base64_standard")) helpers.push("base64_standard");
    if (sources.includes("crate::impl_serde_op_codec")) helpers.push("impl_serde_op_codec");
    if (sources.includes("crate::semantic_fingerprint")) helpers.push("semantic_fingerprint");
    const helperUse = helpers.length ? `\npub(crate) use semio_s_artifact_stdio_contract::{${helpers.join(", ")}};\n` : "";
    const preamble = `${docs}#![allow(async_fn_in_trait)]\n#![allow(long_running_const_eval)]\n\n${aliases.join("\n")}\n${helperUse}\n`;

    const schemaMarker = `pub const ${artifact.name.toUpperCase()}_ARTIFACT_SCHEMA_ID`;
    const markerIndex = current.indexOf(schemaMarker);
    if (markerIndex < 0 && artifact.name !== "gltf") throw new Error(`missing schema id marker in ${artifact.name}`);
    const lineEnd = current.indexOf("\n", markerIndex < 0 ? current.indexOf("pub const GLTF_ARTIFACT_SCHEMA_ID") : markerIndex);
    current = `${current.slice(0, lineEnd + 1)}\n/// 📜 Schema-owned package definition.\npub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("🧬️schema/📜️artifact-definition.json");\n${current.slice(lineEnd + 1)}`;

    const assemblyPattern = new RegExp(`pub fn assembly\\(definition: semio_framework_plugin::ArtifactDefinition\\) -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> \\{\\n    semio_s_artifact_stdio_contract::(runtime_assembly|definition_only_assembly)\\("${artifact.name}", definition${codecSpecs[artifact.name] ? ", declaration" : ""}\\)\\n\\}`);
    const assemblyMatch = assemblyPattern.exec(current);
    if (!assemblyMatch) throw new Error(`missing assembly function in ${artifact.name}`);
    const call = codecSpecs[artifact.name]
      ? `semio_s_artifact_stdio_contract::runtime_assembly("${artifact.name}", definition()?, declaration)`
      : `semio_s_artifact_stdio_contract::definition_only_assembly("${artifact.name}", definition()?)`;
    current = current.replace(assemblyPattern, `pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {\n    ${call}\n}`);

    const extraExecutables = artifact.name === "gltf" ? `\n    executables.extend(crate::gltf_inference_services().into_iter().map(|service| semio_s_artifact_stdio_contract::ArtifactExecutable {\n        identity: service.metadata().inference_schema.to_owned(),\n        executable: service.executable_identity(),\n    }));\n    executables.extend([\n        "s.stdio.gltf.mutation.change-material-alpha-mode.v1",\n        "s.stdio.gltf.mutation.change-material-double-sided.v1",\n        "s.stdio.gltf.mutation.create-scene.v1",\n    ].into_iter().map(|identity| semio_s_artifact_stdio_contract::ArtifactExecutable::from_function_pointer(identity, crate::schema::mutations::apply_gltf_mutation as *const ())));` : "";
    const definitionCode = codecSpecs[artifact.name]
      ? `pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {\n    let factories = native_codecs();\n    let mut executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;${extraExecutables}\n    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)\n}`
      : `pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {\n    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)\n}`;
    const contributionCode = `${definitionCode}\n\npub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {\n    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)\n}\n\n${codecCode(artifact.name)}\n\npub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {\n    semio_s_artifact_stdio_contract::ArtifactContribution {\n        identity: "${artifact.name}",\n        schema: ARTIFACT_DEFINITION_SCHEMA,\n        definition,\n        assembly,\n        formats,\n        native_codecs,\n    }\n}\n\n`;
    const region = current.indexOf("//#region");
    if (region < 0) throw new Error(`missing first region in ${artifact.name}`);
    current = `${current.slice(0, region)}${contributionCode}${current.slice(region)}`;
    await Bun.write(artifactPath, `${preamble}${current}`);
  }
  console.log(JSON.stringify(summary, null, 2));
  process.exit(0);
}

if (command === "preview") {
  console.log(JSON.stringify(summary, null, 2));
  process.exit(0);
}

if (command !== "extract-rust-mounts") throw new Error(`unknown command ${command}`);

for (const artifact of artifacts) {
  const artifactPath = `${stdio}/🗿️artifacts/${artifact.directory}/🦀️.rs`;
  const current = await Bun.file(artifactPath).text();
  const bodyStart = artifact.text.indexOf("\n") + 1;
  const bodyEnd = artifact.text.lastIndexOf("\n    }");
  let mounts = deindent(artifact.text.slice(bodyStart, bodyEnd), 8);
  mounts = mounts.replace(/^#\[path = "[^\n]+\/🦀️\.rs"\]\nmod component;\npub use component::\*;\n+/u, "");
  mounts = localizePaths(mounts, artifact.directory);

  const artifactEditors = editors.filter((item) => item.directory === artifact.directory);
  const artifactViewers = viewers.filter((item) => item.directory === artifact.directory);
  const editorMounts = artifactEditors.length === 0 ? "" : `\n#[cfg(feature = "component-app-assembly")]\n#[path = "."]\npub mod editor {\n${artifactEditors.map((item) => `    #[path = "."]\n${localizePaths(item.text, artifact.directory)}`).join("\n")}\n}\n`;
  const viewerMounts = artifactViewers.length === 0 ? "" : `\n#[cfg(feature = "component-app-assembly")]\n#[path = "."]\npub mod viewer {\n${artifactViewers.map((item) => `    #[path = "."]\n${localizePaths(item.text, artifact.directory)}`).join("\n")}\n}\n`;

  const preamble = `#![allow(async_fn_in_trait)]\n#![allow(long_running_const_eval)]\n\nextern crate semio_framework_os_kernel as dsl;\nextern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as store;\nextern crate semio_framework_schema as schema;\nextern crate semio_framework_value_derive as value_derive;\nextern crate semio_framework_graph as graph_core;\n\npub use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack};\npub use semio_s_artifact_stdio_contract::{base64_standard, impl_serde_op_codec, semantic_fingerprint, ArtifactAssembly};\n\n`;
  let next = `${preamble}${current.trimEnd()}\n\n${mounts.trim()}\n${editorMounts}${viewerMounts}`;
  next = next.replaceAll(`crate::artifacts::${artifact.name}`, "crate");
  next = next.replace(/crate::artifacts::([a-z0-9_]+)/g, (_, dependency: string) => `semio_s_artifact_stdio_${dependency}`);
  next = next.replaceAll("crate::registry::ArtifactAssembly", "semio_s_artifact_stdio_contract::ArtifactAssembly");
  next = next.replaceAll("crate::registry::runtime_assembly", "semio_s_artifact_stdio_contract::runtime_assembly");
  next = next.replaceAll("crate::registry::definition_only_assembly", "semio_s_artifact_stdio_contract::definition_only_assembly");
  next = next.replace(/crate::registry::format_descriptors_for\("[a-z0-9_]+"\)/g, "formats()")
  await Bun.write(artifactPath, next);
}

for (const artifact of artifacts) {
  const directory = `${stdio}/🗿️artifacts/${artifact.directory}`;
  const glob = new Bun.Glob("**/🦀️.rs");
  for await (const relative of glob.scan({ cwd: directory, onlyFiles: true })) {
    const path = `${directory}/${relative}`;
    const current = await Bun.file(path).text();
    let next = current.replaceAll(`crate::artifacts::${artifact.name}`, "crate");
    next = next.replace(/crate::artifacts::([a-z0-9_]+)/g, (_, dependency: string) => `semio_s_artifact_stdio_${dependency}`);
    if (next !== current) await Bun.write(path, next);
  }
}

console.log(JSON.stringify(summary, null, 2));
