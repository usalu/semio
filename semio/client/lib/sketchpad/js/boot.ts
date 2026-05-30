// #region 🧲Header
/** @emoji 🚀 Vite entry: generic {@link PlatformView} via {@link mountPlatform}. */
// #endregion 🧲Header

import "./globals.css";
import { mountPlatform } from "@framework/platform/renderer/react";
import { ensureSketchpadPlatform } from "./index.ts";

void mountPlatform(ensureSketchpadPlatform);
