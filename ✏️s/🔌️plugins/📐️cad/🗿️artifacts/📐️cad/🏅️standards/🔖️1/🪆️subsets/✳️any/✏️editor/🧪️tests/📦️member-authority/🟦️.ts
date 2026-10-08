import { test, expect } from "bun:test";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
test("actual imported member semantic parts agree with independent UTF8/schema corpus", () => {
  for (const member of fixture.members) {
    expect(Buffer.concat([Buffer.from(member.entity),Buffer.from("."),Buffer.from(member.kind)]).toString()).toBe(member.schema);
    expect(new TextEncoder().encode(member.schema)).toEqual(new Uint8Array(Buffer.from(member.schema)));
  }
});
