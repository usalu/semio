// #region 🧲Header
// 💻 elements/client/lib/geometry/spatial/play/main.ts — Vite entry (plain TS): delegates to {@link mountSpatialPlay} in `geometry-spatial-play-host.tsx`.
// #endregion 🧲Header

//#region 🔖Mount
import "./globals.css";

void import("../geometry-spatial-play-host").then((m) => void m.mountSpatialPlay());
//#endregion 🔖Mount
