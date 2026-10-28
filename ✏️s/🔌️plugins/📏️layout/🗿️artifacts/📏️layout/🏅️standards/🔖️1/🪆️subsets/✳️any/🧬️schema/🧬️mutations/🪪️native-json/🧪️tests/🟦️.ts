import { deepStrictEqual, strictEqual, throws } from 'node:assert/strict';
import vectors from '../../../../🧪️tests/🧫️fixtures/📨️mutations/🔣️.json' with { type: 'json' };
import pageDrawing from '../../🖊️set-page-background-drawing/🧪️tests/🔣️transport.json' with {type:'json'};
import { binary64 } from '@semio-tech/framework/value/ieee754';
import { layoutMutationFromNativeJson, layoutMutationNativeJson, parseLayoutMutation } from '../🟦️.ts';

/** 📨️ Every semantic variant crosses its named native JSON boundary without numeric canonical state. */
export function testLayoutMutationTransport(schemaOracle: (mutation: unknown) => boolean): { variants: number; defaults: number; refusals: number } {
  const cases=[...vectors.cases,...pageDrawing.cases];
  for (const vector of [...cases, ...vectors.defaults]) {
    strictEqual(schemaOracle(vector.input), true, vector.name + ': independent schema');
    const saved = structuredClone(vector.input), mutation = layoutMutationFromNativeJson(vector.input);
    deepStrictEqual(layoutMutationNativeJson(mutation), vector.output, vector.name + ': native transport');
    deepStrictEqual(parseLayoutMutation(mutation), mutation, vector.name + ': canonical parse');
    deepStrictEqual(layoutMutationFromNativeJson(layoutMutationNativeJson(mutation)), mutation, vector.name + ': roundtrip');
    deepStrictEqual(vector.input, saved, vector.name + ': immutable');
  }
  for (const vector of [...vectors.refusals,...pageDrawing.refusals]) throws(() => layoutMutationFromNativeJson(vector), 'malformed native command');
  throws(() => parseLayoutMutation({ ChangePageWidth: { id: 'p', newWidth: 1 } }), 'canonical float words are required');
  throws(() => parseLayoutMutation({ CreateLayer: { pageId: 'p', id: 'l', name: 'L', index: null } }), 'canonical boolean presence is required');
  const nan = { ChangePageWidth: { id: 'p', newWidth: binary64(NaN) } };
  deepStrictEqual(parseLayoutMutation(nan), nan);
  throws(() => layoutMutationNativeJson(nan), 'finite JSON refuses NaN without corrupting canonical bits');
  return { variants: new Set(cases.flatMap(vector=>Object.keys(vector.input))).size, defaults: vectors.defaults.length, refusals: vectors.refusals.length + pageDrawing.refusals.length + 3 };
}
