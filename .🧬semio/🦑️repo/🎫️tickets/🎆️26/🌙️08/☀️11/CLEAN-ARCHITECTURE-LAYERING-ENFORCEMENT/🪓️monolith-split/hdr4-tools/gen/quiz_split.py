import os, re, sys
R = "/Users/ueli/Documents/semio"
Q = R + "/🧰️framework/🛍️products/❓️quiz"
M = Q + "/🔨️modules"
T = Q + "/🧪️tests"
SUP = T + "/🧷️companion-support"
SUPID = "semio_framework_quiz_tests_support"
write = "--write" in sys.argv
rd = lambda p: open(p, encoding="utf8").read()
def wr(p, t):
    if write:
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf8").write(t)

SHEET = M + "/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs"
LIFE = M + "/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs"
files = {n: M + "/%s/🧪️tests/🔬️unit/🦀️.rs" % n for n in ["⛰️challenge", "✅️validation", "🃏️sheet", "🏅️badges", "👁️views", "📏️scoring", "🧾️lifecycle"]}
before = {n: rd(p) for n, p in files.items()}
ntests = {n: len(re.findall(r"(?m)^\s*#\[test\]", t)) + len(re.findall(r"(?m)^\s*#\[[a-z_:]*async_test\]", t)) for n, t in before.items()}

st = before["🃏️sheet"]
a = st.index("pub(crate) fn text(")
b = st.index("fn ids(")
sheet_block = st[a:b].rstrip() + "\n"
sheet_rest = st[:a] + st[b:]

lt = before["🧾️lifecycle"]
a1 = lt.index("pub(crate) const ALICE")
b1 = lt.index("#[test]", a1)
life_block1 = lt[a1:b1].rstrip() + "\n"
a2 = lt.index("pub(crate) fn replay(")
b2 = lt.index("#[test]", a2)
life_block2 = lt[a2:b2].rstrip() + "\n"
life_rest = lt[:a1] + lt[b1:a2] + lt[b2:]

pub = lambda s: s.replace("pub(crate) ", "pub ")
sheet_names = ["text", "quantity", "classification", "sorting", "matching", "quiz"]
life_names = ["ALICE", "BOB", "RUN", "REVISION", "REVISED", "catalog", "quizzes", "id", "fold", "step", "registered", "perfect", "command_start_at", "command_start", "play_perfect_at", "play_perfect", "replay"]

sup_sheet = ("//! 🃏️ The shared quiz builders every companion law builds its sheets and runs from.\n\n"
  "use quiz_schema::{Axis, Category, ClassificationItem, ClassificationTask, Dimension, MatchingItem, MatchingTask, Quantity, Quiz, Scale, SortingItem, SortingTask, Task, Text};\nuse std::collections::BTreeMap;\n\n" + pub(sheet_block))
sup_life = ("//! 🧾️ The shared learner-lifecycle fixtures: identities, catalog, perfect answers, replay of the shared vectors.\n\n"
  "use crate::sheet::{quiz, text};\nuse quiz::lifecycle::*;\nuse quiz::randomness::run_seed;\nuse quiz::sheet::sheet_of;\n"
  "use quiz_schema::{Answer, Badge, BadgeRule, Catalog, Challenge, ClassificationAnswer, Command, Event, Id, Identity, Introduction, Limits, MatchingAnswer, Quiz, SheetTask, Slug, SortingAnswer, Task, TaskKind, Timestamp, DEFAULT_LIMITS};\n"
  "use std::collections::{BTreeMap, BTreeSet};\n\n" + pub(life_block1) + "\n" + pub(life_block2))
sup_root = ("//! 🧷️ Shared fixtures of the quiz companion laws: the quiz builders and the learner-lifecycle kit.\n\n"
  "#[path = \"🃏️sheet/🦀️.rs\"]\npub mod sheet;\n\n#[path = \"🧾️lifecycle/🦀️.rs\"]\npub mod lifecycle;\n")

def used(name, text):
    if name.isupper() or name in ("RUN",): return re.search(r"\b%s\b" % name, text) is not None
    return re.search(r"(?<![\w.])%s\s*[(:,)]|(?<![\w.:])%s\b\s*\)" % (name, name), text) is not None or re.search(r"\b%s\(" % name, text) is not None
def localdefs(text):
    return set(re.findall(r"(?m)^\s*(?:pub(?:\([^)]*\))? )?(?:fn|const|static)\s+(\w+)", text))

new = {}
def fix(name, text):
    text = text.replace("crate::sheet::tests::", SUPID + "::sheet::").replace("crate::lifecycle::tests::", SUPID + "::lifecycle::")
    return text
for n in files:
    t = {"🃏️sheet": sheet_rest, "🧾️lifecycle": life_rest}.get(n, before[n])
    t = fix(n, t)
    new[n] = t
# prune / rewrite import lines mentioning support paths
def prune_imports(n, t):
    body_defs = localdefs(t)
    def sub(m):
        path, names = m.group(1), [x.strip() for x in m.group(2).split(",") if x.strip()]
        rest = t.replace(m.group(0), "")
        keep = [x for x in names if used(x, rest) and x not in body_defs]
        return "use %s::%s;" % (path, ("{%s}" % ", ".join(keep)) if len(keep) != 1 else keep[0]) if keep else ""
    return re.sub(r"use (%s::(?:sheet|lifecycle))::\{([^}]*)\};" % SUPID, sub, t)
for n in files:
    t = new[n]
    # add import lines for the extracted fixtures
    if n == "🃏️sheet":
        names = [x for x in sheet_names if used(x, t)]
        t = t.replace("use super::*;\n", "use super::*;\nuse %s::sheet::{%s};\n" % (SUPID, ", ".join(names)), 1)
    if n == "🧾️lifecycle":
        names = [x for x in life_names if used(x, t)]
        t = t.replace("use super::*;\n", "use super::*;\nuse %s::lifecycle::{%s};\n" % (SUPID, ", ".join(names)), 1)
    new[n] = prune_imports(n, t)
def prune_braces(text, path_rx, other):
    def sub(m):
        names=[x.strip() for x in m.group(2).split(",") if x.strip()]
        rest=text.replace(m.group(0),"")+other
        keep=[x for x in names if re.search(r"\b%s\b"%re.escape(x.split(" as ")[-1]),rest)]
        if not keep: return ""
        return "use %s::{%s};"%(m.group(1),", ".join(keep))
    return re.sub(r"use (%s)::\{([^}]*)\};"%path_rx,sub,text)
for n in ("🃏️sheet","🧾️lifecycle"):
    new[n]=prune_braces(new[n],"quiz_schema","")
    new[n]=prune_braces(new[n],"std::collections","")
for n,w in (("🃏️sheet","🃏️sheet"),("🧾️lifecycle","🧾️lifecycle")):
    pass
WR={}
for n in ("🃏️sheet","🧾️lifecycle"):
    wp=T+"/🥣️companion/%s/🦀️.rs"%n
    wt=rd(wp)
    wt=wt.replace("pub mod tests;","mod tests;")
    wt=prune_braces(wt,"quiz_schema",new[n])
    wt=prune_braces(wt,"std::collections",new[n])
    WR[wp]=wt
for n in files:
    t = new[n]
    nt = len(re.findall(r"(?m)^\s*#\[test\]", t)) + len(re.findall(r"(?m)^\s*#\[[a-z_:]*async_test\]", t))
    print(n, "tests before", ntests[n], "after", nt, "lines", before[n].count("\n"), "->", t.count("\n"))
    assert nt == ntests[n]
    head = [l for l in t.split("\n")[:14] if l.startswith("use ")]
    print("   ", *head, sep="\n    ")
if write:
    for n, p in files.items():
        wr(p, new[n])
    wr(SUP + "/🦀️.rs", sup_root); wr(SUP + "/🃏️sheet/🦀️.rs", sup_sheet); wr(SUP + "/🧾️lifecycle/🦀️.rs", sup_life)
    for wp,wt in WR.items(): wr(wp,wt)
    print("written")
