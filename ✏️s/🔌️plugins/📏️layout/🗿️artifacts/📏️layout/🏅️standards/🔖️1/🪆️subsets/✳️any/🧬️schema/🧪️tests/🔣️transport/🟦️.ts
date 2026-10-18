/** 🧪️ Artifact-owned physical transport corpus and independent oracles: the Layout-owned facets without its composed background Drawing, whose embedded child codec is bound and verified by `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing`. */
import {test,expect} from "bun:test";
import vectors from "../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json";
import {layoutArtifactFromNativeJson,layoutArtifactNativeJson} from "../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
import {layoutDiffFromNativeJson,layoutDiffNativeJson} from "../../../🚪️io/📝️text/🔺️diff/🪪️native-json/🟦️.ts";
const withoutDrawing = (value: Record<string, unknown>, absent: unknown): Record<string, unknown> => {const {backgroundDrawing, ...rest} = value; return absent === undefined ? rest : {...rest, backgroundDrawing: absent};};
test("Layout native JSON retains the document and diff without the composed Drawing child",()=>{
  const document = withoutDrawing(vectors.document as Record<string, unknown>, undefined);
  expect(layoutArtifactNativeJson(layoutArtifactFromNativeJson(document))).toEqual(document);
  const diff = withoutDrawing(vectors.diff as Record<string, unknown>, null);
  expect(layoutDiffNativeJson(layoutDiffFromNativeJson(diff))).toEqual(diff);
});
test("Layout native JSON refuses the composed Drawing child until the composition binds its codec",()=>{
  expect(()=>layoutArtifactFromNativeJson(vectors.document)).toThrow(/no bound codec/);
});
