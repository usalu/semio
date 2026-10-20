"""🔥 Item-level churn of one file: every commit's hunks attributed to the enclosing top-level item of the file's *current* text (hunk new-side line ranges mapped through the current item spans; hunks in older revisions whose text no longer exists fall back to the hunk's function-context header). Output: commits per item. `python3 itemchurn.py <file>`"""
from hubenv import *
from rs import *
import re
def item_churn(path):
    rel=os.path.relpath(path,ROOT)
    t,sk,items=parse(path)
    # name per item with line spans (current text)
    spans=[]
    for it in items:
        nm=it.get('name') or (it.get('use','')[:50] if it['kw']=='use' else it['kw'])
        a=t.count('\n',0,it['start'])+1;b=t.count('\n',0,it['end'])+1
        spans.append((a,b,it['kw']+' '+str(nm)))
    out=subprocess.run(['git','log','-p','-U0','--format=@@@%h','--',rel],cwd=ROOT,capture_output=True,text=True).stdout
    per=collections.defaultdict(set);cur=None;ctx={}
    for l in out.splitlines():
        if l.startswith('@@@'): cur=l[3:];continue
        m=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@ ?(.*)',l)
        if not m: continue
        ns=int(m.group(3));nl=int(m.group(4)) if m.group(4) is not None else 1
        hdr=m.group(5)[:70]
        # only the newest revision's line numbers map to the current text; mark by first commit in log (newest)
        per[('hdr',hdr)].add(cur)
    return per,spans
if __name__=='__main__':
    per,spans=item_churn(sys.argv[1])
    rows=sorted(per.items(),key=lambda x:-len(x[1]))
    for (k,h),s in rows[:30]: print(len(s),repr(h))
