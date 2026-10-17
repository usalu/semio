import {test,expect} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {parseCargoPreparationStorageV1} from "../🟦️.ts";

test("preparation storage owns an explicit queue-safe bounded physical directory",()=>{
 const validate=new Ajv({strict:true}).compile(schema);for(const row of fixture.accepted){expect(validate(row)).toBe(true);expect(row).toEqual(parseCargoPreparationStorageV1(row));}for(const row of fixture.rejected){expect(validate(row)).toBe(false);expect(()=>parseCargoPreparationStorageV1(row)).toThrow();}expect(()=>parseCargoPreparationStorageV1({...fixture.accepted[0],directory:"/"+"x".repeat(schema.properties.directory.maxLength)})).toThrow();
});
