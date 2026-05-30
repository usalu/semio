import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/semio/framework/playground/renderer/react";
const boot = `

//#region 🔖Boot
import type { Playground } from "@framework/playground/core";

/** @emoji 🧩 Play package supplies host registration + React mount (one puzzle surface per boot). */
export interface PlaygroundChromeBoot {
	registerHosts(): void;
	mount(playground: Playground, rootId?: string): void;
}

/** @emoji 🛝 Registers hosts, declarative bodies, and mounts play chrome synchronously. */
export function bootPlayground(playground: Playground, boot: PlaygroundChromeBoot, rootId = "root"): void {
	boot.registerHosts();
	playground.registerBodies();
	playground.registerSurfaceHosts();
	boot.mount(playground, rootId);
}
//#endregion 🔖Boot
`;
writeFileSync(join(root, "index.tsx"), readFileSync(join(root, "shell.tsx"), "utf8") + boot);
console.log("merged shell into index.tsx");
