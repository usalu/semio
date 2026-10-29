/** 🧪️ Neutral browser contract and downloaded-media witnesses agree with independent existing readers. */
import assert from "node:assert/strict";
import { test } from "node:test";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { validateMedia, compareEditedGeometry, validateCanvasCapture } from "./media-oracles.mjs";
import { assertSurfaceHealthy } from "./browser-health.mjs";
const require = createRequire(import.meta.url);
const sharp = require("sharp");
const contract = JSON.parse(readFileSync(new URL("./browser-journey.json", import.meta.url), "utf8"));
const svg = edited => Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="400" height="500" viewBox="0 0 400 500"><rect width="400" height="500" fill="white"/><rect x="10" y="10" width="20" height="20" fill="black"/>' + (edited ? '<rect x="251" y="337" width="120" height="64" fill="rgb(51,61,77)"/>' : "") + "</svg>");
const pdf = (text, count = 2) => {
  const pages = Array.from({ length: count }, (_, index) => 4 + index * 2);
  const objects = [
    "<< /Type /Catalog /Pages 2 0 R >>",
    "<< /Type /Pages /Count " + count + " /Kids [" + pages.map(id => id + " 0 R").join(" ") + "] >>",
    "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
  ];
  for (const page of pages) {
    const stream = "BT /F1 12 Tf 20 450 Td (" + text + ") Tj ET";
    objects.push("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 400 500] /Resources << /Font << /F1 3 0 R >> >> /Contents " + (page + 1) + " 0 R >>");
    objects.push("<< /Length " + Buffer.byteLength(stream) + " >>\nstream\n" + stream + "\nendstream");
  }
  let result = "%PDF-1.4\n";
  const offsets = [0];
  objects.forEach((object, index) => {
    offsets.push(Buffer.byteLength(result));
    result += (index + 1) + " 0 obj\n" + object + "\nendobj\n";
  });
  const xref = Buffer.byteLength(result);
  result += "xref\n0 " + (objects.length + 1) + "\n0000000000 65535 f \n" + offsets.slice(1).map(offset => String(offset).padStart(10, "0") + " 00000 n \n").join("");
  result += "trailer\n<< /Size " + (objects.length + 1) + " /Root 1 0 R >>\nstartxref\n" + xref + "\n%%EOF\n";
  return Buffer.from(result);
};
test("portable journey contract requires exact history, edited exports and new-owner reopening", () => {
  assert.equal(contract.version, 2);
  assert.equal(new Set(contract.steps.map(step => step.step)).size, contract.steps.length);
  for (const name of ["boot", "artifact-panel-baseline", "exact-create-undo-redo", "story-marker", "edited-geometry", "export-svg", "export-png", "export-pdf", "reopen-archive"]) assert.ok(contract.steps.some(step => step.step === name));
  assert.deepEqual(contract.frame, { kind: "rect", x: 251, y: 337 });
  const p = contract.geometry.patch;
  assert.ok(p.left > contract.frame.x && p.top > contract.frame.y);
  assert.ok(p.left + p.width < contract.frame.x + 120 && p.top + p.height < contract.frame.y + 64);
  assert.notEqual(contract.marker, contract.demo.stockText);
  assert.notEqual(contract.marker, contract.originalOnlyMarker);
});
test("independent SVG rasterization and PNG decoding prove the edited interior changed", async () => {
  const before = svg(false), after = svg(true);
  const beforePng = await sharp(before).png().toBuffer(), afterPng = await sharp(after).png().toBuffer();
  assert.equal((await validateMedia("svg", after, 2, contract.geometry)).width, 400);
  assert.equal((await validateMedia("png", afterPng, 2, contract.geometry)).height, 500);
  assert.deepEqual(await compareEditedGeometry(before, beforePng, after, afterPng, contract.geometry), { valid: true, pixels: 144, changedSvg: 144, changedPng: 144, agreed: 144, patch: contract.geometry.patch });
  await assert.rejects(compareEditedGeometry(after, afterPng, after, afterPng, contract.geometry), /differs from baseline/);
  await assert.rejects(compareEditedGeometry(before, beforePng, after, afterPng, { ...contract.geometry, color: [255, 0, 0] }), /authored rect fill/);
  await assert.rejects(validateMedia("png", afterPng, 2, { ...contract.geometry, width: 401 }), /page width/);
  assert.equal((await validateCanvasCapture(afterPng)).painted, true);
  const blank = await sharp({ create: { width: 400, height: 500, channels: 3, background: "white" } }).png().toBuffer();
  await assert.rejects(validateCanvasCapture(blank), /nonuniform/);
});
test("pdfjs recovers the authored marker, exact pages and dimensions and rejects stock content", async () => {
  const expected = { ...contract.geometry, text: contract.marker };
  const actual = await validateMedia("pdf", pdf(contract.marker), 2, expected);
  assert.equal(actual.pages, 2);
  assert.ok(actual.text.includes(contract.marker));
  await assert.rejects(validateMedia("pdf", pdf(contract.demo.stockText), 2, expected), /authored text marker/);
  await assert.rejects(validateMedia("pdf", pdf(contract.marker, 1), 2, expected), /page count/);
  await assert.rejects(validateMedia("zip", pdf(contract.marker), 2, expected), /supported media format/);
});

test("actual surface status channels reject malformed or refused payloads without inventing Canvas status", () => {
  const { JSDOM } = require("jsdom");
  const { window } = new JSDOM('<div data-surface-id="fixture"></div>');
  try {
    const node = window.document.querySelector("[data-surface-id]");
    for (const row of contract.surfaceHealthCases) {
      for (const [attribute, value] of [["data-status-json", row.status], ["data-board-status-json", row.board]]) {
        if (value === null) node.removeAttribute(attribute);
        else node.setAttribute(attribute, value);
      }
      const host = { statusJson: node.getAttribute("data-status-json"), boardStatusJson: node.getAttribute("data-board-status-json"), phase: row.phase, fault: row.fault };
      if (row.valid) assert.doesNotThrow(() => assertSurfaceHealthy(host), row.name);
      else assert.throws(() => assertSurfaceHealthy(host), undefined, row.name);
    }
  } finally { window.close(); }
});
