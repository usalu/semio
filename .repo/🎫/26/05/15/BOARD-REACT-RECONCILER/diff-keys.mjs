import fs from "node:fs";

const devPath = "c:/git/compose/elements/client/lib/board/node_modules/react-reconciler/cjs/react-reconciler.development.js";
const s = fs.readFileSync(devPath, "utf8");
const re = /([a-zA-Z0-9$]+) = \$\$\$config\.([a-zA-Z0-9]+)/g;
const keys = new Set();
let m;
while ((m = re.exec(s))) {
	keys.add(m[2]);
}
fs.writeFileSync(new URL("./required-keys.txt", import.meta.url), [...keys].sort().join("\n"));
console.error("count", keys.size);

const stub = fs.readFileSync(new URL("./stubs.txt", import.meta.url), "utf8");
const have = new Set();
for (const line of stub.split(/\n/)) {
	const mm = /^\t([a-zA-Z0-9]+):/.exec(line);
	if (mm) {
		have.add(mm[1]);
	}
}
const missing = [...keys].filter((k) => !have.has(k)).sort();
const extra = [...have].filter((k) => !keys.has(k)).sort();
fs.writeFileSync(new URL("./missing-keys.txt", import.meta.url), missing.join("\n"));
fs.writeFileSync(new URL("./extra-keys.txt", import.meta.url), extra.join("\n"));
console.error("missing", missing.length, "extra", extra.length);
