#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { $ } from "bun";

const root = join(import.meta.dir, "../../../../../..");
const commit = "6e1ceb137";
const ts = await $`git show ${commit}:elements/lib/react/board/index.ts`.text();
const tsx = await $`git show ${commit}:elements/lib/react/board/index.tsx`.text();
const host = await $`git show ${commit}:elements/lib/react/board/board-play-host.tsx`.text();

function stripTsxReexports(content: string): string {
  const lines = content.split(/\r?\n/);
  const out: string[] = [];
  let skip = false;
  for (const line of lines) {
    if (line.startsWith("import {") && line.includes('from "./index"')) {
      skip = true;
      continue;
    }
    if (skip) {
      if (line.startsWith("import {") && line.includes("@elements/ui")) {
        skip = false;
        out.push(line);
      }
      continue;
    }
    if (line.startsWith("export {") && line.includes('from "./index"')) continue;
    if (line.startsWith("export type {") && line.includes('from "./index"')) continue;
    out.push(line);
  }
  return out.join("\n");
}

function stripHostImports(content: string): string {
  return content.replace(/import \{[\s\S]*?\} from "\.\/index";\r?\n/, "").replace(/import \{ BoardCanvas, Edge, Handle, Node, useBoardEvent \} from "\.\/index\.tsx";\r?\n/, "");
}

function renameSceneClasses(content: string): string {
  let s = content;
  s = s.replace(/^export class Node extends/m, "class BoardSceneNode extends");
  s = s.replace(/^export class Handle extends/m, "class BoardSceneHandle extends");
  s = s.replace(/^export class Edge extends/m, "class BoardSceneEdge extends");
  s = s.replace(/^export class Wire extends/m, "class BoardSceneWire extends");
  for (const [from, to] of [
    ["Node", "BoardSceneNode"],
    ["Handle", "BoardSceneHandle"],
    ["Edge", "BoardSceneEdge"],
    ["Wire", "BoardSceneWire"],
  ] as const) {
    s = s.replaceAll(`new ${from}(`, `new ${to}(`);
    s = s.replaceAll(`instanceof ${from}`, `instanceof ${to}`);
    s = s.replaceAll(`Map<string, ${from}>`, `Map<string, ${to}>`);
    s = s.replaceAll(`${from}[]`, `${to}[]`);
  }
  s = s.replaceAll(": Node", ": BoardSceneNode");
  s = s.replaceAll(": Handle", ": BoardSceneHandle");
  s = s.replaceAll(": Edge", ": BoardSceneEdge");
  s = s.replaceAll(": Wire", ": BoardSceneWire");
  s = s.replaceAll("<Node>", "<BoardSceneNode>");
  s = s.replaceAll("<Handle>", "<BoardSceneHandle>");
  s = s.replaceAll("<Edge>", "<BoardSceneEdge>");
  s = s.replaceAll("<Wire>", "<BoardSceneWire>");
  s = s.replaceAll("(Node)", "(BoardSceneNode)");
  s = s.replaceAll("(Handle)", "(BoardSceneHandle)");
  s = s.replaceAll("(Edge)", "(BoardSceneEdge)");
  s = s.replaceAll("(Wire)", "(BoardSceneWire)");
  return s;
}

const tsRenamed = renameSceneClasses(ts);
const tsxBody = stripTsxReexports(tsx).replace(/^[\s\S]*?\/\/ #region 🧲️Header[\s\S]*?#endregion 🧲️Header\n\n?/, "");
const hostBody = stripHostImports(host)
  .replace(/^[\s\S]*?#endregion 🧲️Header\n\n?/, "")
  .replace(/^\/\/ #region 📥️Imports[\s\S]*?\/\/ #endregion 📥️Imports\n\n?/, "");

const merged = `// #region 🧲️Header
/** @emoji 📋️ \`@elements/board\` — WASM board renderer + React canvas + play harness (monolith). */
// #endregion 🧲️Header

${tsRenamed.replace(
  "//#endregion 🔖️Objects",
  `//#endregion 🔖️Objects

type BoardNodeObject = BoardSceneNode;
type BoardHandleObject = BoardSceneHandle;
type BoardEdgeObject = BoardSceneEdge;
type BoardWireObject = BoardSceneWire;`,
)}

// #region 🎨️ReactCanvas
${tsxBody}

// #region 🛝️PlayHost
${hostBody}
// #endregion 🛝️PlayHost
`;

writeFileSync(join(root, "elements/lib/react/board/index.tsx"), merged, "utf8");
console.log("rebuilt board index.tsx", merged.length);
