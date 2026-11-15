import hashlib,os,sys,json,shutil
S=os.path.dirname(os.path.abspath(__file__))
ROOT=sys.argv[1]
ST=ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
def put(path,text):
    tmp=path+".land-tmp";open(tmp,"w",encoding="utf8").write(text);os.replace(tmp,path)
out=[]
# 1 schema
p=ST+"/🧬️schema/🔣️.json";before=sha(p);s=open(p,encoding="utf8").read();a='\n    "DescendantCase": {'
assert s.count(a)==1 and '"AncestryRelocation"' not in s
s=s.replace(a,"\n"+open(S+"/schema-insert.txt",encoding="utf8").read().rstrip("\n")+a);json.loads(s);put(p,s);out.append((p,before,sha(p)))
# 2 boot member
p=ST+"/🌐️platform/🥾️boot/🟦️.ts";before=sha(p);s=open(p,encoding="utf8").read()
a1='import{lstatSync,readFileSync}from"node:fs";\nimport{dirname}from"node:path";';b1='import{closeSync,constants,existsSync,fsyncSync,lstatSync,openSync,readFileSync,readdirSync,renameSync,rmSync,writeFileSync}from"node:fs";\nimport{dirname,isAbsolute,join,relative,resolve,sep}from"node:path";'
a2='import schema from"../../🧬️schema/🔣️.json" with{type:"json"};';b2=a2+'\nimport mountDirectorySchema from"../📋️contract/📂️mount/🧬️schema/🔣️.json" with{type:"json"};'
assert s.count(a1)==1 and s.count(a2)==1 and "rebindNativeImageAncestryV1" not in s
s=s.replace(a1,b1).replace(a2,b2).rstrip("\n")+"\n"+open(S+"/addition.ts.txt",encoding="utf8").read();put(p,s);out.append((p,before,sha(p)))
# 3 opener
p=ST+"/🧹️cleanup/🎛️command/🟦️.ts";before=sha(p);s=open(p,encoding="utf8").read()
a='return"absent";}const folder=lstatSync(directory),image=join(directory,"fixed.dmg"),root=nativeImageRootV1(directory),receipt=join(directory,"image.json"),record='
i='import{readNativeBootSessionV1,nativeBootSessionV1,readNativeImageAncestryV1,requireNativeImageAncestryBindingV1}from"../../🌐️platform/🥾️boot/🟦️.ts";'
assert s.count(a)==1 and s.count(i)==1 and "admitNativeImageRelocationV1" not in s
s=s.replace(i,i.replace("requireNativeImageAncestryBindingV1}","requireNativeImageAncestryBindingV1,admitNativeImageRelocationV1}")).replace(a,a.replace('return"absent";}','return"absent";}admitNativeImageRelocationV1(directory);'));put(p,s);out.append((p,before,sha(p)))
# 4 tests
d=ST+"/🌐️platform/🥾️boot/🧪️tests/🚚️relocation";os.makedirs(d,exist_ok=False)
shutil.copy(S+"/live-test.ts.txt",d+"/🟦️.ts");shutil.copy(S+"/relocation.json",d+"/🔣️.json")
for f in("🟦️.ts","🔣️.json"):out.append((d+"/"+f,"NEW",sha(d+"/"+f)))
for p,b,a in out:print(p.replace(ROOT+"/",""),b[:12],"->",a[:12])
