import sys,re,os
src,fn,out=sys.argv[1:4]
s=open(src).read()
i=s.index('  return (\n');j=s.index('\n  );',i)
lines=s[i+len('  return (\n'):j].split('\n')
cl=[l.strip() for l in lines]
cl=[re.sub(r'\s*&&$','',c) for c in cl if c]
body=s[:i]+'  const clauses: (()=>unknown)[] = [\n'+',\n'.join('    ()=>(%s)'%c for c in cl)+'\n  ];\n  const failed: number[] = [];\n  clauses.forEach((c,i)=>{ let ok=false; try{ok=!!c();}catch{} if(!ok) failed.push(i); });\n  console.log("[DEBUG] failed clauses", failed.map(i=>i+": "+clauses[i].toString().slice(6,200)));\n  return failed.length===0;\n}'+s[j+len('\n  );')+2:]
body=body.replace('from "../../📋️inventory/🟦️.ts"','from "%s/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts"'%os.path.dirname(os.path.dirname(os.path.dirname(src))) if False else 'from "%s"'%os.path.abspath(os.path.join(os.path.dirname(src),'../../📋️inventory/🟦️.ts')))
open(out,'w').write(body)
