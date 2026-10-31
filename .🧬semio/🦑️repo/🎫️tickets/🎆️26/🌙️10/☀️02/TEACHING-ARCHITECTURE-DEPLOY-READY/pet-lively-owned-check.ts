import {observeCarryTransportV1} from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk/🧰️pet-kit/🤏️carry/🧪️tests/🟦️.ts";
import fixture from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk/🧰️pet-kit/🤏️carry/🧪️tests/🔣️.json";
import assert from "node:assert/strict";
import _ from "lodash";
for(const row of fixture.cases){const actual=await observeCarryTransportV1({...row,body:fixture.body,target:fixture.target});assert.equal(actual.moves.length,row.moves);assert.equal(actual.frames.length,row.frameCalls);assert.equal(_.sum(actual.frames),row.frames);assert.deepEqual(actual.buttons,["down"]);if(row.mode==="continuous")assert.deepEqual(actual.moves[1],{...fixture.target,steps:24});console.log(JSON.stringify({mode:row.mode,moves:actual.moves.length,frames:_.sum(actual.frames)}));}
