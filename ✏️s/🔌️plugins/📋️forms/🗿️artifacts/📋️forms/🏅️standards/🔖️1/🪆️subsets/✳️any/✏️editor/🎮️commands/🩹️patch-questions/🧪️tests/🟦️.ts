import {textEditorValueSourceActionV1} from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/📄️value-source/🟦️.ts";
import providerSchema from "../../../../../../../../../../../📖️playbook/🧩️extensions/🌀️procedural/🧬️schema/🔣️.json";
import assert from "node:assert/strict";
import Ajv from "ajv";
import {normalizePatchQuestions} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import large from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📄️value-source/🔣️.json";

/** 🩹️ Scalar and paged authoring produce the same exact JSON value. */
export function testFormsQuestionChunkIngress():void{
 const validate=new Ajv().compile(schema);
 const limit=providerSchema.$defs.QuestionControlTransport.properties.blueprintEncodedBytes.const;
 for(const row of fixture.cases){const value=normalizePatchQuestions(row.input);assert.equal(validate(value),true,JSON.stringify(validate.errors));assert.deepEqual(JSON.parse(value.valueJsonChunks.join("")),row.expected);}
 for(const input of fixture.invalid)assert.throws(()=>normalizePatchQuestions(input));
 for(const row of large.largeSources){const source=row.source.prefix+row.source.unit.repeat(row.source.repeat)+row.source.suffix;const value=normalizePatchQuestions({questionIds:["q"],field:"param",paramKey:"__flow",value:source});assert.equal(Buffer.byteLength(source),row.rawBytes);assert.equal(Buffer.byteLength(JSON.stringify(source)),row.encodedBytes);assert.equal(validate(value),true);assert.equal(JSON.parse(value.valueJsonChunks.join("")),source);const action=textEditorValueSourceActionV1({readOnly:false,commit:"change",editControllerId:"forms",editAction:"patchQuestions",editArgument:"valueJsonChunks",editEncoding:"json-chunks",editMaximumBytes:limit,editArguments:{questionIds:["q"],field:"param",paramKey:"__flow"},label:"Flow"},{controllerId:"editor",action:"textEdit",args:{text:source}});assert.equal(action.kind,"dispatch");if(action.kind!=="dispatch")throw Error("near-bound source must dispatch");assert.deepEqual(normalizePatchQuestions(action.action.args),value);assert.ok(value.valueJsonChunks.every(chunk=>Buffer.byteLength(chunk)<=512));assert.ok(Buffer.byteLength(JSON.stringify({action:"patchQuestions",args:value}))<131072);}
 const maximal=normalizePatchQuestions({questionIds:Array(64).fill("ä".repeat(64)),field:"ä".repeat(64),paramKey:"ä".repeat(64),value:"ä".repeat(16384)});
 const wire=JSON.stringify({action:"patchQuestions",args:maximal});
 const escaped=wire.replace(/[\u007f-\uffff]/g,c=>"\\u"+c.charCodeAt(0).toString(16).padStart(4,"0"));
 assert.ok(Buffer.byteLength(escaped)<131072);

 const boundary=large.encodingBoundaries;
 for(const row of boundary.cases){
  const source=boundary.unit.repeat(boundary.repeat)+row.suffix;
  assert.equal(Buffer.byteLength(JSON.stringify(source)),row.encodedBytes);
  const commit=()=>textEditorValueSourceActionV1({readOnly:false,commit:"change",editControllerId:"forms",editAction:"patchQuestions",editArgument:"valueJsonChunks",editEncoding:"json-chunks",editMaximumBytes:limit,editArguments:{questionIds:["q"],field:"param",paramKey:"note"},label:"Note"},{controllerId:"editor",action:"textEdit",args:{text:source}});
  if(row.accepted){const result=commit();assert.equal(result.kind,"dispatch");if(result.kind!=="dispatch")throw Error("admitted source must dispatch");const value=normalizePatchQuestions(result.action.args);assert.equal(validate(value),true);assert.equal(JSON.parse(value.valueJsonChunks.join("")),source);}
  else{assert.deepEqual(commit(),{kind:"refused",reason:"source-too-large"});assert.throws(()=>normalizePatchQuestions({questionIds:["q"],field:"param",paramKey:"note",value:source}));}
 }
 assert.equal(limit,boundary.maximumBytes);
 assert.equal(providerSchema.$defs.QuestionControlTransport.properties.answerEncodedBytes.const,16384);
}
