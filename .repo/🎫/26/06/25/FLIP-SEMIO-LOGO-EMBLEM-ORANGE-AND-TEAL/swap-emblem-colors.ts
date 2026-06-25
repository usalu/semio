#!/usr/bin/env bun
import * as fs from "fs";
import * as path from "path";
import { JSDOM } from "jsdom";

const ORANGE = "#fa9500";
const TEALS = ["#34d1bf", "#00a69d"] as const;

const repoRoot = path.resolve(import.meta.dir, "../../../../../..");

function swapOrangeTeal(content: string): string {
  const tealInFile = TEALS.find((teal) => content.toLowerCase().includes(teal));
  if (!tealInFile) {
    return content;
  }
  const placeholder = "__SEMIO_SWAP__";
  return content
    .replaceAll(ORANGE, placeholder)
    .replaceAll(tealInFile, ORANGE)
    .replaceAll(placeholder, tealInFile);
}

function swapCompactEmblem(content: string): string {
  if (!content.includes(ORANGE) || !TEALS.some((teal) => content.includes(teal))) {
    return content;
  }
  return swapOrangeTeal(content);
}

function swapInkscapePart(group: Element): void {
  const html = group.innerHTML.toLowerCase();
  const teal = TEALS.find((value) => html.includes(value));
  const hasOrange = html.includes(ORANGE);
  if (!teal && !hasOrange) {
    return;
  }
  const from = hasOrange ? ORANGE : teal!;
  const to = hasOrange ? (teal ?? TEALS[0]) : ORANGE;

  for (const element of group.querySelectorAll("[style], [fill]")) {
    for (const attribute of ["style", "fill"] as const) {
      const value = element.getAttribute(attribute);
      if (!value?.includes(from)) {
        continue;
      }
      element.setAttribute(attribute, value.replaceAll(from, to));
    }
  }

  const groupStyle = group.getAttribute("style");
  if (groupStyle?.includes(from)) {
    group.setAttribute("style", groupStyle.replaceAll(from, to));
  }
}

function swapInkscapeLayers(content: string): string {
  const dom = new JSDOM(content, { contentType: "image/svg+xml" });
  const document = dom.window.document;

  for (const group of document.querySelectorAll("g[inkscape\\:label]")) {
    const label = group.getAttribute("inkscape:label");
    if (label === "i" || label === "e") {
      swapInkscapePart(group);
    }
  }

  for (const group of document.querySelectorAll("g[id='layer3'], g[id='layer5']")) {
    swapInkscapePart(group);
  }

  return dom.serialize();
}

function swapReactLogo(content: string): string {
  return content
    .replace('fill="#fa9500"', 'fill="__SEMIO_SWAP__"')
    .replace('fill="#34d1bf"', 'fill="#fa9500"')
    .replace('fill="__SEMIO_SWAP__"', 'fill="#34d1bf"');
}

const files: Array<{ relative: string; handler: (content: string) => string }> = [
  { relative: "asset/logo/emblem.svg", handler: swapCompactEmblem },
  { relative: "asset/logo/emblem_dark.svg", handler: swapCompactEmblem },
  { relative: "asset/logo/emblem_round.svg", handler: swapCompactEmblem },
  { relative: "asset/logo/emblem_dark_round.svg", handler: swapCompactEmblem },
  { relative: "asset/icon/semio.svg", handler: swapCompactEmblem },
  { relative: "asset/logo/emblem_inkscape.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/emblem_dark_inkscape.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/emblem_round_inkscape.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/emblem_dark_round_inkscape.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/emblem_codeicon_inkscape.svg", handler: swapInkscapeLayers },
  { relative: "asset/icon/semio_inkscape.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/semio.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/semio_horizontal.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/semio_horizontal_dark.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/semio_socialpreview.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/logo.svg", handler: swapInkscapeLayers },
  { relative: "asset/logo/kit_horizontal.svg", handler: swapCompactEmblem },
  { relative: "asset/logo/kit_horizontal_dark.svg", handler: swapCompactEmblem },
  { relative: "asset/logo/kit_horizontal_inkscape.svg", handler: swapCompactEmblem },
  { relative: "asset/logo/kit_horizontal_dark_inkscape.svg", handler: swapCompactEmblem },
  { relative: "ui/react/index.tsx", handler: swapReactLogo },
];

for (const { relative, handler } of files) {
  const filePath = path.join(repoRoot, relative);
  const before = fs.readFileSync(filePath, "utf-8");
  const after = handler(before);
  if (after !== before) {
    fs.writeFileSync(filePath, after);
    console.log(`updated ${relative}`);
  } else {
    console.log(`skipped ${relative}`);
  }
}

const pngSources: Array<{ svg: string; png: string; size: number }> = [
  { svg: "asset/logo/emblem.svg", png: "asset/logo/emblem.png", size: 512 },
  { svg: "asset/logo/emblem.svg", png: "asset/logo/emblem_24x24.png", size: 24 },
  { svg: "asset/logo/emblem.svg", png: "asset/logo/emblem_1920x1920.png", size: 1920 },
  { svg: "asset/logo/emblem_dark.svg", png: "asset/logo/emblem_dark.png", size: 512 },
  { svg: "asset/logo/emblem_dark.svg", png: "asset/logo/emblem_dark_24x24.png", size: 24 },
  { svg: "asset/logo/emblem_dark.svg", png: "asset/logo/emblem_dark_1920x1920.png", size: 1920 },
  { svg: "asset/logo/emblem_round.svg", png: "asset/logo/emblem_round.png", size: 512 },
  { svg: "asset/logo/emblem_round.svg", png: "asset/logo/emblem_round_24x24.png", size: 24 },
  { svg: "asset/logo/emblem_round.svg", png: "asset/logo/emblem_round_1920x1920.png", size: 1920 },
  { svg: "asset/logo/emblem_dark_round.svg", png: "asset/logo/emblem_dark_round.png", size: 512 },
  { svg: "asset/logo/emblem_dark_round.svg", png: "asset/logo/emblem_dark_round_24x24.png", size: 24 },
  { svg: "asset/logo/emblem_dark_round.svg", png: "asset/logo/emblem_dark_round_1920x1920.png", size: 1920 },
  { svg: "asset/logo/kit_horizontal.svg", png: "asset/logo/compose.png", size: 512 },
];

for (const { svg, png, size } of pngSources) {
  const svgPath = path.join(repoRoot, svg);
  const pngPath = path.join(repoRoot, png);
  const tempDir = import.meta.dir;
  const proc = Bun.spawnSync(["qlmanage", "-t", "-s", String(size), "-o", tempDir, svgPath]);
  if (proc.exitCode !== 0) {
    console.error(`failed png export for ${png}`);
    continue;
  }
  const generated = path.join(tempDir, `${path.basename(svgPath)}.png`);
  fs.copyFileSync(generated, pngPath);
  fs.unlinkSync(generated);
  console.log(`regenerated ${png}`);
}

const emblemSvg = path.join(repoRoot, "asset/logo/emblem.svg");
const ql = Bun.spawnSync(["qlmanage", "-t", "-s", "32", "-o", "/tmp", emblemSvg]);
if (ql.exitCode === 0) {
  const png = fs.readFileSync("/tmp/emblem.svg.png");
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0);
  header.writeUInt16LE(1, 2);
  header.writeUInt16LE(1, 4);
  const entry = Buffer.alloc(16);
  entry[0] = 32;
  entry[1] = 32;
  entry[4] = 1;
  entry[5] = 32;
  entry.writeUInt32LE(png.length, 8);
  entry.writeUInt32LE(22, 12);
  const ico = Buffer.concat([header, entry, png]);
  fs.writeFileSync(path.join(repoRoot, "asset/logo/favicon_32x32.ico"), ico);
  console.log("regenerated asset/logo/favicon_32x32.ico");
}

for (const rel of [
  "compose/client/lib/sketchpad/doc/public/favicon.svg",
  "compose/client/lib/sketchpad/play/public/favicon.svg",
  "compose/client/lib/sketchpad/js/public/favicon.svg",
  "compose/client/lib/sketchpad/play/public/favicon.ico",
  "compose/client/lib/sketchpad/js/public/favicon.ico",
]) {
  const dest = path.join(repoRoot, rel);
  if (rel.endsWith(".svg")) {
    fs.copyFileSync(emblemSvg, dest);
  } else {
    fs.copyFileSync(path.join(repoRoot, "asset/logo/favicon_32x32.ico"), dest);
  }
  console.log(`copied ${rel}`);
}
