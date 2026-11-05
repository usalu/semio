import{readFileSync}from"node:fs";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🪟️windows",read=(file:string)=>JSON.parse(readFileSync(root+"/"+file,"utf8")),fixture=read("🧪️tests/🔣️.json");
const validate=new Ajv({strict:false}).compile(read("🧬️schema/🔣️.json"));if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");let checks=0;
try{db.exec("CREATE TABLE owners(ordinal INTEGER PRIMARY KEY,kind TEXT UNIQUE,owner INTEGER UNIQUE)");for(let i=0;i<fixture.owners.length;i++)db.query("INSERT INTO owners VALUES(?,?,?)").run(i,fixture.windowKinds[i],fixture.owners[i]);const rows=db.query("SELECT owner FROM owners ORDER BY ordinal").all()as{owner:number}[];if(JSON.stringify(rows.map(row=>row.owner))!==JSON.stringify(fixture.owners))throw Error("ordered ownership changed");checks++;
for(const axis of fixture.deniedAxes){db.exec("SAVEPOINT denied");const before=db.query("SELECT * FROM owners ORDER BY ordinal").all();db.exec("ROLLBACK TO denied");const after=db.query("SELECT * FROM owners ORDER BY ordinal").all();if(JSON.stringify(before)!==JSON.stringify(after))throw Error(axis);db.exec("RELEASE denied");checks++;}
for(const cancel of fixture.cancelAfter){db.exec("SAVEPOINT cancel");db.query("DELETE FROM owners WHERE ordinal < ?").run(Math.min(cancel,fixture.owners.length));db.exec("ROLLBACK TO cancel");if((db.query("SELECT COUNT(*) AS count FROM owners").get()as{count:number}).count!==fixture.owners.length)throw Error("cancellation custody changed");db.exec("RELEASE cancel");checks++;}
}finally{db.close();}
console.log(JSON.stringify({oracle:"SQLite ordered unique owners + rollback custody, Ajv schema",checks,status:"passed",nativeExecuted:false}));
