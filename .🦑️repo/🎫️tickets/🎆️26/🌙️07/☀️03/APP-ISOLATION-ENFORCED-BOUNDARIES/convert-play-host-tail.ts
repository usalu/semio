#!/usr/bin/env bun
/** One-off helper — converts play-host boot tails to AppRendererContribution exports. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

type Spec = {
  path: string;
  exportName: string;
  surfaceHosts: string;
  panelTabs?: string;
  preload?: string;
  tabIcons?: string;
  extraExports?: string;
  skip?: boolean;
};

const SPECS: Spec[] = [
  {
    path: "imperative/react/play-host.tsx",
    exportName: "imperativeAppRenderer",
    surfaceHosts: `{ [IMPERATIVE_PLAY_SURFACE_ID]: ImperativePlayPaneSurfaceHost }`,
  },
  {
    path: "vcs/react/play-host.tsx",
    exportName: "vcsAppRenderer",
    surfaceHosts: `{
    [VCS_PLAY_SURFACE_ID_EDITOR]: VcsPlayEditorSurfaceHost,
    [VCS_PLAY_SURFACE_ID_HISTORY]: VcsPlayHistorySurfaceHost,
  }`,
  },
  {
    path: "mathematical/graph/port/directed/dag/react/play-host.tsx",
    exportName: "dagAppRenderer",
    surfaceHosts: `{
    [DAG_PLAY_SURFACE_ID]: DagPlayPaneSurfaceHost,
    [DAG_PLAY_SURFACE_ID_JACK]: DagPlayJackSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new DagPlayDocumentPanelDefinition(), new DagPlayCataloguePanelDefinition()],
    details: [new DagPlayInspectionPanelDefinition()],
  }`,
  },
  {
    path: "sequence/react/play-host.tsx",
    exportName: "sequenceAppRenderer",
    surfaceHosts: `{
    [SEQUENCE_PLAY_SURFACE_ID]: SequencePlayPaneSurfaceHost,
    [SEQUENCE_PLAY_SCRIPT_SURFACE_ID]: SequencePlayScriptSurfaceHost,
    [SEQUENCE_PLAY_SURFACE_ID_COMPILED_DAG]: SequencePlayCompiledDagSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new SequencePlayDocumentPanelDefinition(), new SequencePlayCataloguePanelDefinition()],
    details: [new SequencePlayInspectionPanelDefinition()],
  }`,
  },
  {
    path: "gis/2d/react/play-host.tsx",
    exportName: "mapAppRenderer",
    surfaceHosts: `{ [GIS_MAP_PLAY_SURFACE_ID]: MapPlayPaneSurfaceHost }`,
    panelTabs: `{
    workbench: [new MapPlayDocumentPanelDefinition(), new MapPlayCataloguePanelDefinition()],
    details: [new MapPlayInspectionPanelDefinition()],
  }`,
  },
];

function stripBootTail(content: string): string {
  return content
    .replace(/\nlet \w+PlayChromeRegistered = false;\n?/g, "\n")
    .replace(/\nexport function register\w+PlaySurfaceHosts\(\): void \{[\s\S]*?\n\}\n/g, "\n")
    .replace(/\nfunction \w+PlayInner\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\nfunction \w+PlayChrome\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\nexport function mount\w+PlayChrome\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\nconst \w+PlayChromeBoot[\s\S]*?\n\};\n/g, "\n")
    .replace(/\nexport (?:async )?function boot\w+Play\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\n\/\/#endregion 🔖️\w+PlayHost\n?$/, "\n");
}

function fixImports(content: string): string {
  let c = content;
  c = c.replace(/\/\*\* @emoji 🛝️ Playground play host for ([^—]+) — loaded only via `\.\/play` subpath\. \*\//, "/** @emoji 🛝️ $1 app renderer contribution — loaded only via `./play` subpath. */");
  if (!c.includes("AppRendererContribution")) {
    c = c.replace(/import type \{ ReactElement \} from "react";/, `import type { ReactElement } from "react";\nimport type { AppRendererContribution } from "@semio-tech/framework-platform-core";`);
  }
  c = c.replace(/import \{ type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, /, "import { ");
  c = c.replace(/import \{ type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, /, "import { ");
  c = c.replace(/, registerUi\w+SurfaceHost/g, "");
  c = c.replace(/registerUi\w+SurfaceHost, /g, "");
  c = c.replace(/, registerTabIcon/g, "");
  c = c.replace(/registerTabIcon, /g, "");
  c = c.replace(/, registerWindowBody[^,}]*/g, "");
  c = c.replace(/registerWindowBody[^,}]*, /g, "");
  c = c.replace(/,\s*register\w+PlayDeclarativeBodies/g, "");
  c = c.replace(/register\w+PlayDeclarativeBodies,?\s*/g, "");
  return c;
}

for (const spec of SPECS) {
  if (spec.skip) continue;
  const full = join(REPO, spec.path);
  let content = readFileSync(full, "utf8");
  content = fixImports(content);
  content = stripBootTail(content);
  const lines = [`/** @emoji 🛝️ ${spec.exportName.replace("AppRenderer", " app renderer")} for playground and OS shells. */`, `export const ${spec.exportName}: AppRendererContribution = {`, `  surfaceHosts: ${spec.surfaceHosts},`];
  if (spec.panelTabs) lines.push(`  panelTabs: ${spec.panelTabs},`);
  if (spec.tabIcons) lines.push(`  tabIcons: ${spec.tabIcons},`);
  if (spec.preload) lines.push(`  preload: ${spec.preload},`);
  lines.push("};");
  if (spec.extraExports) lines.push("", spec.extraExports);
  content = `${content.trimEnd()}\n\n${lines.join("\n")}\n`;
  writeFileSync(full, content);
  console.log(`converted ${spec.path}`);
}
