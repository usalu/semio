import json,sys,subprocess,re,os
t=json.load(open(sys.argv[1]))
names=set()
for l in subprocess.run(["git","ls-files","-z","--","*Cargo.toml"],capture_output=True).stdout.decode().split("\0"):
    if not l or ".🧬semio" in l or not os.path.exists(l): continue
    m=re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"',open(l,encoding="utf8").read(),re.M|re.S)
    if m: names.add(m.group(1))
V="aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen".split()
miss=set()
for v in V:
    for d in t[f"prepare-{v}-react-prototype"]["dependsOn"]:
        if d.endswith(":component") and d[:-10] not in names: miss.add(d)
print("missing component packages:",sorted(miss))
