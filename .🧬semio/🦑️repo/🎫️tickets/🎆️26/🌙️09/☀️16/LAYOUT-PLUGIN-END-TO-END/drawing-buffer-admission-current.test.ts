/** 🎟️ Actual affected admission, row, ownership, close and caller laws execute from original modules. */
import {test} from "bun:test";
import {Database} from "bun:sqlite";
import {testRetainedWork,testRetainedWorkRetirement} from "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🧪️tests/🟦️.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/⏱️turn/🧪️tests/work.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/⏱️turn/🧪️tests/effects.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🧪️tests/absolute/absolute.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/♻️retirement/🧪️tests/retire.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📚️pages/🧪️tests/🟦️.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📚️pages/🧪️tests/contiguous.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📚️pages/🧪️tests/admission.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📚️pages/🧪️tests/admission-provenance.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📚️pages/🧪️tests/admission-behavior.test.ts";
import "./drawing-buffer-ingress.test.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📦️bytes/🧪️tests/🟦️.ts";
import "../../../../../../../🧰️framework/🔨️modules/🔲️pixels/📷️png/✍️encode/🧪️tests/🟦️.ts";
import "../../../../../../../🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🎟️work/🧪️tests/🟦️.ts";
test("original retained-work ordering, shared holds and materialization",()=>testRetainedWork(values=>{const db=new Database(":memory:");try{return db.query<{value:number},[string]>("SELECT CAST(value AS INTEGER) AS value FROM json_each(?) ORDER BY CAST(value AS INTEGER)").all(JSON.stringify(values)).map(row=>row.value);}finally{db.close();}}));
test("original retained-work paid retirement and denial axes",()=>testRetainedWorkRetirement());
