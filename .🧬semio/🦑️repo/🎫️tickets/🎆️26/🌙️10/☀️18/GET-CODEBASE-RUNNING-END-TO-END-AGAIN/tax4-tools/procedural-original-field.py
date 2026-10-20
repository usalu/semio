# 🔧 Moves the six emoji-named original-field fixture pairs into slugged case directories as 🧫️.json / 🧬️.json and re-points the TS imports and Rust include_str! literals. usage: python3 procedural-original-field.py [--apply]
import os,sys
P="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🚀️original-field/🧪️tests"
CASES=[("🔄️","cycle"),("⭕️","analytic"),("📏️","edge"),("🏳️","surface"),("↔️","translation"),("🗝️","lookup")]
ts=open(f"{P}/🟦️.ts",encoding="utf8").read(); rs=open(f"{P}/🦀️.rs",encoding="utf8").read()
moves=[]
for e,slug in CASES:
    d=f"{e}{slug}"
    assert os.path.isfile(f"{P}/{e}.json") and os.path.isfile(f"{P}/{e}🧬️.json") and not os.path.exists(f"{P}/{d}"),(e,)
    moves+=[(f"{e}.json",f"{d}/🧫️.json"),(f"{e}🧬️.json",f"{d}/🧬️.json")]
    ts=ts.replace(f'"./{e}🧬️.json"',f'"./{d}/🧬️.json"').replace(f'"./{e}.json"',f'"./{d}/🧫️.json"')
    rs=rs.replace(f'include_str!("{e}.json")',f'include_str!("{d}/🧫️.json")').replace(f'include_str!("{e}🧬️.json")',f'include_str!("{d}/🧬️.json")')
print(len(moves),"moves")
if "--apply" in sys.argv:
    for a,b in moves:
        os.makedirs(os.path.dirname(f"{P}/{b}"),exist_ok=True); os.rename(f"{P}/{a}",f"{P}/{b}")
    open(f"{P}/🟦️.ts","w",encoding="utf8").write(ts); open(f"{P}/🦀️.rs","w",encoding="utf8").write(rs)
    print("applied")
