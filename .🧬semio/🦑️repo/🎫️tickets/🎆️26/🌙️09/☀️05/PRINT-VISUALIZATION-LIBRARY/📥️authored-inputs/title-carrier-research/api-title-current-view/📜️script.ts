import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { createRequire } from 'node:module';
import { join } from 'node:path';
const ticket = process.env.SEMIO_TICKET_DIR!, root = 'C:/git/semio/🧰️framework/🛍️products/📓️print', dir = join(root, '📦️packages/🟦️typescript/dist/documents/viz-api');
const log = readFileSync(join(ticket, '🗑️generated/api-title-current-publication2.log'), 'utf8');
if (!log.includes('Published viz-api: 2 PDFs')) throw Error('No current publisher marker');
const receipt = JSON.parse(readFileSync(join(dir, '.nx-artifact.json'), 'utf8'));
if (receipt.version !== 1 || receipt.owner !== '@semio-tech/print:build-viz-api' || receipt.files.length !== 2 || readdirSync(dir).length !== 3) throw Error('Invalid current paired receipt');
const sha = (bytes: Uint8Array) => createHash('sha256').update(bytes).digest('hex');
const source = join(root, '🧾️template/📊️viz-api/🔓️viz-api.tex'), sourceSha256 = sha(readFileSync(source));
const canvas = createRequire('C:/git/semio/node_modules/pdfjs-dist/legacy/build/pdf.mjs')('@napi-rs/canvas');
globalThis.DOMMatrix ??= canvas.DOMMatrix;
const { getDocument } = await import('pdfjs-dist/legacy/build/pdf.mjs'), result = [];
for (const name of receipt.files) {
  const path = join(dir, name), bytes = readFileSync(path);
  if (bytes.subarray(0, 5).toString() !== '%PDF-') throw Error('Invalid PDF signature');
  const pdf = await getDocument({ data: new Uint8Array(bytes) }).promise, pages = [];
  for (let number = 1; number <= pdf.numPages; number++) {
    const page = await pdf.getPage(number), content = await page.getTextContent();
    pages.push({ number, text: content.items.map((item: any) => 'str' in item ? item.str : '').join(' '), items: content.items.filter((item: any) => 'str' in item).map((item: any) => ({ text: item.str, x: item.transform[4], y: item.transform[5], width: item.width, height: item.height })) });
  }
  result.push({ id: 'viz-api', name, path, sha256: sha(bytes), bytes: bytes.length, sourceSha256, pages: pdf.numPages, textPages: pages });
  await pdf.destroy();
  console.log('[DEBUG] Current API ' + name + ' pages=' + pages.length + ' sha=' + sha(bytes));
}
writeFileSync(join(import.meta.dir, 'pdf-pages.json'), JSON.stringify(result, null, 2));
writeFileSync(join(import.meta.dir, 'publication.json'), JSON.stringify({ source, sourceSha256, receipt, selected: result.map(({ textPages, ...file }) => file) }, null, 2));
