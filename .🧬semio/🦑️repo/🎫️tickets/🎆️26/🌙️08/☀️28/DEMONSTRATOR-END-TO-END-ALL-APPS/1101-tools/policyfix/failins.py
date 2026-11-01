import re,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rows=sorted({l.strip().rsplit(":",1)[0] for l in open(C+"/🗑️generated/1101/aud-policy-unwrap-rows.txt")})
pat=re.compile(r'((?:self\.)(?:owners\.)?failure)=Some\(("[^"]*")\.into\(\)\);return Err\(\1\.as_ref\(\)\.unwrap\(\)\.clone\(\)\);')
n=0
for f in rows:
    t=open(f).read()
    t2,k=pat.subn(lambda m:f'return Err({m.group(1)}.insert({m.group(2)}.into()).clone());',t)
    if k:
        n+=k;print(k,f[-60:])
        if '--apply' in sys.argv:open(f,'w').write(t2)
print(n)
