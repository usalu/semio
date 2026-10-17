/** 🧾️ Dumps the workspace packages and the `no-cross-package-relative-*` rules of the dependency policy snapshot. Usage: node dump-policy.cjs <out.json> */
const { loadDependencyDirectionPolicy } = require("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟨️.cjs");
const s = loadDependencyDirectionPolicy("/Users/ueli/Documents/semio");
const rules = s.policy.forbidden.filter(r => r.name.startsWith("no-cross-package-relative-"));
require("node:fs").writeFileSync(process.argv[2], JSON.stringify({ packages: s.workspacePackages, rules }));
console.log(s.workspacePackages.length, rules.length);
