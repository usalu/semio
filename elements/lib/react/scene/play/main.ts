// #region 🧲Header
// 💻 elements/lib/react/scene/play/main.ts — Vite entry: mounts scene play via {@link renderPlayground}.
// #endregion 🧲Header

import { renderPlayground } from "@elements/playground/react";
import { ScenePlayground } from "./index.ts";

renderPlayground(new ScenePlayground());
