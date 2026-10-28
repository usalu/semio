#!/usr/bin/env python3
"""🔭️ e4: witness law - the git ignore kind `i` is decided only by the `ignore` port: no port, a refusing port and a changed answer all break the witness."""
import sys
F = sys.argv[1] + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧪️tests/🟦️.ts"
text = open(F, encoding="utf8").read()
anchor = 'test("the switch reads on, off and verify'
assert text.count(anchor) == 1
law = '''test("an observation of git's ignore sources is decided by the ignore port alone: without a port, with a refusing port and with a changed answer the witness breaks",async()=>{
 const{base,root}=world();
 try{
  const witness=run(base,root,`const w=globalThis.__semioWorldWitnessV1;w.observe("i","excludes","d1");w.finish();`);
  expect(witness.tainted).toEqual([]);expect(witness.observations).toContainEqual(["i","excludes","d1"]);
  expect(await verifyWorldWitnessV1(witness,{signal})).toMatchObject({holds:false,kind:"i"});
  let answer="d1";const ignore=async(_subject:string,expected:string)=>expected===answer;
  expect((await verifyWorldWitnessV1(witness,{signal,ports:{ignore}})).holds).toBe(true);
  answer="d2";expect(await verifyWorldWitnessV1(witness,{signal,ports:{ignore}})).toMatchObject({holds:false,kind:"i",subject:"excludes"});
  expect(Object.keys(schema.definitions.Policy.const.kinds)).toContain("i");
 }finally{rmSync(base,{recursive:true,force:true});}
});

'''
text = text.replace(anchor, law + anchor)
open(F, "w", encoding="utf8").write(text)
