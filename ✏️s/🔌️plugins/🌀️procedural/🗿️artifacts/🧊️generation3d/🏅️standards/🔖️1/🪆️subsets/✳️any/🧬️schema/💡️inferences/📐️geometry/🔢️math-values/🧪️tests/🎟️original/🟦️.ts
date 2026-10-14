import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️.json";
import schema from "./🧬️.json";

test("original math values produce actual scalar outputs on one finite caller",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 expect(new Set(fixture.cases.map(item=>item.kind)).size).toBe(8);
 for(const item of fixture.cases)expect(JSON.parse(JSON.stringify(item.input))).toEqual(item.output);
 const source=await Bun.file(new URL("../../../⚙️compute/🎟️original/🦀️.rs",import.meta.url)).text();
 expect(source).toContain("cx.original_source_parent()?.source_identity()?");
 expect(source).toContain("Insert::admit(map,key,value,grant)");
 expect(source).toContain("ControlledRetirement::new");
 expect(source).not.toContain("new_cold");
 expect(source).not.toContain("finish(");
 expect(source).not.toContain(".clone()");
});
