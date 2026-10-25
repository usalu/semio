import {resourceLeasePathV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {Database} from "bun:sqlite";
import {readdirSync,readFileSync} from "node:fs";
const directory="/Users/ueli/Documents/semio/.cache/b/l/m/data",resource=process.argv[2]!;
const path=resourceLeasePathV1(directory,resource);
console.log(path);
const db=new Database(path,{readonly:true});
for(const table of db.query("select name from sqlite_master where type='table'").all() as {name:string}[]){console.log(table.name,JSON.stringify(db.query(`select * from ${table.name} limit 20`).all()));}
const queue=path.replace(/\.sqlite$/u,".queue");
for(const name of readdirSync(queue).sort())console.log(name,readFileSync(queue+"/"+name,"utf8"));
