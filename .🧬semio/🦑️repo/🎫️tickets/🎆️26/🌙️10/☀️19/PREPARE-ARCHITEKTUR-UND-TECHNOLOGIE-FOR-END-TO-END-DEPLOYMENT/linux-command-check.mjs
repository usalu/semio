import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import {linuxCommandPlanV1} from "./linux-command.mjs";
const rows=JSON.parse(readFileSync(new URL("linux-command-cases.json",import.meta.url),"utf8")),base={systemRoot:"C:\\Windows",localAppData:"C:\\Users\\Anna\\AppData\\Local",artifactDirectory:"D:\\ticket\\generated"},db=new Database(":memory:");
try{const query=db.query("SELECT json_object('program',?1,'args',json(?2)) AS result");for(const row of rows){const expected={program:row.program,args:row.args};assert.deepEqual(JSON.parse(query.get(row.program,JSON.stringify(row.args)).result),expected,row.id);assert.deepEqual(linuxCommandPlanV1({...base,action:row.action}),expected,row.id);}for(const change of [{systemRoot:"C:\\Windows\\..\\else"},{artifactDirectory:"relative"},{localAppData:"\\\\server\\share"},{action:"provision-docker"},{extra:true}])assert.throws(()=>linuxCommandPlanV1({...base,action:"import-debian",...change}));console.log("[DEBUG] Linux command vectors agree with SQLite",rows.length,"cases");}finally{db.close();}
