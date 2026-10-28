const fs = require("node:fs");
const parsers = [require("@iarna/toml"), require("smol-toml")];
const failures = [];
let count = 0;
function walk(path) {
  for (const entry of fs.readdirSync(path, { withFileTypes: true })) {
    if (entry.name.startsWith(".") || ["node_modules", "target", "dist"].includes(entry.name)) continue;
    const file = path + "/" + entry.name;
    if (entry.isDirectory()) walk(file);
    else if (entry.name === "Cargo.toml") {
      count++;
      for (const [index, parser] of parsers.entries()) {
        try { parser.parse(fs.readFileSync(file, "utf8")); }
        catch (error) { failures.push({ file, parser: index, error: error.message }); }
      }
    }
  }
}
walk(".");
console.log(JSON.stringify({ count, failures }, null, 2));
process.exitCode = failures.length ? 1 : 0;
