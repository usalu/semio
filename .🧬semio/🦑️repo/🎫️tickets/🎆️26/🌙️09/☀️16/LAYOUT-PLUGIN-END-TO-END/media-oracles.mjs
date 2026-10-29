import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
import { dirname, join } from "node:path";
const require = createRequire(import.meta.url);

export async function validateMedia(format, bytes, expectedPages, expected = {}) {
  assert.ok(["svg", "png", "pdf"].includes(format), "supported media format");
  assert.ok(bytes.length > 32, "nonempty media payload");
  if (format === "svg") {
    const { JSDOM } = require("jsdom");
    const document = new JSDOM(bytes.toString("utf8"), { contentType: "image/svg+xml" }).window.document;
    assert.equal(document.documentElement.localName, "svg");
    assert.equal(document.documentElement.namespaceURI, "http://www.w3.org/2000/svg");
    assert.ok(document.querySelector("path,rect,text,image,polygon,line"), "painted SVG content");
  }
  if (format === "svg" || format === "png") {
    const sharp = require("sharp");
    const metadata = await sharp(bytes).metadata();
    assert.ok(metadata.width > 0 && metadata.height > 0, "positive raster dimensions");
    if (expected.width !== undefined) assert.equal(metadata.width, expected.width, "authored page width");
    if (expected.height !== undefined) assert.equal(metadata.height, expected.height, "authored page height");
    const stats = await sharp(bytes).removeAlpha().stats();
    assert.ok(stats.channels.some((channel) => channel.max > channel.min), "nonuniform page pixels");
    if (format === "png") assert.equal(metadata.format, "png");
    return { valid: true, bytes: bytes.length, width: metadata.width, height: metadata.height };
  }
  const canvas = require("@napi-rs/canvas");
  for (const key of ["DOMMatrix", "ImageData", "Path2D"]) if (typeof globalThis[key] === "undefined") globalThis[key] = canvas[key];
  const pdf = await import("pdfjs-dist/legacy/build/pdf.mjs");
  pdf.GlobalWorkerOptions.workerSrc = pathToFileURL(require.resolve("pdfjs-dist/legacy/build/pdf.worker.mjs")).href;
  const document = await pdf.getDocument({ data: new Uint8Array(bytes), disableFontFace: true, isEvalSupported: false, standardFontDataUrl: join(dirname(require.resolve("pdfjs-dist/package.json")), "standard_fonts") + "/" }).promise;
  try {
    assert.equal(document.numPages, expectedPages, "exported PDF page count");
    const page = await document.getPage(1);
    const text = (await page.getTextContent()).items.filter((item) => "str" in item).map((item) => item.str).join(" ");
    assert.ok(text.includes(expected.text ?? "Hello layout"), "PDF contains the authored text marker");
    const viewport = page.getViewport({ scale: 1 });
    if (expected.width !== undefined) assert.equal(viewport.width, expected.width, "PDF page width");
    if (expected.height !== undefined) assert.equal(viewport.height, expected.height, "PDF page height");
    return { valid: true, bytes: bytes.length, pages: document.numPages, text };
  } finally { await document.destroy(); }
}

async function patch(bytes, expected) {
  const sharp = require("sharp");
  const metadata = await sharp(bytes).metadata();
  assert.equal(metadata.width, expected.width, "patch source page width");
  assert.equal(metadata.height, expected.height, "patch source page height");
  const { data, info } = await sharp(bytes).removeAlpha().extract(expected.patch).raw().toBuffer({ resolveWithObject: true });
  assert.equal(info.channels, 3, "RGB patch");
  return data;
}

export async function compareEditedGeometry(beforeSvg, beforePng, afterSvg, afterPng, expected) {
  const sources = await Promise.all([beforeSvg, beforePng, afterSvg, afterPng].map(bytes => patch(bytes, expected)));
  const [svgBefore, pngBefore, svgAfter, pngAfter] = sources;
  let changedSvg = 0, changedPng = 0, agreed = 0;
  const pixels = svgAfter.length / 3;
  for (let at = 0; at < svgAfter.length; at += 3) {
    let svgChanged = false, pngChanged = false;
    for (let channel = 0; channel < 3; channel++) {
      assert.ok(Math.abs(svgAfter[at + channel] - expected.color[channel]) <= expected.tolerance, "SVG interior has authored rect fill");
      assert.ok(Math.abs(pngAfter[at + channel] - expected.color[channel]) <= expected.tolerance, "PNG interior has authored rect fill");
      svgChanged ||= Math.abs(svgBefore[at + channel] - svgAfter[at + channel]) >= expected.minimumChange;
      pngChanged ||= Math.abs(pngBefore[at + channel] - pngAfter[at + channel]) >= expected.minimumChange;
    }
    if (svgChanged) changedSvg++;
    if (pngChanged) changedPng++;
    if ([0, 1, 2].every(channel => Math.abs(svgAfter[at + channel] - pngAfter[at + channel]) <= expected.tolerance)) agreed++;
  }
  assert.ok(changedSvg / pixels >= expected.minimumChangedFraction, "edited SVG patch differs from baseline");
  assert.ok(changedPng / pixels >= expected.minimumChangedFraction, "edited PNG patch differs from baseline");
  assert.equal(agreed, pixels, "independent SVG rasterization agrees with PNG interior");
  return { valid: true, pixels, changedSvg, changedPng, agreed, patch: expected.patch };
}

export async function validateCanvasCapture(bytes) {
  const sharp = require("sharp");
  const metadata = await sharp(bytes).metadata();
  assert.ok(metadata.width > 100 && metadata.height > 100, "visible canvas dimensions");
  const stats = await sharp(bytes).removeAlpha().stats();
  const contrast = Math.max(...stats.channels.map(channel => channel.max - channel.min));
  const deviation = Math.max(...stats.channels.map(channel => channel.stdev));
  assert.ok(contrast > 16 && deviation > 2, "canvas paints a nonuniform document");
  return { width: metadata.width, height: metadata.height, painted: true, contrast, deviation };
}
