import{createHash}from"node:crypto";
import{canonicalRustUnitJsonV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📋️plan/🔏️identity/🟦️.ts";
import{planRustUnitsV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts";
import fixture from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🧫️fixtures/🔣️.json";
for(const mode of["prototype","release"]){const units=structuredClone(fixture.units) as any[];for(const u of units){u.mode=mode;u.execution.policy=mode==="prototype"?"compiler-context-v2":"temporary-v1";}const c=canonicalRustUnitJsonV1(planRustUnitsV1(units,"cache/units"));console.log(mode,Buffer.byteLength(c),createHash("sha256").update(c).digest("hex"));}
