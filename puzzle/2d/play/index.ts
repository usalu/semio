// #region 🧲Header
/** @emoji 🛝 Puzzle 2d play Vite entry via {@link boot2dPlay}. */
// #endregion 🧲Header

import "./globals.css";
import { boot2dPlay } from "@framework/playground/renderer/react/puzzle/2d";
import { Playground2d } from "./play.ts";

boot2dPlay(new Playground2d());
