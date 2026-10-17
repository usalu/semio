import subprocess,shutil,sys,os
O="/Users/ueli/.cache/r8b/forms-orig2"; U="/Users/ueli/.cache/r8b/forms-union"; B="/Users/ueli/.cache/r8b/forms-bis"
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
files=[l.strip() for l in open("/Users/ueli/.cache/r8b/differ.txt",encoding='utf8') if l.strip() and '/🧪️tests/' not in l and not l.strip().endswith("📋️forms/🦀️.rs")]
needle=sys.argv[1]
R="✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs"
def run(restore):
    if os.path.exists(B): shutil.rmtree(B)
    shutil.copytree(U,B)
    for f in restore: shutil.copyfile(os.path.join(O,f),os.path.join(B,f))
    subprocess.run(["bash","-c",f"ulimit -n 65536; nice -n 10 bun '{F}/r8b-tools/oracle.ts' '{B}' '{R}' semio_s_artifact_forms_forms /Users/ueli/.cache/r8b/bis.diag >/dev/null"],check=True)
    t=open("/Users/ueli/.cache/r8b/bis.diag",encoding='utf8').read()
    return needle in t
# find minimal set to restore such that needle disappears
cands=files[:]
assert run([]) , "needle must exist initially"
while len(cands)>1:
    half=cands[:len(cands)//2]; rest=cands[len(cands)//2:]
    # try restoring half; if needle disappears, culprit within half
    if not run(half): cands=half
    elif not run(rest): cands=rest
    else:
        print("needs both halves",len(half),len(rest)); break
    print("remaining",len(cands),flush=True)
print("\n".join(cands))
