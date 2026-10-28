const root = process.argv[2], L = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const t0 = performance.now();
const { libraryBootstrap } = await import(L + "/🕸️graph/📥️inference/🟨️.mjs");
await libraryBootstrap;
const { createProjectInferenceEnvironmentV1 } = await import(L + "/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const { observeInferenceSnapshotV1 } = await import(L + "/👁️observation/🟨️.mjs");
const contracts = { "law-generator": { outputs: ["law"] } };
const env = await observeInferenceSnapshotV1(() => createProjectInferenceEnvironmentV1({ version: 2, workspaceRoot: root, registry: null, commandInputs: null, lateClosures: false, projectsByRoot: [], componentRoots: [], stagingBase: "/nonexistent", contracts }));
console.log(JSON.stringify({ ms: Math.round(performance.now() - t0), parentContracts: JSON.stringify(env.contracts) === JSON.stringify(contracts), diskKeys: Object.keys(env.contracts).length }));
