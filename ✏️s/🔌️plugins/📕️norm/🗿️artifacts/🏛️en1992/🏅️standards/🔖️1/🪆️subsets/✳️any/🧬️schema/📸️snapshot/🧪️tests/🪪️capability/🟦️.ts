import{test,expect}from"bun:test";
import*as owner from"../../../../../../../../🟦️.ts";
import fixture from"../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json";
test("EN1992 actual public facade declares its relational capability",()=>{for(const name of fixture.capabilityExports)expect(Object.hasOwn(owner,name)).toBe(true)});
