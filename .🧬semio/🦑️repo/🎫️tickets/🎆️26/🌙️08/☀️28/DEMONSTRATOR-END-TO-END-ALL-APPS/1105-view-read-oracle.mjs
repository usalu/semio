import {readFileSync} from "node:fs";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
const base="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🧪️tests/read-pin";
const rows=JSON.parse(readFileSync(base+"/cases.json","utf8"));
const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(base+"/schema.json","utf8")));
if(!validate(rows))throw Error(JSON.stringify(validate.errors));
for(const c of rows){
 const original=c.present?(c.selected?202:101):null;
 const db=new Database(":memory:");db.run("CREATE TABLE owner(slot TEXT PRIMARY KEY, value INTEGER); INSERT INTO owner VALUES('registry',NULL),('pin',NULL)");
 db.query("UPDATE owner SET value=? WHERE slot='registry'").run(original);
 let s={registry:original,pin:null,locked:false,ready:false,canceled:false,closing:false,borrowed:0,refused:0,actorEpoch:1};
 db.run("CREATE TABLE actor_epoch(ordinal INTEGER PRIMARY KEY,parent INTEGER,decoder INTEGER); INSERT INTO actor_epoch VALUES(1,71,83)");
 let sqlReady=false,sqlCanceled=false,sqlClosing=false,sqlLocked=false,sqlBorrowed=0,sqlRefused=0;
 for(const e of c.events){
  if(e==="next-actor-epoch"){s.actorEpoch++;db.run("INSERT INTO actor_epoch SELECT MAX(ordinal)+1,MIN(parent),MIN(decoder) FROM actor_epoch");continue}
  if(e==="cancel"){s.canceled=true;sqlCanceled=true;continue}
  if(e==="denied"||e==="busy")continue;
  if(e.startsWith("foreign-")){s.refused++;sqlRefused++;continue}
  if(e==="borrow"){if(!s.ready||s.canceled)s.refused++;else if(s.pin!==null)s.borrowed++;if(!sqlReady||sqlCanceled)sqlRefused++;else if(db.query("SELECT value FROM owner WHERE slot='pin'").get().value!==null)sqlBorrowed++;continue}
  if(e==="close"){s.closing=true;sqlClosing=true;continue}
  if(e==="return"){if(s.pin!==null){if(s.registry!==null)throw Error("occupied");s.registry=s.pin;s.pin=null}db.transaction(()=>{const v=db.query("SELECT value FROM owner WHERE slot='pin'").get().value;if(v!==null){if(db.query("SELECT value FROM owner WHERE slot='registry'").get().value!==null)throw Error("occupied SQL");db.query("UPDATE owner SET value=? WHERE slot='registry'").run(v);db.run("UPDATE owner SET value=NULL WHERE slot='pin'")}})();continue}
  if(e==="unlock"){s.locked=false;sqlLocked=false;continue}
  if(s.canceled||s.closing){s.refused++}else if(e==="lock")s.locked=true;else if(e==="move"){s.pin=s.registry;s.registry=null}else if(e==="admit")s.ready=true;
  if(sqlCanceled||sqlClosing){sqlRefused++}else if(e==="lock")sqlLocked=true;else if(e==="move")db.transaction(()=>{db.run("UPDATE owner SET value=(SELECT value FROM owner WHERE slot='registry') WHERE slot='pin'");db.run("UPDATE owner SET value=NULL WHERE slot='registry'")})();else if(e==="admit")sqlReady=true;
 }
 const actual={restored:s.registry===original&&s.pin===null&&!s.locked,borrowed:s.borrowed,refused:s.refused,clones:0};
 const sql={restored:db.query("SELECT value FROM owner WHERE slot='registry'").get().value===original&&db.query("SELECT value FROM owner WHERE slot='pin'").get().value===null&&!sqlLocked,borrowed:sqlBorrowed,refused:sqlRefused,clones:0};
 if(JSON.stringify(actual)!==JSON.stringify(c.expected)||JSON.stringify(sql)!==JSON.stringify(c.expected))throw Error(c.name+JSON.stringify({actual,sql,expected:c.expected}));
 if(c.actorEpochs!==undefined){const epochs=db.query("SELECT COUNT(*) AS n,COUNT(DISTINCT parent) AS parents,COUNT(DISTINCT decoder) AS decoders FROM actor_epoch").get();if(s.actorEpoch!==c.actorEpochs||epochs.n!==c.actorEpochs||epochs.parents!==1||epochs.decoders!==1)throw Error("original actor source changed across poll epochs "+c.name)}
 db.close();
}
console.log("PASS schema+Ajv2020+independent SQLite custody cases="+rows.length+"; native no-clone actual-registry laws NOTRUN");
