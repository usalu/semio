import re,sys,os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60],s.count(a))
        s=s.replace(a,b)
    t=path+".h1tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,path)
which=sys.argv[1]
if which=="tools":
    edit(B+"⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts",[
    ('import {captureNxAuthorizerSourcesV1} from "./🔏️authorizers/🟦️.ts";','import {captureNxAuthorizerSourcesV1,nxAuthorizerSourcesHoldV1} from "./🔏️authorizers/🟦️.ts";\nimport {concurrentEditV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";'),
    ('    await preflightNxAuthorizersV1(staging,control,store.environment);await store.recheck();\n',
     '    await preflightNxAuthorizersV1(staging,control,store.environment);await store.recheck();\n    for(const[sources,name]of[[authorizer,"authorizer"],[compiler,"compiler"]]as const)if(!await nxAuthorizerSourcesHoldV1(root,sources,signal))throw concurrentEditV1("bytes",sources.manifest.entry,"Authorizer sources changed while the "+name+" bundle was built, it is not published","build");\n')])
else:
    import json
    add=[r"Authorizer sources changed while the \w+ bundle was built",r"^ReferenceError: \w+ is not defined",r"^\s*error(?:\[E0\d+\])?: cannot find (?:type|value|function|struct|trait|macro|module|crate|item)\b"]
    old='"Closure proof (?:leaf|authority|pack index) descriptor stamp mismatch"],'
    edit(B+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json",[(old,old[:-2]+", "+", ".join(json.dumps(x,ensure_ascii=False) for x in add)+"],")])
