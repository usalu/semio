import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
patch(L+"🏗️build/👁️observation/🟨️.mjs",[
('''import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"../📥️inputs/🔏️revision/🟨️.mjs";
''','''import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"../📥️inputs/🔏️revision/🟨️.mjs";
import{declareFormatStampV1}from"../🧷️format-stamps/🟨️.mjs";
'''),
('''const recordStampV1=()=>sourceAnalysisSchemaV1.$defs.record.const.stamp;''','''const recordStampV1=()=>sourceAnalysisSchemaV1.$defs.record.const.stamp;
declareFormatStampV1("graph.record",sourceAnalysisSchemaV1);'''),
])
patch(L+"🏗️build/🕸️graph/📥️inference/🔖️stamps/🟨️.mjs",[
('''import schema from "./🧬️schema/🔣️.json" with { type: "json" };
''','''import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { declareFormatStampV1 } from "../../../🧷️format-stamps/🟨️.mjs";

declareFormatStampV1("graph.inferrers", schema);
'''),
])
D=L+"⚡️caching/🚀️bootstrap/📦️dependencies/"
patch(D+"🚦️concurrent/🟦️.ts",[
('''import schema from "./🧬️schema/🔣️.json" with {type:"json"};
''','''import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import generationSchema from "../🧾️generation/🧬️schema/🔣️.json" with {type:"json"};
import {declareFormatStampV1} from "../../../../🏗️build/🧷️format-stamps/🟨️.mjs";

declareFormatStampV1("dependency.walk",generationSchema);
'''),
])
patch(D+"🧾️generation/💾️storage/🟦️.ts",[
('''import {concurrentLanguageV1} from "../../🚦️concurrent/🟦️.ts";
''','''import {concurrentLanguageV1} from "../../🚦️concurrent/🟦️.ts";
import {requireCurrentFormatStampsV1,staleImplementationOfV1} from "../../../../../🏗️build/🧷️format-stamps/🟨️.mjs";
'''),
('''export function reportJavascriptDependencyGenerationMismatchV1(scope:string,mismatch:Readonly<{class:string;path?:string;detail?:string}>,language:string=concurrentLanguageV1(process.env)):void{
''','''export function reportJavascriptDependencyGenerationMismatchV1(scope:string,mismatch:Readonly<{class:string;path?:string;detail?:string}>,language:string=concurrentLanguageV1(process.env)):void{
 if(mismatch.class==="protocol-changed")requireCurrentFormatStampsV1(["dependency.walk"],"dependency generation receipt","refuse");
'''),
('''catch(error){if(control.signal.aborted)throw error;console.error("[DATA] Store dependency generation receipt failed "''','''catch(error){if(control.signal.aborted||staleImplementationOfV1(error))throw error;console.error("[DATA] Store dependency generation receipt failed "'''),
])
