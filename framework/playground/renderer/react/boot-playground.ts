// #region 🧲Header
/** @emoji 🚀 Playground boot contract: play entry registers hosts then mounts chrome (no puzzle imports in shell). */
// #endregion 🧲Header

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
