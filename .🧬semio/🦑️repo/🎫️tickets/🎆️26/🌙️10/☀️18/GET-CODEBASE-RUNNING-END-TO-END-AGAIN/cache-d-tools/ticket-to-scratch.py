"""ticket-to-scratch.py <file> <route>: replaces the developer-ticket scratch (SEMIO_TICKET_DIR/🗑️generated) of a caching test by the per-run test scratch root (testScratchRootV1) and adds the import."""
import os,re,sys
FW="/Users/ueli/Documents/semio/🧰️framework/"
f,route=sys.argv[1],sys.argv[2]
s=open(f,encoding="utf8").read()
n0=s.count("SEMIO_TICKET_DIR")
s=s.replace("process.env.SEMIO_TICKET_DIR",'testScratchRootV1("'+route+'")')
s=s.replace('"🗑️generated/','"').replace('join(resolve(ticket),"🗑️generated")','resolve(ticket)')
depth=len(os.path.relpath(os.path.dirname(os.path.abspath(f)),FW).split("/"))
imp='import{testScratchRootV1}from"'+"../"*depth+'🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";\n'
if "testScratchRootV1}from" not in s and "testScratchRootV1,"not in s:s=imp+s
open(f,"w",encoding="utf8").write(s)
print(f.split("/")[-3:],n0,"->",s.count("SEMIO_TICKET_DIR"),"generated left:",s.count("🗑️generated"))
