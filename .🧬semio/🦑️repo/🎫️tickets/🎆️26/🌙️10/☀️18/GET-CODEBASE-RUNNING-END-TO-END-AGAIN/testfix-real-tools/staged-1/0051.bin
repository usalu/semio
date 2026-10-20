/** 🏷️ Every mutation attribute vector is accepted or refused exactly as the neutral corpus declares under an independent SQLite projection of the attribute grammar. */
import { strict as assert } from "node:assert";
import { Database } from "bun:sqlite";
import { test } from "bun:test";
import corpus from "../🧫️fixtures/🏷️mutation-attributes/🔣️.json" with { type: "json" };

test("mutation attribute grammar agrees with the neutral corpus under SQLite", () => {
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE attribute_meta(key TEXT,value TEXT,allowed INTEGER,requiresValue INTEGER)");
    for(const vector of corpus.cases){
      db.exec("DELETE FROM attribute_meta");let syntax=true;let family="";
      for(const container of vector.attribute.matchAll(/#\[(mutations|composite)\((.*?)\)\]/g)){
        family=container[1]!;const allowed=family==="mutations"?["snapshot","diff","schema","retire_cold","retained","reference","reference_diff"]:["snapshot","op","reference"];
        for(const part of container[2]!.split(",")){const field=/^\s*([a-z_]+)(?:\s*=\s*(.+))?\s*$/.exec(part);if(!field){syntax=false;continue;}db.query("INSERT INTO attribute_meta VALUES(?,?,?,?)").run(field[1]!,field[2]??null,Number(allowed.includes(field[1]!)),Number(field[1]!=="retained"));}
      }
      const row=db.query("SELECT EXISTS(SELECT 1 FROM attribute_meta WHERE allowed=0 OR requiresValue=1 AND value IS NULL) OR EXISTS(SELECT key FROM attribute_meta GROUP BY key HAVING count(*)>1) OR (?='mutations' AND (EXISTS(SELECT 1 FROM attribute_meta WHERE key='reference')<>EXISTS(SELECT 1 FROM attribute_meta WHERE key='reference_diff'))) AS refused").get(family) as {refused:number};
      assert.equal(syntax&&!row.refused,vector.accepted,vector.name);
    }
  } finally { db.close(); }
});
