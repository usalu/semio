/** 🧪️ Checks the replacement renderer against the existing Draw PDF.js comparison. */
import {expect, test} from "bun:test";
import {resolve} from "node:path";

const tests = resolve("✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write/🧪️tests");
const {pdfNodePixels} = await import(resolve(tests, "🔮️oracle/🟦️.ts"));
const rows = await Bun.file(resolve(tests, "🧫️fixtures/🔣️.json")).json();
const sample = (name: string) => new URL("./🗑️generated/" + name, import.meta.url);
const pdfium = (await Bun.file(sample("pdfium-ffi-pixels.log")).text()).split("\n").filter(row => /^\d+ /.test(row)).map(row => row.split(" ").map(Number));

test("PDFium and the existing PDF.js oracle agree on 21 comparable Draw writer samples", async () => {
    let compared = 0;
    for (const [index, ...pixels] of pdfium) {
        if (index >= 24 || rows[Math.floor(index / 3)].styles === "gradient alpha") continue;
        const bytes = new Uint8Array(await Bun.file(sample("pdfium-input-" + index + ".pdf")).arrayBuffer());
        const node = await pdfNodePixels(bytes);
        const row = rows[Math.floor(index / 3)];
        for (let i = 0; i < pixels.length; i++) expect(Math.abs(pixels[i] - node[i]), row.styles).toBeLessThanOrEqual(row.channelTolerance);
        compared++;
    }
    expect(compared).toBe(21);
}, 45000);
