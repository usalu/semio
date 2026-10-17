import re,sys
t=open(sys.argv[1]).read()
if '"oracle"' not in t: print(t[-900:]); raise SystemExit
for name,a,b in [("oracle",'"oracle"','"layout"'),("layout",'"layout"',None)]:
    seg=t[t.index(a):(t.index(b) if b else len(t))]
    for m in re.finditer(r'"(native|--test)": \{(.*?)\n    \}',seg,re.S):
        body=m.group(2)
        cnt=re.findall(r'"only(?:Candidate|Combined)": (\d+)',body)
        print(name,m.group(1),cnt)
        for l in re.findall(r'"((?:E\d+|\\t)[^"]*)"',body)[:14]: print("   ",re.sub(r'✏️s/🔌️plugins/🌍️gis/🗿️artifacts/[^/]*/🏅️standards/🔖️1/🪆️subsets/✳️any/','',l)[:210].replace("\\t"," "))
