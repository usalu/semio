/** 🧪️ Executable semantic mutation, sparse event, and exact undo witnesses for all canonical operations. */
import { deepStrictEqual, strictEqual } from "node:assert";
import { readdirSync, readFileSync } from "node:fs";
import { resolve, join } from "node:path";
import structure from "../../../../🧪️tests/🧫️fixtures/🧬️structure/🔣️.json" with {type:"json"};
import graph from "../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json" with {type:"json"};
import semanticWords from "../../../../🧪️tests/🧫️fixtures/🟰️words/🔣️.json" with {type:"json"};
import identityWords from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};
import semantic from "../../../../🧪️tests/🧫️fixtures/⚙️semantic/🔣️.json" with {type:"json"};
import { layoutArtifactFromNativeJson, layoutArtifactNativeJson } from "../../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
import { layoutMutationFromNativeJson, parseLayoutMutation } from "../../🪪️native-json/🟦️.ts";
import { applyLayoutDiff, inverseLayoutDiff } from "../../../🔺️diff/🧮️diff-apply-and-algebra/🟦️.ts";
import { evaluateLayoutMutation, LAYOUT_MUTATION_KINDS } from "../../⚙️evaluate/🟦️.ts";
import { binary64 } from "@semio-tech/framework/value/ieee754";
import { independentDifference, projectPatch } from "./oracle.mjs";

/** ⚖️ Executes committed semantic inputs, central application and storage-reversed semantic inverses. */
export function testLayoutMutationOracle(): {portableCases:number;goldenCases:number;variants:number;wordCases:number;drawingAuthorityRefusals:number} {
  const covered = new Set<string>();
  const law = (fixture:{name:string;before:unknown;mutation:unknown;after:unknown;patch?:unknown;outcome?:{code?:string;messages?:{code:string}[]}}):void => {
    const patch = fixture.patch ?? independentDifference(fixture.before,fixture.after);
    deepStrictEqual(projectPatch(fixture.before,patch),fixture.after,fixture.name+": independent RFC 6902");
    const base=layoutArtifactFromNativeJson(fixture.before), saved=structuredClone(base), mutation=layoutMutationFromNativeJson(fixture.mutation), expected=layoutArtifactFromNativeJson(fixture.after);
    covered.add(Object.keys(mutation)[0]);
    const result=evaluateLayoutMutation(base,mutation), after=applyLayoutDiff(base,result.diff);
    deepStrictEqual(after,expected,fixture.name+": semantic publication");
    if(fixture.outcome?.code)strictEqual(result.messages.some(message=>message.code===fixture.outcome?.code),true,fixture.name+": refusal code");
    for(const message of fixture.outcome?.messages??[])strictEqual(result.messages.some(actual=>actual.code===message.code),true,fixture.name+": diagnostic code");
    deepStrictEqual(base,saved,fixture.name+": immutable base");
    deepStrictEqual(applyLayoutDiff(after,inverseLayoutDiff(base,result.diff)),base,fixture.name+": central inverse");
    let restored=after;
    for(const operation of [...result.inverse].reverse()) restored=applyLayoutDiff(restored,evaluateLayoutMutation(restored,operation).diff);
    deepStrictEqual(restored,base,fixture.name+": exact semantic undo");
    if(JSON.stringify(fixture.before)===JSON.stringify(fixture.after)) { strictEqual(result.inverse.length,0,fixture.name+": empty unchanged inverse");strictEqual(result.messages.length>0,true,fixture.name+": no-op/refusal explanation"); }
  };
  for(const fixture of [...structure.cases,...graph.cases,...semantic.cases])law(fixture);
  const goldenRoot=resolve(process.cwd(),"✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations");
  let goldenCases=0;
  for(const relative of readdirSync(goldenRoot,{recursive:true}) as string[]) {
    if(!relative.endsWith("📸️snapshot/⬅️before/🔣️.json") && !relative.endsWith("📸️snapshot\\⬅️before\\🔣️.json"))continue;
    const directory=resolve(goldenRoot,relative).replace(/[\\/]📸️snapshot[\\/]⬅️before[\\/]🔣️.json$/u,"");
    const read=(...parts:string[]):unknown=>JSON.parse(readFileSync(join(directory,...parts),"utf8"));
    law({name:relative,before:read("📸️snapshot","⬅️before","🔣️.json"),mutation:read("🦠️mutation","🔣️.json"),after:read("📸️snapshot","➡️after","🔣️.json"),outcome:read("🎯️outcome","🔣️.json") as {code?:string;messages?:{code:string}[]}});goldenCases++;
  }
  strictEqual(covered.size,LAYOUT_MUTATION_KINDS.length,"all semantic variants have portable witnesses");
  for(const kind of LAYOUT_MUTATION_KINDS) strictEqual(covered.has(kind),true,kind+": witnessed");
  const base=layoutArtifactFromNativeJson(graph.cases[0].before);
  const positiveZero=binary64(0),negativeZero=binary64(-0);
  const zeroBase=structuredClone(base);zeroBase.pages[0].frames[0].bounds.x=positiveZero;
  const operation=parseLayoutMutation({MoveFrame:{pageId:"page-1",frameId:"text-1",newX:negativeZero,newY:base.pages[0].frames[0].bounds.y}});
  const changed=evaluateLayoutMutation(zeroBase,operation), after=applyLayoutDiff(zeroBase,changed.diff);
  strictEqual(after.pages[0].frames[0].bounds.x.bits,negativeZero.bits,"negative zero is an exact authored change");
  let restored=after;for(const inverse of [...changed.inverse].reverse())restored=applyLayoutDiff(restored,evaluateLayoutMutation(restored,inverse).diff);
  deepStrictEqual(restored,zeroBase,"zero bit identity undo");
  for(const bits of [0x7ff0000000000000n,0x7ff8000000000042n,0xfff8000000000081n]) {
    const result=evaluateLayoutMutation(base,parseLayoutMutation({MoveFrame:{pageId:"page-1",frameId:"text-1",newX:{bits},newY:positiveZero}}));
    deepStrictEqual(applyLayoutDiff(base,result.diff),base,"nonfinite position refusal");strictEqual(result.messages[0].code,"mutation.invariant");
  }
  for(const fixture of graph.wordCases) {
    const before=layoutArtifactFromNativeJson(fixture.before),operation=layoutMutationFromNativeJson(fixture.mutation);
    if(!("UpdatePageColumns" in operation))throw Error("unsupported portable IEEE witness");
    operation.UpdatePageColumns.gutter={bits:BigInt("0x"+fixture.binary64)};
    const result=evaluateLayoutMutation(before,operation);strictEqual(result.messages[0].code,fixture.expectedCode,fixture.name);deepStrictEqual(applyLayoutDiff(before,result.diff),before,fixture.name);strictEqual(result.inverse.length,0,fixture.name+": refused inverse");
  }
  const opaque=structuredClone(base);opaque.characterStyles[0].color=[{bits:0x7fc00042},{bits:0},{bits:0},{bits:0x3f800000}];
  const rename=evaluateLayoutMutation(opaque,parseLayoutMutation({RenameLayout:{newName:"Preserve words"}}));
  strictEqual(applyLayoutDiff(opaque,rename.diff).characterStyles[0].color?.[0].bits,0x7fc00042,"unrelated NaN payload survives local edit");
  for(const bits of [1n,0x7fefffffffffffffn,0x8000000000000001n,0x8000000000000000n]) {
    const result=evaluateLayoutMutation(base,parseLayoutMutation({MoveFrame:{pageId:"page-1",frameId:"text-1",newX:{bits},newY:positiveZero}})),after=applyLayoutDiff(base,result.diff);
    strictEqual(after.pages[0].frames[0].bounds.x.bits,bits,"finite exact authored word");
    let restored=after;for(const inverse of [...result.inverse].reverse())restored=applyLayoutDiff(restored,evaluateLayoutMutation(restored,inverse).diff);deepStrictEqual(restored,base,"finite word semantic undo");
  }
  const atWord=(value:unknown,pointer:string):{bits:bigint|number}=>{let current=value;for(const key of pointer.split('/').slice(1))current=(current as Record<string,unknown>)[key];return current as {bits:bigint|number};};
  const setWord=(value:unknown,pointer:string,bits:bigint|number):void=>{const keys=pointer.split('/').slice(1);let current=value;for(const key of keys.slice(0,-1))current=(current as Record<string,unknown>)[key];(current as Record<string,unknown>)[keys[keys.length-1]]={bits};};
  for(const fixture of semanticWords.cases) {
    const before=layoutArtifactFromNativeJson(fixture.before),operation=layoutMutationFromNativeJson(fixture.mutation),left=fixture.width===64?BigInt('0x'+fixture.beforeBits):Number.parseInt(fixture.beforeBits,16),right=fixture.width===64?BigInt('0x'+fixture.afterBits):Number.parseInt(fixture.afterBits,16);
    setWord(before,fixture.target,left);setWord(operation,fixture.field,right);
    const event=evaluateLayoutMutation(before,parseLayoutMutation(operation)),after=applyLayoutDiff(before,event.diff);
    strictEqual(atWord(after,fixture.target).bits,right,fixture.name+': exact authored word');
    deepStrictEqual(applyLayoutDiff(after,inverseLayoutDiff(before,event.diff)),before,fixture.name+': exact central inverse');
    let undo=after;for(const inverse of [...event.inverse].reverse())undo=applyLayoutDiff(undo,evaluateLayoutMutation(undo,inverse).diff);deepStrictEqual(undo,before,fixture.name+': exact semantic inverse');
  }
  for(const fixture of semanticWords.refusals) {
    const before=layoutArtifactFromNativeJson(fixture.before),operation=layoutMutationFromNativeJson(fixture.mutation);setWord(operation,fixture.field,fixture.width===64?BigInt('0x'+fixture.bits):Number.parseInt(fixture.bits,16));
    const event=evaluateLayoutMutation(before,parseLayoutMutation(operation));deepStrictEqual(applyLayoutDiff(before,event.diff),before,fixture.name);strictEqual(event.inverse.length,0,fixture.name+': no refused inverse');strictEqual(event.messages[0].code,fixture.expectedCode,fixture.name+': invariant diagnostic');
  }
  for(const fixture of identityWords.cases.filter(row=>row.width===64)) {
    const before=structuredClone(base);before.dataFields={entries:[{questionId:'word',value:{kind:'float',value:{bits:BigInt('0x'+fixture.left)}}}]};
    const document=structuredClone(before);document.dataFields!.entries[0].value={kind:'float',value:{bits:BigInt('0x'+fixture.right)}};
    const event=evaluateLayoutMutation(before,parseLayoutMutation({ReplaceDocument:{document}})),after=applyLayoutDiff(before,event.diff);
    deepStrictEqual(after,document,fixture.name+': opaque whole replacement');strictEqual(event.inverse.length===0,fixture.equal,fixture.name+': exact no-op identity');
    let undo=after;for(const inverse of [...event.inverse].reverse())undo=applyLayoutDiff(undo,evaluateLayoutMutation(undo,inverse).diff);deepStrictEqual(undo,before,fixture.name+': opaque word undo');
  }
  const drawing=structuredClone(base);drawing.backgroundDrawing={handle:{childId:"test-owned-scene",target:{artifactId:"drawing-test",dialect:{artifactKind:"s.draw.drawing",standard:"1",subset:"*"}}}};
  let parentCommandRejected=false;try{parseLayoutMutation({SetDrawingText:{index:1,text:"Caption"}});}catch{parentCommandRejected=true;}strictEqual(parentCommandRejected,true,"parent mutation schema cannot issue a child edit");
  let inlineRejected=false;try{layoutArtifactFromNativeJson({...(layoutArtifactNativeJson(drawing) as Record<string,unknown>),backgroundDrawing:{...drawing.backgroundDrawing,content:{labels:["Plan","Note"]}}});}catch{inlineRejected=true;}strictEqual(inlineRejected,true,"parent transport cannot acquire a second mutable child head");
  return {portableCases:structure.cases.length+graph.cases.length+semantic.cases.length,goldenCases,variants:covered.size,wordCases:9+graph.wordCases.length+semanticWords.cases.length+semanticWords.refusals.length+identityWords.cases.filter(row=>row.width===64).length,drawingAuthorityRefusals:2};
}
