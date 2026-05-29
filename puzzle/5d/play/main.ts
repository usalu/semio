// #region 🧲Header
// 💻 puzzle/5d/play/main.ts — Vite entry: mounts topology play via {@link renderPlayground}.
// #endregion 🧲Header

import { renderPlayground } from "@framework/playground-renderer-react";
import { TopologyPlayground } from "./index.ts";

renderPlayground(new TopologyPlayground());
