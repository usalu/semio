// #region 🧲Header
// 💻 elements/client/lib/scene/play/main.tsx — Vite entry: mounts scene play shell (React-only).
// #endregion 🧲Header

//#region 🔖Mount
import { mountReactApp } from "@elements/ui";

import { createScenePlayElement } from "./react.tsx";

mountReactApp(createScenePlayElement());
//#endregion 🔖Mount
