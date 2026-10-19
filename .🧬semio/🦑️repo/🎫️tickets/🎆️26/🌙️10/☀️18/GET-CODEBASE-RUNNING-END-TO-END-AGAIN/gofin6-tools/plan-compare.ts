import {readFileSync} from "node:fs";
const {canonicalGoPlan} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts");
const [moduleRoot, overlay, packages] = process.argv.slice(2);
const p = canonicalGoPlan(moduleRoot!);
const py = JSON.parse(readFileSync(overlay!, "utf8")).Replace as Record<string, string>;
const pyPackages = readFileSync(packages!, "utf8").split("\n").filter(Boolean).sort();
const same = JSON.stringify(Object.entries(p.replacements).sort()) === JSON.stringify(Object.entries(py).sort());
console.log(JSON.stringify({samePackages: JSON.stringify([...p.packages].sort()) === JSON.stringify(pyPackages), sameOverlay: same, entries: Object.keys(p.replacements).length}));
