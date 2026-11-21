/** 🧪️ Full native Layout JSON uses reference-only genuine Drawing members and preserves sparse setters. */
import {test,expect} from 'bun:test';
import vectors from '../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json';
import {layoutArtifactFromNativeJson,layoutArtifactNativeJson} from '../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts';
import {layoutDiffFromNativeJson,layoutDiffNativeJson} from '../../../🚪️io/📝️text/🔺️diff/🪪️native-json/🟦️.ts';
test('Layout native JSON retains the full document and diff with their real Drawing child',()=>{
 expect(layoutArtifactNativeJson(layoutArtifactFromNativeJson(vectors.document))).toEqual(vectors.document);
 expect(layoutDiffNativeJson(layoutDiffFromNativeJson(vectors.diff))).toEqual(vectors.diff);
});
test('Layout native JSON preserves independent member identity and rejects inline Drawing payloads',()=>{
 const {backgroundDrawing}=vectors.document;
 if(!backgroundDrawing)throw Error('Drawing member fixture required');
 expect(backgroundDrawing.handle.childId).not.toBe(backgroundDrawing.handle.target.artifactId);
 const wrongDialect={...backgroundDrawing,handle:{...backgroundDrawing.handle,target:{...backgroundDrawing.handle.target,dialect:{...backgroundDrawing.handle.target.dialect,artifactKind:'s.stdio.semio'}}}};
 expect(()=>layoutArtifactFromNativeJson({...vectors.document,backgroundDrawing:wrongDialect})).toThrow();
 expect(()=>layoutArtifactFromNativeJson({...vectors.document,backgroundDrawing:{...backgroundDrawing,content:{}}})).toThrow();
});
test('Layout native JSON distinguishes absent and explicitly cleared Drawing setters',()=>{
 const {backgroundDrawing,...unchanged}=vectors.diff;
 const cleared={...unchanged,backgroundDrawing:null};
 expect(layoutDiffNativeJson(layoutDiffFromNativeJson(unchanged))).toEqual(unchanged);
 expect(layoutDiffNativeJson(layoutDiffFromNativeJson(cleared))).toEqual(cleared);
});
