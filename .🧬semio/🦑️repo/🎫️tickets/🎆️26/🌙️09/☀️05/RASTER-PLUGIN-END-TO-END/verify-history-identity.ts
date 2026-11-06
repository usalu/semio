import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
const rows=JSON.parse(readFileSync(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/history-identity-cases.json","utf8")) as {cases:{left:string;right:string;equal:boolean}[]};
const db=new Database(":memory:");const compare=db.query("SELECT CAST(? AS BLOB)=CAST(? AS BLOB) AS equal");
for(const row of rows.cases){const actual=(compare.get(row.left,row.right) as {equal:number}).equal!==0;if(actual!==row.equal)throw new Error("history identity fixture disagrees with SQLite binary equality");}
db.close();console.log(`History identity fixture: ${rows.cases.length} SQLite comparisons passed`);
