#!/usr/bin/env python3
"""R86 stage D: a failed stage mirrors the `request-failed <reference>` block of the service log its refusal names. usage: mk3.py STAGE_DIR"""
import sys
S=sys.argv[1]
def sub(t,old,new):
    assert t.count(old)==1,(old[:70],t.count(old));return t.replace(old,new)
rd=lambda n:open(S+"/"+n,encoding="utf8").read()
t=rd("1.base")
t=sub(t,"export function readyStoreFullV1(",'''/** 🧯️ The block of the service log that belongs to the reference a refusal line names (`... (reference 3a9fba88) ... the details under that reference are in the service log <path>.`): from its `request-failed <reference>` line on, at most `limit` characters; empty while the line names none, the log cannot be read or holds no such block. */
export function readyFailureBlockV1(lines:readonly string[],read:(path:string)=>string|undefined,limit=4000):readonly string[]{
 for(const line of [...lines].reverse()){
  const found=/\\(reference ([0-9a-f]{8})\\).*service log (\\S+?)\\.?\\s*$/u.exec(line);if(found===null)continue;
  const text=read(found[2]!);if(text===undefined)return[];
  const at=text.lastIndexOf("request-failed "+found[1]);if(at<0)return[];
  return text.slice(at,at+limit).split("\\n");
 }
 return[];
}

export function readyStoreFullV1(''')
t=sub(t,"    ports.mirror?.(`${row.name}-failed`,next.tail);","    ports.mirror?.(`${row.name}-failed`,next.tail);\n    {const block=readyFailureBlockV1(next.tail,path=>{try{return readFileSync(path,\"utf8\");}catch{return undefined;}});if(block.length>0)ports.mirror?.(`${row.name}-failure`,block);}")
open(S+"/1.new","w",encoding="utf8").write(t)
t=rd("2.base")
t+='''
test("the block of the service log that a refusal names is found by its reference",()=>{
 const log="2026 info other\\n2026 request-failed 3a9fba88 TypeError: boom\\n  at a (x.mjs:1)\\n  at b (y.mjs:2)\\n";
 const lines=["noise","The resident service met an unexpected failure (reference 3a9fba88); try again, the details under that reference are in the service log /tmp/host-1.log."];
 expect(readyFailureBlockV1(lines,path=>path==="/tmp/host-1.log"?log:undefined)).toEqual(["request-failed 3a9fba88 TypeError: boom","  at a (x.mjs:1)","  at b (y.mjs:2)",""]);
 expect(readyFailureBlockV1(lines,()=>undefined)).toEqual([]);
 expect(readyFailureBlockV1(lines,()=>"request-failed 00000000 other")).toEqual([]);
 expect(readyFailureBlockV1(["nothing"],()=>log)).toEqual([]);
 expect(readyFailureBlockV1(lines,()=>log,20)[0]!.length).toBeLessThanOrEqual(20);
});
'''
open(S+"/2.new","w",encoding="utf8").write(t)
