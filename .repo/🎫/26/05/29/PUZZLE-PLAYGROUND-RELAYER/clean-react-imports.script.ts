#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";

const path2d = "c:/git/semio/puzzle/2d/react/index.tsx";
let c2 = readFileSync(path2d, "utf8");
const marker2d = 'import { Expertise, ProductRuntime';
const end2d = '} from "./play/index.ts";\n';
const i2 = c2.indexOf(marker2d);
const j2 = c2.indexOf(end2d, i2);
if (i2 >= 0 && j2 >= 0) {
	c2 = c2.slice(0, i2) + c2.slice(j2 + end2d.length);
	writeFileSync(path2d, c2);
	console.log("[clean] 2d react imports removed");
}

const path3d = "c:/git/semio/puzzle/3d/react/index.tsx";
let c3 = readFileSync(path3d, "utf8");
c3 = c3.replace(/^import \{ CommandBus.*\n/m, "");
const m3 = c3.indexOf('import { registerTabIcon');
const e3 = c3.indexOf('} from "./play/index.ts";\n', m3);
if (m3 >= 0 && e3 >= 0) {
	c3 = c3.slice(0, m3) + c3.slice(e3 + '} from "./play/index.ts";\n'.length);
	writeFileSync(path3d, c3);
	console.log("[clean] 3d react imports removed");
}
