#!/usr/bin/env python3
"""Replaces the per-case boilerplate mutation test modules of the bim model with the fixture-discovery runner.
usage: cases-b.py [--apply]   (default dry run). Deletes only files whose normalized text equals one of the two law templates; rewrites mount lines in leaf files; converts hand-written CASE consts to lazily loaded cases."""
import subprocess,re,os,sys,collections,json
APPLY="--apply" in sys.argv
S="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
M=S+"/🧬️schema/🧬️mutations/"
files=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","--",M],capture_output=True,text=True).stdout.split("\n")
files=[f for f in files if f]
T5=lambda d: f'''//! ✅️ `x`
use crate::standards::v1::subsets::any::schema::mutations::kit::{{self, Case}};

const CASE: Case = Case {{
    dir: "{d}",
__INC__}};
'''
LAWS5=["declared_outcome_holds","applies_to_the_committed_after_snapshot","produces_the_committed_diff","inverse_restores_the_before_snapshot","committed_json_is_canonical"]
def classify(t):
    m=re.search(r'dir: "([^"]+)"',t)
    if not m: return None
    d=m.group(1)
    # strip header doc, include block
    body=re.sub(r"^//!.*\n","",t,flags=re.M)
    body=re.sub(r"const CASE: Case = Case \{.*?\n\};\n","const CASE: Case = Case {};\n",body,flags=re.S)
    body=re.sub(r"\n+","\n",body).strip()
    laws=re.findall(r"async fn (\w+)\(",body)
    uses=re.findall(r"^use .*;$",body,flags=re.M)
    only_kit = uses==["use crate::standards::v1::subsets::any::schema::mutations::kit::{self, Case};"]
    if only_kit and laws==LAWS5 and "assert_mutation_inverse_sum_law" not in body: return ("boiler",d)
    if only_kit and laws==LAWS5+["inverse_diffs_sum_to_the_negative_diff"]: return ("boiler",d)
    if only_kit and laws==["declared_fixture_and_inverse_laws"]: return ("boiler",d)
    return ("hand",d)
def slug(path):
    head,_,last=path.rpartition("/")
    return head+"/"+last.split("\ufe0f",1)[-1]
delete=[];hand=[];other=[]
for f in files:
    if not f.endswith("🦀️.rs") or "/🧪️tests/" not in f or f.endswith("🧰️kit/🦀️.rs"): continue
    t=open(f).read()
    c=classify(t)
    if c is None: other.append(f)
    elif c[0]=="boiler": delete.append((f,c[1]))
    else: hand.append((f,c[1]))
print("delete",len(delete),"hand-with-CASE",len(hand),"other",len(other))
for f,d in hand: print("  hand",f.split("🧬️mutations/")[1])
dirs_deleted=collections.Counter()
mount=re.compile(r'#\[cfg\(test\)\]\n#\[path = "(🧪️tests/[^"]+)/🦀️\.rs"\]\nmod (\w+);\n')
by_leaf=collections.defaultdict(set)
for f,d in delete:
    rel=f[len(M):]
    leaf=rel.split("/")[0]
    by_leaf[leaf].add(slug("🧪️tests/"+rel.split("/🧪️tests/")[1][:-len("/🦀️.rs")]))
removed_mounts=0
for leaf,paths in by_leaf.items():
    lf=M+leaf+"/🦀️.rs"
    t=open(lf).read()
    def sub(m):
        global removed_mounts
        if slug(m.group(1)) in paths:
            removed_mounts+=1; return ""
        return m.group(0)
    nt=mount.sub(sub,t)
    if APPLY and nt!=t: open(lf,"w").write(nt)
print("mounts removed",removed_mounts,"expected",len(delete))
if APPLY:
    for f,d in delete:
        os.remove(f)
        p=os.path.dirname(f)
        while True:
            try: os.rmdir(p)
            except OSError: break
            p=os.path.dirname(p)
    for f,d in hand:
        t=open(f).read()
        nt=re.sub(r"const CASE: Case = Case \{.*?\n\};\n",f'static CASE: std::sync::LazyLock<Case> = std::sync::LazyLock::new(|| Case::load("{d}"));\n',t,flags=re.S)
        assert nt!=t
        open(f,"w").write(nt)
