import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const ticket = dirname(import.meta.dir);
const cwd = "${workspaceFolder}/" + relative(root, join(ticket, "📥️isolated-verification")).replaceAll("\\", "/");
const profiles = [
  ["🧫️fixtures-testing-only-root-service-current🧪️", "root-service-current", 900.728],
  ["🧫️fixtures-testing-only-root-dev-mirror-source🧪️", "root-dev-mirror-source", 900.725],
  ["🧫️fixtures-testing-only-root-dev-mirror-process🧪️", "root-dev-mirror-process", 900.726],
  ["🧫️fixtures-testing-only-root-service-oracles🧪️", "root-service-oracles", 900.727],
  ["🧫️fixtures-testing-only-root-current-corpus-tests🧪️", "root-current-corpus-tests", 900.716],
  ["🧫️fixtures-testing-only-plugin-value-ghost-closure🧪️", "plugin-value-ghost-closure", 900.714],
  ["🧫️fixtures-testing-only-plugin-value-ghost-proof🧪️", "plugin-value-ghost-proof", 900.715],
  ["🧫️fixtures-testing-only-plugin-table-ghost-closure🧪️", "plugin-table-ghost-closure", 900.717],
  ["🧫️fixtures-testing-only-plugin-table-ghost-proof🧪️", "plugin-table-ghost-proof", 900.718],
  ["🧫️fixtures-testing-only-plugin-concurrent-publication-doc-observation🧪️", "plugin-concurrent-publication-doc-observation", 900.719],
  ["🧫️fixtures-testing-only-root-job-ghost-closure🧪️", "root-job-ghost-closure", 900.720],
  ["🧫️fixtures-testing-only-root-job-ghost-ledger🧪️", "root-job-ghost-ledger", 900.721],
  ["🧫️fixtures-testing-only-root-job-fixed-operation-proof🧪️", "root-job-fixed-operation-proof", 900.722],
  ["🧫️fixtures-testing-only-root-job-shared-action-proof🧪️", "root-job-shared-action-proof", 900.723],
  ["🧫️fixtures-testing-only-root-job-shared-action-native🧪️", "root-job-shared-action-native", 900.724],
  ["🧫️fixtures-testing-only-kernel-canonical-native🧪️", "kernel-native", 900.58],
  ["🧫️fixtures-testing-only-stdio-production-preparation🧪️", "stdio-production", 900.581],
  ["🧫️fixtures-testing-only-print-measurement🧪️", "print-measurement", 900.582],
  ["🧫️fixtures-testing-only-cad-geometry-source🧪️", "cad-geometry-source", 900.583],
  ["🧫️fixtures-testing-only-hub-broker-lifecycle🧪️", "hub-broker", 900.584],
  ["🧫️fixtures-testing-only-hub-creation-authority🧪️", "hub-creation", 900.585],
  ["🧫️fixtures-testing-only-repository-suite🧪️", "repo-test-long", 900.586],
  ["🧫️fixtures-testing-only-plugin-current-plan🧪️", "plugin-current-plan", 900.587],
  ["🧫️fixtures-testing-only-plugin-current-apply🧪️", "plugin-current-apply", 900.588],
  ["🧫️fixtures-testing-only-plugin-current-integrity🧪️", "plugin-current-integrity", 900.5889999999999],
  ["🧫️fixtures-testing-only-plugin-current-verify🧪️", "plugin-current-verify", 900.59],
  ["🧫️fixtures-testing-only-plugin-current-refs🧪️", "plugin-current-refs", 900.591],
  ["🧫️fixtures-testing-only-dev-corpus🧪️", "dev-corpus", 900.592],
  ["🧫️fixtures-testing-only-pack-native🧪️", "pack-native", 900.593],
  ["🧫️fixtures-testing-only-gltf-twin🧪️", "gltf-twin", 900.595],
  ["🧫️fixtures-testing-only-gltf-native🧪️", "gltf-native", 900.596],
  ["🧫️fixtures-testing-only-en1997-native🧪️", "en1997-native", 900.597],
  ["🧫️fixtures-testing-only-print-current-plan🧪️", "print-current-plan", 900.6],
  ["🧫️fixtures-testing-only-print-current-apply🧪️", "print-current-apply", 900.601],
  ["🧫️fixtures-testing-only-print-current-integrity🧪️", "print-current-integrity", 900.602],
  ["🧫️fixtures-testing-only-print-current-behavior🧪️", "print-current-behavior", 900.6030000000001],
  ["🧫️fixtures-testing-only-print-current-verify🧪️", "print-current-verify", 900.604],
  ["🧫️fixtures-testing-only-distribution-generate🧪️", "distribution-generate", 900.605],
  ["🧫️fixtures-testing-only-print-current-followup🧪️", "print-current-followup", 900.606],
  ["🧫️fixtures-testing-only-print-current-office🧪️", "print-current-office", 900.607],
  ["🧫️fixtures-testing-only-print-current-native🧪️", "print-current-native", 900.608],
  ["🧫️fixtures-testing-only-print-current-inference🧪️", "print-current-inference", 900.609],
  ["🧫️fixtures-testing-only-geometry-current-apply🧪️", "geometry-current-apply", 900.61],
  ["🧫️fixtures-testing-only-geometry-current-integrity🧪️", "geometry-current-integrity", 900.611],
  ["🧫️fixtures-testing-only-geometry-current-verify🧪️", "geometry-current-verify", 900.612],
  ["🧫️fixtures-testing-only-ui-corpus🧪️", "ui-corpus", 900.613],
  ["🧫️fixtures-testing-only-print-current-prepare-tex🧪️", "print-current-prepare-tex", 900.614],
  ["🧫️fixtures-testing-only-print-current-prepare-toolchain🧪️", "print-current-prepare-toolchain", 900.615],
  ["🧫️fixtures-testing-only-source-current-apply🧪️", "source-current-apply", 900.616],
  ["🧫️fixtures-testing-only-source-current-verify🧪️", "source-current-verify", 900.617],
  ["🧫️fixtures-testing-only-boolean-current-verify🧪️", "boolean-current-verify", 900.6179999999999],
  ["🧫️fixtures-testing-only-core-corpus🧪️", "core-corpus", 900.619],
  ["🧫️fixtures-testing-only-dev-latency🧪️", "dev-latency", 900.62],
  ["🧫️fixtures-testing-only-hub-core-corpus🧪️", "hub-core-corpus", 900.621],
  ["🧫️fixtures-testing-only-pptx-current-apply🧪️", "pptx-current-apply", 900.622],
  ["🧫️fixtures-testing-only-pptx-current-verify🧪️", "pptx-current-verify", 900.6229999999999],
  ["🧫️fixtures-testing-only-hub-puzzle-current-repin🧪️", "hub-puzzle-current-repin", 900.6239999999999],
  ["🧫️fixtures-testing-only-plugin-value-policy-current-apply🧪️", "plugin-value-policy-current-apply", 900.625],
  ["🧫️fixtures-testing-only-plugin-value-policy-current-verify🧪️", "plugin-value-policy-current-verify", 900.626],
  ["🧫️fixtures-testing-only-plugin-value-policy-current-native🧪️", "plugin-value-policy-current-native", 900.627],
  ["🧫️fixtures-testing-only-register-current-profiles🧪️", "register-current-profiles", 900.6279999999999],
  ["🧫️fixtures-testing-only-ui-layered🧪️", "ui-layered", 900.629],
  ["🧫️fixtures-testing-only-empty-facet🧪️", "empty-facet", 900.63],
  ["🧫️fixtures-testing-only-distribution-session🧪️", "distribution-session", 900.631],
  ["🧫️fixtures-testing-only-embedded-current-plan🧪️", "embedded-current-plan", 900.632],
  ["🧫️fixtures-testing-only-embedded-current-apply🧪️", "embedded-current-apply", 900.6329999999999],
  ["🧫️fixtures-testing-only-embedded-current-integrity🧪️", "embedded-current-integrity", 900.6339999999999],
  ["🧫️fixtures-testing-only-embedded-current-verify🧪️", "embedded-current-verify", 900.635],
  ["🧫️fixtures-testing-only-embedded-current-native🧪️", "embedded-current-native", 900.636],
  ["🧫️fixtures-testing-only-embedded-current-directory🧪️", "embedded-current-directory", 900.637],
  ["🧫️fixtures-testing-only-parser-input-oracles🧪️", "parser-input-oracles", 900.65],
  ["🧫️fixtures-testing-only-embedded-current-followup🧪️", "embedded-current-followup", 900.638],
  ["🧫️fixtures-testing-only-embedded-current-followup-integrity🧪️", "embedded-current-followup-integrity", 900.639],
  ["🧫️fixtures-testing-only-embedded-current-output🧪️", "embedded-current-output", 900.640],
  ["🧫️fixtures-testing-only-embedded-current-directory-native🧪️", "embedded-current-directory-native", 900.641],
  ["🧫️fixtures-testing-only-embedded-current-route🧪️", "embedded-current-route", 900.642],
  ["🧫️fixtures-testing-only-embedded-current-preflight-owner🧪️", "embedded-current-preflight-owner", 900.643],
  ["🧫️fixtures-testing-only-runtime-stale-consumers🧪️", "runtime-stale-consumers", 900.651],
  ["🧫️fixtures-testing-only-runtime-child-worker🧪️", "runtime-child-worker", 900.652],
  ["🧫️fixtures-testing-only-parser-python🧪️", "parser-python", 900.66],
  ["🧫️fixtures-testing-only-draft07-oracle🧪️", "draft07-oracle", 900.661],
  ["🧫️fixtures-testing-only-camera-corpus🧪️", "camera-corpus", 900.6619999999999],
  ["🧫️fixtures-testing-only-hub-final-corpus🧪️", "hub-final-corpus", 900.663],
  ["🧫️fixtures-testing-only-hub-memory-oracle🧪️", "hub-memory-oracle", 900.664],
  ["🧫️fixtures-testing-only-storage-frontiers🧪️", "storage-frontiers", 900.665],
  ["🧫️fixtures-testing-only-parser-input-tests🧪️", "parser-input-tests", 900.6659999999999],
  ["🧫️fixtures-testing-only-registry-rebuild-current🧪️", "registry-rebuild-current", 900.644],
  ["🧫️fixtures-testing-only-runtime-parser-native-manifest🧪️", "runtime-parser-native-manifest", 900.653],
  ["🧫️fixtures-testing-only-runtime-parser-native-schema🧪️", "runtime-parser-native-schema", 900.654],
  ["🧫️fixtures-testing-only-runtime-parser-native-stdio🧪️", "runtime-parser-native-stdio", 900.655],
  ["🧫️fixtures-testing-only-root-collision-oracle🧪️", "root-collision-oracle", 900.667],
  ["🧫️fixtures-testing-only-runtime-closure-ingress🧪️", "runtime-closure-ingress", 900.656],
  ["🧫️fixtures-testing-only-runtime-new-authority🧪️", "runtime-new-authority", 900.6569999999999],
  ["🧫️fixtures-testing-only-runtime-channel-wire🧪️", "runtime-channel-wire", 900.6579999999999],
  ["🧫️fixtures-testing-only-root-lexical-oracles🧪️", "root-lexical-oracles", 900.668],
  ["🧫️fixtures-testing-only-runtime-batch59🧪️", "runtime-batch59", 900.669],
  ["🧫️fixtures-testing-only-plugin-admission-current-plan🧪️", "plugin-admission-current-plan", 900.68],
  ["🧫️fixtures-testing-only-plugin-admission-current-apply🧪️", "plugin-admission-current-apply", 900.6809999999999],
  ["🧫️fixtures-testing-only-plugin-admission-current-integrity🧪️", "plugin-admission-current-integrity", 900.6819999999999],
  ["🧫️fixtures-testing-only-plugin-admission-current-verify🧪️", "plugin-admission-current-verify", 900.683],
  ["🧫️fixtures-testing-only-plugin-admission-current-actor🧪️", "plugin-admission-current-actor", 900.684],
  ["🧫️fixtures-testing-only-plugin-admission-current-followup🧪️", "plugin-admission-current-followup", 900.685],
  ["🧫️fixtures-testing-only-plugin-admission-current-proofs🧪️", "plugin-admission-current-proofs", 900.686],
  ["🧫️fixtures-testing-only-plugin-admission-current-frontier🧪️", "plugin-admission-current-frontier", 900.687],
  ["🧫️fixtures-testing-only-plugin-admission-current-oracles🧪️", "plugin-admission-current-oracles", 900.688],
  ["🧫️fixtures-testing-only-plugin-admission-current-diagnostics🧪️", "plugin-admission-current-diagnostics", 900.689],
  ["🧫️fixtures-testing-only-plugin-admission-current-oracle-final🧪️", "plugin-admission-current-oracle-final", 900.690],
  ["🧫️fixtures-testing-only-plugin-admission-current-behavior🧪️", "plugin-admission-current-behavior", 900.691],
  ["🧫️fixtures-testing-only-root-story-inspect🧪️", "root-story-inspect", 900.69],
  ["🧫️fixtures-testing-only-root-node-oracles🧪️", "root-node-oracles", 900.691],
  ["🧫️fixtures-testing-only-plugin-admission-current-publication-final🧪️", "plugin-admission-current-publication-final", 900.692],
  ["🧫️fixtures-testing-only-plugin-admission-current-welcome🧪️", "plugin-admission-current-welcome", 900.693],
  ["🧫️fixtures-testing-only-plugin-admission-current-owner-final🧪️", "plugin-admission-current-owner-final", 900.694],
  ["🧫️fixtures-testing-only-root-composition-ownership🧪️", "root-composition-ownership", 900.692],
  ["🧫️fixtures-testing-only-plugin-admission-current-ledger🧪️", "plugin-admission-current-ledger", 900.695],
  ["🧫️fixtures-testing-only-plugin-admission-current-frontier-final🧪️", "plugin-admission-current-frontier-final", 900.696],
  ["🧫️fixtures-testing-only-plugin-pack-intrinsic-native🧪️", "plugin-pack-intrinsic-native", 900.697],
  ["🧫️fixtures-testing-only-plugin-world-capacity-native🧪️", "plugin-world-capacity-native", 900.698],
  ["🧫️fixtures-testing-only-plugin-renderer-owner-final🧪️", "plugin-renderer-owner-final", 900.699],
  ["🧫️fixtures-testing-only-registry-rebuild-current-resume🧪️", "registry-rebuild-current-resume", 900.700],
  ["🧫️fixtures-testing-only-plugin-final-ghosts🧪️", "plugin-final-ghosts", 900.701],
  ["🧫️fixtures-testing-only-plugin-shell-proof🧪️", "plugin-shell-proof", 900.702],
  ["🧫️fixtures-testing-only-plugin-reference-inputs🧪️", "plugin-reference-inputs", 900.703],
  ["🧫️fixtures-testing-only-plugin-reference-proof🧪️", "plugin-reference-proof", 900.704],
  ["🧫️fixtures-testing-only-plugin-reference-owners🧪️", "plugin-reference-owners", 900.705],
  ["🧫️fixtures-testing-only-plugin-completed-output-receipts🧪️", "plugin-completed-output-receipts", 900.706],
  ["🧫️fixtures-testing-only-root-schema-generate🧪️", "root-schema-generate", 900.693],
  ["🧫️fixtures-testing-only-root-schema-check🧪️", "root-schema-check", 900.694],
  ["🧫️fixtures-testing-only-root-schema-docs🧪️", "root-schema-docs", 900.695],
  ["🧫️fixtures-testing-only-runtime-member-open-native🧪️", "runtime-member-open-native", 900.696],
  ["🧫️fixtures-testing-only-root-asset-dedup🧪️", "root-asset-dedup", 900.697],
  ["🧫️fixtures-testing-only-plugin-energy-admission-closure🧪️", "plugin-energy-admission-closure", 900.707],
  ["🧫️fixtures-testing-only-plugin-energy-sqlite-proof🧪️", "plugin-energy-sqlite-proof", 900.708],
  ["🧫️fixtures-testing-only-plugin-energy-extent-native🧪️", "plugin-energy-extent-native", 900.709],
  ["🧫️fixtures-testing-only-runtime-dsl-producer-native🧪️", "runtime-dsl-producer-native", 900.710],
  ["🧫️fixtures-testing-only-root-store-payload-tests🧪️", "root-store-payload-tests", 900.711],
  ["🧫️fixtures-testing-only-root-store-replay-oracles🧪️", "root-store-replay-oracles", 900.712],
  ["🧫️fixtures-testing-only-root-store-scalar-wide🧪️", "root-store-scalar-wide", 900.713],
  ["🧫️fixtures-testing-only-plugin-component-ghost-closure🧪️", "plugin-component-ghost-closure", 900.710],
  ["🧫️fixtures-testing-only-plugin-component-ghost-proof🧪️", "plugin-component-ghost-proof", 900.711],
  ["🧫️fixtures-testing-only-plugin-component-inputs🧪️", "plugin-component-inputs", 900.712],
  ["🧫️fixtures-testing-only-plugin-concurrent-ledger-observation🧪️", "plugin-concurrent-ledger-observation", 900.713],
  ["🧫️fixtures-testing-only-runtime-clamp-current🧪️", "runtime-clamp-current", 900.711],
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

const livePath = join(root, ".vscode/launch.json"), live = ts.parseJsonText(livePath, readFileSync(livePath, "utf8"));
const liveObject = (live.statements[0] as ts.ExpressionStatement).expression as ts.ObjectLiteralExpression;
const liveConfigurations = (liveObject.properties.find(property => ts.isPropertyAssignment(property) && property.name.getText(live) === '"configurations"') as ts.PropertyAssignment).initializer as ts.ArrayLiteralExpression;
const routeName = "⚖️test-command-composition-source🦑️repo🔨️modules🧪️test";
const route = liveConfigurations.elements.filter(ts.isObjectLiteralExpression).find(item => item.properties.some(property => ts.isPropertyAssignment(property) && property.name.getText(live) === '"name"' && JSON.parse(property.initializer.getText(live)) === routeName));
if (!route) throw Error("Actual command-composition launch declaration missing");
const seedPath = join(root, ".vscode/🧩️launch.seed.jsonc"), seedSource = readFileSync(seedPath, "utf8"), seed = ts.parseJsonText(seedPath, seedSource);
const seedObject = (seed.statements[0] as ts.ExpressionStatement).expression as ts.ObjectLiteralExpression;
const seedConfigurations = (seedObject.properties.find(property => ts.isPropertyAssignment(property) && property.name.getText(seed) === '"configurations"') as ts.PropertyAssignment).initializer as ts.ArrayLiteralExpression;
if (!seedConfigurations.elements.filter(ts.isObjectLiteralExpression).some(item => item.properties.some(property => ts.isPropertyAssignment(property) && property.name.getText(seed) === '"name"' && JSON.parse(property.initializer.getText(seed)) === routeName))) {
  const at = seedConfigurations.elements[0]!.getStart(seed), declaration = route.getText(live);
  if (readFileSync(seedPath, "utf8") !== seedSource) throw Error("Concurrent launch seed edit; retry registration");
  writeFileSync(seedPath, seedSource.slice(0, at) + declaration + ",\n    " + seedSource.slice(at));
  console.log("[DEBUG] actual command-composition route restored to launch seed");
}

const fixtureBoundaryProfile = {
  name: "⚖️schema-fixture-boundary", type: "node-terminal", request: "launch",
  command: "bun nx run workspace:schema-fixture-boundary", cwd: "${workspaceFolder}",
  presentation: { group: "4_gate", order: 900.00881 },
};
for (const name of ["launch.json", "🧩️launch.seed.jsonc"]) {
  const path = join(root, ".vscode", name), source = readFileSync(path, "utf8"), parsed = ts.parseJsonText(path, source);
  const object = (parsed.statements[0] as ts.ExpressionStatement).expression as ts.ObjectLiteralExpression;
  const configurations = (object.properties.find(property => ts.isPropertyAssignment(property) && property.name.getText(parsed) === '"configurations"') as ts.PropertyAssignment).initializer as ts.ArrayLiteralExpression;
  if (configurations.elements.filter(ts.isObjectLiteralExpression).some(item => item.properties.some(property => ts.isPropertyAssignment(property) && property.name.getText(parsed) === '"name"' && JSON.parse(property.initializer.getText(parsed)) === fixtureBoundaryProfile.name))) continue;
  const anchor = configurations.elements.filter(ts.isObjectLiteralExpression).find(item => item.properties.some(property => ts.isPropertyAssignment(property) && property.name.getText(parsed) === '"name"' && JSON.parse(property.initializer.getText(parsed)) === "⚖️schema-check"));
  const at = anchor?.getStart(parsed) ?? configurations.elements[0]!.getStart(parsed);
  if (readFileSync(path, "utf8") !== source) throw Error("Concurrent schema gate launch edit; retry registration");
  writeFileSync(path, source.slice(0, at) + JSON.stringify(fixtureBoundaryProfile, null, 2) + ",\n    " + source.slice(at));
  console.log("[DEBUG] permanent fixture boundary gate registered " + name);
}
