import json,glob
base=[l.strip() for l in open('tc2-tools/manifests.txt') if l.strip()]
extra=[json.load(open(f))["dir"]+"/📦️packages/🦀️rust/Cargo.toml" for f in sorted(glob.glob('tc2-tools/specs/*.json'))]
open('tc2-tools/manifests-after.txt','w').write("\n".join(base+extra)+"\n")
print(len(base),len(extra))
