import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
G=L+"🏗️build/🕸️graph/"
patch(G+"🕰️generations/🧬️schema/🔣️.json",[(''' "definitions": {
  "Policy": {''',''' "definitions": {
  "Identity": {
   "const": "graph-generations-v3"
  },
  "Policy": {''')])
patch(G+"🟦️.ts",[
('''import{createGenerationFilesV1}from"./🗂️generation-files/🟦️.ts";
''','''import{createGenerationFilesV1}from"./🗂️generation-files/🟦️.ts";
import generationSchemaV1 from"./🕰️generations/🧬️schema/🔣️.json"with{type:"json"};
import{declareFormatStampV1,requireCurrentFormatStampsV1}from"../🧷️format-stamps/🟨️.mjs";
declareFormatStampV1("graph.generation",generationSchemaV1);
/** 🔖️ The declared format stamps whose move makes a loaded graph implementation obsolete: it neither reads nor builds nor publishes a generation any more. */
const graphFormatStampsV1=Object.freeze(["graph.generation","graph.record","graph.inferrers"]);
'''),
('''const requireCurrentGraphModuleV1=():void=>{if(implementationPinnedV1())return;''','''const requireCurrentGraphModuleV1=():void=>{requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");if(implementationPinnedV1())return;'''),
('''async identity(){return createHash("sha256").update(JSON.stringify(["graph-generations-v3",registryDigest])).digest("hex");},''','''async identity(){requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation","refuse");return createHash("sha256").update(JSON.stringify([generationSchemaV1.definitions.Identity.const,registryDigest])).digest("hex");},'''),
])
