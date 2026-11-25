import os
H="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:60],s.count(old))
        s=s.replace(old,new)
    tmp=path+".f41.tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
IMP='../../../../🔖️identity/🟦️.ts'
edit(H+"🎫️gate/📖️status/🟦️.ts",[
('import {spawnSync} from "node:child_process";\n','import {spawnSync} from "node:child_process";\nimport {readProcessStartV1} from "'+IMP+'";\n'),
(' if(platform==="win32"||!Number.isInteger(pid)||pid<1)return undefined;\n try{',' if(!Number.isInteger(pid)||pid<1)return undefined;\n if(platform==="win32"){const read=readProcessStartV1(pid,platform);return typeof read==="number"?read:undefined;}\n try{'),
("(good to a second), `undefined` when it cannot be told (Windows, a missing process, a failing command)","(good to a second), `undefined` when it cannot be told (a missing process, a failing command)") if False else ("`undefined` when it cannot be told (Windows, a missing process, a failing command)","`undefined` when it cannot be told (a missing process, a failing command); Windows reads the creation time through the shared process identity"),
])
edit(H+"🛡️warden/🔁️loop/🟦️.ts",[
('import {createHash} from "node:crypto";\n','import {createHash} from "node:crypto";\nimport {readProcessStartV1} from "'+IMP+'";\n'),
(' if(platform==="win32")return"";\n',' if(platform==="win32"){const read=readProcessStartV1(pid,platform);return typeof read==="number"?String(read):"";}\n'),
])
print("ok")
