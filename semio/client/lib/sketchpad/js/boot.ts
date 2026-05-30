// #region 🧲Header
/** @emoji 🚀 Vite entry: boots the sketchpad React shell (domain is `@semio/sketchpad`). */
// #endregion 🧲Header

import { bootSketchpadShell } from "./shell.tsx";

void bootSketchpadShell().catch((err) => {
	console.error("[semio.sketchpad boot]", err);
});
