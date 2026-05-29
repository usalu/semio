// #region 🧲Header
/** @emoji 🛝 `@framework/playground-renderer-react` — shell + puzzle play boots via subpath exports (no top-level puzzle imports). */
// #endregion 🧲Header

export * from "./shell.tsx";

//#region 🔖Boot
import type { Playground } from "@framework/playground";

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
