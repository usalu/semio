import sys
# usage: fixheader.py <snapshot> <file>: re-insert the leading region block that the splitter dropped (lines of the snapshot up to and including the first '#endregion' line of the leading region)
snap,live=sys.argv[1:3]
s=open(snap,encoding="utf8").read().split("\n")
assert s[0].strip().startswith("// #region"), s[0]
end=next(i for i,l in enumerate(s) if l.strip().startswith("// #endregion"))
hdr=s[:end+1]
t=open(live,encoding="utf8").read()
body_lines=t.split("\n")
n=len(hdr)-2
assert body_lines[:n]==hdr[1:1+n], ("live head differs", body_lines[:n], hdr[1:1+n])
new=hdr+[""]+body_lines[n:]
open(live,"w",encoding="utf8").write("\n".join(new))
print("fixed",live[-40:])
