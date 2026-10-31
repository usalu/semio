import {test,expect} from "bun:test";
import {loadOwnedValidationInputsV1,referenceOwnedValidationRecordsV1,compareOwnedValidationRecordsV1,mutateOwnedValidationRecordsV1,parseOwnedValidationLinesV1} from "./owned-validation-oracle.ts";
import matrix from "./owned-validation-oracle-mutants.json";
import emitterFixture from "./owned-validation-emitter-fixture.json";
import frontierMutants from "./owned-validation-frontier-mutants.json";
import type {OwnedRecordsV1} from "./owned-validation-oracle.ts";

test("independent SQLite reference covers the canonical owned lifecycle matrix",()=>{
 const inputs=loadOwnedValidationInputsV1(),records=referenceOwnedValidationRecordsV1(inputs),result=compareOwnedValidationRecordsV1(inputs,records);
 expect(result).toEqual({cases:6,turns:records.turns.length,nativeCompared:false});
 expect(inputs.cases.maxObservedTurns).toBe(4096);
});
for(const mutation of matrix.mutants)test("independent lifecycle checker refuses "+mutation,()=>{
 const inputs=loadOwnedValidationInputsV1(),records=referenceOwnedValidationRecordsV1(inputs);
 expect(()=>compareOwnedValidationRecordsV1(inputs,mutateOwnedValidationRecordsV1(records,mutation))).toThrow();
 expect(()=>compareOwnedValidationRecordsV1(inputs,mutateOwnedValidationRecordsV1(emitterFixture as OwnedRecordsV1,mutation))).toThrow();
});
test("actual record parser requires complete closed JSON and preserves every observed row",()=>{
 const inputs=loadOwnedValidationInputsV1(),records=referenceOwnedValidationRecordsV1(inputs),lines=[...records.turns.map(row=>"[DEBUG] [owned-layout-turn] "+JSON.stringify(row)),...records.summaries.map(row=>"[DEBUG] [owned-layout-validation] "+JSON.stringify(row))];
 expect(compareOwnedValidationRecordsV1(inputs,parseOwnedValidationLinesV1(lines))).toEqual({cases:6,turns:records.turns.length,nativeCompared:false});
 expect(()=>parseOwnedValidationLinesV1(["[DEBUG] [owned-layout-turn] {"])).toThrow();
 const interleaved=lines.map(line=>"test owned::law ... "+line);
 expect(compareOwnedValidationRecordsV1(inputs,parseOwnedValidationLinesV1(interleaved))).toEqual({cases:6,turns:records.turns.length,nativeCompared:false});
 expect(()=>parseOwnedValidationLinesV1(['20 | println!("'+lines[0]+'");'])).toThrow();
 expect(()=>parseOwnedValidationLinesV1([JSON.stringify({compilerExcerpt:lines[0]})])).toThrow();
 expect(()=>compareOwnedValidationRecordsV1(inputs,parseOwnedValidationLinesV1([]))).toThrow();
});

test("actual observations overflow refuses rather than truncating the bounded record set",()=>{
 const inputs=loadOwnedValidationInputsV1(),records=referenceOwnedValidationRecordsV1(inputs),name=records.summaries[0]!.name,seed=records.turns.find(row=>row.name===name)!;
 records.turns=records.turns.filter(row=>row.name!==name);
 for(let ordinal=1;ordinal<=inputs.cases.maxObservedTurns+1;ordinal++)records.turns.push({...structuredClone(seed),ordinal});
 expect(()=>compareOwnedValidationRecordsV1(inputs,records)).toThrow("bounded contiguous observations");
});

test("literal emitter-shaped matrix preserves one-based ordinals initial move and early index refusal",()=>{
 const inputs=loadOwnedValidationInputsV1(),records=emitterFixture as OwnedRecordsV1;
 expect(compareOwnedValidationRecordsV1(inputs,records)).toEqual({cases:6,turns:records.turns.length,nativeCompared:false});
 expect(records.turns[0]!.ordinal).toBe(1);
 expect(records.summaries.find(row=>row.name==="cancel-before-index")!.relocations).toBe(1);
 expect(records.summaries.find(row=>row.name==="complete-invalid")!.programBirths).toBe(0);
 expect(records.turns.some(row=>row.name==="complete-invalid"&&row.op==="advance"&&row.done&&row.demand[3]>1)).toBe(true);
 const lines=[...records.turns.map(row=>"[DEBUG] [owned-layout-turn] "+JSON.stringify(row)),...records.summaries.map(row=>"[DEBUG] [owned-layout-validation] "+JSON.stringify(row))];
 expect(compareOwnedValidationRecordsV1(inputs,parseOwnedValidationLinesV1(lines))).toEqual({cases:6,turns:records.turns.length,nativeCompared:false});
});

for(const row of frontierMutants.cases)test("canonical cancellation frontier refuses "+row.id,()=>{
 const inputs=loadOwnedValidationInputsV1(),records=structuredClone(emitterFixture as OwnedRecordsV1),donor=records.summaries.find(summary=>summary.name===row.donor)!;
 const transplanted=records.turns.filter(turn=>turn.name===row.donor).map(turn=>({...turn,name:row.target}));
 records.turns=records.turns.filter(turn=>turn.name!==row.target).concat(transplanted);
 records.summaries[records.summaries.findIndex(summary=>summary.name===row.target)]={...structuredClone(donor),name:row.target};
 expect(()=>compareOwnedValidationRecordsV1(inputs,records)).toThrow("canonical cancellation frontier");
});
