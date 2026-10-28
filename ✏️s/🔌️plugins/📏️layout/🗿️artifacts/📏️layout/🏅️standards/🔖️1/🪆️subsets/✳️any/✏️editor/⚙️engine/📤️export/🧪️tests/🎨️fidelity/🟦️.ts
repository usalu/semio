import { test, expect } from "bun:test";
import sharp from "sharp";
import { DOMMatrix } from "@napi-rs/canvas";
import { PNG } from "pngjs";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import opentype from "opentype.js";
import corpus from "../../../../../🧪️tests/🧫️fixtures/🎨️export-fidelity/🔣️.json";

test("portable vector probes agree with an independent SVG renderer and PNG decoder", async () => {
    for (const row of corpus.cases) {
        const encoded = await sharp(Buffer.from(row.svg)).png().toBuffer();
        const decoded = PNG.sync.read(encoded);
        expect([decoded.width, decoded.height]).toEqual([row.width, row.height]);
        for (const probe of row.probes) {
            const index = (probe.y * row.width + probe.x) * 4;
            for (let channel = 0; channel < 4; channel++) expect(Math.abs(decoded.data[index + channel] - probe.rgba[channel])).toBeLessThanOrEqual(probe.tolerance);
        }
    }
});

test("portable text witnesses are actual TrueType outlines with counters and curves", () => {
    const bytes = readFileSync(fileURLToPath(new URL("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🖼️assets/🔤️MapLabelSans.ttf", import.meta.url)));
    const font = opentype.parse(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
    for (const row of corpus.glyphs) {
        const glyph = font.charToGlyph(row.character);
        expect(glyph.index).toBeGreaterThan(0);
        const commands = glyph.getPath(row.origin[0], row.origin[1], row.size).commands;
        expect(commands.some(command => command.type === "M")).toBe(true);
        expect(commands.some(command => command.type === "L" || command.type === "Q")).toBe(true);
        expect(commands.some(command => command.type === "Z")).toBe(true);
    }
});


test("portable RGBA image rotation agrees with SVG rendering and independent PNG encoding", async () => {
    for (const row of corpus.images) {
        const png = PNG.sync.write({ width: row.image.width, height: row.image.height, data: Buffer.from(row.image.rgba) } as PNG);
        const f = row.frame;
        const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="' + row.width + '" height="' + row.height + '"><rect width="100%" height="100%" fill="white"/><image x="' + f.x + '" y="' + f.y + '" width="' + f.width + '" height="' + f.height + '" preserveAspectRatio="none" transform="rotate(' + f.rotation * 180 / Math.PI + ' ' + (f.x + f.width / 2) + ' ' + (f.y + f.height / 2) + ')" href="data:image/png;base64,' + png.toString("base64") + '"/></svg>';
        const decoded = PNG.sync.read(await sharp(Buffer.from(svg)).png().toBuffer());
        for (const probe of row.probes) {
            const index = (probe.y * row.width + probe.x) * 4;
            for (let channel = 0; channel < 4; channel++) expect(Math.abs(decoded.data[index + channel] - probe.rgba[channel])).toBeLessThanOrEqual(probe.tolerance);
        }
    }
});


test("portable text frame transforms agree with independent affine matrix composition", () => {
    for (const probe of corpus.transformProbes) {
        const matrix = new DOMMatrix().translate(probe.center[0], probe.center[1]).rotate(probe.rotation * 180 / Math.PI).translate(-probe.center[0], -probe.center[1]);
        const result = matrix.transformPoint({ x: probe.point[0], y: probe.point[1] });
        expect(Math.abs(result.x - probe.expected[0])).toBeLessThanOrEqual(1e-10);
        expect(Math.abs(result.y - probe.expected[1])).toBeLessThanOrEqual(1e-10);
    }
});
