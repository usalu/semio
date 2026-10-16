"""Include-fragment splitter: cuts verbatim item ranges of a Rust file into `<folder>/🦀️.rs` fragments spliced back with `include!`.
The fragment keeps the module identity of every item (test paths, visibility and `use` scope are unchanged), so no widening or import is needed.
A spec is a dict: target (frslib.TARGETS key), children [{folder, doc, ranges [(a, b) 1-based inclusive], at (optional anchor line)}], path_dot_up (bool).
Rebased on cut: `#[path = "."]` becomes `#[path = ".."]` and relative `include!`/`include_str!`/`include_bytes!` literals get one more `../`."""
import os, re, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
from rslex import lex, outline, multiline_literal_lines

DOCABLE = {"fn", "struct", "enum", "trait", "impl", "mod", "const", "static", "type", "union"}


class FragPlan:
    def __init__(self, spec):
        self.spec = spec
        self.name = spec["target"]
        self.t = TARGETS[self.name]
        self.src = read(snap_dir(self.name) + "/" + self.t["file"])
        self.ls = lines_of(self.src)
        assert not multiline_literal_lines(self.src), "multi-line literals present"
        self.base = os.path.dirname(self.t["file"])

    def build(self):
        ls, spec = self.ls, self.spec
        cov = Coverage(len(ls))
        depths, inlit = lex(self.src)
        out_files, anchors, skip = {}, {}, set()
        report = {}
        for c in spec["children"]:
            body = []
            for a, b in c["ranges"]:
                cov.claim(c["folder"], a, b)
                assert depths[a - 1] == 0 and not inlit[a - 1], (c["folder"], a, "range does not start at depth 0")
                nxt = depths[b] if b < len(depths) else 0
                assert nxt == 0 and not (b < len(inlit) and inlit[b]), (c["folder"], b, "range does not end at depth 0")
                body.extend(ls[a - 1:b])
                skip.update(range(a, b + 1))
            rebased = []
            for i, l in enumerate(body):
                if spec.get("path_dot_up") and l.strip() == '#[path = "."]':
                    body[i] = l.replace('"."', '".."')
                    rebased.append(l.strip())
                if re.search(r'include(?:_str|_bytes)?!\(\s*"[^"/]', l):
                    body[i] = re.sub(r'(include(?:_str|_bytes)?!\(\s*)"([^"/])', r'\1"../\2', body[i])
                    rebased.append(l.strip()[:90])
            while body and body[0].strip() == "":
                body.pop(0)
            while body and body[-1].strip() == "":
                body.pop()
            last = next((l for l in reversed(body) if l.strip()), "")
            assert not last.strip().startswith("///") and not last.strip().startswith("#["), (c["folder"], "fragment ends with an orphan doc or attribute", last)
            its = outline("\n".join(body) + "\n")
            first = its[0] if its else None
            doc = c["doc"]
            marker = "///" if first and first["kind"] in DOCABLE else "//"
            head = f"{marker} {doc}"
            out_files[c["folder"]] = head + "\n\n" + "\n".join(body) + "\n"
            anchors[c.get("at") or c["ranges"][0][0]] = c
            report[c["folder"]] = dict(lines=len(body) + 2, first=(first["kind"], first["name"]) if first else None, marker=marker, rebased=rebased)
        for k in range(1, len(ls) + 1):
            if cov.owner[k] is None:
                cov.owner[k] = "parent"
        cov.check()
        parent = []
        for k in range(1, len(ls) + 1):
            if k in anchors:
                if parent and parent[-1].strip() and not parent[-1].startswith("include!("):
                    parent.append("")
                parent.append(f'include!("{anchors[k]["folder"]}/🦀️.rs");')
            if k in skip:
                continue
            if parent and parent[-1].startswith("include!(") and ls[k - 1].strip():
                parent.append("")
            parent.append(ls[k - 1])
        tidy = []
        for l in parent:
            if l.strip() == "" and tidy and tidy[-1].strip() == "":
                continue
            tidy.append(l)
        self.out = {self.t["file"]: "\n".join(tidy) + "\n"}
        for folder, txt in out_files.items():
            self.out[self.base + "/" + folder + "/🦀️.rs"] = txt
        self.report = report
        return self


def run(SPEC):
    import shutil
    plan = FragPlan(SPEC).build()
    sd = stage_dir(plan.name)
    if os.path.exists(sd):
        shutil.rmtree(sd)
    for rel, txt in plan.out.items():
        write(f"{sd}/{rel}", txt)
    new = []
    for txt in plan.out.values():
        new.extend(lines_of(txt))
    gone, added = diff_multiset(plan.ls, new)
    print("files:", {(k.split("/")[-2] if k != plan.t["file"] else "PARENT"): len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:2500])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:2500])
    for f, r in plan.report.items():
        print(f, r)
    return plan
