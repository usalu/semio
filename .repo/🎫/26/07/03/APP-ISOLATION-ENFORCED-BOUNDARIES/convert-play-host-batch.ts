#!/usr/bin/env bun
/** Batch convert remaining play-host files to AppRendererContribution exports. */
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
};

const SPECS: Spec[] = [
  {
    path: "flow/react/play-host.tsx",
    exportName: "flowAppRenderer",
    surfaceHosts: `{
    [FLOW_PLAY_SURFACE_ID]: FlowPlayPaneSurfaceHost,
    [FLOW_PLAY_SURFACE_ID_GENERATE]: FlowPlayGenerateSurfaceHost,
    [FLOW_PLAY_SURFACE_ID_COMPILED_DAG]: FlowPlayCompiledDagSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new FlowPlayHierarchyPanelDefinition(), new FlowPlayCataloguePanelDefinition()],
    details: [new FlowPlayInspectionPanelDefinition()],
  }`,
    preload: "ensureFlowWasmLoaded",
  },
  {
    path: "forms/react/play-host.tsx",
    exportName: "formsAppRenderer",
    surfaceHosts: `{
    [FORMS_PLAY_SURFACE_ID_EDIT]: FormsEditSurfaceHost,
    [FORMS_PLAY_SURFACE_ID_TRY]: FormsTrySurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new FormsPlayHierarchyPanelDefinition(), new FormsPlayCataloguePanelDefinition()],
    details: [new FormsPlayInspectionPanelDefinition()],
  }`,
  },
  {
    path: "raster/react/play-host.tsx",
    exportName: "rasterAppRenderer",
    surfaceHosts: `{
    [RASTER_PLAY_SURFACE_ID_COMPOSITE]: RasterPlayPaneSurfaceHost,
    [RASTER_PLAY_SURFACE_ID_NAVIGATOR]: RasterPlayPaneSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new RasterPlayLayersPanelDefinition(), new RasterPlayCataloguePanelDefinition(), new RasterPlayMasksPanelDefinition()],
    details: [new RasterPlayPropertiesPanelDefinition()],
  }`,
  },
  {
    path: "shooting/react/play-host.tsx",
    exportName: "shootingAppRenderer",
    surfaceHosts: `{
    [SHOOTING_PLAY_SURFACE_ID_MODEL]: ShootingModelSurfaceHost,
    [SHOOTING_PLAY_SURFACE_ID_ICON]: ShootingIconSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new ShootingPlayHierarchyPanelDefinition(), new ShootingPlayCataloguePanelDefinition()],
    details: [new ShootingPlayInspectionPanelDefinition()],
  }`,
  },
  {
    path: "lowpoly/react/play-host.tsx",
    exportName: "lowpolyAppRenderer",
    surfaceHosts: `{
    [LOWPOLY_PLAY_SURFACE_ID]: LowpolyPlaySurfaceHost,
    [LOWPOLY_PLAY_UV_SURFACE_ID]: LowpolyUvSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new LowpolyPlayHierarchyPanelDefinition(), new LowpolyPlayCataloguePanelDefinition()],
    details: [new LowpolyPlayInspectionPanelDefinition(), new LowpolyPlayLayersPanelDefinition()],
  }`,
    preload: "preloadLowpolyPlay",
  },
  {
    path: "procedural/3d/react/play-host.tsx",
    exportName: "proceduralAppRenderer",
    surfaceHosts: `{
    [PROCEDURAL_PLAY_SURFACE_ID]: ProceduralPlayPaneSurfaceHost,
    [PROCEDURAL_PLAY_SURFACE_ID_PREVIEW]: ProceduralPreviewSurfaceHost,
    [PROCEDURAL_PLAY_SURFACE_ID_GENERATE]: Procedural3dGenerateSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new ProceduralPlayHierarchyPanelDefinition(), new ProceduralPlayCataloguePanelDefinition()],
    details: [new ProceduralPlayInspectionPanelDefinition()],
  }`,
  },
  {
    path: "procedural/2d/react/play-host.tsx",
    exportName: "procedural2dAppRenderer",
    surfaceHosts: `{
    [PROCEDURAL_2D_PLAY_SURFACE_ID]: Procedural2dPlayPaneSurfaceHost,
    [PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW]: Procedural2dPreviewSurfaceHost,
    [PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE]: Procedural2dGenerateSurfaceHost,
  }`,
    panelTabs: `{
    workbench: [new Procedural2dPlayHierarchyPanelDefinition(), new Procedural2dPlayCataloguePanelDefinition()],
    details: [new Procedural2dPlayInspectionPanelDefinition()],
  }`,
  },
];

const TRINITY_SPEC = {
  path: "trinity/react/play-host.tsx",
  jackExport: "trinityJackAppRenderer",
  rewriteExport: "trinityRewriteAppRenderer",
};

function stripBootTail(content: string): string {
  return content
    .replace(/\nlet \w+PlayChromeRegistered = false;\n?/g, "\n")
    .replace(/\nexport function register\w+PlaySurfaceHosts\(\): void \{[\s\S]*?\n\}\n/g, "\n")
    .replace(/\nfunction \w+PlayInner\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\nfunction Trinity\w+PlayInner\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\nfunction \w+PlayChrome\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\nexport function mount\w+PlayChrome\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\nconst \w+PlayChromeBoot[\s\S]*?\n\};\n/g, "\n")
    .replace(/\nexport (?:async )?function boot\w+Play\([\s\S]*?\n\}\n/g, "\n")
    .replace(/\n\/\/#endregion 🔖\w+PlayHost\n?$/g, "\n");
}

function fixImports(content: string, label: string): string {
  let c = content;
  c = c.replace(
    /\/\*\* @emoji 🛝 Playground play host for ([^—]+) — loaded only via `\.\/play` subpath\. \*\//,
    `/** @emoji 🛝 ${label} app renderer contribution — loaded only via \`./play\` subpath. */`,
  );
  if (!c.includes("AppRendererContribution")) {
    c = c.replace(
      /import type \{ ReactElement \} from "react";/,
      `import type { ReactElement } from "react";\nimport type { AppRendererContribution } from "@semio-tech/framework-platform-core";`,
    );
  }
  c = c.replace(
    /import \{ type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, /,
    "import { ",
  );
  c = c.replace(
    /import \{ type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, /,
    "import { ",
  );
  c = c.replace(/, registerUi\w+SurfaceHost/g, "");
  c = c.replace(/registerUi\w+SurfaceHost, /g, "");
  c = c.replace(/,\s*register\w+PlayDeclarativeBodies/g, "");
  c = c.replace(/register\w+PlayDeclarativeBodies,?\s*/g, "");
  return c;
}

function appendExport(content: string, spec: Spec): string {
  const lines = [
    `/** @emoji 🛝 ${spec.exportName.replace(/AppRenderer$/, "")} app renderer for playground and OS shells. */`,
    `export const ${spec.exportName}: AppRendererContribution = {`,
    `  surfaceHosts: ${spec.surfaceHosts},`,
  ];
  if (spec.panelTabs) lines.push(`  panelTabs: ${spec.panelTabs},`);
  if (spec.tabIcons) lines.push(`  tabIcons: ${spec.tabIcons},`);
  if (spec.preload) lines.push(`  preload: ${spec.preload},`);
  lines.push("};");
  if (spec.extraExports) lines.push("", spec.extraExports);
  return `${content.trimEnd()}\n\n${lines.join("\n")}\n`;
}

for (const spec of SPECS) {
  const full = join(REPO, spec.path);
  let content = readFileSync(full, "utf8");
  const label = spec.path.split("/")[0] === "procedural" ? spec.path.split("/")[1] === "3d" ? "Procedural" : "Procedural2d" : spec.path.split("/")[0];
  content = fixImports(content, label.charAt(0).toUpperCase() + label.slice(1));
  content = stripBootTail(content);
  content = appendExport(content, spec);
  writeFileSync(full, content);
  console.log(`converted ${spec.path}`);
}

{
  const full = join(REPO, TRINITY_SPEC.path);
  let content = readFileSync(full, "utf8");
  content = fixImports(content, "Trinity");
  content = stripBootTail(content);
  content = `${content.trimEnd()}

/** @emoji 🛝 Trinity Jack app renderer for playground and OS shells. */
export const ${TRINITY_SPEC.jackExport}: AppRendererContribution = {
  surfaceHosts: {
    [TRINITY_JACK_PLAY_SURFACE_ID]: TrinityJackPlaySurfaceHost,
    [TRINITY_JACK_PLAY_EDITOR_SURFACE_ID]: TrinityJackEditorSurfaceHost,
    [TRINITY_JACK_PLAY_RESULTS_SURFACE_ID]: TrinityJackResultsSurfaceHost,
  },
  panelTabs: {
    workbench: [new TrinityJackHierarchyPanelDefinition(), new TrinityJackCataloguePanelDefinition()],
    details: [new TrinityJackInspectionPanelDefinition()],
  },
};

/** @emoji 🛝 Trinity Rewrite app renderer for playground and OS shells. */
export const ${TRINITY_SPEC.rewriteExport}: AppRendererContribution = {
  surfaceHosts: {
    [TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE]: TrinityRewriteBeforeSurfaceHost,
    [TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER]: TrinityRewriteAfterSurfaceHost,
    [TRINITY_REWRITE_PLAY_SURFACE_ID_LHS]: TrinityRewriteLhsSurfaceHost,
    [TRINITY_REWRITE_PLAY_SURFACE_ID_RHS]: TrinityRewriteRhsSurfaceHost,
    [TRINITY_REWRITE_PLAY_SURFACE_ID_JACK]: TrinityRewriteJackSurfaceHost,
    [TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS]: TrinityRewriteParametersSurfaceHost,
  },
  panelTabs: {
    workbench: [new TrinityRewriteHierarchyPanelDefinition(), new TrinityRewriteCataloguePanelDefinition()],
    details: [new TrinityRewriteInspectionPanelDefinition()],
  },
};
`;
  writeFileSync(full, content);
  console.log(`converted ${TRINITY_SPEC.path}`);
}

console.log("DONE");
