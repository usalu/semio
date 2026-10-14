import{test,expect}from"bun:test";
import*as owner from"../../../../../../../../🟦️.ts";
import fixture from"../../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json";
test("EN1993 public owner declares complete relational capability",()=>{for(const name of fixture.capabilityExports)expect(Object.hasOwn(owner,name)).toBe(true)});
