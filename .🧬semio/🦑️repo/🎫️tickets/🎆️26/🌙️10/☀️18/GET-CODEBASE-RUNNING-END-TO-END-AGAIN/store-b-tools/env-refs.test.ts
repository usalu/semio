import {test,expect} from "bun:test";
import {rustCompileEnvironmentReferencesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📦️package/🌱️environment/📖️references/🟦️.ts";
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📦️package/🌱️environment/📖️references/🧪️tests/🔣️.json" with {type:"json"};
test("environment references fixture",()=>{for(const row of fixture.cases){const request={version:1,source:row.source,cfg:row.cfg,environment:row.environment};if("refusal" in row)expect(()=>rustCompileEnvironmentReferencesV1(request),row.id).toThrow(row.refusal);else expect(rustCompileEnvironmentReferencesV1(request),row.id).toEqual(row.expected as any);}});
