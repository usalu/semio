"""🧱 Plan builder: accumulates whole-text replacements of source files and manifests (turned into anchored span edits / line operations by diff), created files and workspace member additions, and writes `staged/plans/<tag>.json`. Library."""
import difflib, hashlib, json, os, sys, time
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from manifest_edit import line_ops, apply_ops
ROOT = "/Users/ueli/Documents/semio"


def sha(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


def occ(text, sub):
    r, i = [], text.find(sub)
    while i >= 0:
        r.append(i)
        i = text.find(sub, i + 1)
    return r


class Plan:
    def __init__(self, tag, hub, hub_package):
        self.tag, self.hub, self.hub_package = tag, hub, hub_package
        self.orig, self.cur, self.kind, self.creates, self.crates = {}, {}, {}, {}, {}
        self.notes, self.deletes = [], {}

    def text(self, rel):
        if rel in self.creates:
            return self.creates[rel]
        if rel not in self.cur:
            self.orig[rel] = open(os.path.join(ROOT, rel), "rb").read().decode("utf-8")
            self.cur[rel] = self.orig[rel]
        return self.cur[rel]

    def put(self, rel, new, kind="file", crate=None):
        self.text(rel)
        self.cur[rel] = new
        self.kind[rel] = kind
        if crate:
            self.crates.setdefault(crate, set()).add(rel)

    def delete(self, rel):
        self.text(rel)
        self.deletes[rel] = sha(self.orig[rel])

    def create(self, rel, content):
        if os.path.exists(os.path.join(ROOT, rel)) and open(os.path.join(ROOT, rel), "rb").read().decode("utf-8") != content:
            raise ValueError("create target exists with other content: " + rel)
        self.creates[rel] = content

    def span_edits(self, rel):
        old, new = self.orig[rel], self.cur[rel]
        a, b = old.split("\n"), new.split("\n")
        pos = [0]
        for l in a:
            pos.append(pos[-1] + len(l) + 1)
        out = []
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
            if tag == "equal":
                continue
            ins = "\n".join(b[j1:j2])
            if i1 == i2:
                k = i1 - 1
                while k >= 0 and a[k] == "":
                    k -= 1
                if k < 0:
                    o = a[0] if a else ""
                    start, o2, n2 = 0, a[0], ins + "\n" + a[0]
                else:
                    o2 = "\n".join(a[k:i1])
                    start, n2 = pos[k], o2 + "\n" + ins
            elif j1 == j2:
                if i2 < len(a):
                    start, o2, n2 = pos[i1], "\n".join(a[i1:i2]) + "\n", ""
                else:
                    start, o2, n2 = pos[i1] - 1, "\n" + "\n".join(a[i1:i2]), ""
            else:
                start, o2, n2 = pos[i1], "\n".join(a[i1:i2]), ins
            oc = occ(old, o2)
            out.append([start, start + len(o2), o2, n2, oc.index(start), len(oc), [old[max(0, start - 32):start], old[start + len(o2):start + len(o2) + 32]]])
        return out

    def write(self, outdir, extra_stats=None):
        files, manifests = [], []
        for rel in sorted(self.cur):
            if rel in self.deletes or self.cur[rel] == self.orig[rel]:
                continue
            if self.kind[rel] == "manifest":
                ops = line_ops(self.orig[rel], self.cur[rel])
                if apply_ops(self.orig[rel], ops) != self.cur[rel]:
                    raise ValueError("manifest ops not exact-once: " + rel)
                manifests.append({"path": rel, "sha256": sha(self.orig[rel]), "new_text": self.cur[rel], "sha256_after": sha(self.cur[rel]), "ops": ops, "crate": rel})
            else:
                eds = self.span_edits(rel)
                new = self.orig[rel]
                for a, b, o, n, *_r in sorted(eds, reverse=True):
                    assert new[a:b] == o, rel
                    new = new[:a] + n + new[b:]
                assert new == self.cur[rel], "span edits do not reproduce " + rel
                files.append({"path": rel, "sha256": sha(self.orig[rel]), "sha256_after": sha(self.cur[rel]), "edits": eds, "crates": sorted(c for c, fs in self.crates.items() if rel in fs) or [self.tag]})
        creates = [{"path": r, "content": c, "sha256_after": sha(c)} for r, c in sorted(self.creates.items())]
        plan = {"hub": self.hub, "hub_package": self.hub_package, "tag": self.tag, "created": time.strftime("%Y-%m-%d %H:%M"), "stats": dict({"files": len(files), "manifests": len(manifests), "creates": len(creates), "edits": sum(len(f["edits"]) for f in files)}, **(extra_stats or {})), "crates": {c: {} for c in self.crates}, "files": files, "manifests": manifests, "creates": creates, "deletes": [{"path": r, "sha256": h} for r, h in sorted(self.deletes.items())], "notes": self.notes}
        os.makedirs(outdir, exist_ok=True)
        json.dump(plan, open(os.path.join(outdir, self.tag + ".json"), "w"), ensure_ascii=False)
        return plan["stats"]


def manifest_text(name, id_, desc, deps, ws_rel, lib_rel, extra_features="", gated=(), features=None):
    """Crate manifest in the repo's conventions: role/id metadata, source boundary, workspace lints, path dependencies (name, relative path)."""
    d = "".join(f'{n} = {{ path = "{p}" }}\n' for n, p in sorted(deps))
    opt = "".join(f'{n} = {{ path = "{p}", optional = true }}\n' for n, p, cfg, o in sorted(gated) if o and not cfg)
    d += opt
    by_cfg = {}
    for n, p, cfg, o in sorted(gated):
        if cfg:
            by_cfg.setdefault(cfg, []).append(f'{n} = {{ path = "{p}"{", optional = true" if o else ""} }}\n')
    tgt = "".join(f"\n[target.'{cfg}'.dependencies]\n" + "".join(v) for cfg, v in sorted(by_cfg.items()))
    feat = ""
    if features:
        feat = "\n[features]\n" + "".join(f'{k} = [{", ".join(chr(34) + x + chr(34) for x in v)}]\n' for k, v in sorted(features.items()))
    return f'''[package]
workspace = "{ws_rel}"
name = "{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{desc}"

[package.metadata.semio]
role = "framework"
id = "{id_}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "{lib_rel}"
{extra_features}{feat}
[dependencies]
{d}{tgt}'''


def add_member(plan, rel, after_entry, new_entry):
    """Inserts `"new_entry",` on the line after the line holding `"after_entry",` (same indentation) in a workspace manifest."""
    t = plan.text(rel)
    needle = '"%s",' % after_entry
    i = t.find(needle)
    if i < 0:
        raise ValueError(f"{rel}: member anchor {after_entry!r} not found")
    ls = t.rfind("\n", 0, i) + 1
    le = t.find("\n", i)
    indent = t[ls:i]
    line = t[ls:le]
    if t.count(line) != 1 or '"%s",' % new_entry in t:
        if '"%s",' % new_entry in t:
            return
        raise ValueError(f"{rel}: member anchor line not unique")
    plan.put(rel, t[:le] + "\n" + indent + '"%s",' % new_entry + t[le:], kind="manifest")


def rel_to(frm, to):
    return os.path.relpath(os.path.join(ROOT, to), os.path.join(ROOT, frm)).replace(os.sep, "/")
