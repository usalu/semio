import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import law from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧱️components/🏛️declarations/🧪️tests/🔣️.json";
test("declared catalog row capability agrees with independent composite source identity",()=>{
 expect(new Ajv().compile({type:"object",required:["rowAuthority"],properties:{rowAuthority:{type:"object",required:["role","ordinal","sameCatalog","foreignCatalog"],additionalProperties:false,properties:{role:{const:"framework"},ordinal:{const:0},sameCatalog:{const:true},foreignCatalog:{const:false}}}}})(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE registration(source INTEGER,role TEXT,ordinal INTEGER,tool TEXT,PRIMARY KEY(source,role,ordinal))");for(const source of[1,2])for(const[ordinal,tool]of [...law.frameworkTools].sort().entries())db.query("INSERT INTO registration VALUES(?,?,?,?)").run(source,law.rowAuthority.role,ordinal,tool);
 const row=db.query("SELECT source,role,ordinal,tool FROM registration WHERE source=1 AND role=? AND ordinal=?").get(law.rowAuthority.role,law.rowAuthority.ordinal)as{source:number,role:string,ordinal:number,tool:string};expect(row.tool).toBe([...law.frameworkTools].sort()[0]);expect(row.source===1).toBe(law.rowAuthority.sameCatalog);expect(row.source===2).toBe(law.rowAuthority.foreignCatalog);
 }finally{db.close()}
});
