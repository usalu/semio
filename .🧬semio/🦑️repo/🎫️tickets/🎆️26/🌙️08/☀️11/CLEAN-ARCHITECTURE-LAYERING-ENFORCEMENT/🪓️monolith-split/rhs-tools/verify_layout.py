import sys,os,re,collections,unicodedata,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from verify_installed import family_files,FAM,REPO
SC="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
used={}
for l in open(SC+"/foldemoji.txt",encoding='utf-8'):
    k,v=l.rstrip('\n').split('\t');used[k]=int(v)
def first_grapheme_emoji(s):
    i=0
    while i<len(s) and ord(s[i])>127:i+=1
    return s[:i]
extra={'sqlite':[],'postgres':[],'neo4j':[],'model':[],'tickets':[],'gateway':[]}
def run(name):
    root,kids=family_files(name)
    files=[root]+kids
    # path lengths
    longest=max(len(f.encode('utf-8')) for f in files)
    print(name,'files',len(files),'longest path bytes',longest)
    # folder names
    seen=collections.Counter();bad=[]
    for k in kids:
        folder=os.path.basename(os.path.dirname(k))
        em=first_grapheme_emoji(folder)
        if not em.endswith('️'):bad.append(('no FE0F',folder))
        if used.get(em,0)==0:bad.append(('emoji not used elsewhere as folder prefix',folder))
        seen[em]+=1
        rest=folder[len(em):]
        if not re.fullmatch(r'[a-z0-9]+(-[a-z0-9]+)*',rest):bad.append(('slug not kebab',folder))
    dup=[e for e,c in seen.items() if c>1]
    # sibling dirs in parent dir
    pdir=os.path.dirname(root)
    sib=collections.Counter()
    for e in os.listdir(pdir):
        if os.path.isdir(os.path.join(pdir,e)) and not e.startswith('.'):sib[first_grapheme_emoji(e)]+=1
    sibdup={e:c for e,c in sib.items() if c>1}
    print('  folder problems',bad,'duplicate emoji among new children',dup,'duplicate emoji among ALL siblings (incl. old)',sibdup)
    # docstring first chars
    docs=collections.Counter()
    for f in files:
        first=open(f,encoding='utf-8').readline()
        m=re.match(r'^//!\s*(\S+)',first)
        if not m:
            print('  no //! first line:',os.path.relpath(f,pdir),first[:60]);continue
        docs[m.group(1)[0:2] if False else first_grapheme_emoji(m.group(1))]+=1
    print('  docstring emoji duplicates:',{k:v for k,v in docs.items() if v>1})
    # path attrs resolve
    miss=0;n=0
    for f in files:
        t=open(f,encoding='utf-8').read()
        for m in re.finditer(r'#\[path = "([^"]+)"\]',t):
            n+=1
            p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            if not os.path.exists(p):miss+=1;print('  MISSING path',m.group(1))
        for m in re.finditer(r'include_str!\("([^"]+)"\)',t):
            n+=1
            p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            if not os.path.exists(p):miss+=1;print('  MISSING include',m.group(1))
    print('  path/include refs',n,'missing',miss)
    # rustfmt parse
    bad=0
    for f in files:
        r=subprocess.run(['rustfmt','+nightly-2026-07-20','--edition','2021','--config','skip_children=true','--emit','stdout',f],capture_output=True,text=True)
        if r.returncode!=0:bad+=1;print('  rustfmt parse failed',os.path.relpath(f,pdir),r.stderr[:200])
    print('  rustfmt parse failures',bad,'of',len(files))
    # [DEBUG]
    dbg=sum(open(f,encoding='utf-8').read().count('[DEBUG]') for f in files)
    print('  [DEBUG] occurrences',dbg)
if __name__=='__main__':
    for n in sys.argv[1:] or FAM:run(n)
