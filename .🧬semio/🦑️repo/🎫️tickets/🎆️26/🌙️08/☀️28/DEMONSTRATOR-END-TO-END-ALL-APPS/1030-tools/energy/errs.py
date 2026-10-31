import re,sys
t=open(sys.argv[1],errors='replace').read().split('\n')
flt=sys.argv[2] if len(sys.argv)>2 else ''
seen=set()
i=0
while i<len(t):
    l=t[i]
    if re.match(r'^error(\[E\d+\])?:',l) and 'could not compile' not in l and 'aborting' not in l:
        msg=l
        loc=''
        for j in range(i+1,min(i+6,len(t))):
            m=re.match(r'\s+--> (.*)',t[j])
            if m: loc=m.group(1); break
        loc=loc.replace('/Users/ueli/Documents/semio/','')
        loc=re.sub(r'✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/','M/',loc)
        loc=loc.replace('/📦️packages/🦀️rust/..','')
        k=(msg,loc)
        if k not in seen and flt in (msg+loc):
            seen.add(k); print(loc,'|',msg[:200])
    i+=1
