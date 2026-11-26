// [DEBUG] runs the stdio composition generator directly (the router entry needs a parent invocation envelope)
import { prepareStdioComposition, prepareStdioCatalogDefinition } from "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio", hubRoot = "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio";
console.log(JSON.stringify(prepareStdioComposition(repoRoot, hubRoot)));
console.log(JSON.stringify(prepareStdioCatalogDefinition(repoRoot, hubRoot)));
