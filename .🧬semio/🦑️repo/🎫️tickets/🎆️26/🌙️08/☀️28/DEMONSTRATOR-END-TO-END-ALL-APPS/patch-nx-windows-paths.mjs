import { readFileSync, writeFileSync } from "node:fs";

const lib = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs";
let t = readFileSync(lib, "utf8");
const cargoOld = `  for (const configFile of configFiles.filter((file) => file.endsWith("Cargo.toml"))) {`;
const cargoNew = `  for (const configFile of [...new Set([...configFiles.filter((file) => file.endsWith("Cargo.toml")), ...walkCargoToml(workspaceRoot)])]) {`;
if (!t.includes(cargoOld)) throw new Error("cargoOld missing");
t = t.replace(cargoOld, cargoNew);
const existsOld = `    if (configFile.includes("\\uFFFD") || configFile.startsWith("compose/") || configFile.startsWith("temp/compose/") || configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) continue;
    const projectDir = dirname(join(workspaceRoot, configFile));`;
const existsNew = existsOld.replace(
  "continue;\n    const projectDir",
  "continue;\n    if (!existsSync(join(workspaceRoot, configFile))) continue;\n    const projectDir",
);
if (!t.includes(existsOld)) throw new Error("exists block missing");
t = t.replace(existsOld, existsNew);
const cacheOld = "projectInputs, rootCommandTargets };";
const cacheNew = "projectInputs, rootCommandTargets, nxTrackedSourceFile, walkCargoToml };";
if (!t.includes(cacheOld)) throw new Error("cache export missing");
t = t.replace(cacheOld, cacheNew);
writeFileSync(lib, t);
console.log("library patched");

const testPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs";
let tt = readFileSync(testPath, "utf8");
const loopOld = `  for (const configFile of configFiles) {
    if (configFile.includes("\\uFFFD")) continue;
    const rel = nxPath(configFile);`;
const loopNew = `  const featureCandidates = [...new Set([
    ...discoverCaseDirs(workspaceRoot).map((dir) => \`\${dir}/\${featureFilename}\`),
    ...configFiles.map((file) => nxPath(file)),
  ])];
  for (const configFile of featureCandidates) {
    if (configFile.includes("\\uFFFD")) continue;
    const rel = nxPath(configFile);
    if (!existsSync(join(workspaceRoot, rel))) continue;`;
if (!tt.includes(loopOld)) throw new Error("test loop missing");
tt = tt.replace(loopOld, loopNew);
const depOld = `    const sourceFile = \`\${project.root}/\${filenameForKind(vocabulary, vocabulary.testFeatureFileKindId)}\`;`;
const depNew = `    const featureName = filenameForKind(vocabulary, vocabulary.testFeatureFileKindId);
    const preferred = \`\${project.root}/\${featureName}\`;
    const files = context.fileMap?.projectFileMap?.[source];
    const sourceFile = files?.find((entry) => entry.file === preferred)?.file
      ?? files?.find((entry) => entry.file.endsWith(\`/\${featureName}\`))?.file
      ?? preferred;`;
if (!tt.includes(depOld)) throw new Error("dep missing");
tt = tt.replace(depOld, depNew);
writeFileSync(testPath, tt);
console.log("test patched");
