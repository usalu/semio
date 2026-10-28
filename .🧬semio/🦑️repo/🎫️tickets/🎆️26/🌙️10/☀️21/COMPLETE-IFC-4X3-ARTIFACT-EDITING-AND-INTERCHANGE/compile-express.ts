/** 🔬 Inspect every retained source construct before claiming semantic support. */
import {writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {compileIfc4x3Express} from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/compiler.ts";
const result=await compileIfc4x3Express();
await writeFile(resolve(import.meta.dir,"🗑️generated/express-compilation.json"),JSON.stringify(result,null,2));
console.log(JSON.stringify({complete:result.complete,rules:result.program.rules.length,derived:result.program.derived.length,functions:result.program.functions.length,globals:result.program.globalRules.length,inverse:result.program.inverse.length,bounds:result.program.bounds.length,diagnostics:result.diagnostics}));
