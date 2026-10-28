import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import law from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🔣️.json";
import schema from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/📐️.json";
test("native single-Window registration follows independently stored source identity, donation and closure",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);
 for(const cutoff of [null,...law.cancelCuts]){const db=new Database(":memory:");try{
 db.exec("CREATE TABLE source(kind TEXT PRIMARY KEY,identity INTEGER,owner TEXT); CREATE TABLE factory(lane TEXT PRIMARY KEY,identity INTEGER,owner TEXT)");
 for(const [rank,phase]of law.phases.entries()){if(phase==="ready")continue;if(cutoff!==null&&rank>=cutoff)break;db.query("INSERT INTO source VALUES(?,?,?)").run(phase,rank+1,"registration");}
 for(const [rank,lane]of ["document","config","draft"].entries())db.query("INSERT INTO factory VALUES(?,?,?)").run(lane,rank+101,"store");
 const original=db.query("SELECT kind,identity FROM source ORDER BY identity").all();
 if(cutoff===null){db.exec("UPDATE source SET owner='app'; UPDATE factory SET owner='app'");expect(db.query("SELECT kind,identity FROM source ORDER BY identity").all()).toEqual(original);expect(db.query("SELECT identity FROM factory ORDER BY identity").all()).toEqual([{identity:101},{identity:102},{identity:103}]);}
 db.exec("DELETE FROM source; DELETE FROM factory");expect(db.query("SELECT COUNT(*) AS n FROM source").get()).toEqual({n:0});expect(db.query("SELECT COUNT(*) AS n FROM factory").get()).toEqual({n:0});
 }finally{db.close();}}
});
