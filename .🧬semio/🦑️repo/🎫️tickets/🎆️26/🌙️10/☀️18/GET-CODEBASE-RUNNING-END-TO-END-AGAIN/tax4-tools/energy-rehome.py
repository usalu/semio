# 🔧 Re-homes the seven energy orphan fixture leaves to the domain-nested schema leaf and rewrites flat, long-alias and leaf-test directory references. usage: python3 energy-rehome.py [--apply]
import os,subprocess,sys
apply="--apply" in sys.argv
S="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
F=f"{S}/🧪️tests/🧫️fixtures/🧬️mutations"
MOVES=[
 ("☔️change-humidistat","💧️humidistat/☔️change-humidifying-setpoint-schedule","☔️change-humidistat-humidifying-setpoint-schedule"),
 ("🎈️change-mechanical","🌪️ventilation/🎈️change-fan-delta-pressure","🎈️change-mechanical-ventilation-fan-delta-pressure"),
 ("💠️change-mechanical","🌪️ventilation/💠️change-fan-total-efficiency","💠️change-mechanical-ventilation-fan-total-efficiency"),
 ("🔳️change-ideal-loads","💯️ideal-loads/🔳️change-outdoor-air-per-area","🔳️change-ideal-loads-system-outdoor-air-per-area"),
 ("🧍️change-ideal-loads","💯️ideal-loads/🧍️change-outdoor-air-per-person","🧍️change-ideal-loads-system-outdoor-air-per-person"),
 ("🅱️change-infiltration","🕳️leakage/🅱️change-temperature-term-coefficient","🅱️change-infiltration-temperature-term-coefficient"),
 ("🆑️change-infiltration","🕳️leakage/🆑️change-velocity-squared-term-coefficient","🆑️change-infiltration-velocity-squared-term-coefficient"),
]
files=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",S],capture_output=True,text=True).stdout.split("\n") if f.endswith((".rs",".ts",".py",".feature",".json",".md")) and "/target/" not in f and "node_modules" not in f and "/🧫️fixtures/" not in f]
changed={}
for f in files:
    try: t=open(f,encoding="utf8").read()
    except Exception: continue
    n=t
    for flat,nested,long in MOVES:
        n=n.replace(long,nested)
        n=n.replace(f"🧫️fixtures/🧬️mutations/{flat}/",f"🧫️fixtures/🧬️mutations/{nested}/")
        n=n.replace(f'directory: "{flat}/',f'directory: "{nested}/')
    if n!=t: changed[f]=n
for flat,nested,long in MOVES:
    assert os.path.isdir(f"{F}/{flat}") and not os.path.exists(f"{F}/{nested}"),(flat,nested)
    assert os.path.isdir(f"{S}/🧬️schema/🧬️mutations/{nested}")
print("moves",len(MOVES),"files",len(changed))
for f in changed: print("  ",f.split("✳️any/")[-1])
if apply:
    for flat,nested,long in MOVES:
        os.makedirs(os.path.dirname(f"{F}/{nested}"),exist_ok=True)
        if not os.path.isdir(f"{F}/{nested.split('/')[0]}"): pass
        os.rename(f"{F}/{flat}",f"{F}/{nested}")
    for f,t in changed.items(): open(f,"w",encoding="utf8").write(t)
    print("applied")
