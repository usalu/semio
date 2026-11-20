/** 🌐️ Exercise the production canvas painter with a real Chromium drawing context. */
import { join, resolve } from "node:path";
import { ensureRepositoryBrowserPathV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";

const repository = process.cwd();
const generated = join(import.meta.dir, "🗑️generated");
const paint = resolve(repository, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🎨️paint/🟦️.ts");
const fixturePath = process.argv[2];
const stem = fixturePath ? "stroke-style" : "path-clips";
const fixture = await Bun.file(fixturePath ?? resolve(repository, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🧪️path-clips.json")).json();
const entry = join(generated, stem + "-entry.ts");
await Bun.write(entry, "import {drawSceneNode} from " + JSON.stringify(paint) + "; globalThis.paintPdfProbe = drawSceneNode;");
const bundle = await Bun.build({ entrypoints: [entry], target: "browser", format: "esm", define: {"import.meta.vitest": "false"} });
if (!bundle.success) throw new AggregateError(bundle.logs);
const source = await bundle.outputs[0].text();
ensureRepositoryBrowserPathV1(repository, process.env);
const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const browser = await chromium.launch({headless: true, args: process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [], env: {...process.env, TMPDIR: generated, TEMP: generated, TMP: generated}});
try {
    const page = await browser.newPage({viewport: {width: fixture.width, height: fixture.height}});
    await page.route("http://pdf-probe.test/**", route => {
        const script = route.request().url().endsWith("/paint.js");
        return route.fulfill({status: 200, contentType: script ? "application/javascript" : "text/html", body: script ? source : '<script type="module" src="/paint.js"></script>'});
    });
    await page.goto("http://pdf-probe.test/");
    await page.waitForFunction(() => typeof (globalThis as any).paintPdfProbe === "function", undefined, {timeout: 15000});
    const matched = await page.evaluate((fixture) => {

        const drawSceneNode = (globalThis as any).paintPdfProbe;
        const canvas = document.createElement("canvas");

        [canvas.width, canvas.height] = [fixture.width, fixture.height];

        document.body.style.margin = "0";
        document.body.append(canvas);

        const ctx = canvas.getContext("2d", {willReadFrequently: true})!;

        ctx.fillStyle = "white";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        for (const layer of fixture.layers) drawSceneNode(ctx, layer, new Map());

        for (const {at, rgb} of fixture.probes) {
            const pixel = ctx.getImageData(at[0], at[1], 1, 1).data;
            if (!rgb.every((value: number, index: number) => pixel[index] === value)) throw Error("Unexpected Chromium pixel at " + at);
        }
        return fixture.probes.length;
    }, fixture);
    await page.screenshot({path: join(generated, stem + "-chromium.png")});
    console.log("Chromium matched " + matched + " MuPDF pixel probes using the production canvas painter.");
} finally {
    await browser.close();
}
