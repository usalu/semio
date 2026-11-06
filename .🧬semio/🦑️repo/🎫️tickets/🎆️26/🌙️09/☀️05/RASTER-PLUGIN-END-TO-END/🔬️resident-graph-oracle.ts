import {Database} from "bun:sqlite";
import fixture from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧫️fixtures/🕸️graph-source.json" with {type:"json"};
const db=new Database(":memory:"),row=db.query("SELECT ?1=0 AND ?2='current' AND json_extract(?3,'$[0]')='graph' AS valid").get(Number(fixture.reuse),fixture.policy,JSON.stringify(fixture.arguments)) as {valid:number};db.close();if(!row.valid)throw Error("Resident graph source contract differs");console.log(JSON.stringify({cases:1,oracle:"SQLite",status:"passed"}));
