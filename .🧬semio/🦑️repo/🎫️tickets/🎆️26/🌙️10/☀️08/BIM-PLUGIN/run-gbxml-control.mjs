import { _setArgs, _appendEnv } from "@bytecodealliance/preview2-shim/cli";
import { _setPreopens } from "@bytecodealliance/preview2-shim/filesystem";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const [component, ...tests] = process.argv.slice(2);
_setPreopens({ ".": process.cwd() });
_setArgs(["gbxml-control-tests", "--test-threads=1", ...tests]);
if (process.env.BIM_GBXML_HOUSE) _appendEnv({ BIM_GBXML_HOUSE: process.env.BIM_GBXML_HOUSE });
const { run } = await import(pathToFileURL(resolve(component)));
run.run();
