/** @emoji 🔧 Replace stale `topo` variable references with `model` in spatial/js. */
import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/compose/spatial/js";

function walk(dir: string, out: string[] = []): string[] {
	for (const name of readdirSync(dir)) {
		const p = join(dir, name);
		if (statSync(p).isDirectory()) {
			if (name === "node_modules") continue;
			walk(p, out);
		} else if (/\.(ts|tsx)$/.test(name)) out.push(p);
	}
	return out;
}

for (const file of walk(root)) {
	let text = readFileSync(file, "utf8");
	const orig = text;
	text = text.replace(/\btopo:/g, "model:");
	text = text.replace(/\btopo!/g, "model!");
	text = text.replace(/\btopo\./g, "model.");
	text = text.replace(/\btopo,/g, "model,");
	text = text.replace(/\btopo\)/g, "model)");
	text = text.replace(/\(topo,/g, "(model,");
	text = text.replace(/\(topo\)/g, "(model)");
	text = text.replace(/\(topo /g, "(model ");
	text = text.replace(/`topo\./g, "`model.");
	text = text.replace(/ to `topo/g, " to `model");
	text = text.replace(/for `topo/g, "for `model");
	if (text !== orig) writeFileSync(file, text);
}
