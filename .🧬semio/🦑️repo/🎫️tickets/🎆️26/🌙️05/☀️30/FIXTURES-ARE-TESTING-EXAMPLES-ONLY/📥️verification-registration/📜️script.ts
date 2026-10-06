import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const ticket = dirname(import.meta.dir);
const cwd = "${workspaceFolder}/" + relative(root, join(ticket, "📥️isolated-verification")).replaceAll("\\", "/");
const profiles = [
  ["🧫️fixtures-testing-only-kernel-canonical-native🧪️", "kernel-native", 900.58],
  ["🧫️fixtures-testing-only-stdio-production-preparation🧪️", "stdio-production", 900.581],
  ["🧫️fixtures-testing-only-print-measurement🧪️", "print-measurement", 900.582],
  ["🧫️fixtures-testing-only-cad-geometry-source🧪️", "cad-geometry-source", 900.583],
  ["🧫️fixtures-testing-only-hub-broker-lifecycle🧪️", "hub-broker", 900.584],
  ["🧫️fixtures-testing-only-hub-creation-authority🧪️", "hub-creation", 900.585],
  ["🧫️fixtures-testing-only-repository-suite🧪️", "repo-test-long", 900.586],
] as const;
for (const name of ["launch.json", "🧩️launch.seed.jsonc"]) {
  const path = join(root, ".vscode", name), source = readFileSync(path, "utf8");
  const ast = ts.parseJsonText(path, source);
  const object = (ast.statements[0] as ts.ExpressionStatement).expression as ts.ObjectLiteralExpression;
  const list = (object.properties.find((property) => ts.isPropertyAssignment(property) && property.name.getText(ast) === '"configurations"') as ts.PropertyAssignment).initializer as ts.ArrayLiteralExpression;
  const names = new Set(list.elements.filter(ts.isObjectLiteralExpression).flatMap((item) => item.properties.filter(ts.isPropertyAssignment).filter((property) => property.name.getText(ast) === '"name"').map((property) => JSON.parse(property.initializer.getText(ast)))));
  const added = profiles.filter(([name]) => !names.has(name)).map(([name, target, order]) => ({
    name, type: "node-terminal", request: "launch",
    command: `bun "\${workspaceFolder}/node_modules/nx/dist/bin/nx.js" run ticket-fixture-verification:${target} --skip-nx-cache --outputStyle=static`, cwd,
    env: { NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_WORKSPACE_DATA_DIRECTORY: cwd.replace("/📥️isolated-verification", "/🗑️generated/launch-nx-data"), SEMIO_TEST_ARTIFACT_DIR: cwd.replace("/📥️isolated-verification", "/🗑️generated"), FORCE_COLOR: "0" },
    presentation: { group: "4_gate", order },
  }));
  if (added.length) {
    const at = list.elements[0]!.getStart(ast);
    const insertion = added.map((item) => JSON.stringify(item, null, 2).split("\n").map((line, index) => index ? "    " + line : line).join("\n") + ",\n    ").join("");
    if (readFileSync(path, "utf8") !== source) throw Error("Concurrent launch edit; retry registration");
    writeFileSync(path, source.slice(0, at) + insertion + source.slice(at));
  }
  console.log(`[DEBUG] fixture verification profiles ${name} added=${added.length}`);
}
