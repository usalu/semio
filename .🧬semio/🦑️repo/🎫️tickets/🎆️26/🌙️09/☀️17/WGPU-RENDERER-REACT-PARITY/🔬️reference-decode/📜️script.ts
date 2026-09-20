import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const outputRoot = join(import.meta.dir, "../🗑️generated/astra-reference-decode/browser-codec-oracle");
const decoderPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts");
const intrinsicFixturePath = join(repoRoot, "🧰️framework/🔨️modules/📏️intrinsic-size/🧫️fixtures/🔣️.json");
const planPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg");

type CodecCase = { readonly id: string; readonly mediaType: string; readonly bytesBase64: string };

function exifOrientation(source: Uint8Array, orientation: 6 | 8): Uint8Array {
  const exif = new Uint8Array([0xff, 0xe1, 0, 34, 69, 120, 105, 102, 0, 0, 73, 73, 42, 0, 8, 0, 0, 0, 1, 0, 18, 1, 3, 0, 1, 0, 0, 0, orientation, 0, 0, 0, 0, 0, 0, 0]);
  const bytes = new Uint8Array(source.length + exif.length);
  bytes.set(source.subarray(0, 2));
  bytes.set(exif, 2);
  bytes.set(source.subarray(2), 2 + exif.length);
  return bytes;
}

function bmpFixture(): Uint8Array {
  const bytes = new Uint8Array(78);
  const view = new DataView(bytes.buffer);
  bytes.set([0x42, 0x4d]);
  view.setUint32(2, bytes.length, true);
  view.setUint32(10, 54, true);
  view.setUint32(14, 40, true);
  view.setInt32(18, 2, true);
  view.setInt32(22, 3, true);
  view.setUint16(26, 1, true);
  view.setUint16(28, 24, true);
  view.setUint32(34, 24, true);
  bytes.set([0, 0, 255, 0, 255, 0, 0, 0, 255, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0], 54);
  return bytes;
}

async function inputs(): Promise<CodecCase[]> {
  const fixture = JSON.parse(await readFile(intrinsicFixturePath, "utf8"));
  const raster = (name: string) => Uint8Array.from(fixture.rasterCases.find((entry: { name: string }) => entry.name === name).bytes);
  const plan = new Uint8Array(await readFile(planPath));
  const svg = new TextEncoder().encode(`<svg xmlns="http://www.w3.org/2000/svg" width="3" height="2" viewBox="0 0 3 2"><rect width="2" height="2" fill="#ff0000"/><rect x="2" width="1" height="2" fill="#0000ff"/></svg>`);
  return [
    ["png", "image/png", raster("png-2x3")],
    ["gif", "image/gif", raster("gif-1x1")],
    ["webp", "image/webp", raster("webp-vp8l-2x3")],
    ["bmp", "image/bmp", bmpFixture()],
    ["svg", "image/svg+xml", svg],
    ["jpeg-normal", "image/jpeg", plan],
    ["jpeg-exif-90", "image/jpeg", exifOrientation(plan, 6)],
    ["jpeg-exif-270", "image/jpeg", exifOrientation(plan, 8)],
  ].map(([id, mediaType, bytes]) => ({ id: id as string, mediaType: mediaType as string, bytesBase64: Buffer.from(bytes as Uint8Array).toString("base64") }));
}

const browserSource = String.raw`
import * as THREE from "three";
import { REFERENCE_IMAGE_READBACK_BYTE_CAPACITY, decodeReferenceImage, decodeReferenceImageOnPage, referenceImageBitmapForStage, referenceImageSourceDimensions, streamReferenceImageBitmapRows } from "semio-reference-decoder";

function bytesFromBase64(value) {
  const binary = atob(value);
  return Uint8Array.from(binary, character => character.charCodeAt(0));
}

async function reactImage(bytes, mediaType) {
  const url = URL.createObjectURL(new Blob([bytes], { type: mediaType }));
  try {
    if (mediaType === "image/svg+xml") {
      const image = new Image();
      await new Promise((resolve, reject) => {
        image.onload = resolve;
        image.onerror = reject;
        image.src = url;
      });
      return { image, width: image.naturalWidth, height: image.naturalHeight, close() {} };
    }
    const texture = await new THREE.TextureLoader().loadAsync(url);
    const image = texture.image;
    return { image, width: image.naturalWidth ?? image.width, height: image.naturalHeight ?? image.height, close() { texture.dispose(); } };
  } finally {
    URL.revokeObjectURL(url);
  }
}

async function wgpuImage(bytes, dimensions) {
  const source = new Blob([bytes], { type: dimensions.mediaType });
  const controller = new AbortController();
  return referenceImageBitmapForStage(
    1,
    () => decodeReferenceImage(source, dimensions),
    () => decodeReferenceImageOnPage(source, dimensions, controller.signal),
  );
}

async function compare(decoded, react) {
  const canvas = document.createElement("canvas");
  canvas.width = decoded.width;
  canvas.height = decoded.height;
  const context = canvas.getContext("2d", { alpha: true, colorSpace: "srgb", willReadFrequently: true });
  context.imageSmoothingEnabled = true;
  context.imageSmoothingQuality = "medium";
  context.globalCompositeOperation = "copy";
  context.drawImage(react.image, 0, 0, decoded.width, decoded.height);
  const expected = context.getImageData(0, 0, decoded.width, decoded.height, { colorSpace: "srgb" }).data;
  const stride = Math.max(4, Math.floor(expected.length / 4096 / 4) * 4);
  let samples = 0;
  let sum = 0;
  let max = 0;
  await streamReferenceImageBitmapRows(decoded, decoded.width, decoded.height, (stripOffset, pixels) => {
    const stripEnd = stripOffset + pixels.byteLength;
    for (let offset = Math.ceil(stripOffset / stride) * stride; offset < stripEnd; offset += stride) {
      for (let channel = 0; channel < 4; channel++) {
        const delta = Math.abs(expected[offset + channel] - pixels[offset - stripOffset + channel]);
        sum += delta;
        max = Math.max(max, delta);
        samples += 1;
      }
    }
    return true;
  }, () => true, async () => Promise.resolve());
  return { meanChannelDelta: sum / samples, maxChannelDelta: max };
}

async function stripProbe(bitmap, requireMultiple = true) {
  const readbacks = [];
  const pushes = [];
  const sequence = [];
  let yields = 0;
  const prototype = OffscreenCanvasRenderingContext2D.prototype;
  const original = prototype.getImageData;
  prototype.getImageData = function(x, y, width, height, settings) {
    const result = original.call(this, x, y, width, height, settings);
    readbacks.push({ x, y, width, height, bytes: result.data.byteLength });
    sequence.push("readback");
    return result;
  };
  try {
    await streamReferenceImageBitmapRows(bitmap, bitmap.width, bitmap.height, (offset, pixels) => {
      pushes.push({ offset, bytes: pixels.byteLength });
      sequence.push("push");
      return true;
    }, () => true, async () => {
      yields += 1;
      sequence.push("yield");
      await new Promise(resolve => setTimeout(resolve, 0));
    });
  } finally {
    prototype.getImageData = original;
  }
  let cursor = 0;
  const contiguous = pushes.every(push => {
    const current = push.offset === cursor;
    cursor += push.bytes;
    return current;
  });
  const bounded = readbacks.every(readback => readback.x === 0 && readback.y === 0 && readback.width === bitmap.width && readback.height > 0 && readback.bytes === readback.width * readback.height * 4 && readback.bytes <= REFERENCE_IMAGE_READBACK_BYTE_CAPACITY)
    && pushes.every(push => push.bytes > 0 && push.bytes <= REFERENCE_IMAGE_READBACK_BYTE_CAPACITY);
  const immediate = sequence.every((step, index) => step === ["readback", "push", "yield"][index % 3]);
  return { dimensions: [bitmap.width, bitmap.height], readbacks, pushes, yields, contiguous, bounded, immediate, passed: (!requireMultiple || readbacks.length > 1) && readbacks.length === pushes.length && pushes.length === yields && contiguous && bounded && immediate };
}

async function cancellationProbe(bitmap) {
  let current = true;
  let pushes = 0;
  let yields = 0;
  let cancellation = false;
  try {
    await streamReferenceImageBitmapRows(bitmap, bitmap.width, bitmap.height, () => {
      pushes += 1;
      current = false;
      return true;
    }, () => current, async () => {
      yields += 1;
      await new Promise(resolve => setTimeout(resolve, 0));
    });
  } catch (error) {
    cancellation = error instanceof DOMException && error.name === "AbortError";
  }
  return { pushes, yields, cancellation, passed: cancellation && pushes === 1 && yields === 1 };
}

async function fallbackProbe(bytes) {
  const dimensions = referenceImageSourceDimensions(bytes);
  const source = new Blob([bytes], { type: dimensions.mediaType });
  const controller = new AbortController();
  let decodeCalls = 0;
  let fallbackCalls = 0;
  const bitmap = await referenceImageBitmapForStage(1, async () => {
    decodeCalls += 1;
    throw new Error("forced worker decoder refusal");
  }, async () => {
    fallbackCalls += 1;
    return decodeReferenceImageOnPage(source, dimensions, controller.signal);
  });
  if (!bitmap) throw new Error("fallback produced no bitmap");
  try {
    const strips = await stripProbe(bitmap, false);
    return { decodeCalls, fallbackCalls, strips, passed: decodeCalls === 1 && fallbackCalls === 1 && strips.bounded && strips.immediate };
  } finally {
    bitmap.close();
  }
}

async function reuseProbe() {
  let decodeCalls = 0;
  let fallbackCalls = 0;
  const bitmap = await referenceImageBitmapForStage(2, async () => {
    decodeCalls += 1;
    throw new Error("reuse decoded");
  }, async () => {
    fallbackCalls += 1;
    throw new Error("reuse fell back");
  });
  return { decodeCalls, fallbackCalls, returnedBitmap: bitmap !== undefined, passed: bitmap === undefined && decodeCalls === 0 && fallbackCalls === 0 };
}

async function orientationProbe(vector) {
  const bytes = bytesFromBase64(vector.bytesBase64);
  const source = new Blob([bytes], { type: vector.mediaType });
  const dimensions = referenceImageSourceDimensions(bytes);
  const quarterTurn = dimensions.orientation >= 5 && dimensions.orientation <= 8;
  const rows = [];
  for (const [id, options] of [
    ["default", {}],
    ["from-image", { imageOrientation: "from-image" }],
    ["from-image-natural", { imageOrientation: "from-image", resizeWidth: dimensions.width, resizeHeight: dimensions.height }],
    ["from-image-raw", { imageOrientation: "from-image", resizeWidth: quarterTurn ? dimensions.height : dimensions.width, resizeHeight: quarterTurn ? dimensions.width : dimensions.height }],
  ]) {
    const bitmap = await createImageBitmap(source, options);
    rows.push({ id, dimensions: [bitmap.width, bitmap.height] });
    bitmap.close();
  }
  return rows;
}

function failure(error) {
  return error instanceof Error || error instanceof DOMException
    ? { name: error.name, message: error.message }
    : { name: "UnknownError", message: String(error) };
}

globalThis.runReferenceDecodeOracle = async cases => {
  const rows = [];
  for (const vector of cases) {
    const bytes = bytesFromBase64(vector.bytesBase64);
    let react;
    try {
      const dimensions = referenceImageSourceDimensions(bytes);
      const outcomes = await Promise.allSettled([
        wgpuImage(bytes, dimensions),
        reactImage(bytes, vector.mediaType),
      ]);
      if (outcomes[0].status === "rejected" || outcomes[1].status === "rejected") {
        if (outcomes[1].status === "fulfilled") outcomes[1].value.close();
        rows.push({
          id: vector.id,
          decoderFailure: outcomes[0].status === "rejected" ? failure(outcomes[0].reason) : undefined,
          reactFailure: outcomes[1].status === "rejected" ? failure(outcomes[1].reason) : undefined,
          passed: false,
        });
        continue;
      }
      const decoded = outcomes[0].value;
      if (!decoded) throw new Error("writer mode produced no bitmap");
      react = outcomes[1].value;
      try {
        const delta = await compare(decoded, react);
        rows.push({ id: vector.id, source: [dimensions.width, dimensions.height, dimensions.orientation], decoded: [decoded.width, decoded.height], react: [react.width, react.height], ...delta, passed: decoded.width === react.width && decoded.height === react.height && delta.meanChannelDelta <= 4 && delta.maxChannelDelta <= 48 });
      } finally {
        decoded.close();
      }
    } catch (error) {
      rows.push({ id: vector.id, oracleFailure: failure(error), passed: false });
    } finally {
      react?.close();
    }
  }
  const first = bytesFromBase64(cases[0].bytesBase64);
  let current = true;
  const retired = decodeReferenceImage(new Blob([first], { type: "image/png" }), referenceImageSourceDimensions(first), () => current);
  current = false;
  let cancellation = false;
  try { (await retired).close(); } catch (error) { cancellation = error instanceof DOMException && error.name === "AbortError"; }
  const svg = bytesFromBase64(cases.find(vector => vector.id === "svg").bytesBase64);
  const svgDimensions = referenceImageSourceDimensions(svg);
  const pageController = new AbortController();
  const retiredPageDecode = decodeReferenceImageOnPage(new Blob([svg], { type: svgDimensions.mediaType }), svgDimensions, pageController.signal);
  pageController.abort();
  let pageCancellation = false;
  try { (await retiredPageDecode).close(); } catch (error) { pageCancellation = error instanceof DOMException && error.name === "AbortError"; }
  const plan = cases.find(vector => vector.id === "jpeg-normal");
  const planBytes = bytesFromBase64(plan.bytesBase64);
  const planBitmap = await wgpuImage(planBytes, referenceImageSourceDimensions(planBytes));
  if (!planBitmap) throw new Error("plan writer produced no bitmap");
  let strips;
  let stripCancellation;
  try {
    strips = await stripProbe(planBitmap);
    stripCancellation = await cancellationProbe(planBitmap);
  } finally {
    planBitmap.close();
  }
  const fallback = await fallbackProbe(svg);
  const reuse = await reuseProbe();
  const orientation = await orientationProbe(cases.find(vector => vector.id === "jpeg-exif-90"));
  const workerCapabilities = await new Promise(resolve => {
    const worker = new Worker(URL.createObjectURL(new Blob(["postMessage({ imageDecoder: typeof globalThis.ImageDecoder, createImageBitmap: typeof globalThis.createImageBitmap, offscreenCanvas: typeof globalThis.OffscreenCanvas })"], { type: "text/javascript" })));
    worker.onmessage = event => { resolve(event.data); worker.terminate(); };
  });
  return { rows, cancellation, pageCancellation, strips, stripCancellation, fallback, reuse, orientation, workerCapabilities, passed: cancellation && pageCancellation && strips.passed && stripCancellation.passed && fallback.passed && reuse.passed && rows.every(row => row.passed) };
};
`;

async function main(): Promise<void> {
  if (process.argv[2] !== "browser-codec-oracle") throw new Error("usage: bun 📜️script.ts browser-codec-oracle");
  const threePath = Bun.resolveSync("three", repoRoot);
  const build = await Bun.build({
    entrypoints: ["semio-reference-oracle"],
    target: "browser",
    format: "iife",
    plugins: [{
      name: "semio-reference-oracle",
      setup(builder) {
        builder.onResolve({ filter: /^semio-reference-oracle$/ }, () => ({ path: "semio-reference-oracle", namespace: "oracle" }));
        builder.onResolve({ filter: /^semio-reference-decoder$/ }, () => ({ path: decoderPath }));
        builder.onResolve({ filter: /^three$/ }, () => ({ path: threePath }));
        builder.onLoad({ filter: /.*/, namespace: "oracle" }, () => ({ contents: browserSource, loader: "js" }));
      },
    }],
  });
  if (!build.success) throw new Error(build.logs.map(String).join("\n"));
  const javascript = await build.outputs[0].text();
  const playwrightSpecifier = "playwright";
  const { chromium } = (await import(playwrightSpecifier)) as typeof import("playwright");
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 64, height: 64 }, deviceScaleFactor: 1 });
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(String(error)));
    await page.setContent("<!doctype html><html><body></body></html>");
    await page.addScriptTag({ content: javascript });
    const result = await page.evaluate(async vectors => (globalThis as any).runReferenceDecodeOracle(vectors), await inputs());
    if (errors.length) throw new Error(errors.join("\n"));
    await mkdir(outputRoot, { recursive: true });
    await writeFile(join(outputRoot, "result.json"), `${JSON.stringify(result, null, 2)}\n`);
    console.log(JSON.stringify(result));
    if (!result.passed) process.exitCode = 1;
  } finally {
    await browser.close();
  }
}

await main();
