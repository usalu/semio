/** @emoji 🔧 One-off: retarget attributes to object + geometrySelector. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dir = "c:/git/compose/spatial/assets/extension/builtin/attribute";
for (const name of readdirSync(dir)) {
	if (!name.endsWith(".json")) continue;
	const file = join(dir, name);
	const doc = JSON.parse(readFileSync(file, "utf8")) as Record<string, unknown>;
	const old = doc.targets as string[] | undefined;
	doc.targets = ["object"];
	if (old?.length) doc.geometrySelector = { kinds: old };
	writeFileSync(file, JSON.stringify(doc, null, 2) + "\n");
}
