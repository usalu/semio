import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🔣️fixture.json";
import schema from "./📋️schema.json";
test("FEM variant tags follow declared family order",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 db.run("CREATE TABLE variants(dimension INTEGER,family TEXT,variant TEXT,ordinal INTEGER,tag INTEGER,UNIQUE(dimension,family,variant),UNIQUE(dimension,family,ordinal))");
 for(const value of fixture.cases)db.run("INSERT INTO variants VALUES (?,?,?,?,?)",value.dimension,value.family,value.variant,value.ordinal,value.tag);
 const mismatches=db.query("SELECT count(*) AS n FROM (SELECT tag,row_number() OVER(PARTITION BY dimension,family ORDER BY ordinal)-1 AS expected FROM variants) WHERE tag<>expected").get() as {n:number};
 expect(mismatches.n).toBe(0);db.close();
});
