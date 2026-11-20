/** 🔡 Checks PDF Type 3 rendering-mode behavior against PDF.js pixels. */
import { createCanvas, DOMMatrix, ImageData, Path2D } from "@napi-rs/canvas";

Object.assign(globalThis, { DOMMatrix, ImageData, Path2D });
const { getDocument } = await import("pdfjs-dist/legacy/build/pdf.mjs");

const fixture = await Bun.file(process.argv[2]).json();
const [width, height] = fixture.page;
const font = fixture.type3;
const stream = (bytes: string) => `<< /Length ${bytes.length} >>\nstream\n${bytes}\nendstream`;
for (const render of fixture.renderModes) {
  const content = fixture.pageContent.replace(" Tf ", ` Tf ${render.mode} Tr `);
  const objects = [
    "<< /Type /Catalog /Pages 2 0 R >>",
    "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
    `<< /Type /Page /Parent 2 0 R /MediaBox [0 0 ${width} ${height}] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>`,
    stream(content),
    `<< /Type /Font /Subtype /Type3 /FontBBox [${font.bbox.join(" ")}] /FontMatrix [${font.matrix.join(" ")}] /FirstChar 65 /LastChar 66 /Widths [${font.widths.join(" ")}] /Encoding << /Differences [65 /A /B] >> /CharProcs << /A 6 0 R /B 7 0 R >> /Resources <<>> >>`,
    ...font.glyphs.map((glyph: string, index: number) => stream(`${font.widths[index]} 0 d0 ${glyph}`)),
  ];
  let bytes = "%PDF-1.7\n";
  const rows = ["0000000000 65535 f \n"];
  for (const [index, object] of objects.entries()) {
    rows.push(`${String(bytes.length).padStart(10, "0")} 00000 n \n`);
    bytes += `${index + 1} 0 obj\n${object}\nendobj\n`;
  }
  const xref = bytes.length;
  bytes += `xref\n0 ${rows.length}\n${rows.join("")}trailer\n<< /Size ${rows.length} /Root 1 0 R >>\nstartxref\n${xref}\n%%EOF\n`;
  const task = getDocument({ data: new TextEncoder().encode(bytes), useSystemFonts: false, stopAtErrors: true });
  const pdf = await task.promise;
  try {
    const page = await pdf.getPage(1);
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    await page.render({ canvasContext: context, viewport: page.getViewport({ scale: 1 }) }).promise;
    for (const probe of fixture.probes) {
      const [x, y] = probe.at;
      const actual = [...context.getImageData(x, y, 1, 1).data].slice(0, 3);
      const expected = render.visible ? probe.rgb : [255, 255, 255];
      if (actual.some((value, index) => value !== expected[index])) {
        throw Error(`Mode ${render.mode} at ${probe.at}: ${actual}, expected ${expected}`);
      }
    }
  } finally {
    await task.destroy();
  }
}
console.log("PDF.js verified Type 3 glyph procedures in rendering mode 7 and invisible text in mode 3.");
