/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import {testLayoutDocumentContractOracle} from "./../🪪️document/🟦️.ts";
test("Layout snapshot and diff codecs retain the independent document and Drawing corpus",()=>testLayoutDocumentContractOracle());
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/layout-layout-rs"));
