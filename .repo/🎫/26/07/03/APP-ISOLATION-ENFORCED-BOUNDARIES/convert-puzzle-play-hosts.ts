#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

// puzzle 2d: strip Inner + boot tail, add AppRendererContribution export
{
  const path = join(REPO, "puzzle/2d/react/play-host.tsx");
  let content = readFileSync(path, "utf8");
  content = content.replace(/\/\*\* @emoji 🛝 Playground play host for Puzzle2d/, "/** @emoji 🛝 Puzzle 2D app renderer contribution");
  if (!content.includes("AppRendererContribution")) {
    content = content.replace('import type { ReactElement } from "react";', 'import type { ReactElement } from "react";\nimport type { AppRendererContribution } from "@semio-tech/framework-platform-core";');
  }
  content = content.replace(
    /import \{ type Playground, type PlaygroundChromeBoot, type PlaygroundKeybinding, bootPlayground, mountPlaygroundApp, PlaygroundView, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiPuzzle2dSurfaceHost, registerUiWriterSurfaceHost, registerTabIcon,/,
    "import { PureSidePanelTabDefinition, CallbackTreePanelDefinition,",
  );
  content = content.replace(/\nlet puzzle2dPlayChromeRegistered = false;\n\n\/\*\* @emoji 🧊 Registers puzzle 2d play surface host[\s\S]*?registerTabIcon\("puzzle\.2d-play\.icon\.settings", "settings"\);\n\}/, "");
  const innerStart = content.indexOf("\nfunction Puzzle2dPlayInner(");
  const entryEnd = content.indexOf("// #endregion 🔖Entrypoint");
  if (innerStart >= 0 && entryEnd > innerStart) {
    content = content.slice(0, innerStart) + "\n" + content.slice(entryEnd);
  }
  if (!content.includes("puzzle2dAppRenderer")) {
    content =
      content.trimEnd() +
      `

/** @emoji 🛝 Puzzle 2D app renderer for playground and OS shells. */
export const puzzle2dAppRenderer: AppRendererContribution = {
  surfaceHosts: {
    [PUZZLE_2D_PLAY_SURFACE_ID]: Puzzle2dPlayPaneSurfaceHost,
    [PUZZLE_2D_PLAY_SURFACE_ID_COMPILED_DAG]: Puzzle2dPlayCompiledDagSurfaceHost,
  },
  panelTabs: {
    workbench: [new Puzzle2dPlayDocumentPanelDefinition(), new Puzzle2dPlayKindsPanelDefinition()],
    details: [new Puzzle2dPlayInspectorPanelDefinition()],
  },
  tabIcons: {
    [PUZZLE_2D_PLAY_ICON_KINDS]: "tags",
    "puzzle.2d-play.icon.inspector": "clipboard-list",
    "puzzle.2d-play.icon.settings": "settings",
  },
};

/** @emoji 🔗 WIRES play renderer — same surfaces and panels as puzzle 2D play. */
export const wiresAppRenderer: AppRendererContribution = puzzle2dAppRenderer;
`;
  }
  writeFileSync(path, content);
  console.log("converted puzzle/2d/react/play-host.tsx");
}
