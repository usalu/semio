// #region 🧲Header
// 💻 elements/client/lib/system/renderer/react/windows/board/play/main.ts — Vite entry (plain TS): delegates to {@link mountBoardPlay} in `board-play-host.tsx`.
// #endregion 🧲Header

//#region 🔖Mount
void import("../board-play-host").then((m) => {
	m.mountBoardPlay();
});
//#endregion 🔖Mount
