import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import law from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🌱️initialization/🧪️tests/🔣️.json";
import schema from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🌱️initialization/🧪️tests/📐️.json";
import config from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌱️initialization/🧪️tests/🔣️.json";
import configSchema from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌱️initialization/🧪️tests/📐️.json";
test("original Energy genesis source contract agrees with independent calendar and nullable-link queries",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  const calendar=db.query("SELECT strftime('%Y',?||'-01-01')AS year,julianday((?+1)||'-01-01')-julianday(?||'-01-01')AS days").get(law.year,law.year,law.year)as{year:string,days:number};
  expect(Number(calendar.year)).toBe(law.year);expect(calendar.days).toBe(365);
  const links=db.query("SELECT json_type(?,'$.referencedModel')AS model,json_type(?,'$.weatherLink')AS weather").get(JSON.stringify(law.links),JSON.stringify(law.links))as{model:string,weather:string};
  expect(links).toEqual({model:"null",weather:"null"});
  const cancelled=db.query("SELECT count(*)AS n,min(value)AS first,max(value)AS last FROM json_each(?)").get(JSON.stringify(law.cancelCuts))as{n:number,first:number,last:number};
  expect(cancelled).toEqual({n:law.cancelCuts.length,first:0,last:law.creationStages});
 }finally{db.close();}
});
test("original Energy config defaults agree with SQLite scalar bounds and published field identity",()=>{
 expect(new Ajv().compile(configSchema)(config)).toBe(true);
 const db=new Database(":memory:");
 try{const row=db.query("SELECT json_extract(?,'$.zoneTimestepMinutes')BETWEEN 1 AND 60 AS zone,json_extract(?,'$.systemTimestepMinutes')BETWEEN 1 AND 60 AS system,json_extract(?,'$.warmupDays')BETWEEN 0 AND 365 AS warmup,json_extract(?,'$.resultField')AS field").get(...Array(4).fill(JSON.stringify(config.default)))as{zone:number,system:number,warmup:number,field:string};expect(row).toEqual({zone:1,system:1,warmup:1,field:"conductionLoss"});}finally{db.close();}
});
