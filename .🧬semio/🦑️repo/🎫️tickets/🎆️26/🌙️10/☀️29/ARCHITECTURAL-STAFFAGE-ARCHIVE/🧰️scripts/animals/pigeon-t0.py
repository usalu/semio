import sys, math
A, F = sys.argv[1], sys.argv[2]
sys.path.insert(0, A)
import kin
a = kin.load(F, "pigeon")
w = a.fk({})
print({k: tuple(round(v,3) for v in a.anchor(w,k)) for k in ("contact-n","eye","beak-tip","nostril")})
for H in (0.09,0.095,0.10,0.105,0.11):
  for tx in (0.0,0.03,0.06):
    o,e = a.solve_limb({}, ["thigh-n","shank-n","tarsus-n","toes-n"], "contact-n", (tx,-H), {"thigh-n":("solve",),"shank-n":("solve",),"tarsus-n":("abs",-80),"toes-n":("abs",-3)})
    print(H, tx, {k: round(v,1) for k,v in o.items()}, round(e,4))
