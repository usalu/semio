import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const FRAMEWORK = join(ROOT, "🧰️framework");
const MODULES = join(FRAMEWORK, "🔨️modules");
const PACKAGES = join(FRAMEWORK, "📦️packages");
const rustDir = readdirSync(PACKAGES).find((n) => n.includes("rust"));
const tsDir = readdirSync(PACKAGES).find((n) => n.includes("typescript"));
const RUST = join(PACKAGES, rustDir);
const PKG = join(PACKAGES, tsDir);
const manifestDir = readdirSync(MODULES).find((n) => n.includes("manifest"));
const emoji = manifestDir.replace("manifest", "");
const glueRsName = readdirSync(RUST).find((n) => n.includes("glue") && n.endsWith(".rs"));
const glueTsName = readdirSync(PKG).find((n) => n.includes("glue") && n.endsWith(".ts"));
const componentRs = "🦀️component.rs";

console.log({ rustDir, tsDir, manifestDir, glueRsName, glueTsName });

{
  const gluePath = join(RUST, glueRsName);
  let glue = readFileSync(gluePath, "utf8");
  glue = glue.replace(
    /#\[path = "[^"]*manifest\/🦀️component\.rs"\]/,
    `#[path = "../../🔨️modules/${manifestDir}/${componentRs}"]`,
  );
  glue = glue
    .split("\n")
    .map((l) => {
      if (l.includes("The declarative component model")) {
        return `// ${emoji} The declarative component model (layout/utilities/UiNode) lives in \`ui_wgpu\` now — re-import`;
      }
      return l;
    })
    .join("\n");
  writeFileSync(gluePath, glue);
  const line = glue.split("\n").find((l) => l.includes("manifest/") && l.includes("path"));
  console.log("glue path line", line);
  console.log("hex", Buffer.from(line).toString("hex"));
}

{
  const cargoPath = join(RUST, "Cargo.toml");
  let cargo = readFileSync(cargoPath, "utf8");
  cargo = cargo.replace('name = "semio-framework-core"', 'name = "semio-framework"');
  cargo = cargo.replace('id = "core"', 'id = "framework"');
  writeFileSync(cargoPath, cargo);
  console.log("cargo", cargo.split("\n").slice(0, 11).join(" | "));
}

{
  const glueTs = join(PKG, glueTsName);
  let t = readFileSync(glueTs, "utf8");
  t = t.replace(
    /import \{\n  createMemoryStoragePort,\n  emptyPaneState,\n  emptySkeleton,\n  emptyUiState,\n  DockLayoutStore,\n  DockUiStateStore,\n  WindowPaneStateStore,\n\} from ([^;]+);/,
    "import {\n  createMemoryStoragePort,\n  DockLayoutStore,\n  DockUiStateStore,\n  WindowPaneStateStore,\n} from $1;",
  );
  writeFileSync(glueTs, t);
  console.log("glue.ts has emptyPane?", t.includes("emptyPaneState"));
}
console.log("done");
