"""🛠 pai generator: stages the split of the PluginApp impl from the snapshot `pai.orig.rs` into /Users/ueli/.cache/pai/stage (root + one child per role). usage: gen.py"""
import os, re, shutil, sys, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from methods import parse, split_sig, params
from spec import CHILDREN, ROOT, IMPL_HEADER
STAGE = "/Users/ueli/.cache/pai/stage"
text = open(HERE + "/pai.orig.rs", encoding="utf-8").read()
ls, a, b, ms = parse(text)
by = {m["name"]: m for m in ms}
moved = [n for c in CHILDREN for n in c["methods"]]
assert len(moved) == len(set(moved)) and all(n in by for n in moved), [n for n in moved if n not in by]
MUT = re.compile(r"(?<=[(,])(\s*)mut (\w+)\s*:")


def forwarder(m):
    sig, body, pl = split_sig(ls, m)
    sig = re.sub(r"\s*\n\s*", " ", sig.strip())
    sig = MUT.sub(lambda x: x.group(1) + x.group(2) + ":", sig)
    names = params(pl)
    call = "self.%s(%s)%s" % (m["name"], ", ".join(names), ".await" if m["is_async"] else "")
    return "    %s { %s }" % (sig, call)


def inherent(m):
    block = ls[m["s"]: m["e"] + 1]
    h = m["h"] - m["s"]
    assert block[h].startswith("    async fn ") or block[h].startswith("    fn "), block[h]
    block[h] = "    pub(super) " + block[h][4:]
    return block


out = list(ls)
repl = {}
for n in moved:
    m = by[n]
    repl[(m["s"], m["e"])] = [forwarder(m)]
res, i = [], 0
spans = sorted(repl)
k = 0
while i < len(ls):
    if k < len(spans) and i == spans[k][0]:
        res += repl[spans[k]]
        i = spans[k][1] + 1
        k += 1
    else:
        res.append(ls[i]); i += 1
mounts = []
for c in CHILDREN:
    mounts += ['#[path = "%s/🦀️.rs"]' % c["folder"], "mod %s;" % c["mod"], ""]
anchor = next(i for i, l in enumerate(res) if l.startswith("use semio_framework_plugin_instance_jobs::jobs_instance_root::OriginalRenderTreeIdentity;"))
res[anchor + 1:anchor + 1] = [""] + mounts[:-1]
root_text = "\n".join(res)
shutil.rmtree(STAGE, ignore_errors=True)
base = os.path.dirname(ROOT)
os.makedirs(os.path.join(STAGE, base), exist_ok=True)
open(os.path.join(STAGE, ROOT), "w", encoding="utf-8").write(root_text)
for c in CHILDREN:
    d = os.path.join(STAGE, base, c["folder"])
    os.makedirs(d, exist_ok=True)
    body = []
    for n in c["methods"]:
        if body:
            body.append("")
        body += inherent(by[n])
    open(os.path.join(d, "🦀️.rs"), "w", encoding="utf-8").write("\n".join([c["doc"], "", "use super::*;", "", IMPL_HEADER] + body + ["}", ""]))
json.dump(dict(root=ROOT, folders=[c["folder"] for c in CHILDREN], mods=[c["mod"] for c in CHILDREN]), open(HERE + "/staged-pai.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("staged", len(moved), "methods in", len(CHILDREN), "children; root lines", len(root_text.split("\n")))
