import {test,expect} from "bun:test";
import {generation3dWidgetInputSelfTests} from "./🔬️unit/🟦️.ts";
import {generation3dMeshSelectionSelfTests} from "../../🥽️edit-mesh-selection/🧪️tests/🔬️unit/🟦️.ts";
/** 🎛️ Executes every real widget edit, geometry, accessibility and independent schema/DOM vector. */
test("neutral checked procedural widget values preserve existing source and DOM vectors",()=>{const started=performance.now(),checks=generation3dWidgetInputSelfTests();expect(checks).toBeGreaterThan(0);},5000);
/** 🥽️ Executes every independent exact mesh selection and schema vector. */
test("neutral checked procedural mesh selection preserves existing source vectors",()=>{const started=performance.now(),checks=generation3dMeshSelectionSelfTests();expect(checks).toBeGreaterThan(0);},5000);
