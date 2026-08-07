import { readFileSync, writeFileSync } from "fs";
const path = process.argv[2];
let text = readFileSync(path, "utf8");

const oldRegistry = `//#region 🔖️UiInterpreter
//#region ComponentSceneHostRegistry
/** 🧩️ Wraps a dynamic host-module import into a lazily-loaded component bound to a named export. */
function lazyHost<P>(loader: () => Promise<Record<string, unknown>>, exportName: string): LazyExoticComponent<ComponentType<P>> {
  return lazy(async () => {
    const module = await loader();
    return { default: module[exportName] as ComponentType<P> };
  });
}

const COMPONENT_SCENE_HOSTS: Record<ComponentKind, LazyExoticComponent<ComponentType<ComponentSceneHostProps>>> = {
  "canvas-2d": lazyHost(() => Promise.resolve({ Canvas2dHost }), "Canvas2dHost"),
  "world-3d": lazyHost(() => Promise.resolve({ World3dHost }), "World3dHost"),
  "node-graph": lazyHost(() => Promise.resolve({ NodeGraphHost }), "NodeGraphHost"),
  "text-editor": lazyHost(() => Promise.resolve({ TextEditorHost }), "TextEditorHost"),
  table: lazyHost(() => Promise.resolve({ TableHost }), "TableHost"),
  "paint-2d": lazyHost(() => Promise.resolve({ Paint2dHost }), "Paint2dHost"),
  "tiled-map": lazyHost(() => Promise.resolve({ TiledMapHost }), "TiledMapHost"),
  "board-2d": lazyHost(() => Promise.resolve({ Board2dHost }), "Board2dHost"),
  "icon-render": lazyHost(() => Promise.resolve({ IconRenderHost }), "IconRenderHost"),
  "ink-canvas": lazyHost(() => Promise.resolve({ InkCanvasHost }), "InkCanvasHost"),
  "graph-timeline": lazyHost(() => Promise.resolve({ GraphTimelineHost }), "GraphTimelineHost"),
  "block-list": lazyHost(() => Promise.resolve({ BlockListHost }), "BlockListHost"),
  "diff-view": lazyHost(() => Promise.resolve({ DiffViewHost }), "DiffViewHost"),
  "event-feed": lazyHost(() => Promise.resolve({ EventFeedHost }), "EventFeedHost"),
};
//#endregion ComponentSceneHostRegistry`;

const newRegistry = `//#region 🔖️UiInterpreter
//#region ComponentSceneHostRegistry
/** 🧭️ Resolve scene hosts at render time — these modules form a cycle with Interpreter
 * (\`World3dHost\` imports \`openSurfaceContextMenu\` from here), so a module-init
 * \`Record\` / fake \`React.lazy(Promise.resolve({ Host }))\` can capture \`undefined\`
 * and leave Suspense forever on "Loading surface…". Live bindings are ready by first paint. */
function resolveComponentSceneHost(kind: ComponentKind): ComponentType<ComponentSceneHostProps> | undefined {
  switch (kind) {
    case "canvas-2d":
      return Canvas2dHost;
    case "world-3d":
      return World3dHost;
    case "node-graph":
      return NodeGraphHost;
    case "text-editor":
      return TextEditorHost;
    case "table":
      return TableHost;
    case "paint-2d":
      return Paint2dHost;
    case "tiled-map":
      return TiledMapHost;
    case "board-2d":
      return Board2dHost;
    case "icon-render":
      return IconRenderHost;
    case "ink-canvas":
      return InkCanvasHost;
    case "graph-timeline":
      return GraphTimelineHost;
    case "block-list":
      return BlockListHost;
    case "diff-view":
      return DiffViewHost;
    case "event-feed":
      return EventFeedHost;
    default:
      return undefined;
  }
}
//#endregion ComponentSceneHostRegistry`;

if (!text.includes(oldRegistry)) {
  console.error("registry block not found");
  const idx = text.indexOf("ComponentSceneHostRegistry");
  console.error(text.slice(idx, idx + 900));
  process.exit(1);
}
text = text.replace(oldRegistry, newRegistry);

const oldRender = `  const Host = COMPONENT_SCENE_HOSTS[node.componentKind as ComponentKind];
  if (!Host) {
    return (
      <p className="text-muted-foreground text-xs">
        {interpLabel("ui.common.unknownComponent")}: {node.componentKind}
      </p>
    );
  }
  return (
    <Suspense fallback={<ComponentSceneFallback />}>
      <ShellFaultBoundary boundaryId={\`surface-\${node.componentKind}\`} fallbackLabel={shellLabel("ui.common.renderError")}>
        <Host node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
      </ShellFaultBoundary>
    </Suspense>
  );`;

const newRender = `  const Host = resolveComponentSceneHost(node.componentKind as ComponentKind);
  if (!Host) {
    console.log("[DEBUG] resolveComponentSceneHost miss", node.componentKind);
    return (
      <p className="text-muted-foreground text-xs">
        {interpLabel("ui.common.unknownComponent")}: {node.componentKind}
      </p>
    );
  }
  return (
    <ShellFaultBoundary boundaryId={\`surface-\${node.componentKind}\`} fallbackLabel={shellLabel("ui.common.renderError")}>
      <Host node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
    </ShellFaultBoundary>
  );`;

if (!text.includes(oldRender)) {
  console.error("render block not found");
  process.exit(1);
}
text = text.replace(oldRender, newRender);

// Clean unused lazy import if no longer used
if (!text.includes("lazy(") && !text.includes("lazyHost") && !/\blazy\b/.test(text.replace(/LazyExoticComponent/g, ""))) {
  text = text.replace(
    `import { createContext, lazy, memo, Suspense, useCallback, useContext, useMemo, useState, type ComponentType, type LazyExoticComponent, type ReactElement, type ReactNode } from "react";`,
    `import { createContext, memo, Suspense, useCallback, useContext, useMemo, useState, type ComponentType, type ReactElement, type ReactNode } from "react";`,
  );
} else {
  // still uses Suspense elsewhere; drop only lazy + LazyExoticComponent if unused
  const usesLazy = /\blazy\s*\(/.test(text) || /\blazyHost\b/.test(text);
  const usesLazyType = /\bLazyExoticComponent\b/.test(text);
  if (!usesLazy && !usesLazyType) {
    text = text.replace(
      `import { createContext, lazy, memo, Suspense, useCallback, useContext, useMemo, useState, type ComponentType, type LazyExoticComponent, type ReactElement, type ReactNode } from "react";`,
      `import { createContext, memo, Suspense, useCallback, useContext, useMemo, useState, type ComponentType, type ReactElement, type ReactNode } from "react";`,
    );
  } else if (!usesLazy && usesLazyType) {
    // keep as-is
  } else if (!usesLazy) {
    text = text.replace(", lazy,", ",").replace("type LazyExoticComponent, ", "");
  }
}

writeFileSync(path, text);
console.log("patched interpreter hosts");
