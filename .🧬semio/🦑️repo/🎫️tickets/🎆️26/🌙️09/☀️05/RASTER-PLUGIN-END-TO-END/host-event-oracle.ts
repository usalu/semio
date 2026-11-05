import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🛂️metadata/🔗️view/fixture.json";
const law=JSON.parse(readFileSync(path,"utf8"));
const db=new Database(":memory:");
db.run("CREATE TABLE window_routes (ordinal INTEGER, id TEXT PRIMARY KEY)");
for(const[index,row]of law.windows.entries())db.run("INSERT INTO window_routes VALUES (?,?)",[index,row.id]);
for(const row of law.cases){const actual=row.view&&!row.activeAction&&!row.cancelled?(db.query("SELECT id FROM window_routes WHERE id NOT IN (SELECT value FROM json_each(?)) ORDER BY ordinal").all(JSON.stringify(row.retired??[]))as{id:string}[]).map(value=>value.id):[];if(JSON.stringify(actual)!==JSON.stringify(row.expected))throw Error(row.name+": independent SQLite routing differs");}
db.close();console.log(JSON.stringify({cases:law.cases.length,oracle:"SQLite ordered relation and JSON membership",result:"pass"}));
