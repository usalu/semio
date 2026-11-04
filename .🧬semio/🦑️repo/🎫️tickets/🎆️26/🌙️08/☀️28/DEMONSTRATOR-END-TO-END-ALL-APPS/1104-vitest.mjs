import {startVitest} from "vitest/node";
import {resolve,join} from "node:path";
process.env.SEMIO_TEST_LEVEL="long";
const root=resolve("♻️mit-bestand/🧺️demonstrator");
const context=await startVitest("test",[],{root,config:join(root,"🧪️tests/🎚️config/🟦️.ts"),watch:false,cache:false,configLoader:"native",cacheDir:join(import.meta.dirname,"🗑️generated/1104-vitest-cache")});
if(!context)process.exitCode=1;else{process.exitCode=context.state.getCountOfFailedTests()>0||context.state.getUnhandledErrors().length>0?1:0;await context.close();}
