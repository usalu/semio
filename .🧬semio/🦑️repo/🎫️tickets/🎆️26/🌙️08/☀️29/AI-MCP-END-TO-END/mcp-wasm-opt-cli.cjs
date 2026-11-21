const { spawnSync } = require("node:child_process");

const binary = process.env.SEMIO_BINARYEN_EXECUTABLE;
if (!binary) throw Error("The pinned Binaryen executable path is required");
const result = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });
if (result.error) throw result.error;
process.exitCode = result.status ?? 1;
