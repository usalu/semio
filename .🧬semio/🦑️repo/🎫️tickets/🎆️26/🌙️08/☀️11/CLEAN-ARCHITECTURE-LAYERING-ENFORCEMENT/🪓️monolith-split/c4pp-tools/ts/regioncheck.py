import sys,re
# usage: regioncheck.py <file>... : report unbalanced //#region markers (stack discipline)
bad=0
for f in sys.argv[1:]:
    st=[]; probs=[]
    for i,l in enumerate(open(f,encoding="utf8").read().split("\n"),1):
        m=re.match(r'^\s*//\s*#(end)?region\s+(.*?)\s*$',l)
        if not m: continue
        if not m.group(1): st.append((m.group(2),i))
        else:
            if st and st[-1][0]==m.group(2): st.pop()
            else: probs.append(f"{i}: unmatched #endregion {m.group(2)} (open: {st[-1] if st else None})")
    for n,i in st: probs.append(f"{i}: unclosed #region {n}")
    if probs: bad+=1; print(f[-70:],probs)
print("files",len(sys.argv)-1,"unbalanced",bad)
