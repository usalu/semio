// #region 🧲Header
/** @emoji 🛝 Puzzle 3D play Vite entry via {@link bootPuzzle3dPlay}. */
// #endregion 🧲Header

import "./globals.css";
import { bootPuzzle3dPlay } from "@framework/playground/renderer/react/puzzle/3d";
import { Playground3d } from "./play.ts";

bootPuzzle3dPlay(new Playground3d());
