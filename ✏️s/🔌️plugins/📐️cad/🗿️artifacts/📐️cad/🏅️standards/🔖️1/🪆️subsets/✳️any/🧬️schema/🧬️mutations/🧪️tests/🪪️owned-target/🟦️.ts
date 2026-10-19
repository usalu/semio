import {test,expect} from "bun:test";
import Ajv from "ajv";
import cadFixture from "../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪪️owned-target/🔣️.json";
import { artifactReferenceSchema as referenceSchema } from "@semio-tech/framework/schema/artifact-reference";
import {parseCadChildPayload} from "../../🟦️.ts";
const reference=new Ajv({strict:false}).addSchema(referenceSchema).compile({$ref:referenceSchema.$id+"#/$defs/ArtifactRef"});
test("CAD typed target admission preserves neutral identities and agrees with the independent reference schema",()=>{for(const row of cadFixture.cases){expect(reference(row.payload.target)).toBe(row.name==="model-rejects-drawing-dialect"||row.valid);if(row.valid){const owned=parseCadChildPayload(row.payload,row.subset as "model"|"drawing");expect(owned.target).toEqual(row.payload.target as unknown as typeof owned.target);expect(owned.childId).toBe(row.payload.childId);}else expect(()=>parseCadChildPayload(row.payload,row.subset as "model"|"drawing")).toThrow();}});
