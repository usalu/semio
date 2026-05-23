// #region 🧲Header
// 💻 elements/client/lib/board/play/main.tsx — Vite entry: mounts board play shell (React-only).
// #endregion 🧲Header

//#region 🔖Mount
import { mountReactApp } from "@elements/ui";

import { createBoardPlayElement } from "./react.tsx";

mountReactApp(createBoardPlayElement());
//#endregion 🔖Mount
