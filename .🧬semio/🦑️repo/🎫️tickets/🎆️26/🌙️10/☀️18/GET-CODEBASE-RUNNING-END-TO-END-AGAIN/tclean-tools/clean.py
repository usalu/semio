import os,sys,re,time,shutil,collections,json
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
M=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN="🗑️generated"
NOW=time.time()
RECENT=30*60
ORIG_DIR=re.compile(r'^(orig|orig-.*|orig_.*|.*-orig|backup|pre-.*|before|.*-before|snapshot|snap|fmt-tmp|tsc-orig)$')
COPY_FILE=re.compile(r'(^orig[-_.]|[-_.]orig(inal)?[-_.]|[-_.]orig(inal)?$|-original\.|-before\.|-base\.rs$|-head\.rs$|\.orig$|\.pre(-.*)?$|\.prev$|\.bak$|\.src$|-before\.|snapshot\.)')
OUT_EXT={".log",".err",".jsonl",".out",".tgz",".tar",".gz",".diff",".patch",".tmp",".list",".sha256"}
KEEP_NAMES={"exports-needed.json","exports-final.json","dependencies-needed.json","files.tsv"}
JSON_OUT=re.compile(r'^(dump\d*|.*-report\d*|verify\d*-report|touched.*|changed-.*|.*result.*|.*output.*)\.json$')
SCRIPT_EXT={".py",".ts",".mjs",".sh",".cjs",".js",".go",".tsx",".mts"}
READ=re.compile(r'readFileSync|read_text|open\(|\bcat\b|json\.load|Bun\.file|readTextFile|readlines|\bwhile read|< *["\$]|\.read\(')
WRITE=re.compile(r'writeFileSync|write_text|Bun\.write|\btee\b|> *["\$\w]|["\']w["\']|appendFileSync|\.write\(')
def newest(p):
    n=os.lstat(p).st_mtime
    if os.path.isdir(p) and not os.path.islink(p):
        for dp,dn,fn in os.walk(p):
            for f in fn+dn:
                try:n=max(n,os.lstat(os.path.join(dp,f)).st_mtime)
                except OSError:pass
    return n
def alloc(st): return st.st_blocks*512
rows=[]
scripts_cache={}
def tool_scripts(top):
    if top in scripts_cache: return scripts_cache[top]
    txt=[]
    for dp,dn,fn in os.walk(top):
        if GEN in dp.split(os.sep): dn[:]=[];continue
        for f in fn:
            if os.path.splitext(f)[1] in SCRIPT_EXT or f.endswith(".md"):
                try:
                    if os.path.getsize(os.path.join(dp,f))<2_000_000: txt.append(open(os.path.join(dp,f),encoding="utf8",errors="replace").read().splitlines())
                except OSError:pass
    scripts_cache[top]=txt;return txt
def is_input(top,base):
    for lines in tool_scripts(top):
        for l in lines:
            if base in l and READ.search(l) and not WRITE.search(l): return True
    return False
def classify(root,rel,st):
    parts=rel.split(os.sep);base=parts[-1];ext=os.path.splitext(base)[1].lower()
    top=os.path.join(root,parts[0])
    if root==T and (parts[0] in ("repoapi-tools","cc-tools","stale-publication-intents") or re.match(r'^lw[^/]*-tools$',parts[0])): return None,"protected"

    if ext==".md": return None,"keep-script"
    if base in KEEP_NAMES or "specs" in parts[:-1]: return None,"keep-input"
    if NOW-st.st_mtime<RECENT: return None,"recent"
    if any(ORIG_DIR.match(x) for x in parts[:-1]): return "A-copy-dir",""
    if COPY_FILE.search(base) and ext not in (".md",".py",".sh",".mjs",".ts",".cjs") : return "A-copy-file",""
    if base.endswith(".bak") or base.endswith(".prev") or ext==".orig": return "A-copy-file",""
    if ext in SCRIPT_EXT or ext==".md" or ext==".toml": return None,"keep-script"
    if ext in OUT_EXT: return "B-output-ext",""
    if ext==".txt" or ext==".tsv" and False: 
        if is_input(top,base): return None,"keep-read-input"
        return "B-txt",""
    if ext==".json" and JSON_OUT.match(base): return "B-json-output",""
    if ext==".rs": return None,"keep-rs-unclassified"
    return None,"keep-other"
def walk(root):
    for dp,dn,fn in os.walk(root):
        rel=os.path.relpath(dp,root)
        if rel==GEN or rel.startswith(GEN+os.sep): dn[:]=[];continue
        dn[:]=[d for d in dn if not (rel=="." and root==T and (d in ("repoapi-tools","cc-tools") or re.match(r'^lw[^/]*-tools$',d)))]
        for f in fn:
            p=os.path.join(dp,f)
            try:st=os.lstat(p)
            except OSError:continue
            r=os.path.relpath(p,root)
            cat,why=classify(root,r,st)
            yield root,r,st,cat,why
def generated(root):
    g=os.path.join(root,GEN)
    for e in sorted(os.listdir(g)):
        p=os.path.join(g,e)
        st=os.lstat(p)
        n=newest(p)
        if root==T and re.match(r'^(cc-|repoapi|lw)',e): cat,why=None,"protected"
        elif root==T and e.startswith("tclean"): cat,why=None,"self"
        elif NOW-n<RECENT: cat,why=None,"recent"
        else: cat,why="C-generated",""
        size=st.st_size;al=alloc(st)
        if os.path.isdir(p) and not os.path.islink(p):
            size=al=0
            for dp,dn,fn in os.walk(p):
                for f in fn:
                    try:s=os.lstat(os.path.join(dp,f));size+=s.st_size;al+=alloc(s)
                    except OSError:pass
        yield root,os.path.join(GEN,e),size,al,cat,why
if __name__=="__main__":
    apply="--apply" in sys.argv
    stat=collections.defaultdict(lambda:[0,0,0])
    dels=[]
    for root in (T,M):
        for r,rel,st,cat,why in walk(root):
            k=cat or ("KEEP:"+why)
            stat[k][0]+=1;stat[k][1]+=st.st_size;stat[k][2]+=alloc(st)
            if cat: dels.append((os.path.join(root,rel),st.st_size,alloc(st),cat))
        for r,rel,size,al,cat,why in generated(root):
            k=cat or ("KEEP-GEN:"+why)
            stat[k][0]+=1;stat[k][1]+=size;stat[k][2]+=al
            if cat: dels.append((os.path.join(root,rel),size,al,cat))
    for k,v in sorted(stat.items()): print(f"{k:28} n={v[0]:7} bytes={v[1]:14} alloc={v[2]:14}")
    with open(sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else os.devnull,"w") as f:
        for p,s,a,c in dels: f.write(f"{c}\t{s}\t{p}\n")
    if apply:
        dirs=set();n=0;b=0
        for p,s,a,c in dels:
            try:
                if os.path.isdir(p) and not os.path.islink(p): shutil.rmtree(p)
                else: os.remove(p)
                n+=1;b+=s;dirs.add(os.path.dirname(p))
            except OSError as e: print("ERR",p,e)
        removed=0
        for d in sorted(dirs,key=len,reverse=True):
            while d.startswith(T) or d.startswith(M):
                if d in (T,M) or os.path.basename(d)==GEN: break
                try:os.rmdir(d);removed+=1
                except OSError:break
                d=os.path.dirname(d)
        print("deleted entries",n,"bytes",b,"empty dirs removed",removed)
