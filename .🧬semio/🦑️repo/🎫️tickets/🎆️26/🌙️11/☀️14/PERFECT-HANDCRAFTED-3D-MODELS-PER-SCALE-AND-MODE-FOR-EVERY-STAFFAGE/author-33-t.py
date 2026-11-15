from author_33_lib import *
slug="jogger"
print(coarsen(slug,50,100,0.995,drop=("sleeve-r","shorts-stripe-r","headband","shoulder-l","shoulder-r")))
print(thermal(slug,100,"torso-clothing=shirt-hem+shoulder-l+shoulder-r")[:100])
for n in (50,100):
    person_structure(slug,n)
    print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", str(n), "--write")[-200:])
for n in (50,100,200,500): print(n, check(slug,n))
