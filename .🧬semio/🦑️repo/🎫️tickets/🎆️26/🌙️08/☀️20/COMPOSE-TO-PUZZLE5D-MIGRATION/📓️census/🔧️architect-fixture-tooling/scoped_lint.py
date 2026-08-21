# 🧹 Re-implementation of 📜️script.ts `lintArtifact`/`lintCase`, scoped to the architect/program tree.
import os, re, json
ROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
NON_MUTATION={"💾️binary","📝️text"}
CORE=["🦠️mutation/🔣️component.json","🔺️diff/🔣️component.json","🎯️outcome/🔣️component.json","🦀️component.rs"]
DERIVED=["🦠️mutation/🔧️component.op.semio","🦠️mutation/📡️component.spr.semio","🔺️diff/🩹️component.patch.semio","🔺️diff/📡️component.patch.spr.semio"]
SNAP_CORE="🔣️component.json"; SNAP_DERIVED=["🗣️component.dsl.semio","🎒️component.pack.semio"]; SNAP_REF="🔗️component.ref.json"
errors=[]; warns=[]
def dirs_in(p): return [e for e in os.listdir(p) if os.path.isdir(os.path.join(p,e))] if os.path.isdir(p) else []

src=open(os.path.join(ROOT,"🦀️component.rs")).read()
m=re.search(r'pub enum \w*Mutation\w* \{([\s\S]*?)\n\}', src)
variants=re.findall(r'^\s+([A-Z][A-Za-z0-9]*)\(', m.group(1), re.M)
leaves=[]
for e in sorted(dirs_in(ROOT)):
    if e in NON_MUTATION: continue
    mf=os.path.join(ROOT,e,"🦠️mutation/🦀️component.rs")
    if not os.path.isfile(mf): continue
    s=open(mf).read()
    st=re.search(r'^pub struct ([A-Za-z0-9]+)', s, re.M)
    leaves.append({'dir':e,'struct':st.group(1) if st else None,'path':os.path.join(ROOT,e)})
by={l['struct']:l for l in leaves if l['struct']}
for v in variants:
    if v not in by: errors.append(f"{v}: enum variant has no mutation directory")
covered=0
for leaf in leaves:
    cases=dirs_in(os.path.join(leaf['path'],"🧪️tests"))
    if not cases:
        errors.append(f"{leaf['dir']}: no 🧪️tests cases"); continue
    covered+=1
    for c in cases:
        cd=os.path.join(leaf['path'],"🧪️tests",c); label=f"{leaf['dir']}/{c}"
        rejected=False; of=os.path.join(cd,"🎯️outcome/🔣️component.json")
        if os.path.isfile(of):
            try:
                o=json.load(open(of)); rejected=o.get('status')=='rejected'
                if o.get('status') not in ('applied','rejected'): errors.append(f"{label}: bad status")
                if rejected and not isinstance(o.get('code'),str): errors.append(f"{label}: rejected outcome must carry a code")
            except Exception as ex: errors.append(f"{label}: outcome not valid JSON: {ex}")
        for rel in CORE:
            if rejected and rel.startswith("🔺️diff/"): continue
            if not os.path.isfile(os.path.join(cd,rel)): errors.append(f"{label}: missing {rel}")
        for rel in DERIVED:
            if rejected and rel.startswith("🔺️diff/"): continue
            if not os.path.isfile(os.path.join(cd,rel)): warns.append(f"{label}: missing derived {rel}")
        if rejected and not os.path.isfile(os.path.join(cd,"🔺️diff/🚫️component.absent")):
            errors.append(f"{label}: rejected case must carry 🔺️diff/🚫️component.absent")
        for side in ["⬅️before","➡️after"]:
            sd=os.path.join(cd,"📸️snapshot",side)
            if not os.path.isdir(sd): errors.append(f"{label}: missing 📸️snapshot/{side}"); continue
            if os.path.isfile(os.path.join(sd,SNAP_REF)):
                if os.path.isfile(os.path.join(sd,SNAP_CORE)): errors.append(f"{label}: {side} has both ref and inline")
                continue
            if not os.path.isfile(os.path.join(sd,SNAP_CORE)): errors.append(f"{label}: 📸️snapshot/{side} missing {SNAP_CORE}")
            for n in SNAP_DERIVED:
                if not os.path.isfile(os.path.join(sd,n)): warns.append(f"{label}: missing derived 📸️snapshot/{side}/{n}")
print(f"variants={len(variants)} leaves={len(leaves)} covered={covered} uncovered={len(leaves)-covered}")
print(f"ERRORS={len(errors)}  WARNINGS(derived-encoding, expected)={len(warns)}")
for e in errors[:40]: print("  ❌️", e)
