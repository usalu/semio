import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const rendererDir = join(root, "framework/playground/renderer/react");
const indexPath = join(rendererDir, "index.tsx");

const HOSTS = [
  { file: "puzzle/3d-play-host.tsx", region: "Puzzle3dPlayHost" },
  { file: "puzzle/5d-play-host.tsx", region: "Puzzle5dPlayHost" },
  { file: "puzzle/2d-play-host.tsx", region: "Puzzle2dPlayHost" },
] as const;

function stripHeader(src: string): string {
  const lines = src.split(/\r?\n/);
  let i = 0;
  if (lines[i]?.includes("#region") && lines[i]?.includes("Header")) {
    i++;
    while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
    i++;
  }
  while (i < lines.length && lines[i]?.trim() === "") i++;
  return lines.slice(i).join("\n");
}

function stripSelfImports(src: string): string {
  return src.replace(/import\s+(?:type\s+)?[\s\S]*?\s+from\s+["']@framework\/playground\/renderer\/react\/(?:shell|boot)["'];?\s*\n/g, "").trim();
}

let index = readFileSync(indexPath, "utf8");
const bootMarker = "//#region 🔖Boot";
const bootIdx = index.indexOf(bootMarker);
if (bootIdx === -1) throw new Error("Boot region not found");

let insert = "";
for (const { file, region } of HOSTS) {
  const raw = readFileSync(join(rendererDir, file), "utf8");
  const body = stripSelfImports(stripHeader(raw));
  insert += `\n//#region 🔖${region}\n${body}\n//#endregion 🔖${region}\n`;
}

index = index.slice(0, bootIdx) + insert + "\n" + index.slice(bootIdx);
writeFileSync(indexPath, index);

for (const { file } of HOSTS) {
  unlinkSync(join(rendererDir, file));
}

console.log("merged puzzle play hosts into index.tsx");
