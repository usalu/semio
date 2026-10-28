import {OutputWork,type OutputGrant} from '../../../📤️output/🟦️.ts';
import { deepStrictEqual, strictEqual, throws } from 'node:assert/strict';
import corpus from '../../../../../../🧪️tests/🧫️fixtures/📕️pdf/🔣️.json' with { type: 'json' };
import integration from '../../../../../../🧪️tests/🧫️fixtures/📕️pdf/integration.json' with { type: 'json' };
import { pdfRasterChunk, pdfRasterLength, type PdfRasterSource, type PdfRasterPlane } from '../🟦️.ts';

const RASTER_TURN:OutputGrant={maximumItems:1,maximumCopyBytes:8192,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:1};
/** 🎟️ A declared original raster turn owns each caller scratch write and records aggregate test stock. */
function rasterTurn(source:PdfRasterSource,plane:PdfRasterPlane,cursor:number,maximumBytes:number):Uint8Array{const target=new Uint8Array(maximumBytes),work=new OutputWork(RASTER_TURN);const count=pdfRasterChunk(source,plane,cursor,target,work),receipt=work.receipt();strictEqual(receipt.copiedBytes,count===0?0:count+24);strictEqual(receipt.copiedItems,count===0?0:1);strictEqual(receipt.retainedCapacityBytes,0);strictEqual(receipt.releasedBytes,0);return target.subarray(0,count);}

/** 🖼️ Checks bounded image planes against an independent image decoder/exporter. */
export async function testPdfRasterPlanes(oracle: (source: PdfRasterSource, plane: PdfRasterPlane) => Promise<Uint8Array>): Promise<{ cases: number; grants: number; refusals: number }> {
  for (const vector of corpus.cases) {
    const source = { width: vector.width, height: vector.height, rgba: Uint8Array.from(vector.rgba) }, saved = source.rgba.slice();
    for (const plane of ['rgb', 'alpha'] as const) {
      deepStrictEqual([...await oracle(source, plane)], vector[plane], vector.name + ': independent channel extraction');
      for (const grant of [1, 2, 3, 4096]) {
        const result: number[] = []; let cursor = 0;
        while (cursor < pdfRasterLength(source, plane)) { const chunk = rasterTurn(source, plane, cursor, grant); strictEqual(chunk.length <= grant, true); strictEqual(chunk.length > 0, true); result.push(...chunk); cursor += chunk.length; }
        deepStrictEqual(result, vector[plane], vector.name + ': bounded plane');
        strictEqual(rasterTurn(source, plane, cursor, grant).length, 0);
      }
    }
    deepStrictEqual(source.rgba, saved, vector.name + ': immutable RGBA');
  }
  for (const vector of corpus.refusals) throws(() => rasterTurn({ ...vector, rgba: Uint8Array.from(vector.rgba) }, 'rgb', 0, 1));
  const source = { width: 1, height: 1, rgba: Uint8Array.of(1, 2, 3, 4) };
  throws(() => rasterTurn(source, 'rgb', 4, 1)); throws(() => rasterTurn(source, 'rgb', 0, 0)); throws(() => rasterTurn(source, 'rgb', 0, 4097));
  const target=new Uint8Array(1).fill(77),saved=source.rgba.slice();for(const grant of [{...RASTER_TURN,maximumItems:0},{...RASTER_TURN,maximumCopyBytes:24},{...RASTER_TURN,maximumDepth:0}]){const work=new OutputWork(grant);throws(()=>pdfRasterChunk(source,'rgb',0,target,work));strictEqual(target[0],77);strictEqual(work.receipt().copiedBytes,0);strictEqual(work.receipt().copiedItems,0);deepStrictEqual(source.rgba,saved);}
  return { cases: corpus.cases.length, grants: 4, refusals: corpus.refusals.length + 3 };
}

/** 🖼️ Exercises the real job's large transparent image recipe through the bounded plane helpers. */
export async function testPdfIntegrationPlanes(oracle: (source: PdfRasterSource, plane: PdfRasterPlane) => Promise<Uint8Array>): Promise<{ pixels: number; planes: number }> {
  const recipe = integration.image, pixels = recipe.width * recipe.height, rgba = new Uint8Array(pixels * 4);
  for (let at = 0; at < pixels; at++) rgba.set(recipe.pattern[at % recipe.pattern.length]!, at * 4);
  const source = { width: recipe.width, height: recipe.height, rgba };
  for (const plane of ['rgb', 'alpha'] as const) {
    const actual = new Uint8Array(pdfRasterLength(source, plane));
    for (let at = 0; at < actual.length;) { const chunk = rasterTurn(source, plane, at, 4096); strictEqual(chunk.length > 0 && chunk.length <= 4096, true); actual.set(chunk, at); at += chunk.length; }
    deepStrictEqual([...actual], [...await oracle(source, plane)]);
  }
  return { pixels, planes: 2 };
}
