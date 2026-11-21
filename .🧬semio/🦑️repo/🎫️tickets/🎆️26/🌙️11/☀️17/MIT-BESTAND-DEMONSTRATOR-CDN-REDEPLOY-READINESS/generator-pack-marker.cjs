const { readFileSync } = require("node:fs");
const { join } = require("node:path");
const expected = require("./generator-pack-marker.json");

for (const [name, law] of Object.entries(expected)) {
  const image = new WebAssembly.Module(readFileSync(join(process.argv[2], name + ".wasm")));
  const host = {};
  for (const { module, name: imported } of WebAssembly.Module.imports(image)) {
    (host[module] ??= {})[imported] = () => {
      throw new Error("unexpected host call " + imported);
    };
  }
  const result = new WebAssembly.Instance(image, host).exports.check();
  const actual = {
    zeroWorkDeniesWithoutAllocation: (result & 1) !== 0,
    fundedTurnPostsExtentAndWork: (result & 2) !== 0,
  };
  console.log("[DEBUG]", name, actual);
  if (actual.zeroWorkDeniesWithoutAllocation !== law.zeroWorkDeniesWithoutAllocation ||
      actual.fundedTurnPostsExtentAndWork !== law.fundedTurnPostsExtentAndWork)
    process.exitCode = 1;
}
