import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!, output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
const data = JSON.parse(readFileSync(join(output, "full-suite-nx/project-graph.json"), "utf8")), graph = data.graph ?? data;
const launch = Bun.JSONC.parse(readFileSync(join(root, ".vscode/🧩️launch.seed.jsonc"), "utf8"));
const commands = new Set(launch.configurations.map((row: any) => row?.command));
const missing = [];
for (const node of Object.values(graph.nodes) as any[]) for (const name of ["build", "check", "test"]) {
  const target = node.data.targets?.[name];
  if (!target?.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")) continue;
  const command = `bun nx run ${node.name}:${name}`;
  if (!commands.has(command)) missing.push({ project: node.name, root: node.data.root, target: name, command });
}
writeFileSync(join(output, "missing-native-launchers.json"), JSON.stringify(missing, null, 2));
console.log("[DEBUG] Missing native launchers " + JSON.stringify(missing));
