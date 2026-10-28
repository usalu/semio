import { strictEqual, throws } from 'node:assert/strict';
import vectors from './cases.json' with { type: 'json' };
import { PDF_UNICODE_PREFIX, PDF_UNICODE_SUFFIX, PDF_ENCODING_CMAP, pdfUnicodeRange, pdfUnicodeScalar } from '../🟦️.ts';
/** 🔤️ Checks the emitted CMap with an independent PDF text extractor, including supplementary planes. */
export async function testPdfUnicodeCmap(oracle: (unicode: string, encoding: string, encoded: string) => Promise<string>): Promise<{ ranges: number; texts: number }> {
  for (const vector of vectors.ranges) strictEqual(pdfUnicodeRange(vector.at), '1 beginbfrange\n<' + vector.from + '> <' + vector.to + '> <' + vector.target + '>\nendbfrange\n');
  const cmap = PDF_UNICODE_PREFIX + Array.from({ length: 1026 }, (_, at) => pdfUnicodeRange(at)).join('') + PDF_UNICODE_SUFFIX;
  for (const text of vectors.texts) strictEqual(await oracle(cmap, PDF_ENCODING_CMAP, [...text].map(pdfUnicodeScalar).join('')), text);
  throws(() => pdfUnicodeScalar('ab')); throws(() => pdfUnicodeScalar('\ud800')); throws(() => pdfUnicodeRange(-1)); throws(() => pdfUnicodeRange(1026));
  return { ranges: 1026, texts: vectors.texts.length };
}
