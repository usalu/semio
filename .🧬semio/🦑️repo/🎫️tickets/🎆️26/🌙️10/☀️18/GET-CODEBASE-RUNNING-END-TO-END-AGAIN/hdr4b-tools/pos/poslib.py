"""🛡️ POS shared helpers: write guard (sandbox-only writes), repo sources (working tree or a git revision, read-only), Cargo manifest index and feature resolution.
Nothing here ever opens a repo path for writing; every write goes through `Sandbox.write`, which resolves symlinks and asserts the target lies under the sandbox root."""
import json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/hdr4b-x"
SCRATCH = "/Users/ueli/.cache/hdr4"
PREFIX = os.environ.get("POS_PREFIX", "hdr4pos")
SANDBOX_BASE = SCRATCH + "/" + PREFIX + "-work"
TOOLCHAIN = "+nightly-2026-07-20"


class Sandbox:
    def __init__(self, root):
        self.root = os.path.realpath(root)
        assert self.root.startswith(os.path.realpath(SANDBOX_BASE) + os.sep) or self.root == os.path.realpath(SANDBOX_BASE), f"sandbox root outside {SANDBOX_BASE}: {self.root}"
        assert not self.root.startswith(os.path.realpath(ROOT) + os.sep), f"sandbox inside the repo: {self.root}"
        os.makedirs(self.root, exist_ok=True)

    def check(self, path):
        d = os.path.realpath(os.path.dirname(os.path.abspath(path)))
        assert d == self.root or d.startswith(self.root + os.sep), f"write outside sandbox: {path} -> {d}"
        return path

    def write(self, path, data):
        path = os.path.abspath(path)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        self.check(path)
        if os.path.islink(path):
            os.unlink(path)
        mode = "wb" if isinstance(data, bytes) else "w"
        with open(path, mode, **({} if mode == "wb" else {"encoding": "utf-8"})) as f:
            f.write(data)

    def remove(self, path):
        self.check(path)
        if os.path.lexists(path):
            os.unlink(path)


def log_write(path, text):
    p = os.path.realpath(os.path.dirname(os.path.abspath(path)))
    assert p.startswith(os.path.realpath(GEN) + os.sep) or p == os.path.realpath(GEN), f"log outside generated: {path}"
    assert os.path.basename(path).startswith(PREFIX + "-"), f"log name must start with {PREFIX}-: {path}"
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


class Source:
    """Read-only access to the repo: the working tree (rev None) or a git revision."""

    def __init__(self, rev=None):
        self.rev = rev
        self.proc = None
        self.cache = {}

    def _batch(self):
        if self.proc is None:
            self.proc = subprocess.Popen(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "cat-file", "--batch"], cwd=ROOT, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        return self.proc

    def read(self, rel):
        if rel in self.cache:
            return self.cache[rel]
        if self.rev is None:
            p = os.path.join(ROOT, rel)
            try:
                with open(p, "rb") as f:
                    data = f.read()
            except (FileNotFoundError, IsADirectoryError, NotADirectoryError):
                data = None
        else:
            pr = self._batch()
            pr.stdin.write((f"{self.rev}:{rel}\n").encode("utf-8"))
            pr.stdin.flush()
            line = pr.stdout.readline().decode("utf-8").rstrip("\n")
            if line.endswith(" missing") or len(line.split(" ")) < 3:
                data = None
            else:
                parts = line.split(" ")
                size = int(parts[2])
                data = pr.stdout.read(size)
                pr.stdout.read(1)
                if parts[1] != "blob":
                    data = None
        self.cache[rel] = data
        return data

    def exists(self, rel):
        return self.read(rel) is not None

    def close(self):
        if self.proc:
            self.proc.stdin.close()
            self.proc.wait()
            self.proc = None


def norm(p):
    return os.path.normpath(p)


def ident(pkg):
    return pkg.replace("-", "_")


def parse_toml(text):
    """Minimal Cargo.toml reader: {section: {key: raw value string}}; multi-line arrays/tables are joined by bracket balance."""
    out = {}
    sec = ""
    out[sec] = {}
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        raw = lines[i]
        i += 1
        s = raw.strip()
        if not s or s.startswith("#"):
            continue
        m = re.match(r"^\[\[?([^\]]+)\]\]?\s*(?:#.*)?$", s)
        if m:
            sec = m.group(1).strip().strip('"')
            out.setdefault(sec, {})
            continue
        m = re.match(r"^([A-Za-z0-9_\-\.\"']+)\s*=\s*(.*)$", s)
        if not m:
            continue
        key, val = m.group(1).strip('"'), m.group(2)
        depth = val.count("[") + val.count("{") - val.count("]") - val.count("}")
        while depth > 0 and i < len(lines):
            nxt = re.sub(r"#.*$", "", lines[i]) if '"' not in lines[i] else lines[i]
            val += " " + nxt.strip()
            depth += nxt.count("[") + nxt.count("{") - nxt.count("]") - nxt.count("}")
            i += 1
        out[sec][key] = val.strip()
    return out


def dep_sections(sections):
    """Yield (kind, depname, spec_string) for dependencies / dev-dependencies / build-dependencies incl. target tables."""
    for sec, kv in sections.items():
        m = re.match(r"^(?:target\..*\.)?(dependencies|dev-dependencies|build-dependencies)(?:\.(.+))?$", sec)
        if not m:
            continue
        kind = m.group(1)
        if m.group(2):
            spec = "{ " + ", ".join(f"{k} = {v}" for k, v in kv.items()) + " }"
            yield kind, m.group(2).strip('"'), spec
        else:
            for k, v in kv.items():
                if "." in k and k.endswith(".workspace"):
                    yield kind, k[: -len(".workspace")], "{ workspace = true }"
                else:
                    yield kind, k, v


class Manifests:
    CACHE = GEN + "/" + PREFIX + "-manifest-index.json"

    def __init__(self, src):
        self.src = src
        self.by_name = {}
        self.parsed = {}
        self.ws_deps = {}

    def build_index(self, force=False):
        if not force and os.path.exists(self.CACHE):
            self.by_name = json.load(open(self.CACHE, encoding="utf-8"))
            return
        tracked = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "--", "*Cargo.toml", "Cargo.toml"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
        untracked = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "--others", "--exclude-standard", "--", "*Cargo.toml"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
        idx = {}
        for rel in sorted(set(tracked + untracked)):
            if not rel or rel.startswith(".") or "node_modules" in rel or "/target/" in rel or "🗑️generated" in rel or "/dist/" in rel or "pre-a1" in rel:
                continue
            data = self.src.read(rel)
            if data is None:
                continue
            nm = parse_toml(data.decode("utf-8", "replace")).get("package", {}).get("name")
            if nm:
                idx.setdefault(nm.strip().strip('"'), rel)
        self.by_name = idx
        from_json = json.dumps(idx, ensure_ascii=False, indent=0)
        os.makedirs(GEN, exist_ok=True)
        with open(self.CACHE, "w", encoding="utf-8") as f:
            f.write(from_json)

    def manifest(self, pkg, src=None):
        src = src or self.src
        key = (pkg, src.rev)
        if key in self.parsed:
            return self.parsed[key]
        rel = self.by_name.get(pkg)
        res = None
        if rel is not None:
            data = src.read(rel)
            if data is not None:
                sec = parse_toml(data.decode("utf-8", "replace"))
                res = dict(pkg=pkg, rel=rel, dir=os.path.dirname(rel), sections=sec)
        self.parsed[key] = res
        return res

    def lib_rel(self, m):
        sec = m["sections"].get("lib", {})
        p = sec.get("path")
        base = m["dir"]
        if p:
            return norm(os.path.join(base, p.strip().strip('"')))
        return norm(os.path.join(base, "src/lib.rs"))

    def lib_name(self, m):
        sec = m["sections"].get("lib", {})
        n = sec.get("name")
        return n.strip().strip('"') if n else ident(m["pkg"])

    def deps(self, m):
        """[(kind, extern_name, package, features, optional, default_features)] for the manifest."""
        out = []
        for kind, name, spec in dep_sections(m["sections"]):
            pkg = name
            mm = re.search(r"package\s*=\s*\"([^\"]+)\"", spec)
            renamed = False
            if mm:
                pkg = mm.group(1)
                renamed = pkg != name
            feats = []
            mf = re.search(r"features\s*=\s*\[([^\]]*)\]", spec)
            if mf:
                feats = re.findall(r"\"([^\"]+)\"", mf.group(1))
            optional = bool(re.search(r"optional\s*=\s*true", spec))
            out.append((kind, name, pkg, feats, optional, renamed))
        return out

    def features_table(self, m):
        tab = {}
        for k, v in m["sections"].get("features", {}).items():
            tab[k] = re.findall(r"\"([^\"]+)\"", v)
        return tab
