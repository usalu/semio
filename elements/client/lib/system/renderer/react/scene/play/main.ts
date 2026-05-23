// #region 🧲Header
// 💻 elements/client/lib/system/renderer/react/scene/play/main.ts — Vite entry (plain TS): delegates to {@link mountScenePlay} in `scene-play-host.tsx`.
// #endregion 🧲Header

//#region 🔖Mount
void import("../scene-play-host").then((m) => {
	m.mountScenePlay();
});
//#endregion 🔖Mount
