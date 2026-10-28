"""Sixth increment of the nxret-g change: the delegate resolves the bound policy with the machine's idle-wait override (`SEMIO_NX_TOOLING_IDLE_WAIT_MS`). Usage: python3 stage6.py check|apply <root>."""
import hashlib,os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
BACKUP=os.path.join(HERE,"backup6")
COMMAND="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
PATCHES=[
 ('{nxToolingStoreBoundPolicyV1}=await import(','{resolveNxToolingStoreBoundPolicyV1}=await import('),
 ('policy=nxToolingStoreBoundPolicyV1(),mount=','policy=resolveNxToolingStoreBoundPolicyV1(process.env),mount='),
]
def patch_text(text):
    for old,new in PATCHES:
        if new!=old and text.count(new)==1:continue
        if text.count(old)!=1:raise SystemExit(f"anchor missing or ambiguous ({text.count(old)}x): {old[:90]}")
        text=text.replace(old,new)
    return text
mode,root=sys.argv[1],os.path.abspath(sys.argv[2])
with open(os.path.join(root,COMMAND),encoding="utf8") as handle:text=patch_text(handle.read())
if mode=="check":print("ok");sys.exit(0)
os.makedirs(BACKUP,exist_ok=True)
name=hashlib.sha1(COMMAND.encode()).hexdigest()[:16]
if not os.path.exists(os.path.join(BACKUP,name+".bak")):shutil.copy2(os.path.join(root,COMMAND),os.path.join(BACKUP,name+".bak"))
temporary=os.path.join(root,COMMAND)+".nxret-apply"
with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
os.replace(temporary,os.path.join(root,COMMAND));print("applied",COMMAND[-50:])
