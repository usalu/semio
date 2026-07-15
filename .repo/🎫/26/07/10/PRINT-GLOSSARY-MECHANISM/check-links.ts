import { readFileSync } from "node:fs";
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync("snap-dist/verify-glossary.pdf")) }).promise;
for (let i = 1; i <= doc.numPages; i++) {
  const page = await doc.getPage(i);
  const annots = await page.getAnnotations();
  console.log(
    `page ${i}: ${annots.length} annotations`,
    annots.map((a: any) => ({ subtype: a.subtype, dest: a.dest, url: a.url })),
  );
}
