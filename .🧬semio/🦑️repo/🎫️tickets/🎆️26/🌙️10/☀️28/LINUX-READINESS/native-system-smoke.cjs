const fs = require("node:fs"), cp = require("node:child_process"), path = require("node:path");
const root = __dirname, fixture = JSON.parse(fs.readFileSync(path.join(root, "native-system-smoke.json"), "utf8"));
const execute = (command, args) => {
  const result = cp.spawnSync(command, args, { encoding: "utf8" });
  if (result.status !== 0) throw Error(command + " " + result.status + " " + result.stderr);
  return result.stdout.trim();
};
const flags = execute("pkg-config", ["--cflags", "--libs", "cairo", "pango", "uuid", "icu-uc"]).split(/\s+/);
const generated = path.join(root, "🗑️generated");
fs.mkdirSync(generated, { recursive: true });
const binary = path.join(generated, "native-system-smoke");
execute("cc", ["-std=c11", path.join(root, "native-system-smoke.c"), "-o", binary, ...flags]);
const actual = JSON.parse(execute(binary, []));
const expected = { cairo: execute("pkg-config", ["--modversion", "cairo"]), pango: execute("pkg-config", ["--modversion", "pango"]), icu: execute("pkg-config", ["--modversion", "icu-uc"]), uuid: fixture.uuid };
if (JSON.stringify(actual) !== JSON.stringify(expected)) throw Error(JSON.stringify({ actual, expected }));
console.log(JSON.stringify({ compiled: true, linked: true, executed: true, actual, expected }));
