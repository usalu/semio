import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/📋️profiles/🔣️.json";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧬️schema/🔣️.json";
test("the reset profile fixture validates against the schema and agrees with the profile oracle",()=>{
 expect(new Ajv({strict:true}).validate({$defs:schema.$defs,$ref:"#/$defs/ResetOwnerFixture"},fixture)).toBe(true);
 const db=new Database(":memory:");for(const row of fixture.cases){const accepted=(db.query("select (?1 in('graph','nx','rust','products') and ?2 in('success','absent','resume')) as ok").get(row.profile,row.scenario) as {ok:number}).ok;expect(Boolean(accepted),row.id).toBe(row.accepted);}
 expect(fixture.cases.some(row=>row.profile==="rust"&&row.accepted)).toBe(true);expect(fixture.cases.find(row=>row.id==="foreign-profile")!.accepted).toBe(false);expect(fixture.cases.some(row=>row.profile==="products"&&row.accepted)).toBe(true);
});
