import { readdirSync, readFileSync, writeFileSync, mkdirSync, existsSync, statSync } from "fs";
import { join, dirname, relative, basename } from "path";

const ticket = process.argv[2];
if (!ticket) throw new Error("pass ticket dir");
const paths = JSON.parse(readFileSync(join(ticket, "scratch-w6-paths.json"), "utf8"));
const { barrel, coreDir, elDir } = paths;

function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  if (!hits.length) throw new Error("cannot resolve " + bare);
  hits.sort((a, b) => a.length - b.length);
  return join(parent, hits[0]);
}
function compFile(dir) {
  const name = readdirSync(dir).find((n) => n.endsWith("component.tsx"));
  if (!name) throw new Error("no tsx in " + dir);
  return join(dir, name);
}
const rel = (fromDir, toFile) => {
  let r = relative(fromDir, toFile).replaceAll("\\", "/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};

const portsFile = compFile(resolveUnder(coreDir, "Ports"));
const classNamesFile = compFile(resolveUnder(coreDir, "ClassNames"));
const uiLabelFile = compFile(resolveUnder(coreDir, "UiLabel"));
const surfaceFile = compFile(resolveUnder(coreDir, "Surface"));
const labelDir = resolveUnder(coreDir, "Label");
const cnHeader = readFileSync(classNamesFile, "utf8").split("\n").slice(0, 6);
function headerCore(dirName) {
  return [cnHeader[0], cnHeader[1].replace(/core\/[^/]+/, "core/" + dirName), ...cnHeader.slice(2)].join("\n");
}
const btnDir = join(elDir, readdirSync(elDir).find((n) => n.endsWith("Button") && !n.includes("Group")));
const W3_LINE = readFileSync(compFile(btnDir), "utf8").split("\n").find((l) => l.includes("W3-interim"));

let lines = readFileSync(barrel, "utf8").split("\n");
const log = [];
function findLine(re, from = 0) { for (let i = from; i < lines.length; i++) if (re.test(lines[i])) return i; return -1; }
function extractBlockByBrace(startLineIdx) {
  let depth = 0, started = false;
  for (let i = startLineIdx; i < lines.length; i++) {
    for (const ch of lines[i]) { if (ch === "{") { depth++; started = true; } if (ch === "}") depth--; }
    if (started && depth === 0) return { start: startLineIdx, end: i };
  }
  throw new Error("unclosed at " + (startLineIdx + 1));
}
function replaceRange(start, end, replacementLines) { lines.splice(start, end - start + 1, ...replacementLines); }

// ---- FLOW ----
{
  const start = findLine(/^export type FlowInline/);
  const useFlow = findLine(/^export function useFlow\(/, start);
  const end = extractBlockByBrace(useFlow).end;
  const body = lines.slice(start, end + 1).join("\n");
  const dirName = "\u{1F9ED}\uFE0FFlow";
  // resolve actual emoji prefix from Surface sibling naming
  const surfaceBase = basename(dirname(surfaceFile));
  const flowDirName = surfaceBase.replace("Surface", "Flow");
  const flowDir = join(coreDir, flowDirName);
  mkdirSync(flowDir, { recursive: true });
  const flowFile = join(flowDir, "\u{1F7E2}\uFE0Fcomponent.tsx");
  const realComp = basename(classNamesFile);
  const flowFileReal = join(flowDir, realComp);
  const content = [
    headerCore(flowDirName),
    "",
    "// #region \u{1F50C}\uFE0FAdapters",
    'import * as React from "react";',
    'import { reactHostPort } from "' + rel(flowDir, portsFile) + '";',
    "// #endregion \u{1F50C}\uFE0FAdapters",
    "",
    "// #region \u{1F9ED}\uFE0FFlow",
    body,
    "// #endregion \u{1F9ED}\uFE0FFlow",
    "",
  ].join("\n");
  // Fix region markers using real emoji from existing files
  const adaptersOpen = readFileSync(compFile(btnDir), "utf8").split("\n").find(l => l.startsWith("// #region ") && l.includes("Adapters"));
  const adaptersClose = readFileSync(compFile(btnDir), "utf8").split("\n").find(l => l.startsWith("// #endregion ") && l.includes("Adapters"));
  const content2 = [
    headerCore(flowDirName), "", adaptersOpen,
    'import * as React from "react";',
    'import { reactHostPort } from "' + rel(flowDir, portsFile) + '";',
    adaptersClose, "",
    "// #region Flow",
    body,
    "// #endregion Flow",
  ].join("\n");
  writeFileSync(flowFileReal, content2);
  const importPath = rel(dirname(barrel), flowFileReal);
  replaceRange(start, end, [
    'import { type FlowInline, type FlowBlock, type Flow, FlowProvider, useFlow } from "' + importPath + '";',
    "export { type FlowInline, type FlowBlock, type Flow, FlowProvider, useFlow };",
  ]);
  log.push("Flow -> " + flowFileReal);
}