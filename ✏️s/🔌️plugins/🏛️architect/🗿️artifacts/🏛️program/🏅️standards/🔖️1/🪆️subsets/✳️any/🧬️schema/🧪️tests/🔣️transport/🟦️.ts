/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import {testProgramDocumentContract} from "./../🔬️document-contract/🟦️.ts";
test("Program snapshot and diff codecs retain the independent document corpus",()=>testProgramDocumentContract(),{timeout:60000});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/architect-program-rs"));
