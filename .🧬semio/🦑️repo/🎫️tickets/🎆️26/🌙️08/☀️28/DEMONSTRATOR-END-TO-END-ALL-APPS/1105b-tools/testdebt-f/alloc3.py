import sys
sys.path.insert(0,'.')
from rep import apply
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs"
def obs(g,call): return ("crate::observe_retirement_allocations(||eff::settled(eff::with_effect(%s,1<<20,&mut retirement_actual,|effect|%s)));"%(g,call),"eff::observed(%s,1<<20,&mut retirement_actual,|effect|eff::settled(%s));"%(g,call))
e=[]
for g,c in [("denied","recipient.close_step(effect)"),("remaining","recipient.close_step(effect)"),("grant","control.close_retirement_recipient(effect)"),("grant","recipient.close_step(effect)")]:
    o,n=obs(g,c);e.append((o,n,1))
e.append(("use std::sync::atomic::{AtomicUsize,Ordering};\n","use std::sync::atomic::{AtomicUsize,Ordering};\nuse crate::eff;\n",1))
apply(P,e)
print("ok")
