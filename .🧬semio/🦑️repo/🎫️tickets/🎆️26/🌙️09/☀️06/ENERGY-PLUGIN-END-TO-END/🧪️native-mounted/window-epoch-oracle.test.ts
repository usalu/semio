import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import law from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🔒️epoch/🧪️tests/🔣️.json";
import schema from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🔒️epoch/🧪️tests/📐️.json";
test("two original Window candidates publish under one SQLite epoch decision",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);const db=new Database(":memory:");
 try{db.exec("CREATE TABLE visible(window TEXT PRIMARY KEY,value REAL); CREATE TEMP TABLE candidate(window TEXT PRIMARY KEY,value REAL)");for(const[i,w]of law.windows.entries())db.query("INSERT INTO visible VALUES(?,?)").run(w,law.committed[i]);
 for(const epoch of law.epochs){const before=db.query("SELECT value FROM visible ORDER BY window").all();db.exec("DELETE FROM candidate");for(const[i,w]of law.windows.entries()){db.query("INSERT INTO candidate VALUES(?,?)").run(w,epoch.values?.[i]??law.committed[i]);expect(db.query("SELECT value FROM visible ORDER BY window").all()).toEqual(before)}if(epoch.publish)db.exec("BEGIN; DELETE FROM visible; INSERT INTO visible SELECT * FROM candidate; COMMIT");db.exec("DELETE FROM candidate");expect(db.query("SELECT value FROM visible ORDER BY window").all()).toEqual(epoch.visible.map(value=>({value})));}
 }finally{db.close()}
});
