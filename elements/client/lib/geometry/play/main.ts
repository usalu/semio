// #region 🧲Header
// 💻 elements/client/lib/geometry/play/main.ts — Vite entry (plain TS): delegates DOM/React to {@link mountGeometryPlay} in `geometry-play-host.tsx`.
// #endregion 🧲Header

//#region 🔖Mount
import "./globals.css";

void import("../geometry-play-host").then((m) => void m.mountGeometryPlay());
//#endregion 🔖Mount
