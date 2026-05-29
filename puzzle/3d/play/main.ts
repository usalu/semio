// #region 🧲Header
// 💻 elements/lib/react/scene/play/main.ts — Vite entry: mounts scene play via {@link renderPlayground}.
// #endregion 🧲Header

import { renderPlayground } from "@framework/playground-renderer-react";
import { ScenePlayground } from "./index.ts";

renderPlayground(new ScenePlayground());
