// #region 🧲Header
// 💻 elements/spatial/play/main.ts — Vite entry: delegates to {@link mountSpatialPlay} in `spatial-play-host.tsx`.
// #endregion 🧲Header

//#region 🔖Mount
import "./globals.css";

void import("../index.tsx").then((m) => void m.mountSpatialPlay());
//#endregion 🔖Mount
