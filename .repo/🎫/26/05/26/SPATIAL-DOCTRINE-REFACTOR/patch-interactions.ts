/** @emoji 🔧 One-off: add produces.typology and strip editableEntities from builtin interactions. */
import { readdirSync, readFileSync, writeFileSync, statSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/semio/spatial/assets/extension/builtin/interaction";

function walk(dir: string, out: string[] = []): string[] {
	for (const name of readdirSync(dir)) {
		const p = join(dir, name);
		if (statSync(p).isDirectory()) walk(p, out);
		else if (name.endsWith(".json")) out.push(p);
	}
	return out;
}

for (const file of walk(root)) {
	const rel = file.replace(root + "\\", "").replace(root + "/", "").replace(/\\/g, "/").replace(".json", "");
	const parts = rel.split("/");
	const typologyId = `builtin.${parts[0]}.${parts[1]}`;
	const doc = JSON.parse(readFileSync(file, "utf8")) as Record<string, unknown>;
	doc.produces = { typology: typologyId };
	const req = doc.requires as Record<string, unknown> | undefined;
	if (req?.kernel && typeof req.kernel === "object") {
		const k = { ...(req.kernel as Record<string, unknown>) };
		delete k.editableEntities;
		delete k.derivedEntities;
		if (Object.keys(k).length > 0) doc.requires = { ...req, kernel: k };
		else {
			const next = { ...req };
			delete next.kernel;
			if (Object.keys(next).length > 0) doc.requires = next;
			else delete doc.requires;
		}
	}
	writeFileSync(file, JSON.stringify(doc, null, 2) + "\n");
}
