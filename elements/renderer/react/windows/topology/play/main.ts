// #region ­ƒº▓Header
// ­ƒÆ╗ elements/client/lib/topology/play/main.ts ÔÇö Vite entry (plain TS): delegates to {@link mountTopologyPlay} in `topology-play-host.tsx`.
// #endregion ­ƒº▓Header

//#region ­ƒöûMount
void import("../topology-play-host").then((m) => {
	m.mountTopologyPlay();
});
//#endregion ­ƒöûMount
