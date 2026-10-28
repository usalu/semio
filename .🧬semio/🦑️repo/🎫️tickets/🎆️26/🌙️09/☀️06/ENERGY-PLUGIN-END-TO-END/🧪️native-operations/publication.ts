import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import law from "./publication.json";
import schema from "./publication.schema.json";
test("original publication commits or cancels snapshot, event and outbox together",()=>{
 expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
 for(const commit of [false,true]){const db=new Database(":memory:");db.exec("CREATE TABLE snapshot (zoom REAL); CREATE TABLE history (event TEXT); CREATE TABLE outbox (event TEXT)");db.query("INSERT INTO snapshot VALUES (?)").run(law.input.zoom);db.exec("BEGIN");db.query("UPDATE snapshot SET zoom=?").run(law.operation.zoom);const event=JSON.stringify(law.operation);db.query("INSERT INTO history VALUES (?)").run(event);db.query("INSERT INTO outbox VALUES (?)").run(event);db.exec(commit?"COMMIT":"ROLLBACK");expect(db.query("SELECT zoom FROM snapshot").get()).toEqual(commit?law.expected:{zoom:law.cancelled.zoom});expect(db.query("SELECT COUNT(*) AS count FROM history").get()).toEqual({count:commit?law.historyCount:law.cancelled.historyCount});expect(db.query("SELECT COUNT(*) AS count FROM outbox").get()).toEqual({count:commit?law.outboxCount:law.cancelled.outboxCount});db.close();}
});
