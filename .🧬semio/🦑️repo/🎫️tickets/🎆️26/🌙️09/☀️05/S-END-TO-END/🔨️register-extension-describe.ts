#!/usr/bin/env bun
/** 🧩️ One-shot registration of the shared extension `describe` route across every extension owner:
 * adds the router command to each `📜️script.ts` and the matching nx target to each `📋️project.json`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const owners = readFileSync(join(import.meta.dir, "🧩️extension-owners.txt"), "utf8").split("\n").filter(Boolean);
const describeClass = `class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describeExtensionComponent(this.repoRoot, import.meta.dir));
  }
}

`;
const changed: string[] = [];
for (const owner of owners) {
  const scriptPath = join(repoRoot, owner, "📜️script.ts");
  let script = readFileSync(scriptPath, "utf8");
  if (!script.includes("describeExtensionComponent")) {
    script = script.replace(/^(import \{[^}]*runExtensionComponentPackage \} from "[^"]+";\n)/mu, `$1import { describeExtensionComponent } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";\n`);
    script = script.replace("`bun ./📜️script.ts <test|package>`", "`bun ./📜️script.ts <test|package|describe>`");
    script = script.replace(/^const router = /mu, `${describeClass}const router = `);
    script = script.replace('.register("package", PackageScript);', '.register("package", PackageScript).register("describe", DescribeScript);');
    if (!script.includes("describeExtensionComponent") || !script.includes('.register("describe", DescribeScript)')) throw new Error(`unrecognised router shape in ${scriptPath}`);
    writeFileSync(scriptPath, script);
    changed.push(`${owner}/📜️script.ts`);
  }
  const projectPath = join(repoRoot, owner, "📋️project.json");
  const project = JSON.parse(readFileSync(projectPath, "utf8")) as { targets: Record<string, unknown> };
  if (!project.targets.describe) {
    if (!project.targets.package) throw new Error(`missing package target in ${projectPath}`);
    project.targets.describe = { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts describe", forwardAllArgs: true } };
    writeFileSync(projectPath, `${JSON.stringify(project, null, 2)}\n`);
    changed.push(`${owner}/📋️project.json`);
  }
}
console.log(`${changed.length} files updated across ${owners.length} extension owners`);
for (const file of changed) console.log(`  ${file}`);
