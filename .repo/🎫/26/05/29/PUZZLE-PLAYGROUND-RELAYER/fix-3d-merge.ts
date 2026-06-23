#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";

const path = "c:/git/compose/puzzle/3d/react/index.tsx";
let c = readFileSync(path, "utf8");
const start = c.indexOf("/** @emoji 🛝 Scene play React host");
const anchorPlay = '} from "./index.ts";\n\n';
const startPlay = c.indexOf(anchorPlay, start);
if (start < 0 || startPlay < 0) {
	console.error("[fix-3d] block not found");
	process.exit(1);
}
const hostImports = c.slice(start, startPlay + anchorPlay.length);
c = c.slice(0, start) + c.slice(startPlay + anchorPlay.length);
hostImports.replaceAll('./fixtures/', '../play/fixtures/').replaceAll('from "./index.ts"', 'from "../play/index.ts"');

const inject = hostImports
	.replace("/** @emoji 🛝 Scene play React host — entry-only via play/main.ts. */\n", "")
	.replaceAll("./fixtures/", "../play/fixtures/");

const anchor = "} from \"three\";\n";
const pos = c.lastIndexOf(anchor);
if (pos < 0) {
	console.error("[fix-3d] three anchor missing");
	process.exit(1);
}
c = c.slice(0, pos + anchor.length) + "\n" + inject + c.slice(pos + anchor.length);

writeFileSync(path, c);
console.log("[fix-3d] cleaned");
