/** 🧪️ Full native Layout JSON uses reference-only genuine Drawing members and preserves sparse setters. */
import {test,expect} from 'bun:test';
import vectors from '../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json';
import {layoutArtifactFromNativeJson,layoutArtifactNativeJson} from '../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts';
import {layoutDiffFromNativeJson,layoutDiffNativeJson} from '../../../🚪️io/📝️text/🔺️diff/🪪️native-json/🟦️.ts';
test('Layout native JSON retains the full document and diff with their real Drawing child',()=>{
 expect(layoutArtifactNativeJson(layoutArtifactFromNativeJson(vectors.document))).toEqual(vectors.document);
 expect(layoutDiffNativeJson(layoutDiffFromNativeJson(vectors.diff))).toEqual(vectors.diff);
});
test('Layout native JSON distinguishes absent and explicitly cleared Drawing setters',()=>{
 const {backgroundDrawing,...unchanged}=vectors.diff;
 const cleared={...unchanged,backgroundDrawing:null};
 expect(layoutDiffNativeJson(layoutDiffFromNativeJson(unchanged))).toEqual(unchanged);
 expect(layoutDiffNativeJson(layoutDiffFromNativeJson(cleared))).toEqual(cleared);
});
