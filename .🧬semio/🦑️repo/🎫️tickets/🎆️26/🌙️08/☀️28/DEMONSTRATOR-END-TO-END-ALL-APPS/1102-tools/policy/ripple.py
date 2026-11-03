#!/usr/bin/env python3
"""🔁️ Compiler-driven call-site rewrite for the Result-returning cold retire APIs: reads the text diagnostics of `repo cargo check` (the wrapper does not forward JSON), and for every `unused_must_use` on a `Result` appends `?` (production) or `.unwrap()` (test paths) after the exact flagged call expression. Fail-closed: only single-line spans whose underlined text ends with `)`, only files unchanged since the compile. Everything else goes to `<out>.manual`."""
import os,re,sys
ROOT="/Users/ueli/Documents/semio"
def is_test(p):return bool(re.search(r"🧪|/tests?/|_tests?\.rs|tests\.rs",p))
def parse(out):
    L=open(out,encoding="utf8",errors="replace").read().split("\n");found=[];manual=[]
    for i,l in enumerate(L):
        if not (l.startswith("warning: unused") and "must be used" in l and "Result" in l):
            if l.startswith("error") and i+1<len(L) and "-->" in L[i+1]:manual.append(l+" "+L[i+1].strip())
            continue
        m=re.match(r"\s*--> (.*?):(\d+):(\d+)$",L[i+1])
        if not m:manual.append(l);continue
        f,line,col=m.group(1),int(m.group(2)),int(m.group(3))
        for j in range(i+2,min(i+12,len(L))):
            c=re.match(r"\s*\|\s*(\^+)",L[j])
            if c:
                found.append((f if os.path.isabs(f) else os.path.join(ROOT,f),line,col,len(c.group(1))));break
        else:manual.append(f"{f}:{line}: no underline")
    return found,manual
MINE=re.compile(r"retire_cold|push_cold|retire_raster_value_cold|close_raster_owner_cold")
def main(out,compiled_at):
    found,manual=parse(out);byfile={}
    for f,line,col,n in found:byfile.setdefault(f,set()).add((line,col,n))
    done=0
    for f,spans in byfile.items():
        if os.path.getmtime(f)>compiled_at:manual.append(f"{f}: changed since compile, skipped");continue
        lines=open(f,encoding="utf8").read().split("\n")
        for line,col,n in sorted(spans,reverse=True):
            t=lines[line-1];end=col-1+n
            if not MINE.search(t[col-1:end]):continue
            if not t[col-1:end].rstrip().endswith(")"):manual.append(f"{f}:{line}: span does not end in a call: {t[col-1:end][:60]}");continue
            lines[line-1]=t[:end]+(".unwrap()" if is_test(f) else "?")+t[end:];done+=1
        open(f,"w",encoding="utf8").write("\n".join(lines))
    open(out+".manual","w",encoding="utf8").write("\n".join(manual)+"\n")
    print("rewritten",done,"manual",len(manual))
if __name__=="__main__":main(sys.argv[1],float(sys.argv[2]))
