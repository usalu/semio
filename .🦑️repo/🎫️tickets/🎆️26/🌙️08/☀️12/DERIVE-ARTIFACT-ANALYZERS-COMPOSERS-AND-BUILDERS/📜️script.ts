import { existsSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🔖️Paths
const root = "/Users/ueli/Documents/semio";
const plugins = join(root, "✏️s/🔌️plugins");

function files(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    return entry.isDirectory() ? files(path) : [path];
  });
}

function read(path: string): string {
  return readFileSync(path, "utf8");
}

function write(path: string, content: string): void {
  writeFileSync(path, content.endsWith("\n") ? content : `${content}\n`);
}
//#endregion 🔖️Paths

//#region 🔖️Records
type Subset = {
  directory: string;
  schema: string;
  io: string;
  builder: string;
  analyzer: string;
  composer: string;
  snapshot: string;
  mutation: string;
  diff: string;
  builderName: string;
  analyzerName: string;
  composerName: string;
};

function capture(source: string, expression: RegExp, label: string, path: string): string {
  const value = source.match(expression)?.[1]?.trim();
  if (!value) throw new Error(`${label} not found in ${path}`);
  return value;
}

function subsets(): Subset[] {
  return files(plugins)
    .filter((path) => path.includes("/🗿️artifacts/") && path.endsWith("/🏗️builder/🦀️component.rs") && path.includes("/🪆️subsets/"))
    .map((builder) => {
      const directory = builder.slice(0, -"/🏗️builder/🦀️component.rs".length);
      const analyzer = join(directory, "🧐️analyzer/🦀️component.rs");
      const composer = join(directory, "🎹️composer/🦀️component.rs");
      const schema = join(directory, "🧬️schema/🦀️component.rs");
      const io = join(directory, "🚪️io/🦀️component.rs");
      for (const path of [analyzer, composer, schema, io]) if (!statSync(path).isFile()) throw new Error(`missing ${path}`);
      const builderSource = read(builder);
      const analyzerSource = read(analyzer);
      const composerSource = read(composer);
      return {
        directory,
        schema,
        io,
        builder,
        analyzer,
        composer,
        snapshot: capture(builderSource, /type\s+Snapshot\s*=\s*([^;]+);/, "snapshot", builder),
        mutation: capture(builderSource, /type\s+Mutation\s*=\s*([^;]+);/, "mutation", builder),
        diff: capture(builderSource, /type\s+Diff\s*=\s*([^;]+);/, "diff", builder),
        builderName: capture(builderSource, /impl\s+ArtifactBuilder\s+for\s+(\w+)/, "builder", builder),
        analyzerName: capture(analyzerSource, /impl\s+ArtifactAnalyzer\s+for\s+(\w+)/, "analyzer", analyzer),
        composerName: capture(composerSource, /impl\s+ArtifactComposer\s+for\s+(\w+)/, "composer", composer),
      };
    });
}
//#endregion 🔖️Records

//#region 🔖️Transform
function stripInnerDocs(source: string): string {
  return source.split("\n").filter((line) => !line.startsWith("//!")).join("\n").trim();
}

function replaceIdentifier(source: string, from: string, to: string): string {
  return source.replace(new RegExp(`\\b${from}\\b`, "g"), to);
}

function indent(source: string): string {
  return source.split("\n").map((line) => line ? `    ${line}` : "").join("\n");
}

function moduleRegion(name: string, moduleName: string, source: string): string {
  return `\n//#region ${name}\npub mod ${moduleName} {\n${indent(source)}\n}\npub use ${moduleName}::*;\n//#endregion ${name}\n`;
}

function hook(source: string, oldTrait: string, newTrait: string, oldName: string, newName: string): string {
  return replaceIdentifier(replaceIdentifier(stripInnerDocs(source), oldTrait, newTrait), oldName, newName);
}

function deriveBlock(subset: Subset): string {
  const spec = `${subset.builderName}Facets`;
  const construction = `${subset.builderName}Construction`;
  const analysis = `${subset.analyzerName}Analysis`;
  const composition = `${subset.composerName}Composition`;
  const io = existsSync(join(subset.directory, "🧬️schema/📸️snapshot/🦀️component.rs")) ? "super::super::io" : "super::io";
  return `\n//#region 🧬️DerivedArtifactFacets\nsemio_framework_plugin::derive_artifact_facets!(\n    pub spec ${spec} {\n        construction: derived_construction::${construction},\n        analysis: derived_analysis::${analysis},\n        composition: ${io}::derived_composition::${composition},\n    }\n    builder: ${subset.builderName},\n    analyzer: ${subset.analyzerName},\n    composer: ${subset.composerName},\n);\n//#endregion 🧬️DerivedArtifactFacets\n`;
}

function rustModuleMaps(allFiles: string[]): Map<string, string> {
  const maps = new Map<string, string>();
  for (const path of allFiles.filter((candidate) => candidate.endsWith("🦀️component.rs") && /\/(🏗️builder|🧐️analyzer)\//.test(candidate) && candidate.includes("/🗿️artifacts/") && !candidate.includes("/🪆️subsets/"))) {
    const capability = path.includes("/🏗️builder/") ? "builder" : "analyzer";
    const source = read(path);
    const target = source.match(new RegExp(`(crate::artifacts::\\w+(?:::\\w+)*::${capability})(?=::)`))?.[1];
    if (!target) throw new Error(`delegate target not found in ${path}`);
    const ownBase = path.includes("🏅️standards")
      ? target.match(/^(crate::artifacts::\w+::standards::\w+)/)?.[1]
      : target.match(/^(crate::artifacts::\w+)/)?.[1];
    if (!ownBase) throw new Error(`module base not found in ${path}`);
    maps.set(`${ownBase}::${capability}`, target);
  }
  return maps;
}

function rewriteRustPaths(source: string, maps: Map<string, string>): string {
  let rewritten = source;
  for (let pass = 0; pass < 4; pass += 1) {
    const previous = rewritten;
    for (const [from, to] of maps) rewritten = rewritten.replaceAll(from, to);
    if (rewritten === previous) break;
  }
  return rewritten
    .replace(/(crate::artifacts::\w+::standards::\w+::subsets::\w+)::(?:builder|analyzer|composer)/g, "$1::schema")
    .replace(/(::subsets::\w+)::schema::(\w+Validator|register)/g, "$1::io::$2");
}

function relocateRegistries(allFiles: string[], maps: Map<string, string>, apply: boolean): number {
  const composers = allFiles.filter((path) => path.endsWith("🎹️composer/🦀️component.rs") && path.includes("/🗿️artifacts/") && !path.includes("/🪆️subsets/"));
  for (const composer of composers) {
    const source = read(composer);
    const standardBase = source.match(/(crate::artifacts::\w+::standards::\w+)::(?:subsets|composer)/)?.[1];
    const artifactBase = source.match(/(crate::artifacts::\w+)::standards/)?.[1];
    const standard = composer.includes("/🏅️standards/");
    const base = standard ? standardBase : artifactBase;
    if (!base) throw new Error(`composer module base not found in ${composer}`);
    const target = standard
      ? composer.replace(/\/🎹️composer\/🦀️component\.rs$/, "/⚙️engine/🦀️component.rs")
      : composer.replace(/\/🎹️composer\/🦀️component\.rs$/, "/🦀️component.rs");
    if (!statSync(target).isFile()) throw new Error(`registry target missing ${target}`);
    maps.set(`${base}::composer`, standard ? `${base}::engine::io_registry` : `${base}::io_registry`);
    const registry = `\n//#region 🚪️DerivedIoRegistry\npub mod io_registry {\n${indent(stripInnerDocs(source))}\n}\n//#endregion 🚪️DerivedIoRegistry\n`;
    if (apply) write(target, `${read(target).trimEnd()}${registry}`);
  }
  return composers.length;
}

function capabilityDirectories(allFiles: string[]): Set<string> {
  const directories = new Set<string>();
  for (const path of allFiles.filter((candidate) => candidate.includes("/🗿️artifacts/"))) {
    const segments = path.split("/");
    const artifactIndex = segments.indexOf("🗿️artifacts");
    for (let index = artifactIndex + 1; index < segments.length; index += 1) {
      if (["🏗️builder", "🧐️analyzer", "🎹️composer"].includes(segments[index])) directories.add(segments.slice(0, index + 1).join("/"));
    }
  }
  return directories;
}

function removeGlueModules(source: string): string {
  return source.replace(/^[ \t]*#\[path = "[^"]*\/(?:🏗️builder|🧐️analyzer|🎹️composer)\/🦀️component\.rs"\]\n[ \t]*(?:pub )?mod \w+;\n/gm, "");
}
//#endregion 🔖️Transform

//#region 🔖️Main
const records = subsets();
const pluginFiles = files(plugins);
if (process.argv.includes("--check")) {
  const explicitDirectories = capabilityDirectories(pluginFiles);
  console.log(JSON.stringify({ subsets: records.length, explicitDirectories: explicitDirectories.size }, null, 2));
  process.exit(0);
}

const moduleMaps = rustModuleMaps(pluginFiles);
const registries = relocateRegistries(pluginFiles, moduleMaps, !process.argv.includes("--preflight"));
if (process.argv.includes("--preflight")) {
  console.log(JSON.stringify({ subsets: records.length, registries, moduleMaps: moduleMaps.size }, null, 2));
  process.exit(0);
}

for (const subset of records) {
  const constructionName = `${subset.builderName}Construction`;
  const analysisName = `${subset.analyzerName}Analysis`;
  const compositionName = `${subset.composerName}Composition`;
  const construction = hook(read(subset.builder), "ArtifactBuilder", "ArtifactBuilder", subset.builderName, constructionName);
  const analysis = hook(read(subset.analyzer), "ArtifactAnalyzer", "ArtifactAnalysis", subset.analyzerName, analysisName);
  const composition = hook(read(subset.composer), "ArtifactComposer", "ArtifactComposition", subset.composerName, compositionName).replaceAll("super::io::", "super::super::");
  write(subset.schema, `${read(subset.schema).trimEnd()}${moduleRegion("🏗️DerivedConstruction", "derived_construction", construction)}${moduleRegion("🧐️DerivedAnalysis", "derived_analysis", analysis)}${deriveBlock(subset)}`);
  write(subset.io, `${read(subset.io).trimEnd()}${moduleRegion("🎹️DerivedComposition", "derived_composition", composition)}`);
}

for (const path of files(plugins)) {
  if (path.endsWith(".rs")) write(path, rewriteRustPaths(removeGlueModules(read(path)), moduleMaps));
  if (path.endsWith(".ts") && !/\/(🏗️builder|🧐️analyzer|🎹️composer)\//.test(path)) {
    write(path, read(path).split("\n").filter((line) => !/\/(🏗️builder|🧐️analyzer|🎹️composer)\//.test(line)).join("\n"));
  }
}

const explicitDirectories = capabilityDirectories(files(plugins));
for (const path of explicitDirectories) {
  if (!path.includes("/🗿️artifacts/") || !/[\/](🏗️builder|🧐️analyzer|🎹️composer)$/.test(path)) throw new Error(`unsafe removal target ${path}`);
  rmSync(path, { recursive: true });
}

console.log(JSON.stringify({ migrated: records.length, removed: explicitDirectories.size, moduleMaps: moduleMaps.size }, null, 2));
//#endregion 🔖️Main
