import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import law from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native/🗓️schedule/🧪️tests/🔣️.json";
import schema from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native/🗓️schedule/🧪️tests/📐️.json";
test("original ordered schedule intents and exact inverse agree with SQLite JSON rows",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE items(ord INTEGER PRIMARY KEY,val TEXT)");
 const apply=(action:string,p:any,before:any[])=>{db.exec("DELETE FROM items");db.query("INSERT INTO items SELECT key,json(value)FROM json_each(?)").run(JSON.stringify(before));const n=before.length;
 let outcome="changed";if(p.id!==(action==="change-day"?20:10))outcome="missing";
 else{const result=db.query(`SELECT CASE
 WHEN ?='insert-rule' AND ?>? THEN 'missing'
 WHEN ?='insert-rule' AND (? NOT BETWEEN 1 AND 12 OR ? NOT BETWEEN 1 AND 12 OR ? NOT BETWEEN 1 AND 31 OR ? NOT BETWEEN 1 AND 31) THEN 'invalid'
 WHEN ? IN('insert-rule','change-day') AND ?='change-day' AND ? NOT BETWEEN 0 AND 6 THEN 'invalid'
 WHEN ? IN('insert-rule','change-day') AND ? NOT IN(1,2,3) THEN 'invalid-reference'
 WHEN ?='add-holiday' AND (? NOT BETWEEN 1 AND 12 OR ? NOT BETWEEN 1 AND 31) THEN 'invalid'
 WHEN ? IN('add-holiday','remove-rule') AND (?<0 OR ? >= ?+CASE WHEN ?='add-holiday' THEN 1 ELSE 0 END) THEN 'missing'
 WHEN ?='reorder-rules' AND (? >= ? OR ? >= ?) THEN 'missing'
 WHEN ?='reorder-rules' AND ?=? THEN 'no-op'
 WHEN ?='add-holiday' AND EXISTS(SELECT 1 FROM items WHERE json(val)=json(?)) THEN 'no-op'
 WHEN ?='remove-holiday' AND NOT EXISTS(SELECT 1 FROM items WHERE json(val)=json(?)) THEN 'missing'
 WHEN ?='change-day' AND (SELECT json_extract(val,'$') FROM items WHERE ord=?)=? THEN 'no-op'
 ELSE 'changed' END AS outcome`).get(action,p.index??0,n,action,p.startMonth??1,p.endMonth??1,p.startDay??1,p.endDay??1,action,action,p.dayIndex??0,action,p.dailyScheduleId??p.newDailyScheduleId??1,action,p.month??1,p.day??1,action,p.index??0,p.index??0,n,action,action,p.from??0,n,p.to??0,n,action,p.from??0,p.to??0,action,JSON.stringify([p.year,p.month,p.day]),action,JSON.stringify([p.year,p.month,p.day]),action,p.dayIndex??0,p.newDailyScheduleId??0)as {outcome:string};outcome=result.outcome;}
 if(outcome!=="changed")return {outcome,after:before};
 if(action==="insert-rule"||action==="add-holiday"){db.query("UPDATE items SET ord=ord+100000 WHERE ord>=?").run(p.index);db.exec("UPDATE items SET ord=ord-99999 WHERE ord>=100000");const value=action==="insert-rule"?{start_month:p.startMonth,start_day:p.startDay,end_month:p.endMonth,end_day:p.endDay,daily_schedule_id:p.dailyScheduleId}:[p.year,p.month,p.day];db.query("INSERT INTO items VALUES(?,?)").run(p.index,JSON.stringify(value));}
 else if(action==="remove-rule")db.query("DELETE FROM items WHERE ord=?").run(p.index);
 else if(action==="remove-holiday")db.query("DELETE FROM items WHERE json(val)=json(?)").run(JSON.stringify([p.year,p.month,p.day]));
 else if(action==="reorder-rules"){db.query("UPDATE items SET ord=ord+100000").run();db.query(`UPDATE items SET ord=CASE WHEN ord-100000=? THEN ? WHEN ?<? AND ord-100000 BETWEEN ? AND ? THEN ord-100001 WHEN ?>? AND ord-100000 BETWEEN ? AND ? THEN ord-99999 ELSE ord-100000 END`).run(p.from,p.to,p.from,p.to,p.from+1,p.to,p.from,p.to,p.to,p.from-1);}
 else db.query("UPDATE items SET val=? WHERE ord=?").run(JSON.stringify(p.newDailyScheduleId),p.dayIndex);
 const after=JSON.parse((db.query("SELECT json_group_array(json(val))AS result FROM(SELECT val FROM items ORDER BY ord)").get()as {result:string}).result);return {outcome,after};};
 const inverseActions:Record<string,string>={insertAnnualScheduleRule:"insert-rule",removeAnnualScheduleRule:"remove-rule",reorderAnnualScheduleRules:"reorder-rules",addAnnualScheduleHoliday:"add-holiday",removeAnnualScheduleHoliday:"remove-holiday",changeWeeklyScheduleDay:"change-day"};
 const wideBefore=Array.from({length:law.wideRuleCount},(_,rank)=>law.rules[rank%3]);const wideAfter=[...wideBefore.slice(1),wideBefore[0]];expect(apply("reorder-rules",{id:10,from:0,to:law.wideRuleCount-1},wideBefore)).toEqual({outcome:"changed",after:wideAfter});expect(apply("reorder-rules",{id:10,from:law.wideRuleCount-1,to:0},wideAfter)).toEqual({outcome:"changed",after:wideBefore});
 for(const row of law.cases){const actual=apply(row.action,row.payload,row.before);expect(actual.outcome).toBe(row.outcome);expect(actual.after).toEqual(row.after);if(row.inverse){expect(apply(inverseActions[row.inverse.mutation],row.inverse,actual.after)).toEqual({outcome:"changed",after:row.before});}}
 }finally{db.close();}
});
