// #region 🧲Header
// 💻 elements/client/lib/topology/play/main.ts — Vite entry (plain TS): delegates to {@link mountTopologyPlay} in `topology-play-host.tsx`.
// #endregion 🧲Header

//#region 🔖Mount
void import("../topology-play-host").then((m) => {
	m.mountTopologyPlay();
});
//#endregion 🔖Mount
