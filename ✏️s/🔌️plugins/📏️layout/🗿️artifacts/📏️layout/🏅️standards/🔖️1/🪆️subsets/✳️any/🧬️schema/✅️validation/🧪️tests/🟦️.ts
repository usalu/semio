import { deepStrictEqual, strictEqual } from 'node:assert/strict';
import jsonPatch from 'fast-json-patch';
import type { Operation } from 'fast-json-patch';
import corpus from '../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json' with { type: 'json' };
import { binary64 } from '@semio-tech/framework/value/ieee754';
import { layoutArtifactFromNativeJson, layoutArtifactNativeJson, page as nativePage } from '../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts';
import { parseParentPage } from '../../🟦️.ts';
import { normalizeParentContent, validateDocument, validateFrameCreation, validateFrameRemoval, validatePageCreation, validateStyleRuns, validateThreadEdge } from '../🟦️.ts';

/** 🧪️ Runs portable graph admission through canonical words and an independent graph oracle. */
export function testLayoutGraphValidation(oracle: (document: unknown) => boolean): { documents: number; parents: number; targeted: number } {
  for (const vector of corpus.documents) {
    const candidate = jsonPatch.applyPatch(structuredClone(corpus.base), vector.patch as Operation[], true, false).newDocument;
    strictEqual(oracle(candidate), vector.valid, vector.name + ': independent graph');
    const document = layoutArtifactFromNativeJson(candidate), saved = structuredClone(document);
    strictEqual(validateDocument(document) === null, vector.valid, vector.name + ': canonical graph');
    deepStrictEqual(document, saved, vector.name + ': immutable');
  }
  const base = layoutArtifactFromNativeJson(corpus.base);
  for (const vector of corpus.parents) {
    const parent = parseParentPage(nativePage(vector.page, false, true)), saved = structuredClone(parent);
    const result = normalizeParentContent(base, parent);
    strictEqual(result.error === null, vector.valid, vector.name + ': parent admission');
    if (vector.valid) {
      const normalized = layoutArtifactNativeJson({ ...base, parentPages: [result.parent] }) as { parentPages: unknown[] };
      deepStrictEqual(normalized.parentPages[0], vector.normalizedPage, vector.name + ': derived membership');
    }
    deepStrictEqual(parent, saved, vector.name + ': immutable parent');
  }
  const page = base.pages[0]!, frame = page.frames[0]!;
  strictEqual(validateStyleRuns(base, 'é🌍', [{ start: 0n, end: 6n, paragraphStyleId: null, characterStyleId: null }]), null);
  strictEqual(validateStyleRuns(base, 'é🌍', [{ start: 1n, end: 6n, paragraphStyleId: null, characterStyleId: null }]) !== null, true);
  strictEqual(validateThreadEdge(base, 'text-2', 'story-1', 'text-1') !== null, true);
  strictEqual(validateThreadEdge(base,'text-1','story-2','text-2')!==null,true);
  strictEqual(validateThreadEdge(base,'text-2','story-2',null)!==null,true);
  strictEqual(validateThreadEdge(base,'new-frame','story-1','text-2')!==null,true);
  strictEqual(validateThreadEdge(base,'new-frame','story-2',null),null);
  strictEqual(validateFrameRemoval(base, ['text-2']) !== null, true);
  strictEqual(validateFrameRemoval(base, ['text-1', 'text-2']), null);
  const independentFrame=structuredClone(frame);independentFrame.id='new-frame';if(independentFrame.kind==='text')independentFrame.threadNext=null;
  strictEqual(validateFrameCreation(base, page, independentFrame), null);
  strictEqual(validateFrameCreation(base, page, frame) !== null, true);
  strictEqual(validatePageCreation(base, { ...page, id: 'new-page', frames: [] , layers: page.layers.map(layer => ({ ...layer, objectIds: [] })) }), null);
  for (const value of [NaN, Infinity, -Infinity]) strictEqual(validateDocument({ ...base, grid: { ...base.grid, baselineGrid: binary64(value) } }) !== null, true);
  strictEqual(validateDocument({ ...base, paragraphStyles: base.paragraphStyles.map(style => ({ ...style, id: '\u0085' })) }) !== null, true);
  return { documents: corpus.documents.length, parents: corpus.parents.length, targeted: 16 };
}
