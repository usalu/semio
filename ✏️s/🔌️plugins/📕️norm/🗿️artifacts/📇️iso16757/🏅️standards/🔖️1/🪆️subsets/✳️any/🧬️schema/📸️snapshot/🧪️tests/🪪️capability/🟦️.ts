import{test,expect}from"bun:test";
import*as owner from"../../../../../../../../🟦️.ts";
import fixture from"../../🧫️fixtures/🪶️sqlite/🔣️.json";
test("ISO16757 public owner declares complete semantic relational capability",()=>{for(const name of fixture.capabilityExports)expect(Object.hasOwn(owner,name)).toBe(true)});
