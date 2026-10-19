const fs = require("node:fs");
const base = require("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs");
const taxonomy = JSON.parse(fs.readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", "utf8"));
const esc = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const foundation = taxonomy.dependencyDirections.roles["ui-foundation"].ownerPaths.map((p) => `^${esc(p)}/`);
const support = taxonomy.dependencyDirections.roles["ui-test-support"].ownerPaths.map((p) => `^${esc(p)}/`);
module.exports = { ...base, forbidden: [{
  name: "ui-foundation-closure", severity: "error",
  from: { path: foundation, pathNot: ["(^|/)📜️script\\.ts$", "(^|/)🧪️(?:tests|testing)/", "\\.(?:test|spec|story)\\.[cm]?[jt]sx?$", "(^|/)🧪️tests/🎚️config/"] },
  to: { path: ["^🧰️framework/", "^@semio-tech/framework-"], pathNot: foundation.concat(["^🧰️framework/🔨️modules/🖱️ui/", "^@semio-tech/ui-"]) },
}] };
