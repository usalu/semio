/** @emoji 🔧 Restore produces.typology corrupted to builtin.c:.git */
import { readFileSync, writeFileSync } from "node:fs";
import { globSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/compose/spatial/assets/extension/builtin/interaction";
const files = globSync(join(root, "**/*.json").replace(/\\/g, "/"));

for (const file of files) {
	const raw = JSON.parse(readFileSync(file, "utf8")) as { id?: string; produces?: { typology?: string } };
	if (!raw.id || raw.produces?.typology !== "builtin.c:.git") continue;
	const rel = file.replace(/\\/g, "/").split("/interaction/")[1]!.replace(/\.json$/, "");
	const typologyId = `builtin.${rel.replace(/\//g, ".")}`;
	raw.produces = { ...raw.produces, typology: typologyId };
	writeFileSync(file, `${JSON.stringify(raw, null, 2)}\n`);
	console.log(file, "->", typologyId);
}
