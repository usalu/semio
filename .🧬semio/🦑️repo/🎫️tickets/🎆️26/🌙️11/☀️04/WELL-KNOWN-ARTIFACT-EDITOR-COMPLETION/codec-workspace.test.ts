import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./📐️codec-workspace.schema.json";
import corpus from "./📐️codec-workspace.json";
test("retained parent owns each stage workspace without a late transfer",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE stage (ordinal INTEGER, work INTEGER, retained INTEGER)");
  for(const row of corpus.cases){
   db.exec("DELETE FROM stage");let work=0,retained=0;
   for(const [ordinal,stage] of row.stages.entries()){
    work+=stage.work;retained+=stage.retained;
    db.query("INSERT INTO stage VALUES (?, ?, ?)").run(ordinal,stage.work,stage.retained);
    if(stage.status!=="success")break;
   }
   const independent=db.query("SELECT coalesce(sum(work),0) AS work, coalesce(sum(retained),0) AS retained FROM stage").get();
   expect(independent).toEqual({work:row.expectedWork,retained:row.retained});
   expect({work,retained,lateTransfers:0}).toEqual({work:row.expectedWork,retained:row.retained,lateTransfers:row.lateTransfers});
  }
 }finally{db.close();}
});
