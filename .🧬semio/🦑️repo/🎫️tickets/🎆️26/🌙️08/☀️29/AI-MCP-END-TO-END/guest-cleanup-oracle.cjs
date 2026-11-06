const fs=require("fs"),{DatabaseSync}=require("node:sqlite");
const law=JSON.parse(fs.readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/📃️policy/📥️operation/🪙️guest/🧪️tests/🧫️fixtures/cleanup.json","utf8"));
const db=new DatabaseSync(":memory:");const q=db.prepare("SELECT min(? + ?, ? + ? - (? - ?)) AS deadline");
for(const row of law.cases){const deadline=q.get(row.now,law.guestWindow,law.birth,law.lifetime,law.hostReserve,law.guestWindow).deadline;const allowed=row.now<deadline;if(allowed!==row.allowed||(allowed&&deadline!==row.deadline))throw Error("cleanup deadline");}
db.close();console.log("SQLite guest cleanup deadline oracle passed "+law.cases.length+" cases");
