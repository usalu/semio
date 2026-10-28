// closure-of: bun closure-of.ts <workspace-relative entry>... : the Bun-runtime import closure the engine host derives for the entries.
import {realHost} from "./realhost.ts";
const {host} = await realHost("");
const closure = await host.importClosure(process.argv.slice(2));
console.log(JSON.stringify({count: closure.length, files: closure}));
