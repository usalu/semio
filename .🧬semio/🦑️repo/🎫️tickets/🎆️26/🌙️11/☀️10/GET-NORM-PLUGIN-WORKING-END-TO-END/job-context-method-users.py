"""🔎️ Counts the files outside a module that call each public StepContext method it defines."""
import re,subprocess,sys
mod=sys.argv[1]
t=open(mod,encoding="utf8").read()
names=sorted(set(re.findall(r'pub fn ([a-z_0-9]+)',t)))
pat="|".join(names)
out=subprocess.run(["grep","-rIEoh","--include=*.rs",r"\b("+pat+r")\b","🧰️framework/🔨️modules","🧰️framework/🛍️products"],capture_output=True,text=True).stdout.split()
from collections import Counter
c=Counter(out)
print({n:c.get(n,0) for n in names})
