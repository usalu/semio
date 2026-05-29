// #region 🧲Header
// 💻 puzzle/2d/play/main.ts — Vite entry: mounts board play via {@link renderPlayground}.
// #endregion 🧲Header

import { renderPlayground } from "@framework/playground-renderer-react";
import { BoardPlayground } from "./index.ts";

renderPlayground(new BoardPlayground());
