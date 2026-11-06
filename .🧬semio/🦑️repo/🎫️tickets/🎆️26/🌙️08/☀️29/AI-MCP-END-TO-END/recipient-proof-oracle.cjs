const fs=require("fs"),{DatabaseSync}=require("node:sqlite");
const law=JSON.parse(fs.readFileSync("🧰️framework/🔨️modules/🧵️job/⏱️context/🎟️source/🏠️native/🧪️tests/cases.json","utf8")).individualClosure;
const db=new DatabaseSync(":memory:");db.exec("CREATE TABLE slot(source INTEGER, generation INTEGER, closed INTEGER, loans INTEGER); INSERT INTO slot VALUES(1,4,0,1),(1,5,0,1)");
const own=db.prepare("SELECT closed>=generation AS proof FROM slot WHERE source=? AND generation=?");
if(Boolean(own.get(1,4).proof)!==law.before||Boolean(own.get(2,4))===law.foreignRefused)throw Error("initial identity");
db.exec("UPDATE slot SET loans=0,closed=generation WHERE source=1 AND generation=4");
if(Boolean(own.get(1,4).proof)!==law.after)throw Error("closed proof");
const sibling=db.prepare("SELECT loans FROM slot WHERE source=1 AND generation=5").get();if(Boolean(sibling.loans)!==law.siblingLive)throw Error("sibling");
const global=db.prepare("SELECT NOT EXISTS(SELECT 1 FROM slot WHERE loans>0) AS proof").get();if(Boolean(global.proof)!==law.globalWitness)throw Error("global forfeit proof");
const vm=JSON.parse(fs.readFileSync("🧰️framework/🔨️modules/🧵️job/⏱️context/🎟️source/🏠️native/🧪️tests/cases.json","utf8")).nativeExecution;
db.exec("CREATE TABLE frontier(return_pending INTEGER, loans INTEGER, needs_vm INTEGER);");
for(const [key,values] of [["pendingReturn",[1,0,1]],["ordinary",[0,1,1]],["guest",[0,0,1]]]){db.exec("DELETE FROM frontier");db.prepare("INSERT INTO frontier VALUES(?,?,?)").run(...values);const row=db.prepare("SELECT CASE WHEN return_pending=1 OR loans>0 THEN 0 ELSE needs_vm END AS credit FROM frontier").get();if(Boolean(row.credit)!==vm[key])throw Error("native VM selection differs: "+key);}
db.close();console.log("SQLite exact-recipient retirement oracle passed");
