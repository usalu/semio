// #region 🧲Header
/** @emoji 🚀 Vite entry: {@link mountPlatform} + sketchpad {@link Platform}. */
// #endregion 🧲Header

import "./globals.css";
import { mountPlatform } from "@framework/platform/renderer/react";
import { ensureSketchpadPlatform } from "./index.ts";
import { registerSketchpadComponentKindRenderers } from "./sketchpad-renderers.tsx";

registerSketchpadComponentKindRenderers();
void mountPlatform(ensureSketchpadPlatform);
