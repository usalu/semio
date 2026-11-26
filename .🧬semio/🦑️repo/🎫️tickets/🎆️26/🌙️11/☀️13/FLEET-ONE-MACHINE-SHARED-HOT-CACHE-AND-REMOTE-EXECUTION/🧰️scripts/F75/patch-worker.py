import sys,os
F=sys.argv[1]
t=open(F,encoding="utf8").read()
def once(a,b):
    global t
    assert t.count(a)==1,(t.count(a),a[:80]);t=t.replace(a,b)
once('import document from"../🧬️schema/🔣️.json"with{type:"json"};\n','import document from"../🧬️schema/🔣️.json"with{type:"json"};\nimport{descriptorFactsOfStampV1,descriptorFactsV1,settleDescriptorV1}from"../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧷️descriptor/🟦️.ts";\nimport{isConcurrentEditV1}from"../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";\n')
once('/** 🪪️ Requires the same regular, bounded descriptor the producer observed and reports only mismatched field names. */\nfunction requireClosureReadDescriptorV1(fd,row,role){\n const actual=fstatSync(fd,{bigint:true});',
'/** 🧷️ Whether the opened leaf still carries the stamp the producer observed: a leaf nobody may touch while it is open (class `immutable` of the descriptor protocol), observed again a bounded number of times before it counts as changed. */\nfunction sameClosureReadDescriptorV1(fd,row){\n if(typeof row.stamp!=="string"||row.stamp.split(":").length!==stampFields.length||row.stamp.split(":").some(part=>!/^[0-9]+$/u.test(part)))return stamp(fstatSync(fd,{bigint:true}))===row.stamp;\n try{settleDescriptorV1({kind:"immutable",path:row.path,label:{en:"Closure proof leaf",de:"Abschlussnachweis"},expected:descriptorFactsOfStampV1(row.stamp),descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true}))});return true;}catch(error){if(isConcurrentEditV1(error))return false;throw error;}\n}\n/** 🪪️ Requires the same regular, bounded descriptor the producer observed and reports only mismatched field names. */\nfunction requireClosureReadDescriptorV1(fd,row,role){\n let actual=fstatSync(fd,{bigint:true});')
once(' if(stamp(actual)!==row.stamp){\n  const expected',' if(stamp(actual)!==row.stamp&&sameClosureReadDescriptorV1(fd,row))actual=fstatSync(fd,{bigint:true});\n if(stamp(actual)!==row.stamp){\n  const expected')
a='if(readSync(fd,Buffer.alloc(1),0,1,bytes.length)||stamp(fstatSync(fd,{bigint:true}))!==row.stamp)throw Error("Closure proof descriptor changed");'
once(a,'if(readSync(fd,Buffer.alloc(1),0,1,bytes.length)||!sameClosureReadDescriptorV1(fd,row))throw Error("Closure proof descriptor changed");')
b='if(readSync(fd,Buffer.alloc(1),0,1,size)||stamp(fstatSync(fd,{bigint:true}))!==row.stamp)throw Error("Closure proof descriptor changed");'
once(b,'if(readSync(fd,Buffer.alloc(1),0,1,size)||!sameClosureReadDescriptorV1(fd,row))throw Error("Closure proof descriptor changed");')
tmp=F+".f75tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,F);print("ok")
