import fs from "node:fs";

const required = fs
	.readFileSync(new URL("./required-keys.txt", import.meta.url), "utf8")
	.split(/\n/)
	.filter(Boolean);
const src = fs.readFileSync("c:/git/compose/elements/client/lib/board/index.ts", "utf8");
const missing = required.filter((k) => {
	const prop = new RegExp(`\\b${k}\\s*:`);
	const method = new RegExp(`\\b${k}\\s*\\(`);
	return !prop.test(src) && !method.test(src);
});
console.error("missing from defaults", missing.length);
console.log(missing.join("\n"));
