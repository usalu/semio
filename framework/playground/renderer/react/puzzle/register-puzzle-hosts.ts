// #region 🧲Header
/** @emoji 🧩 Registers all puzzle play surface hosts before {@link renderPlayground} mounts. */
// #endregion 🧲Header

import { registerBoardPlaySurfaceHosts } from "./board-play-host.tsx";
import { registerSceneSurfaceHosts } from "./scene-play-host.tsx";
import { registerTopologyPlaySurfaceHosts } from "./topology-play-host.tsx";

/** @emoji 🧊 Idempotent registration of board, scene, and topology React surface hosts. */
export function registerPuzzleReactHosts(): void {
	registerBoardPlaySurfaceHosts();
	registerSceneSurfaceHosts();
	registerTopologyPlaySurfaceHosts();
}
