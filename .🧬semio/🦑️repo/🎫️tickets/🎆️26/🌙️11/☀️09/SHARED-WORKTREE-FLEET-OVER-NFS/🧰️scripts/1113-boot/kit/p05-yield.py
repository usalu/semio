import sys;sys.path.insert(0,__import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from lib import *
R='🔁️replica/🟦️.ts'
J='📒️journal/🟦️.ts'
sub(R,'readHealthV1,StoreStalledV1,','readHealthV1,StoreStalledV1,yieldingV1,')
sub(J,'import schema from "./🧬️schema/🔣️.json" with {type:"json"};','import {yieldingV1} from "../🛡️resilience/🟦️.ts";\nimport schema from "./🧬️schema/🔣️.json" with {type:"json"};')
sub(J,'''  let local=0;
  for(;;){
   const newline=data.indexOf(10,local);
   if(newline<0)break;
   const line=data.toString("utf8",local,newline);''','''  let local=0;
  for(;;){
   await turn();
   const newline=data.indexOf(10,local);
   if(newline<0)break;
   const line=data.toString("utf8",local,newline);''')
sub(J,''' const directory=join(storePathsV1(storeRoot).origins,originId,"segments"),events:FleetEventV1[]=[],ends:FleetCursorV1[]=[];
 let cursor:FleetCursorV1=start;
 const last=segments===undefined''',''' const directory=join(storePathsV1(storeRoot).origins,originId,"segments"),events:FleetEventV1[]=[],ends:FleetCursorV1[]=[],turn=yieldingV1();
 let cursor:FleetCursorV1=start;
 const last=segments===undefined''')
sub(R,''' const keys=new Map<number,string>(),byKey=new Map<string,number[]>(),below=new Map<string,number[]>(),scans=new Map<string,number[]>(),rivals=new Map<number,number[]>(),beaten=new Map<number,number|undefined>();''',''' const keys=new Map<number,string>(),byKey=new Map<string,number[]>(),below=new Map<string,number[]>(),scans=new Map<string,number[]>(),rivals=new Map<number,number[]>(),beaten=new Map<number,number|undefined>(),turn=yieldingV1();''')
sub(R,''' for(const id of items)if(manifestLive(replica,id))enrol(id);
 for(const id of retired)if(!keys.has(id))enrol(id);''',''' for(const id of items){if(manifestLive(replica,id))enrol(id);await turn();}
 for(const id of retired){if(!keys.has(id))enrol(id);await turn();}
 if([...keys.values()].some(key=>(foldedDirectories(replica).get(key)??0)>0)){
  ordered=[];
  for(const other of files.ids()){if(files.operationAt(other)!=="delete")ordered.push({key:clashKeyV1(book.name(other)),id:other});await turn();}
  ordered.sort((left,right)=>left.key<right.key?-1:left.key>right.key?1:0);
 }''')
sub(R,''' for(const id of items){
  if(retired.has(id))continue;
  if(!manifestLive(replica,id)){remaining.push(id);continue;}''',''' for(const id of items){
  await turn();
  if(retired.has(id))continue;
  if(!manifestLive(replica,id)){remaining.push(id);continue;}''')
sub(R,''' for(const id of former){
  if(!retired.has(id)||winnerOf(id)!==undefined)continue;''',''' for(const id of former){
  await turn();
  if(!retired.has(id)||winnerOf(id)!==undefined)continue;''')
