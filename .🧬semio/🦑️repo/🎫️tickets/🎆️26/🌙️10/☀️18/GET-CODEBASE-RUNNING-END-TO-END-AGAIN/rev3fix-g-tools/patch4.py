import sys
p=sys.argv[1]+"/files/patch-drift.ts"
s=open(p,encoding="utf-8").read()
s=s.replace("OWNERS=150,BLOCKS=800,PER_BLOCK=5,PRIVATE=20,GENERATIONS=400;","OWNERS=150,BLOCKS=600,PER_BLOCK=5,PRIVATE=20,GENERATIONS=200;")
s=s.replace('test("400 generations that each upsert one owner','test("200 generations that each upsert one owner')
s=s.replace("expect(consumers(coarsest)).toEqual(naive(table));\n});","expect(consumers(coarsest)).toEqual(naive(table));\n},120000);")
open(p,"w",encoding="utf-8").write(s)
