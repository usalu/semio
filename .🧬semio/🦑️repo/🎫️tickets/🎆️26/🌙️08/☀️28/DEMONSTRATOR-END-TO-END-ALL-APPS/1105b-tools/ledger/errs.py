import re,sys
t=open(sys.argv[1],encoding='utf8',errors='replace').read().split("\n\n")
flt=sys.argv[2] if len(sys.argv)>2 else ""
n=0
for b in t:
    if b.startswith("error") and flt in b:
        lines=b.split("\n")
        loc=[l for l in lines if "-->" in l]
        loc=re.sub(r'.*rust/\.\./','',loc[0].strip()[4:]) if loc else ""
        src=[l for l in lines if re.match(r'\s*\d+ \|',l)]
        print(lines[0][:140],"|",loc[-70:])
        for s in src[:1]: print("   ",s[:230])
        n+=1
print(n)
