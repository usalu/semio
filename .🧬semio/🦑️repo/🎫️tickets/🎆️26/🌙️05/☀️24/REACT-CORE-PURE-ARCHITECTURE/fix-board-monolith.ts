#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");
const path = join(root, "elements/lib/react/board/index.tsx");
let s = readFileSync(path, "utf8");

s = s.replace(/^export class Node extends/m, "class BoardSceneNode extends");
s = s.replace(/^export class Handle extends/m, "class BoardSceneHandle extends");
s = s.replace(/^export class Edge extends/m, "class BoardSceneEdge extends");
s = s.replace(/^export class Wire extends/m, "class BoardSceneWire extends");

const classMap: Record<string, string> = {
  Node: "BoardSceneNode",
  Handle: "BoardSceneHandle",
  Edge: "BoardSceneEdge",
  Wire: "BoardSceneWire",
};

for (const [from, to] of Object.entries(classMap)) {
  s = s.replaceAll(`new ${from}(`, `new ${to}(`);
  s = s.replaceAll(`instanceof ${from}`, `instanceof ${to}`);
  s = s.replaceAll(`Map<string, ${from}>`, `Map<string, ${to}>`);
  s = s.replaceAll(`${from}[]`, `${to}[]`);
  s = s.replaceAll(`: ${from}`, `: ${to}`);
  s = s.replaceAll(`<${from}>`, `<${to}>`);
  s = s.replaceAll(`(${from})`, `(${to})`);
  s = s.replaceAll(` ${from} `, ` ${to} `);
}

s = s.replace(
  "//#endregion 🔖️Objects\n\n//#region 🔖️Scene",
  `//#endregion 🔖️Objects\n\ntype BoardNodeObject = BoardSceneNode;\ntype BoardHandleObject = BoardSceneHandle;\ntype BoardEdgeObject = BoardSceneEdge;\ntype BoardWireObject = BoardSceneWire;\n\n//#region 🔖️Scene`,
);

const syncRenames: [string, string][] = [
  ["function applyNodeProps(renderer: BoardRenderer, instance: BoardNodeObject", "function syncNodeFromDescriptor(renderer: BoardRenderer, instance: BoardNodeObject"],
  ["function applyHandleProps(instance: BoardHandleObject", "function syncHandleFromDescriptor(instance: BoardHandleObject"],
  ["function applyEdgeProps(instance: BoardEdgeObject", "function syncEdgeFromDescriptor(instance: BoardEdgeObject"],
  ["function applyWireProps(instance: BoardWireObject", "function syncWireFromDescriptor(instance: BoardWireObject"],
  ["function nodeShapeSyncKey(descriptor: NodeDescriptor)", "function descriptorNodeShapeKey(descriptor: NodeDescriptor)"],
  ["function instanceShapeSyncKey(node: BoardNodeObject)", "function boardNodeInstanceShapeKey(node: BoardNodeObject)"],
];
for (const [from, to] of syncRenames) {
  s = s.replace(from, to);
}
s = s.replaceAll("applyNodeProps(renderer, node, nodeDescriptor)", "syncNodeFromDescriptor(renderer, node, nodeDescriptor)");
s = s.replaceAll("applyHandleProps(", "syncHandleFromDescriptor(");
s = s.replaceAll("applyEdgeProps(", "syncEdgeFromDescriptor(");
s = s.replaceAll("applyWireProps(", "syncWireFromDescriptor(");
s = s.replaceAll("nodeShapeSyncKey(nodeDescriptor)", "descriptorNodeShapeKey(nodeDescriptor)");
s = s.replaceAll("instanceShapeSyncKey(existingNode)", "boardNodeInstanceShapeKey(existingNode)");

const reactImports = `import { FiberProvider as HostMountProvider, useContextBridge as useHostMountBridge } from "its-fine";
import {
  Children,
  Fragment,
  act,
  createContext,
  createElement,
  isValidElement,
  useCallback,
  useContext,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  useSyncExternalStore,
  type CSSProperties,
  type DragEvent,
  type ReactNode,
} from "react";
import { createRoot } from "react-dom/client";
import { ContextMenuController } from "@elements/ui";
`;

if (!s.includes("useHostMountBridge")) {
  s = s.replace(
    'import type { ContextMenuItem } from "@elements/ui";\nimport type { ReactElement } from "react";\nimport React from "react";',
    `import type { ContextMenuItem } from "@elements/ui";\nimport type { ReactElement } from "react";\nimport React from "react";\n${reactImports}`,
  );
}

s = s.replace(/(\/\/#endregion 🔖️HostMountInternals\n\n)\/\/ #region 🎨️ReactCanvas\nimport[\s\S]*?import \{ ContextMenuController, type ContextMenuItem \} from "@elements\/ui";\n\n/, "$1");

writeFileSync(path, s, "utf8");
console.log("board monolith fixed");
