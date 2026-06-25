#!/usr/bin/env bun
import * as fs from "fs";
import * as path from "path";
import { JSDOM } from "jsdom";

const ORANGE = "#fa9500";
const TEALS = ["#34d1bf", "#00a69d"] as const;
const PINK = "#ff344f";

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

function swapInkscapeLayers(content: string): string {
  const dom = new JSDOM(content, { contentType: "image/svg+xml" });
  const document = dom.window.document;

  for (const group of document.querySelectorAll("g[inkscape\\:label]")) {
    const label = group.getAttribute("inkscape:label");
    if (label !== "i" && label !== "e") {
      continue;
    }

    const target = label === "i" ? ORANGE : TEALS.find((teal) => group.innerHTML.toLowerCase().includes(teal));
    if (!target) {
      continue;
    }

    const replacement = label === "i"
      ? TEALS.find((teal) => group.innerHTML.toLowerCase().includes(teal)) ?? TEALS[0]
      : ORANGE;

    for (const element of group.querySelectorAll("[style]")) {
      const style = element.getAttribute("style");
      if (!style?.includes(target)) {
        continue;
      }
      element.setAttribute("style", style.replaceAll(target, "__SEMIO_SWAP__").replaceAll("__SEMIO_SWAP__", replacement));
    }

    const groupStyle = group.getAttribute("style");
    if (groupStyle?.includes(target)) {
      group.setAttribute("style", groupStyle.replaceAll(target, "__SEMIO_SWAP__").replaceAll("__SEMIO_SWAP__", replacement));
    }
  }

  for (const group of document.querySelectorAll("g[id='layer3'], g[id='layer5']")) {
    const isI = group.getAttribute("id") === "layer3";
    const html = group.innerHTML.toLowerCase();
    const teal = TEALS.find((value) => html.includes(value));
    if (!teal) {
      continue;
    }
    const from = isI ? ORANGE : teal;
    const to = isI ? teal : ORANGE;

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

  return dom.serialize();
}

function swapKitHorizontal(content: string): string {
  if (!content.includes(ORANGE) || !TEALS.some((teal) => content.includes(teal))) {
    return content;
  }
  return swapOrangeTeal(content);
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
  { relative: "asset/logo/kit_horizontal.svg", handler: swapKitHorizontal },
  { relative: "asset/logo/kit_horizontal_dark.svg", handler: swapKitHorizontal },
  { relative: "asset/logo/kit_horizontal_inkscape.svg", handler: swapKitHorizontal },
  { relative: "asset/logo/kit_horizontal_dark_inkscape.svg", handler: swapKitHorizontal },
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
];

for (const { svg, png, size } of pngSources) {
  const svgPath = path.join(repoRoot, svg);
  const pngPath = path.join(repoRoot, png);
  const tempDir = import.meta.dir;
  const proc = Bun.spawnSync([
    "qlmanage",
    "-t",
    "-s",
    String(size),
    "-o",
    tempDir,
    svgPath,
  ]);
  if (proc.exitCode !== 0) {
    console.error(`failed png export for ${png}`);
    continue;
  }
  const generated = path.join(tempDir, `${path.basename(svgPath)}.png`);
  fs.copyFileSync(generated, pngPath);
  fs.unlinkSync(generated);
  console.log(`regenerated ${png}`);
}
