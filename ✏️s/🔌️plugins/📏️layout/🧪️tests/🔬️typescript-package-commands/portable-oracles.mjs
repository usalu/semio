/** 🔮️ Independent library witnesses implement the existing owned portable test interfaces. */
import {createRequire} from "node:module";
import {pathToFileURL} from "node:url";
const require=createRequire(import.meta.url),three=require("three"),lodash=require("lodash");
import {sourcePlanOracle as source} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🧭️source-plan/🧪️tests/oracles.mjs";
import {createFrameGeometryOracle as frame} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/📐️geometry/🧪️tests/oracle.mjs";
import {createBackgroundGeometryOracle as background} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/🖍️background/📐️geometry/🧪️tests/oracle.mjs";
import {createDrawingLiftOracle as lift} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🧪️tests/oracles.mjs";
import {createDrawingBirthOracle as birth} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🌱️birth/🧪️tests/oracles.mjs";
import {createTextXmlOracle as xml} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/oracle.mjs";
import {createPdfPathOracle as pdf} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📕️pdf/🖍️paths/🧪️tests/oracle.mjs";
import {drawingGraphOracle as graph} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧵️graph-oracle.mjs";
import {run as plan} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🔮️oracle.mjs";
import {run as birthSchema} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/🧪️tests/🔮️oracle.mjs";
import {run as transferSchema} from "../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/📤️transfer/🧪️tests/🔮️oracle.mjs";
export const sourcePlanOracle=source;
export const frameGeometryOracle=frame(three);
export const backgroundGeometryOracle=background(three);
export const xmlOracle=xml(require("fast-xml-parser"));
export const drawingGraphOracle=graph;
export const equal=(a,b)=>lodash.isEqual(a,b);
export const targetOracle={findIndex:(keys,target)=>lodash.findIndex(keys,key=>key===target)};
export const patchOracle={apply:(before,operations)=>require("fast-json-patch").applyPatch(structuredClone(before),structuredClone(operations),true,false).newDocument};
export const drawingLiftOracle=lift;
export const drawingBirthOracle=birth;
export const runExportPlan=plan;
export const runDrawingBirthSchema=birthSchema;
export const runDrawingTransferSchema=transferSchema;
export async function pdfPathOracle(){
 const canvas=require("@napi-rs/canvas");
 for(const name of ["DOMMatrix","ImageData","Path2D"])if(typeof globalThis[name]==="undefined")globalThis[name]=canvas[name];
 const pdfjs=await import("pdfjs-dist/legacy/build/pdf.mjs");
 pdfjs.GlobalWorkerOptions.workerSrc=pathToFileURL(require.resolve("pdfjs-dist/legacy/build/pdf.worker.mjs")).href;
 return pdf(pdfjs,canvas,three);
}
