import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📦️bytes/🧪️tests/🟦️.ts";
/** 🔬️ Canonical neutral primitive laws use the actual managed Bun test entry and independent SQLite order. */
import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import {testRetainedWork,testRetainedWorkRetirement} from "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🧪️tests/🟦️.ts";
import schema from "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🔣️.json";
import fixture from "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🧪️tests/🧫️fixtures/🔣️.json";
test("canonical retained work schema is closed and current",()=>{const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,authority:true})).toBe(false);});
test("canonical retained work scalar/page/original laws match independent SQLite order",()=>{const result=testRetainedWork(values=>{const database=new Database(":memory:");try{database.run("CREATE TABLE entries(value INTEGER NOT NULL)");for(const value of values)database.run("INSERT INTO entries VALUES(?)",value);return(database.query("SELECT value FROM entries ORDER BY value").all()as {value:number}[]).map(row=>row.value);}finally{database.close();}});expect(result.sharedLedger).toBe(true);expect(result.blocked).toBeGreaterThan(0);expect(result.materialized).toBe(fixture.materializationValues);});
test("canonical retained work original retirement denies before effects and closes genuine pages",()=>{const result=testRetainedWorkRetirement();expect(result.blocked).toBeGreaterThan(0);expect(result.released).toBeGreaterThan(0);expect(result.copied).toBeGreaterThan(0);});
