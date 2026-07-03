#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { execFileSync } from "node:child_process";

const REPO = "/Users/ueli/Documents/semio";
const path = join(REPO, "puzzle/2d/react/play-host.tsx");
const oldContent = execFileSync("git", ["show", "HEAD:puzzle/2d/react/play-host.tsx"], { cwd: REPO, encoding: "utf8" });
const current = readFileSync(path, "utf8");

const innerStart = oldContent.indexOf("\nfunction Puzzle2dPlayInner(");
const innerEnd = oldContent.indexOf("\nexport function boot2dPlay(");
if (innerStart < 0 || innerEnd < 0) throw new Error("Puzzle2dPlayInner not found in git HEAD");

const innerBlock = oldContent.slice(innerStart, innerEnd);

const exportIdx = current.indexOf("/** @emoji 🛝 Puzzle 2D app renderer");
if (exportIdx < 0) throw new Error("puzzle2dAppRenderer export not found");

const head = current.slice(0, exportIdx).trimEnd();
const tail = current.slice(exportIdx);

const importsPatch = head.includes("PlaygroundView")
  ? head
  : head.replace(
      'import { PureSidePanelTabDefinition',
      'import { type Playground, mountPlaygroundApp, PlaygroundView, PureSidePanelTabDefinition',
    );

const merged = `${importsPatch}\n${innerBlock}

function puzzle2dMountChrome({ runtime, appId, panelTabs }: import("@semio-tech/framework-platform-core").PlaygroundMountProps) {
  return <Puzzle2dPlayInner puzzle2dRuntime={runtime as Platform} />;
}

${tail.replace(
  "export const puzzle2dAppRenderer: AppRendererContribution = {",
  `export const puzzle2dAppRenderer: AppRendererContribution = {
  mountChrome: puzzle2dMountChrome,`,
)}`;

writeFileSync(path, merged);
console.log("Restored Puzzle2dPlayInner from git HEAD");
