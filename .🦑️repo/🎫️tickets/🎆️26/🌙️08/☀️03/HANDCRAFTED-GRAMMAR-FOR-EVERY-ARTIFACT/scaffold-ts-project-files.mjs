#!/usr/bin/env bun
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../../../../../");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const libIndexAbs = join(
  repoRoot,
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts",
);
const facetDirs = ["🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr"];
const tsLeaf = "🟦️component.ts";
const artifactsDirName = "🗿️artifacts";

function stripEmoji(segment) {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

function listDirs(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((name) => statSync(join(dir, name)).isDirectory());
}

function libImportFrom(pkgDir) {
  const rel = relative(pkgDir, libIndexAbs).replaceAll("\\", "/");
  return rel.startsWith(".") ? rel : `./${rel}`;
}

let createdScript = 0;
let createdProject = 0;
let rewrittenIndex = 0;
let fixedScriptImport = 0;

const scriptImportRe = /from\s+"([^"]+)";\s*$/m;

for (const plugin of listDirs(pluginsRoot)) {
  const pkg = join(pluginsRoot, plugin, "📦️packages/🟦️typescript");
  if (!existsSync(join(pkg, "package.json"))) continue;

  const pluginAscii = stripEmoji(plugin) || "plugin";
  const pluginRel = `✏️s/🔌️plugins/${plugin}`;
  const libImport = libImportFrom(pkg);

  const scriptPath = join(pkg, "📜️script.ts");
  const scriptBody = `#!/usr/bin/env bun
/** ${pluginAscii} TypeScript package */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "${libImport}";
class TestScript extends BundleScript {
  run(): void {
    console.log("[DEBUG] ${pluginAscii} ts facade package ok");
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
`;
  if (!existsSync(scriptPath)) {
    writeFileSync(scriptPath, scriptBody);
    createdScript++;
  } else {
    const priorScript = readFileSync(scriptPath, "utf8");
    const importMatch = priorScript.match(scriptImportRe);
    if (!importMatch || importMatch[1] !== libImport) {
      const updated = importMatch
        ? priorScript.replace(scriptImportRe, `from "${libImport}";`)
        : scriptBody;
      if (updated !== priorScript) {
        writeFileSync(scriptPath, updated.endsWith("\n") ? updated : `${updated}\n`);
        fixedScriptImport++;
      }
    }
  }

  const projectPath = join(pkg, "📋️project.json");
  if (!existsSync(projectPath)) {
    writeFileSync(
      projectPath,
      `${JSON.stringify(
        {
          name: `@semio-tech/${pluginAscii}-js`,
          $schema: "../../../../../node_modules/nx/schemas/project-schema.json",
          targets: {
            test: {
              executor: "nx:run-commands",
              options: {
                cwd: `${pluginRel}/📦️packages/🟦️typescript`,
                command: "bun ./📜️script.ts test",
                forwardAllArgs: true,
              },
            },
          },
        },
        null,
        2,
      )}\n`,
    );
    createdProject++;
  }

  const exports = [];
  const artifactsRoot = join(pluginsRoot, plugin, artifactsDirName);
  for (const artifact of listDirs(artifactsRoot)) {
    const artifactAscii = stripEmoji(artifact) || "artifact";
    for (const facet of facetDirs) {
      const componentAbs = join(artifactsRoot, artifact, facet, tsLeaf);
      if (!existsSync(componentAbs)) continue;
      const facetAscii = stripEmoji(facet) || "facet";
      const exportPath = `../../${artifactsDirName}/${artifact}/${facet}/${tsLeaf}`;
      exports.push(`export * as ${artifactAscii}_${facetAscii} from "${exportPath}";`);
    }
  }

  const indexPath = join(pkg, "📦️index.ts");
  const indexBody = `/** ${pluginAscii} facet WASM facades */\n${exports.join("\n")}\n`;
  const prior = existsSync(indexPath) ? readFileSync(indexPath, "utf8") : "";
  if (prior !== indexBody) {
    writeFileSync(indexPath, indexBody);
    rewrittenIndex++;
  }
}

const summary = {
  pluginsWithTsPackage: listDirs(pluginsRoot).filter((p) =>
    existsSync(join(pluginsRoot, p, "📦️packages/🟦️typescript/package.json")),
  ).length,
  createdScript,
  fixedScriptImport,
  createdProject,
  rewrittenIndex,
};
console.log(JSON.stringify(summary, null, 2));
