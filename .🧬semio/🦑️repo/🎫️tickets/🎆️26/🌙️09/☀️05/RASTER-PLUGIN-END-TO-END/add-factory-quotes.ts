import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const one = new Set(['DemoInitialSnapshotRetirementFactory','DemoMutationRetirementFactory','UnitOwnedRetirementFactory','ExactDemoInitialSnapshotRetirementFactory','SnapshotFactory','MutationFactory','TerminalSnapshotFactory','UnitFactory','RoundTripValueRetirementFactory','OriginalMutationIssuer','ZstEditOriginal','ZstEditOther','ExactMutationIssuer','MapRetirementFactory','FixtureMutationRetirement','BoolFactory','ErrorRootRetirement','FlowOwnedHostSnapshotRetirementFactory','FlowMutationRetirementFactory','TrackedChildRetirementFactory','RefusingTrackedChildRetirementFactory','DagOwnedSnapshotRetirementFactory','DagMutationRetirementFactory','OriginalIssuer','PartialIssuer','OpaqueSnapshotIssuer','ZstOriginal','ZstOther','ZstInverseOriginal','ReadFactory','HashOwnedRetirementFactory']);
function endBlock(text: string, start: number) { let depth=1, end=start+1; while(depth&&end<text.length){const c=text[end++];if(c==='{')depth++;else if(c==='}')depth--;} return end; }
function walk(path: string) {
 for(const entry of readdirSync(path,{withFileTypes:true})){
  const file=join(path,entry.name);if(entry.isDirectory()){walk(file);continue}if(!entry.name.endsWith('.rs'))continue;
  const original=readFileSync(file,'utf8');let text=original;
  const matches=[...original.matchAll(/\bimpl(?:<[^\n{]*>)?\s+(?:[\w:]+::)?ArtifactOwnedValueRetirementFactory<([^\n{]+)>\s+for\s+(\w+)(?:<[^\n{]+>)?\s*\{/g)];
  for(const m of matches.reverse()){
   const start=m.index!,end=endBlock(text,start+m[0].length-1);const block=text.slice(start,end);if(block.includes('retire_owned_work_demand'))continue;
   const [type,name]=[m[1],m[2]];let quote:string|undefined;
   if(one.has(name))quote='1';
   if(name==='Issuer' && block.includes('work+=1'))quote='2';
   else if(name==='Issuer')quote='1';
   if(name==='RefusingFactory')quote='3';
   if(name==='Factory' && block.includes('work+=1'))quote='2';
   else if(name==='Factory')quote='1';
   if(name==='InverseIssuer')quote='1';
   if(name==='ZstInverseOther')quote='0';
   if(name==='TestLyingOwnedValueRetirementFactory')quote='2';
   if(quote===undefined){console.log('UNHANDLED '+file+' '+name);continue}
   const error=file.startsWith('🧰️framework/🔨️modules/🌱️value/')?'crate::ValueError':'semio_framework_value::ValueError';
   const insertion=`fn retire_owned_work_demand(&self,_:&${type})->Result<u64,${error}>{Ok(${quote})}`;
   text=text.slice(0,start+m[0].length)+insertion+text.slice(start+m[0].length);console.log(file+' '+name+' '+quote);
  }
  if(text!==original)writeFileSync(file,text);
 }
}
walk('🧰️framework');
