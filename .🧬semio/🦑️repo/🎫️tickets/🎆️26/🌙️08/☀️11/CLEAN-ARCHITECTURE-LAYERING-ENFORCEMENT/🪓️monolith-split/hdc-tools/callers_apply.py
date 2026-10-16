"""Applies the consumer import edits of callers_plan: HubDirectory imports become the capability traits each file (and its glob children) uses."""
import os, re, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
import callers_plan

SELF_CRATE = ("🌎️hub/📇️directory/🦀️.rs", "🌎️hub/📇️directory/🔤️wire/🦀️.rs", "🌎️hub/📇️directory/🧪️tests/🔮️backend-corpus/🦀️.rs")
EDGE = re.compile(r"^(\s*)use semio_hub_directory::(HubDirectory|\{[^}]*\bHubDirectory\b[^}]*\});[ \t]*$", re.M)


def new_import(indent, imp):
    if len(imp) == 1:
        return "%suse semio_hub_directory::%s;" % (indent, imp[0])
    return "%suse semio_hub_directory::{%s};" % (indent, ", ".join(sorted(imp)))


def insert_use(text, line):
    m = mask(text)
    first_item = re.search(r"^(?:#\[|pub |fn |async fn |struct |enum |impl |trait |mod |const |static |type |macro_rules|extern )", m, re.M)
    limit = first_item.start() if first_item else len(m)
    last = None
    for mm in re.finditer(r"^use [^;]*;[ \t]*$", m, re.M):
        if mm.start() < limit:
            last = mm
    if last:
        return text[:last.end()] + "\n" + line + text[last.end():]
    # after the leading //! block
    lines = text.split("\n")
    i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].startswith("#![")):
        i += 1
    if i < len(lines) and lines[i] == "":
        i += 1
    lines[i:i] = [line, ""] if i < len(lines) and lines[i] != "" else [line]
    return "\n".join(lines)


def main():
    dry = "--apply" not in sys.argv
    plan = callers_plan.plan()
    report = []
    for r in plan:
        f = r["file"]
        if f in SELF_CRATE:
            continue
        p = os.path.join(ROOT, f)
        t = read(p)
        imp = r["imp"]
        edits = []
        if EDGE.search(t):
            def sub(m):
                indent, what = m.group(1), m.group(2)
                if what == "HubDirectory":
                    return new_import(indent, imp) if imp else ""
                line = m.group(0)
                if not imp:
                    names = [x for x in split_top(what[1:-1]) if x != "HubDirectory"]
                    return "%suse semio_hub_directory::{%s};" % (indent, ", ".join(names)) if names else ""
                return merge_use_list(line, {"HubDirectory"}, imp)
            t2 = EDGE.sub(sub, t)
            t2 = re.sub(r"\n\n\n+", "\n\n", t2) if False else t2
            edits.append("rewrite")
        elif r["where"] == "own" and imp:
            have = {leaf for leaf, _ in callers_plan.imports_of(t) if leaf in ALL_TRAITS}
            missing = [x for x in imp if x not in have]
            if not missing:
                t2 = t
            else:
                one = re.search(r"^use semio_hub_directory::\{[^}]*\};[ \t]*$", t, re.M)
                single = re.search(r"^use semio_hub_directory::(\w+);[ \t]*$", t, re.M)
                if one:
                    t2 = t[:one.start()] + merge_use_list(one.group(0), set(), missing) + t[one.end():]
                elif single:
                    t2 = t[:single.start()] + merge_use_list("use semio_hub_directory::{%s};" % single.group(1), set(), missing) + t[single.end():]
                else:
                    t2 = insert_use(t, new_import("", missing))
                edits.append("insert")
        else:
            t2 = t
        if t2 != t:
            report.append((f, edits[0], imp))
            if not dry:
                write(p, t2)
        else:
            report.append((f, "none(%s)" % r["where"], imp))
    for f, e, imp in report:
        print(e.ljust(12), f, imp)
    print("DRY RUN" if dry else "APPLIED")


if __name__ == "__main__":
    main()
