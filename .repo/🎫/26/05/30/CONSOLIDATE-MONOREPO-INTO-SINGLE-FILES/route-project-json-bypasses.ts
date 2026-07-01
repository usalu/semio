import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");

type Fix = string | string[];

const projectJsonFixes: Record<string, Record<string, Fix>> = {
	"reasoning/mindmap/project.json": { test: "bun ./script.ts test" },
	"procedural/2d/rs/project.json": { test: "bun ./script.ts test" },
	"procedural/3d/rs/project.json": { test: "bun ./script.ts test" },
	"puzzle/3d/rs/project.json": { test: "bun ./script.ts test" },
	"puzzle/5d/rs/project.json": { test: "bun ./script.ts test" },
	"gis/2d/rs/project.json": { test: "bun ./script.ts test" },
	"mathematical/graph/manifest/project.json": { test: "bun ./script.ts test" },
	"trinity/jack/lsp/project.json": { test: "bun ./script.ts test" },
	"writer/rs/project.json": { test: "bun ./script.ts test" },
	"repo/lib/js/project.json": { lint: "bun ./script.ts lint", test: "bun ./script.ts test" },
	"ui/styling/js/project.json": { test: "bun ./script.ts test" },
	"repo/lib/go/project.json": { test: "bun ./script.ts test" },
	"kernel/2d/js/project.json": { test: "bun ./script.ts test" },
	"compose/client/lib/rs/project.json": {
		test: "bun ./script.ts test",
		wasm: "bun ./script.ts wasm",
		build: "bun ./script.ts build",
		setup: ["bun ./script.ts setup"],
	},
	"compose/client/lib/query/project.json": {
		test: "bun ./script.ts test",
		build: ["bun ./script.ts wasm", "bun ./script.ts build"],
		setup: ["bun ./script.ts setup"],
	},
	"compose/server/hub/project.json": {
		setup: "bun ./script.ts setup",
		build: "bun ./script.ts build",
		test: "bun ./script.ts test",
	},
	"compose/client/lib/go/project.json": { test: "bun ./script.ts test" },
	"compose/client/lib/py/project.json": { build: "bun ./script.ts build", test: "bun ./script.ts test" },
	"compose/client/lib/net/Compose/project.json": { test: "bun ./script.ts test" },
	"asset/project.json": { build: "bun ./script.ts build" },
	"coda/client/lib/programming/go/project.json": { test: "bun ./script.ts test" },
	"coda/client/lib/blnbo/go/project.json": { test: "bun ./script.ts test" },
};

for (const [rel, targets] of Object.entries(projectJsonFixes)) {
	const p = join(root, rel);
	const json = JSON.parse(readFileSync(p, "utf8"));
	for (const [target, command] of Object.entries(targets)) {
		if (!json.targets?.[target]) continue;
		if (Array.isArray(command)) {
			json.targets[target].options.commands = command;
			delete json.targets[target].options.command;
		} else {
			json.targets[target].options.command = command;
			delete json.targets[target].options.commands;
		}
	}
	writeFileSync(p, `${JSON.stringify(json, null, 2)}\n`);
	console.log(`updated ${rel}`);
}
