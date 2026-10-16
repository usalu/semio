"""Plans the consumer import edits: per hub Rust file, the capability traits its directory calls need, and where the import goes."""
import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
from scope import *

SKIP = ("/📇️directory/🪶️sqlite/", "/📇️directory/🐘️postgres/", "/📇️directory/🌐️neo4j/", "/📇️directory/🔌️port/", "/📇️directory/🔀️dispatch/")
NON_DIR_LAST = {"service", "directory_service", "authority", "reserve_service"}
RECV_RX = r"([A-Za-z_][\w]*(?:\s*\.\s*[A-Za-z_]\w*|\s*\(\s*[^()]*\)|\s*\[[^\]]*\])*)\s*$"


def skipped(r):
    if "/📇️directory/🔌️port/" in r or "/📇️directory/🔀️dispatch/" in r:
        return True
    if any(x in r for x in ("/📇️directory/🪶️sqlite/", "/📇️directory/🐘️postgres/", "/📇️directory/🌐️neo4j/")):
        return not ("/🧪️tests/" in r or "/🌱️creation-v1/" in r)
    return False


def receiver_last(masked_line_prefix):
    """last identifier segment of the receiver chain ending right before the '.' of the method call."""
    pre = masked_line_prefix.rstrip()
    if pre.endswith(")"):
        # a call result: take the identifier before the matching '(' pair
        depth = 0
        i = len(pre) - 1
        while i >= 0:
            if pre[i] == ")":
                depth += 1
            elif pre[i] == "(":
                depth -= 1
                if depth == 0:
                    break
            i -= 1
        pre = pre[:i].rstrip()
    m = re.search(r"([A-Za-z_]\w*)$", pre)
    return m.group(1) if m else ""


def need_of(text):
    m = mask(text)
    need = collections.defaultdict(set)
    for name, tr in TRAIT_OF.items():
        for mm in re.finditer(r"(\.|::)" + re.escape(name) + r"\s*(?:\(|::<)", m):
            pre = m[max(0, mm.start() - 160):mm.start()]
            if mm.group(1) == "::":
                # path call (UFCS) -> receiver irrelevant; trait imported by the path itself when written as Trait::method
                head = re.search(r"([A-Za-z_]\w*)\s*$", pre)
                if head and head.group(1) in ALL_TRAITS:
                    need[head.group(1)].add(name)
                    continue
                need[tr].add(name)
                continue
            last = receiver_last(pre)
            if last in NON_DIR_LAST or last.endswith("_service"):
                continue
            need[tr].add(name)
    return need


def plan():
    files, texts, parent = build()
    rel = lambda p: os.path.relpath(p, ROOT)
    has_super_glob = {p: bool(USE_GLOB_SUPER.search(mask(texts[p]))) for p in files}
    children = collections.defaultdict(list)
    for c, ps in parent.items():
        for p in ps:
            if has_super_glob.get(c):
                children[p].append(c)
    own_need = {}
    for p in files:
        r = rel(p)
        if skipped(r):
            continue
        own_need[p] = need_of(texts[p])

    def has_old(p):
        return any(leaf == "HubDirectory" for leaf, _ in imports_of(texts[p]))

    memo = {}

    def import_need(p, seen=()):
        if p in memo:
            return memo[p]
        s = set(own_need.get(p, {}).keys())
        for c in children.get(p, []):
            if c in seen or c not in own_need:
                continue
            s |= import_need(c, seen + (p,))
        memo[p] = s
        return s

    result = []
    for p in sorted(own_need):
        r = rel(p)
        inherits = has_super_glob[p] and p in parent
        old = has_old(p)
        # does this file get its names from a parent through the glob?
        if inherits:
            where = "inherit"
        else:
            where = "own"
        imp = import_need(p)
        if not imp and not old:
            continue
        result.append(dict(file=r, old=old, where=where, own=sorted(own_need[p]), imp=sorted(imp), parents=[rel(q) for q in parent.get(p, [])]))
    return result


if __name__ == "__main__":
    res = plan()
    json.dump(res, open(SC + "/hdc-callers-plan.json", "w"), ensure_ascii=False, indent=1)
    for r in res:
        flag = ("OLD " if r["old"] else "    ") + r["where"].ljust(8)
        print(flag, r["file"])
        print("          own:", {k: v for k, v in [(t, 1) for t in r["own"]]}.keys() if r["own"] else "-", " import:", r["imp"])
