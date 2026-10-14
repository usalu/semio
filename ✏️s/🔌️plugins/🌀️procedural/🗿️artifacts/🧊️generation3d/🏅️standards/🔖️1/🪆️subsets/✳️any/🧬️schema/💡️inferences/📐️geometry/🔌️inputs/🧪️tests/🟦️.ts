import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️.json";
import schema from "./🧬️.json";

test("original WidgetInputs builds actual ordered labels and donates resolved bodies",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 expect([...new Map(fixture.ports.map(port=>[port.name,{en:port.en,de:port.de}])).keys()].sort()).toEqual(fixture.expectedOrder);
 const source=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();
 expect(source).toContain("labels: RetainedOrderedMap<String, Localized>");
 expect(source).toContain("values: Outputs");
 const constructor=await Bun.file(new URL("../🏗️constructor/🦀️.rs",import.meta.url)).text();
 expect(constructor).not.toContain("WidgetInputs::new(");
 expect(constructor).not.toContain(".clone()");
 expect(constructor).toContain("cx.original_source_parent()?.source_identity()?");
 expect(fixture.valuesDonation).toEqual({headerOnly:true,payloadCopies:0});
});
