#!/usr/bin/env python3
"""🧰️ Writes staged/🧩️units/🧪️tests/🔣️.json: the language-agnostic test vectors. Graph, scenario, python and go expectations are hand-written here; the key hex values are computed by the independent oracle (stdlib only) and must be reproduced by the TypeScript implementation."""
import importlib.util, json, os

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged", "🧩️units")
spec = importlib.util.spec_from_file_location("oracle", os.path.join(STAGED, "🧪️tests", "🐍️oracle", "🐍️.py"))
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
policy = json.load(open(os.path.join(STAGED, "🔣️.json"), encoding="utf8"))
KINDS = policy["interfaceKinds"]


def unit(id, language="go", kind="library", deps=(), interface="auto", sources=None, inputs=(), externals=(), toolchain="tool 1", parts=(), argv=("build",), env=None, outputs=("out",)):
    name = id.split(":", 1)[1]
    if interface == "auto":
        interface = {"go": "go-export-data", "python": "python-api", "rust": "rust-rmeta", "typescript": "typescript-declarations", "tex": "tex-aux"}[language] if kind == "library" else None
    return {
        "id": id, "language": language, "kind": kind, "label": name,
        "sources": list(sources if sources is not None else [name.replace("#", "_") + ".src"]), "inputs": list(inputs),
        "dependencies": [{"unit": d[0], "use": d[1]} for d in deps], "externals": list(externals),
        "interface": {"kind": interface} if interface else None, "outputs": list(outputs),
        "toolchain": {"id": toolchain, "parts": list(parts)}, "action": {"argv": list(argv), "env": env or {}},
    }


def graph(*units):
    return {"version": 1, "units": list(units)}


graphs = [
    {"name": "chain", "graph": graph(unit("go:a"), unit("go:b", deps=[("go:a", "interface")]), unit("go:c", deps=[("go:b", "interface")])),
     "expect": {"valid": True, "order": ["go:a", "go:b", "go:c"], "layer": {"go:a": 0, "go:b": 1, "go:c": 2}}},
    {"name": "code-point-order", "graph": graph(unit("go:🧰"), unit("go:｡"), unit("go:z")),
     "expect": {"valid": True, "order": ["go:z", "go:｡", "go:🧰"], "layer": {"go:z": 0, "go:｡": 0, "go:🧰": 0}}},
    {"name": "layers-by-longest-path", "graph": graph(unit("go:t"), unit("go:u", deps=[("go:t", "interface")]), unit("go:v", deps=[("go:t", "interface"), ("go:u", "interface")]), unit("go:w", deps=[("go:v", "declared")], interface=None, kind="binary")),
     "expect": {"valid": True, "order": ["go:t", "go:u", "go:v", "go:w"], "layer": {"go:t": 0, "go:u": 1, "go:v": 2, "go:w": 3}}},
    {"name": "cycle", "graph": graph(unit("go:a", deps=[("go:b", "interface")]), unit("go:b", deps=[("go:a", "interface")])), "expect": {"valid": False, "kinds": ["cycle"]}},
    {"name": "unknown-dependency", "graph": graph(unit("go:a", deps=[("go:ghost", "interface")])), "expect": {"valid": False, "kinds": ["unknown-dependency"]}},
    {"name": "interface-of-binary", "graph": graph(unit("go:m", kind="binary", interface=None), unit("go:a", deps=[("go:m", "interface")])), "expect": {"valid": False, "kinds": ["no-interface"]}},
    {"name": "implementation-of-binary-is-fine", "graph": graph(unit("go:m", kind="binary", interface=None), unit("go:t", kind="test", interface=None, deps=[("go:m", "implementation")])),
     "expect": {"valid": True, "order": ["go:m", "go:t"], "layer": {"go:m": 0, "go:t": 1}}},
    {"name": "shared-source", "graph": graph(unit("go:a", sources=["x.go"]), unit("go:b", sources=["x.go"])), "expect": {"valid": False, "kinds": ["shared-source"]}},
    {"name": "interface-kind-of-another-language", "graph": graph(unit("python:p", language="python", interface="go-export-data")), "expect": {"valid": False, "kinds": ["kind"]}},
    {"name": "id-prefix", "graph": graph(unit("python:x", language="go")), "expect": {"valid": False, "kinds": ["prefix"]}},
    {"name": "duplicate-dependency", "graph": graph(unit("go:a"), unit("go:b", deps=[("go:a", "interface"), ("go:a", "implementation")])), "expect": {"valid": False, "kinds": ["duplicate-dependency"]}},
    {"name": "self-dependency", "graph": graph(unit("go:a", deps=[("go:a", "declared")])), "expect": {"valid": False, "kinds": ["self"]}},
]

# ---------------------------------------------------------------- keys (expected values from the oracle)
digest = lambda text: oracle.sha(text)
key_units = [
    ("emoji-paths-and-all-uses",
     unit("rust:app", language="rust", kind="library", deps=[("rust:core", "interface"), ("rust:mac", "implementation"), ("rust:doc", "declared")],
          sources=["🧰️framework/z.rs", "a/b.rs", "｡/c.rs", "🧪️tests/t.rs"], inputs=["Cargo.toml", "🔣️.json"], externals=["cargo-registry:serde@1.0.1", "cargo-registry:anyhow@1.0.0"], toolchain="rustc 1.95.0", parts=["resolution=abc", "flags=-O"], argv=["cargo", "build", "-p", "app"], env={"B": "2", "A": "1"}, outputs=["libapp.rlib", "app.d"]),
     {"rust:core": {"interfaceClosure": digest("core-i"), "outputClosure": digest("core-o")}, "rust:mac": {"interfaceClosure": digest("mac-i"), "outputClosure": digest("mac-o")}, "rust:doc": {"interfaceClosure": digest("doc-i"), "outputClosure": digest("doc-o")}},
     digest("app-iface"), digest("app-out"), None),
    ("no-interface-no-dependencies", unit("go:main", kind="binary", interface=None, sources=["main.go"], outputs=["bin"]), {}, None, digest("main-out"), None),
    ("direct-interface-kind", unit("tex:doc", language="tex", deps=[("tex:chap", "interface")], sources=["doc.tex"]), {"tex:chap": {"interfaceClosure": digest("chap-i"), "outputClosure": digest("chap-o")}}, digest("doc-aux"), digest("doc-out"), None),
    ("exposes-subset", unit("go:b", deps=[("go:a", "interface"), ("go:z", "interface")], sources=["b.go"]),
     {"go:a": {"interfaceClosure": digest("a-i"), "outputClosure": digest("a-o")}, "go:z": {"interfaceClosure": digest("z-i"), "outputClosure": digest("z-o")}}, digest("b-iface"), digest("b-out"), ["go:a"]),
]
keys = []
for name, u, deps, interface, outputs, exposes in key_units:
    digests = {p: digest("file:" + p) for p in u["sources"] + u["inputs"]}
    case = {"name": name, "unit": u, "digests": digests, "dependencies": deps, "interface": interface, "outputs": outputs}
    if exposes is not None:
        case["exposes"] = exposes
    case["expect"] = oracle.answer_keys(case, KINDS)
    keys.append(case)

# ---------------------------------------------------------------- early-cutoff scenarios (hand-written expectations)
chain = graph(
    unit("go:a"), unit("go:b", deps=[("go:a", "interface")]), unit("go:c", deps=[("go:b", "interface")]),
    unit("go:c#test", kind="test", interface=None, deps=[("go:c", "implementation")], sources=["c_test.go"]),
    unit("go:d", deps=[("go:c", "interface")]), unit("go:x"),
)
def step(edits, ran=(), hit=(), failed=(), blocked=(), cutoff=()):
    return {"edits": [{"unit": u, "effect": e} for u, e in edits], "expect": {"ran": sorted(ran), "hit": sorted(hit), "failed": sorted(failed), "blocked": sorted(blocked), "cutoff": sorted(cutoff)}}
ALL = ["go:a", "go:b", "go:c", "go:c#test", "go:d", "go:x"]
scenarios = [
    {"name": "chain-early-cutoff", "graph": chain, "steps": [
        step([], ran=ALL),
        step([], hit=ALL),
        step([("go:a", "body")], ran=["go:a", "go:c#test"], hit=["go:b", "go:c", "go:d", "go:x"], cutoff=["go:a"]),
        step([("go:a", "interface")], ran=["go:a", "go:b", "go:c", "go:c#test", "go:d"], hit=["go:x"]),
        step([("go:a", "revert")], hit=ALL),
        step([("go:b", "body")], ran=["go:b", "go:c#test"], hit=["go:a", "go:c", "go:d", "go:x"], cutoff=["go:b"]),
        step([("go:c", "fail")], failed=["go:c"], blocked=["go:c#test"], hit=["go:a", "go:b", "go:d", "go:x"]),
        step([], failed=["go:c"], blocked=["go:c#test"], hit=["go:a", "go:b", "go:d", "go:x"]),
        step([("go:c", "fix")], ran=["go:c", "go:c#test"], hit=["go:a", "go:b", "go:d", "go:x"], cutoff=["go:c"]),
        step([("go:c", "fail-hard")], failed=["go:c"], blocked=["go:c#test", "go:d"], hit=["go:a", "go:b", "go:x"]),
        step([("go:c", "fix")], ran=["go:c", "go:c#test"], hit=["go:a", "go:b", "go:d", "go:x"]),
    ]},
    {"name": "exposes-limits-the-cone", "graph": graph(unit("go:a"), unit("go:b", deps=[("go:a", "interface")]), unit("go:c", deps=[("go:b", "interface")])), "exposes": {"go:b": []}, "steps": [
        step([], ran=["go:a", "go:b", "go:c"]),
        step([("go:a", "interface")], ran=["go:a", "go:b"], hit=["go:c"], cutoff=["go:b"]),
        step([("go:b", "interface")], ran=["go:b", "go:c"], hit=["go:a"]),
    ]},
    {"name": "declared-edges-and-inputs", "graph": graph(unit("go:p", deps=[("go:q", "declared")], inputs=["p.cfg"]), unit("go:q", kind="binary", interface=None)), "steps": [
        step([], ran=["go:p", "go:q"]),
        step([("go:q", "body")], ran=["go:q"], hit=["go:p"]),
        step([("go:p", "input")], ran=["go:p"], hit=["go:q"], cutoff=["go:p"]),
        step([("go:p", "revert")], hit=["go:p", "go:q"]),
    ]},
    {"name": "diamond", "graph": graph(unit("go:base"), unit("go:l", deps=[("go:base", "interface")]), unit("go:r", deps=[("go:base", "interface")]), unit("go:top", deps=[("go:l", "interface"), ("go:r", "interface")])), "steps": [
        step([], ran=["go:base", "go:l", "go:r", "go:top"]),
        step([("go:l", "body")], ran=["go:l"], hit=["go:base", "go:r", "go:top"], cutoff=["go:l"]),
        step([("go:l", "interface")], ran=["go:l", "go:top"], hit=["go:base", "go:r"]),
        step([("go:base", "interface")], ran=["go:base", "go:l", "go:r", "go:top"]),
    ]},
]

names = [{"id": i} for i in ["go:example.com/lab/a", "python:pkg.mod", "typescript:dir:🧰️framework/📦️packages", "rust:serde@1.0.1#bin:tool", "go:🧰", "go:x" * 30]]
for row in names:
    row["expect"] = oracle.unit_target_name(row["id"])

# ---------------------------------------------------------------- python
PY = {}
PY["analyze"] = [
    {"name": "imports-and-names", "source": "import os, sys as system\nimport a.b.c\nfrom . import util as util\nfrom .m import a, b as c\nfrom ..x.y import z\nfrom pkg.sub import *\n\ndef f():\n    import json\n    return json.dumps(1)\n",
     "expect": {"imports": [["", 0, ["os", "sys"]], ["", 0, ["a.b.c"]], ["", 0, ["json"]], ["", 1, ["util"]], ["m", 1, ["a", "b"]], ["x.y", 2, ["z"]], ["pkg.sub", 0, ["*"]]], "names": ["a", "c", "f", "os", "system", "util", "z"], "dynamic": True}},
    {"name": "structure-and-targets", "source": "X: int = 3\nY = Z = [1, 2]\n(w := 5)\nfor i, (j, k) in pairs: pass\nwith open('f') as fh, other() as (aa, bb): pass\nobj.attr = 1\nd['k'] = 2\nclass C:\n    inner = 1\n    def m(self): pass\n@dec\ndef g(): global G; G = 1\nif cond:\n    def h(): pass\nelse:\n    h = None\n",
     "expect": {"imports": [], "names": ["C", "G", "X", "Y", "Z", "aa", "bb", "fh", "g", "h", "i", "j", "k", "w"], "dynamic": False}},
    {"name": "dynamic-markers", "source": "def __getattr__(name): return 1\n", "expect": {"imports": [], "names": ["__getattr__"], "dynamic": True}},
    {"name": "globals-call-is-dynamic", "source": "def f():\n    globals()['a'] = 1\n", "expect": {"imports": [], "names": ["f"], "dynamic": True}},
    {"name": "strings-and-continuations", "source": "s = '''import not_an_import\nclass NotAClass: pass'''\nt = 'from x import y'\nu = (1 +\n     2)\nv = 3 \\\n    + 4\ndef f(a,\n      b=')'):\n    return a\n",
     "expect": {"imports": [], "names": ["f", "s", "t", "u", "v"], "dynamic": False}},
    {"name": "init-reexports", "init": True, "source": "from .a import A\nimport os\n", "expect": {"imports": [["", 0, ["os"]], ["a", 1, ["A"]]], "names": ["A", "os"], "dynamic": False}},
]
PY["pairs"] = [
    {"name": "body-edit", "same": True, "before": "def f(a):\n    return a\n", "after": "def f(a):\n    x = a\n    return x + 1\n"},
    {"name": "comments-blank-lines-reformat", "same": True, "before": "def f(a, b=1):\n    return a\n", "after": "# note\n\n\ndef f(\n    a,\n    b=1,\n):  # trailing\n    return a\n"},
    {"name": "docstring-edit", "same": True, "before": "def f():\n    '''one'''\n    return 1\n", "after": "def f():\n    '''two'''\n    return 1\n"},
    {"name": "private-value-edit", "same": True, "before": "_x = 1\nY: int = 2\n", "after": "_x = 99\nY: int = 2\n"},
    {"name": "annotated-module-value-edit", "same": True, "before": "X: int = 1\n", "after": "X: int = 2\n"},
    {"name": "local-import-added", "same": True, "before": "def f():\n    return 1\n", "after": "def f():\n    import json\n    return json.dumps(1)\n"},
    {"name": "import-added-changes-names", "same": False, "before": "def f():\n    return 1\n", "after": "import os\n\ndef f():\n    return os.getcwd() and 1\n"},
    {"name": "method-body-edit", "same": True, "before": "class C:\n    def m(self):\n        return 1\n", "after": "class C:\n    def m(self):\n        return 2\n"},
    {"name": "unannotated-return-ignored", "same": True, "before": "def f(a):\n    return 1\n", "after": "def f(a):\n    return 'x'\n"},
    {"name": "tuple-default-comma-is-significant", "same": False, "before": "def f(a=(1,)): pass\n", "after": "def f(a=(1)): pass\n"},
    {"name": "default-change", "same": False, "before": "def f(a=1): pass\n", "after": "def f(a=2): pass\n"},
    {"name": "parameter-rename", "same": False, "before": "def f(a): pass\n", "after": "def f(b): pass\n"},
    {"name": "return-annotation", "same": False, "before": "def f() -> int: return 1\n", "after": "def f() -> str: return 'x'\n"},
    {"name": "new-public-function", "same": False, "before": "def f(): pass\n", "after": "def f(): pass\ndef g(): pass\n"},
    {"name": "private-name-added-changes-names", "same": False, "before": "def f(): pass\n", "after": "def f(): pass\ndef _g(): pass\n"},
    {"name": "base-class-change", "same": False, "before": "class C(A): pass\n", "after": "class C(B): pass\n"},
    {"name": "decorator-change", "same": False, "before": "@a\ndef f(): pass\n", "after": "@b\ndef f(): pass\n"},
    {"name": "unannotated-public-value", "same": False, "before": "X = 1\n", "after": "X = 2\n"},
    {"name": "all-change", "same": False, "before": "__all__ = ['a']\na = 1\n_b = 2\n", "after": "__all__ = ['a', '_b']\na = 1\n_b = 2\n"},
    {"name": "class-field-default", "same": False, "before": "class C:\n    x: int = 1\n", "after": "class C:\n    x: int = 2\n"},
    {"name": "init-reexport-added", "same": False, "init": True, "before": "from .a import A\n", "after": "from .a import A\nfrom .b import B\n"},
    {"name": "final-value", "same": False, "before": "from typing import Final\nX: Final = 1\n", "after": "from typing import Final\nX: Final = 2\n"},
    {"name": "body-mode-unannotated", "same": False, "unannotatedReturns": "body", "before": "def f(a):\n    return 1\n", "after": "def f(a):\n    return 'x'\n"},
    {"name": "body-mode-annotated", "same": True, "unannotatedReturns": "body", "before": "def f(a) -> int:\n    return 1\n", "after": "def f(a) -> int:\n    return 2\n"},
]
PY["plan"] = [{
    "name": "workspace", "roots": ["", "src"], "pins": {"requests": "2.31.0"},
    "files": {
        "pkg/__init__.py": "", "pkg/a.py": "from . import b\nfrom pkg import b as bb\n", "pkg/b.py": "from .c import f\n", "pkg/c.py": "import os\nimport requests\nimport numpy\ndef f(): pass\n",
        "other.py": "from pkg.a import x\nimport pkg.b\n", "cyc1.py": "import cyc2\n", "cyc2.py": "import cyc1\n", "ns/mod.py": "import other\n", "src/lib/m.py": "from lib import n\n", "src/lib/n.py": "", "src/lib/n.pyi": "def k() -> int: ...\n", "🐍️.py": "import other\n",
        "node_modules/skip/x.py": "",
    },
    "expect": {
        "units": {
            "python:pkg": {"sources": ["pkg/__init__.py"], "dependencies": [], "externals": []},
            "python:pkg.a": {"sources": ["pkg/a.py"], "dependencies": ["python:pkg", "python:pkg.b"], "externals": []},
            "python:pkg.b": {"sources": ["pkg/b.py"], "dependencies": ["python:pkg.c"], "externals": []},
            "python:pkg.c": {"sources": ["pkg/c.py"], "dependencies": [], "externals": ["python-import:numpy@unpinned", "python-import:requests@2.31.0"]},
            "python:other": {"sources": ["other.py"], "dependencies": ["python:pkg.a", "python:pkg.b"], "externals": []},
            "python:cycle:cyc1": {"sources": ["cyc1.py", "cyc2.py"], "dependencies": [], "externals": []},
            "python:ns.mod": {"sources": ["ns/mod.py"], "dependencies": ["python:other"], "externals": []},
            "python:lib.m": {"sources": ["src/lib/m.py"], "dependencies": ["python:lib.n"], "externals": []},
            "python:lib.n": {"sources": ["src/lib/n.py", "src/lib/n.pyi"], "dependencies": [], "externals": []},
            "python:file:🐍️.py": {"sources": ["🐍️.py"], "dependencies": ["python:other"], "externals": []},
        },
    },
}]

# ---------------------------------------------------------------- go
GO = {}
GO["scan"] = [
    {"name": "block-with-aliases", "source": "// Package doc.\npackage p\n\nimport (\n\t\"fmt\"\n\t_ \"embed\"\n\tm \"math\"\n\t. \"strings\"\n\t`raw/path`\n)\n\nfunc F() {}\n", "expect": {"package": "p", "imports": ["embed", "fmt", "math", "raw/path", "strings"], "constraint": None, "embeds": []}},
    {"name": "constraint-and-cgo", "source": "//go:build linux && !cgo\n\n// Package q.\npackage q\n\nimport \"C\"\nimport \"unsafe\"\n\nvar x = 1\nimport \"never\"\n", "expect": {"package": "q", "imports": ["C", "unsafe"], "constraint": "linux && !cgo", "embeds": []}},
    {"name": "separate-and-semicolon", "source": "package s\nimport \"a\"\nimport \"b\"; import \"c\"\n/* block */ import (\n  \"d\" ; \"e\"\n)\n", "expect": {"package": "s", "imports": ["a", "b", "c", "d", "e"], "constraint": None, "embeds": []}},
    {"name": "embed-directives", "source": "package e\n\nimport \"embed\"\n\n//go:embed assets/* \"name with space.txt\" other.txt\nvar fs embed.FS\n", "expect": {"package": "e", "imports": ["embed"], "constraint": None, "embeds": ["assets/*", "name with space.txt", "other.txt"]}},
    {"name": "no-imports", "source": "package n\n\nfunc F() {}\nimport \"late\"\n", "expect": {"package": "n", "imports": [], "constraint": None, "embeds": []}},
    {"name": "string-with-import-word", "source": "package w\n\nimport \"fmt\"\n\nvar s = \"import \\\"x\\\"\"\n", "expect": {"package": "w", "imports": ["fmt"], "constraint": None, "embeds": []}},
]
LIN = {"goos": "linux", "goarch": "amd64", "goMinor": 25, "cgo": True, "tags": []}
DAR = {"goos": "darwin", "goarch": "arm64", "goMinor": 25, "cgo": False, "tags": ["extra"]}
IOS = {"goos": "ios", "goarch": "arm64", "goMinor": 25, "cgo": True, "tags": []}
AND = {"goos": "android", "goarch": "arm64", "goMinor": 25, "cgo": True, "tags": []}
GO["constraints"] = [
    {"expr": "linux && amd64", "context": LIN, "expect": True}, {"expr": "linux && amd64", "context": DAR, "expect": False}, {"expr": "!windows", "context": LIN, "expect": True},
    {"expr": "cgo", "context": LIN, "expect": True}, {"expr": "cgo", "context": DAR, "expect": False}, {"expr": "ignore", "context": LIN, "expect": False}, {"expr": "unix", "context": DAR, "expect": True},
    {"expr": "unix", "context": {**LIN, "goos": "windows"}, "expect": False}, {"expr": "go1.21", "context": LIN, "expect": True}, {"expr": "go1.30", "context": LIN, "expect": False},
    {"expr": "(linux || darwin) && !386", "context": DAR, "expect": True}, {"expr": "(linux || darwin) && !arm64", "context": DAR, "expect": False}, {"expr": "linux &&", "context": LIN, "expect": False},
    {"expr": "extra", "context": DAR, "expect": True}, {"expr": "extra", "context": LIN, "expect": False}, {"expr": "darwin", "context": IOS, "expect": True}, {"expr": "linux", "context": AND, "expect": True},
    {"expr": "gc && !gccgo", "context": LIN, "expect": True}, {"expr": "!(linux || darwin)", "context": LIN, "expect": False},
]
GO["names"] = [
    {"name": "x_linux.go", "context": LIN, "expect": True}, {"name": "x_linux.go", "context": DAR, "expect": False}, {"name": "x_linux_amd64.go", "context": LIN, "expect": True}, {"name": "x_linux_arm64.go", "context": LIN, "expect": False},
    {"name": "x_test.go", "context": LIN, "expect": True}, {"name": "linux.go", "context": DAR, "expect": True}, {"name": "x_amd64_linux.go", "context": LIN, "expect": True}, {"name": "x_unix.go", "context": DAR, "expect": True},
    {"name": "x_linux_test.go", "context": DAR, "expect": False}, {"name": "x_darwin_arm64_test.go", "context": DAR, "expect": True}, {"name": "x_arm64.go", "context": DAR, "expect": True}, {"name": "x_windows.go", "context": AND, "expect": False},
]
GO["gomod"] = [
    {"text": "module example.com/m // c\n\ngo 1.25\n\nrequire (\n\tgithub.com/a/b v1.2.3 // indirect\n\tgolang.org/x/y v0.1.0\n)\nrequire github.com/c/d v2.0.0+incompatible\nreplace github.com/a/b => ../b\nreplace (\n\tgolang.org/x/y v0.1.0 => example.com/fork/y v0.1.1\n)\n",
     "expect": {"module": "example.com/m", "go": "1.25", "requires": [{"path": "github.com/a/b", "version": "v1.2.3"}, {"path": "golang.org/x/y", "version": "v0.1.0"}, {"path": "github.com/c/d", "version": "v2.0.0+incompatible"}], "replaces": [{"from": "github.com/a/b", "to": "../b", "toVersion": None}, {"from": "golang.org/x/y", "to": "example.com/fork/y", "toVersion": "v0.1.1"}]}},
]
GO["work"] = [{"text": "go 1.25\n\nuse (\n\t./a\n\t./b/c\n)\nuse ./d\n", "expect": {"go": "1.25", "use": ["./a", "./b/c", "./d"]}}]
GO["plan"] = [{
    "name": "workspace", "context": LIN,
    "files": {
        "go.mod": "module example.com/lab\n\ngo 1.25\n\nrequire github.com/ext/lib v1.0.0\n",
        "a/a.go": "package a\n\nimport \"strings\"\n\nfunc A() string { return strings.ToUpper(\"a\") }\n",
        "a/a_test.go": "package a\n\nimport \"testing\"\n\nfunc TestA(t *testing.T) {}\n",
        "a/a_windows.go": "package a\n\nfunc W() {}\n",
        "a/tags.go": "//go:build ignore\n\npackage main\n",
        "a/testdata/fixture.txt": "data",
        "b/b.go": "package b\n\nimport (\n\t\"example.com/lab/a\"\n\t\"github.com/ext/lib/sub\"\n)\n\nfunc B() string { return a.A() + sub.S }\n",
        "b/b_x_test.go": "package b_test\n\nimport (\n\t\"testing\"\n\n\t\"example.com/lab/b\"\n)\n\nfunc TestB(t *testing.T) { _ = b.B() }\n",
        "cmd/tool/main.go": "package main\n\nimport \"example.com/lab/b\"\n\nfunc main() { _ = b.B() }\n",
        "inner/go.mod": "module example.com/inner\n\ngo 1.25\n", "inner/i.go": "package inner\n",
        "vendor/v/v.go": "package v\n", "_hidden/h.go": "package h\n",
    },
    "expect": {
        "packages": {"example.com/lab/a": {"name": "a", "goFiles": ["a/a.go"], "testGoFiles": ["a/a_test.go"], "imports": ["strings"]}, "example.com/lab/b": {"name": "b", "goFiles": ["b/b.go"], "xtestGoFiles": ["b/b_x_test.go"], "imports": ["example.com/lab/a", "github.com/ext/lib/sub"]}, "example.com/lab/cmd/tool": {"name": "main", "goFiles": ["cmd/tool/main.go"], "imports": ["example.com/lab/b"]}},
        "units": {
            "go:example.com/lab/a": {"kind": "library", "dependencies": [], "externals": [], "sources": ["a/a.go"]},
            "go:example.com/lab/a#test": {"kind": "test", "dependencies": [["go:example.com/lab/a", "implementation"]], "externals": [], "sources": ["a/a_test.go"], "inputs": ["a/testdata/fixture.txt"]},
            "go:example.com/lab/b": {"kind": "library", "dependencies": [["go:example.com/lab/a", "interface"]], "externals": ["go-module:github.com/ext/lib@v1.0.0"], "sources": ["b/b.go"]},
            "go:example.com/lab/b#test": {"kind": "test", "dependencies": [["go:example.com/lab/b", "implementation"]], "externals": [], "sources": ["b/b_x_test.go"]},
            "go:example.com/inner": {"kind": "library", "dependencies": [], "externals": [], "sources": ["inner/i.go"]},
            "go:example.com/lab/cmd/tool": {"kind": "binary", "dependencies": [["go:example.com/lab/b", "interface"]], "externals": [], "sources": ["cmd/tool/main.go"]},
        },
    },
}]

TS = {"plan": {
    "version": 1, "family": "tsconfig.json", "ambient": ["🌿️ambient/g.d.ts"], "manifests": ["package.json"], "order": ["dir:a", "dir:b", "dir:t"], "components": [], "digest": "x",
    "units": [
        {"id": "dir:a", "label": "a", "files": ["a/i.ts"], "json": ["a/c.json"], "dependencies": [], "leaf": False},
        {"id": "dir:b", "label": "b", "files": ["b/i.ts"], "json": [], "dependencies": ["dir:a"], "leaf": False},
        {"id": "dir:t", "label": "t", "files": ["b/🧪️tests/t.ts"], "json": [], "dependencies": ["dir:b", "dir:a"], "leaf": True},
    ],
}}

fixture = {"policy": {"interfaceKinds": KINDS}, "graphs": graphs, "keys": keys, "scenarios": scenarios, "names": names, "python": PY, "go": GO, "typescript": TS}
out = os.path.join(STAGED, "🧪️tests", "🔣️.json")
os.makedirs(os.path.dirname(out), exist_ok=True)
with open(out, "w", encoding="utf8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
print("wrote", out, "graphs", len(graphs), "keys", len(keys), "scenarios", len(scenarios))
