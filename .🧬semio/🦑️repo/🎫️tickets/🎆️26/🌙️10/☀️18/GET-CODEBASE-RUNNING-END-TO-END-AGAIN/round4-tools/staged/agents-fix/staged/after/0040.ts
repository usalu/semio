import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
type Case={name:string;snapshotClone:boolean;mutationClone:boolean;coldMutation:boolean;referenceSelected:boolean;native:boolean;reference:boolean};
/** 🪪️ Independent SQLite capability selection preserves the native retained facet for immutable original members. */
test("native and explicitly selected reference member facets have independent capabilities",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));const fixture=read("./🧫️fixtures/🔣️.json"),schema=read("../🧬️schema/🔣️.json");const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(fixture)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE inputs(name TEXT PRIMARY KEY,ordinary_clone INTEGER,mutation_clone INTEGER,cold_mutation INTEGER,reference_selected INTEGER,native INTEGER,reference INTEGER)");for(const row of fixture.cases as Case[])db.query("INSERT INTO inputs VALUES(?,?,?,?,?,?,?)").run(row.name,+row.snapshotClone,+row.mutationClone,+row.coldMutation,+row.referenceSelected,+row.native,+row.reference);const rows=db.query("SELECT name,native,(ordinary_clone AND mutation_clone AND cold_mutation AND reference_selected) AS admitted,reference FROM inputs ORDER BY name").all()as{name:string;native:number;admitted:number;reference:number}[];for(const row of rows){expect(row.native).toBe(1);expect(row.admitted).toBe(row.reference);}expect(new Set([...fixture.nativeMethods,...fixture.referenceMethods]).size).toBe(fixture.nativeMethods.length+fixture.referenceMethods.length);}finally{db.close();}
});