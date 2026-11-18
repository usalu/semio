import sys,os
R=os.environ.get("R148_ROOT","/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident")
F=R+"/🏛️service/🧯️failure/🟨️.mjs";D=R+"/🏛️service/🧯️failure/🟨️.d.mts";H=R+"/🏛️service/🌐️host/🟨️.mjs";S=R+"/🧬️schema/🔣️.json"
mode=sys.argv[1];backup=sys.argv[2]
files=[F,D,H,S]
EDITS={
F:[('import {renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";','import {HOST_POLICY_V1,renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";'),
 ('function failureRefusalCoreV1(error,{log,logPath,retry="later",root,platform,home,fullImage=fullImageOfV1,inspect,environment=process.env,pid=process.pid,ppid=process.ppid}){','function failureRefusalCoreV1(error,{log,logPath,retry="later",root,platform,home,fullImage=fullImageOfV1,inspect,environment=process.env,pid=process.pid,ppid=process.ppid,streak,sameCauseMaximum=HOST_POLICY_V1.sliceRestartSameCauseMaximum}){'),
 (' return{code:"internal",key:"internal",fields:{reference,log:logPath,message:failureMessageV1(error),frames:failureFramesV1(error)},retry};\n}',
  ' const message=failureMessageV1(error),count=streak===undefined?1:(streak.count=streak.cause===message?streak.count+1:1,streak.cause=message,streak.count),fields={reference,log:logPath,message,frames:failureFramesV1(error)};\n return count>=sameCauseMaximum?{code:"internal",key:"internal-repeated",fields:{...fields,count:String(count)},retry:"never"}:{code:"internal",key:"internal",fields,retry};\n}'),
 ('/** 🧯 Turns an unexpected failure of a request into the refusal the client receives. A full disk','/** 🧯 Turns an unexpected failure of a request into the refusal the client receives. The same unexpected failure (name, code and message) `sameCauseMaximum` times in a row, counted in `streak`, is deterministic and becomes the never retried `internal-repeated` refusal, as a slice worker that ends that often with the same text stops the plan. A full disk')],
D:[('ppid?:number}):{code:','ppid?:number;streak?:{cause:string;count:number};sameCauseMaximum?:number}):{code:')],
H:[(' const posix=process.platform!=="win32",instance=',' const failureStreak={cause:"",count:0};\n const posix=process.platform!=="win32",instance='),
 ('const failure=failureRefusalV1(error,{log,logPath:runtime.log,','const failure=failureRefusalV1(error,{streak:failureStreak,sameCauseMaximum:policy.sliceRestartSameCauseMaximum,log,logPath:runtime.log,')],
S:[('      "internal": "The resident service met an unexpected failure (reference {reference}): {message}{frames}; try again, the details under that reference are in the service log {log}.",\n',
   '      "internal": "The resident service met an unexpected failure (reference {reference}): {message}{frames}; try again, the details under that reference are in the service log {log}.",\n      "internal-repeated": "The resident service met the same unexpected failure {count} times in a row (reference {reference}): {message}{frames}; it is deterministic and trying again does not help, the code or the install on this machine has to change first; the details under that reference are in the service log {log}.",\n'),
  ('      "internal": "Der residente Dienst ist auf einen unerwarteten Fehler gestoßen (Referenz {reference}): {message}{frames}; versuche es erneut, die Einzelheiten zu dieser Referenz stehen im Dienstprotokoll {log}.",\n',
   '      "internal": "Der residente Dienst ist auf einen unerwarteten Fehler gestoßen (Referenz {reference}): {message}{frames}; versuche es erneut, die Einzelheiten zu dieser Referenz stehen im Dienstprotokoll {log}.",\n      "internal-repeated": "Der residente Dienst ist {count} Mal hintereinander auf denselben unerwarteten Fehler gestoßen (Referenz {reference}): {message}{frames}; er ist deterministisch und ein erneuter Versuch hilft nicht, zuerst muss sich der Code oder die Installation auf diesem Rechner ändern; die Einzelheiten zu dieser Referenz stehen im Dienstprotokoll {log}.",\n')]}
if mode=="apply":
    data={f:open(f,encoding="utf8").read() for f in files}
    for f,edits in EDITS.items():
        for a,b in edits: assert data[f].count(a)==1,(f,a[:60],data[f].count(a))
    os.makedirs(backup,exist_ok=True)
    for i,f in enumerate(files): open(backup+"/"+str(i),"w",encoding="utf8").write(data[f])
    for f,edits in EDITS.items():
        t=data[f]
        for a,b in edits: t=t.replace(a,b,1)
        open(f,"w",encoding="utf8").write(t)
else:
    for i,f in enumerate(files): open(f,"w",encoding="utf8").write(open(backup+"/"+str(i),encoding="utf8").read())
