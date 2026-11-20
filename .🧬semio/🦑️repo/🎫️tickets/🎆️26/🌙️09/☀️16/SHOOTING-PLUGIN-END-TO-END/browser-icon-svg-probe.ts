import { resolve } from "node:path";
import { loadPlaywrightV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { browserLaunchOptionsV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";

const workerModule = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🖼️icon-svg-preview/🟦️.ts");
const pageBridge = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🖼️page-image-decode/🟦️.ts");
const bitmapModule = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts");
const worker = await Bun.build({
  entrypoints: ["probe:worker"],
  target: "browser",
  format: "iife",
  plugins: [{
    name: "preview transport probe",
    setup(build) {
      build.onResolve({ filter: /^probe:/ }, ({ path }) => ({ path, namespace: "probe" }));
      build.onResolve({ filter: /state\/🟦️\.ts$/ }, () => ({ path: "probe:state", namespace: "probe" }));
      build.onResolve({ filter: /steps-and-reporting\/🟦️\.ts$/ }, () => ({ path: "probe:steps", namespace: "probe" }));
      build.onLoad({ filter: /.*/, namespace: "probe" }, ({ path }) => ({
        loader: "ts",
        contents: path === "probe:state" ? `
          export const lifecycle = 1, closed = false, closing = false, failed = false;
          export let nextImageDecodeRequestId = 0, pageImageDecode;
          export const setNextImageDecodeRequestId = value => nextImageDecodeRequestId = value;
          export const setPageImageDecode = value => pageImageDecode = value;
        ` : path === "probe:steps" ? `
          export const post = (value, transfers) => self.postMessage(value, transfers);
          export const monitoredSuspension = (_name, operation) => operation();
          export const macrotask = () => new Promise(resolve => setTimeout(resolve, 0));
        ` : `
          import { rasterizeIconSvgPreview } from ${JSON.stringify(workerModule)};
          import { settlePageImageDecode } from ${JSON.stringify(pageBridge)};
          const decode = globalThis.createImageBitmap;
          self.onmessage = async ({ data }) => {
            if (data.kind === "image-decode-result") { settlePageImageDecode(data); return; }
            if (data.forceFallback) globalThis.createImageBitmap = () => Promise.reject(new Error("forced page fallback"));
            try {
              const rgba = await rasterizeIconSvgPreview(data.source.length, 512, 512,
                offset => data.source.slice(offset, offset + 262144), new Promise(() => {}));
              self.postMessage({ kind: "done", offset: rgba.length, rgba: [...rgba.slice((256 * 512 + 256) * 4, (256 * 512 + 256) * 4 + 4)], corner: [...rgba.slice(0, 4)] });
            } catch (error) {
              self.postMessage({ kind: "failed", reason: String(error) });
            } finally { globalThis.createImageBitmap = decode; }
          };
        `,
      }));
    },
  }],
});
const pageDecode = await Bun.build({
  entrypoints: ["probe:page"],
  target: "browser",
  format: "iife",
  plugins: [{
    name: "page decoder probe",
    setup(build) {
      build.onResolve({ filter: /^probe:page$/ }, ({ path }) => ({ path, namespace: "probe" }));
      build.onLoad({ filter: /.*/, namespace: "probe" }, () => ({
        loader: "ts",
        contents: `import { decodeReferenceImageOnPage } from ${JSON.stringify(bitmapModule)}; window.probePageDecode = decodeReferenceImageOnPage;`,
      }));
    },
  }],
});
for (const result of [worker, pageDecode]) {
  if (!result.success) throw new Error(result.logs.map(String).join("\n"));
}
const workerSource = await worker.outputs[0]!.text();
const { chromium } = await loadPlaywrightV1(findWorkspaceRoot());
const browser = await chromium.launch(browserLaunchOptionsV1());
try {
  const page = await browser.newPage();
  await page.addScriptTag({ content: await pageDecode.outputs[0]!.text() });
  const outcomes = await page.evaluate(async (source) => {
    async function run(forceFallback: boolean) {
      const workerUrl = URL.createObjectURL(new Blob([source], { type: "text/javascript" }));
      const worker = new Worker(workerUrl);
      let fallback = 0;
      try {
        return await new Promise((resolve, reject) => {
          const timer = setTimeout(() => reject(new Error("preview worker stalled")), 8000);
          worker.onerror = ({ message }) => reject(new Error(message));
          worker.onmessage = async ({ data }) => {
            if (data.kind === "image-decode") {
              fallback++;
              try {
                const decode = (window as unknown as { probePageDecode: (blob: Blob, dimensions: unknown, signal: AbortSignal) => Promise<ImageBitmap> }).probePageDecode;
                const bitmap = await decode(new Blob([data.source], { type: "image/svg+xml" }), data.dimensions, new AbortController().signal);
                worker.postMessage({ kind: "image-decode-result", requestId: data.requestId, lifecycle: data.lifecycle, bitmap }, [bitmap]);
              } catch (error) { reject(error); }
            } else {
              clearTimeout(timer);
              if (data.kind === "failed") reject(new Error(data.reason));
              else resolve({ ...data, fallback });
            }
          };
          const source = new TextEncoder().encode('<svg xmlns="http://www.w3.org/2000/svg" viewBox="-4000 -4000 8000 8000" width="8000" height="8000"><circle r="2000" fill="#44cc88" fill-opacity="0.5"/></svg>');
          worker.postMessage({ kind: "render", forceFallback, source });
        });
      } finally {
        worker.terminate();
        URL.revokeObjectURL(workerUrl);
      }
    }
    return [await run(false), await run(true)];
  }, workerSource);
  for (const result of outcomes as { offset: number; rgba: number[]; corner: number[]; fallback: number }[]) {
    if (result.offset !== 1024 * 1024 || result.rgba.some((value, axis) => Math.abs(value - [68, 204, 136, 128][axis]!) > 1)
      || result.corner.some(value => value !== 0)) throw new Error(JSON.stringify(result));
  }
  if ((outcomes[1] as { fallback: number }).fallback !== 1) throw new Error("The forced fallback did not use the shared page path");
  console.log(JSON.stringify(outcomes, null, 2));
} finally {
  await browser.close();
}
