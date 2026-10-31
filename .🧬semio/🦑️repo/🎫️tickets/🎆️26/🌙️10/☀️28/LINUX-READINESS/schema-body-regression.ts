import {test,expect} from "bun:test";
import {proveJsonSchemaSubsetContractV1} from "./../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🟦️.ts";
test("the shared body-only validator retains its Bun, Node and Ajv corpus",async()=>{expect(await proveJsonSchemaSubsetContractV1()).toBeGreaterThan(0);},30000);
