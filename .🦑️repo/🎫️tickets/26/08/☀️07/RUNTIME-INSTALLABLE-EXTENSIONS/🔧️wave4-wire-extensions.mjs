import fs from "node:fs";
import path from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const PLUGINS = path.join(ROOT, "✏️s/🔌️plugins");

function walkExtensions(dir, out = []) {
  for (const name of fs.readdirSync(dir, { withFileTypes: true })) {
    if (!name.isDirectory() || name.name.startsWith(".")) continue;
    const p = path.join(dir, name.name);
    if (name.name.includes("extensions")) {
      for (const child of fs.readdirSync(p, { withFileTypes: true })) {
        if (!child.isDirectory()) continue;
        const rust = path.join(p, child.name, "📦️packages/🦀️rust");
        if (fs.existsSync(path.join(rust, "Cargo.toml"))) out.push(rust);
      }
      continue;
    }
    walkExtensions(p, out);
  }
  return out;
}

function repoLibRel(fromRustDir) {
  const rel = path.relative(fromRustDir, path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts"));
  return rel.split(path.sep).join("/");
}

function patchScript(rustDir) {
  const scriptPath = path.join(rustDir, "📜️script.ts");
  if (!fs.existsSync(scriptPath)) return { rustDir, skipped: "no script" };
  let text = fs.readFileSync(scriptPath, "utf8");
  const cargo = fs.readFileSync(path.join(rustDir, "Cargo.toml"), "utf8");
  const packageName = cargo.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
  if (!packageName) throw new Error(`no package name in ${rustDir}`);

  const libImport = repoLibRel(rustDir);
  if (!text.includes("runExtensionComponentPackage")) {
    text = text.replace(
      /import \{([^}]+)\} from "([^"]+)";/,
      (m, imports, from) => {
        if (!from.includes("repo") || !from.includes("lib")) return m;
        const names = imports.split(",").map((s) => s.trim());
        if (!names.includes("runExtensionComponentPackage")) names.push("runExtensionComponentPackage");
        return `import { ${names.join(", ")} } from "${from}";`;
      },
    );
    if (!text.includes("class PackageScript")) {
      const testEnd = text.indexOf("const router");
      const insert = `
class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runExtensionComponentPackage({ rsDir: this.root, repoRoot: this.repoRoot, outPath: segments[0] });
  }
}

`;
      text = text.slice(0, testEnd) + insert + text.slice(testEnd);
    }
    text = text.replace(
      /\.register\("test", TestScript\)/,
      '.register("test", TestScript).register("package", PackageScript)',
    );
    fs.writeFileSync(scriptPath, text);
  }

  return { rustDir, packageName, script: true };
}

function patchProject(rustDir) {
  const projectPath = path.join(rustDir, "📋️project.json");
  if (!fs.existsSync(projectPath)) return { project: false };
  const project = JSON.parse(fs.readFileSync(projectPath, "utf8"));
  const cwd = path.relative(ROOT, rustDir);
  if (!project.targets) project.targets = {};
  if (!project.targets.package) {
    project.targets.package = {
      executor: "nx:run-commands",
      options: {
        cwd,
        command: "bun ./📜️script.ts package",
        forwardAllArgs: true,
      },
    };
    fs.writeFileSync(projectPath, `${JSON.stringify(project, null, 2)}\n`);
  }
  return { project: true };
}

const rustDirs = walkExtensions(PLUGINS).sort();
const log = [];
for (const rustDir of rustDirs) {
  log.push({ ...patchScript(rustDir), ...patchProject(rustDir) });
}
fs.writeFileSync(path.join(ROOT, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS/wave4-wire-log.json"), JSON.stringify(log, null, 2));
console.log(`[DEBUG] wave4 wired ${rustDirs.length} extension rust packages`);
