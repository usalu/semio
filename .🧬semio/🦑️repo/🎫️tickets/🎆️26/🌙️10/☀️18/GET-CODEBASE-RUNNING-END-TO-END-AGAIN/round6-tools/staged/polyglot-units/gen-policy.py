#!/usr/bin/env python3
"""🧰️ Writes staged/🧩️units/🔣️.json (the policy document) from data: interface kinds, go build lists, the interpreter's top-level module names. Run with the interpreter whose stdlib is described (3.9 or newer; `sys.stdlib_module_names` is used when present)."""
import json, os, sys, sysconfig
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "staged", "🧩️units", "🔣️.json")

def stdlib():
    names = set(sys.builtin_module_names)
    if hasattr(sys, "stdlib_module_names"):
        names |= set(sys.stdlib_module_names)
    root = sysconfig.get_paths()["stdlib"]
    for entry in os.listdir(root):
        if entry in ("site-packages", "__pycache__", "config-" + sysconfig.get_python_version() + "-darwin"):
            continue
        path = os.path.join(root, entry)
        if os.path.isdir(path) and os.path.exists(os.path.join(path, "__init__.py")):
            names.add(entry)
        elif entry.endswith(".py"):
            names.add(entry[:-3])
    dynload = os.path.join(root, "lib-dynload")
    if os.path.isdir(dynload):
        for entry in os.listdir(dynload):
            names.add(entry.split(".")[0])
    names |= {"__future__", "__main__", "typing_extensions_placeholder"}
    names.discard("typing_extensions_placeholder")
    return sorted(n for n in names if n.isidentifier())

KEY_TEXT = ("key = hex(SHA-256(UTF-8(canonical JSON of {v:1, id, language, kind, toolchain:{id,parts}, action:{argv,env}, interface:(kind or null), "
            "outputs (sorted), sources:[[path,digest]] sorted by path, inputs:[[path,digest]] sorted by path, externals sorted, "
            "dependencies:[[unit,use,digest]] sorted by unit}))). The dependency digest is the interface closure of the dependency for use `interface`, "
            "its output closure for `implementation`, `<interface closure>|<output closure>` for `both`, the empty string for `declared`. A path digest is the SHA-256 of the file bytes, `absent` for a missing file. "
            "interfaceClosure(u) = SHA-256 of canonical JSON {i: interface digest of u, d: [[dependency id, its interfaceClosure] for every `interface` dependency the executor reported as exposed by the interface (all of them when it reported none), sorted by id]}, null when u has no interface. "
            "outputClosure(u) = SHA-256 of canonical JSON {o: outputs digest of u, d: [[dependency id, its outputClosure] for every `interface` or `implementation` dependency, sorted by id]}. "
            "Canonical JSON: object keys sorted by code point, no whitespace, strings exactly as JSON.stringify writes them. Every sorted list is sorted by code point (UTF-8 byte order).")

PY_API = ("The interface of a module is SHA-256 over the lines `polyglot-python-api-v1` and then, in source order, one line `<depth>|<tokens>` per structural statement, "
          "where tokens are the lexical tokens (comments, blank lines, docstrings and formatting dropped) joined by one space. Structural statements: the header of every def, class, "
          "if, elif, else, try, except, finally, with, for and while at module or class level (function bodies are skipped; class and compound bodies are walked, depth is the nesting), "
          "decorators of public defs and classes, public names only (a name is public unless it starts with a single underscore; dunder names are public; `__all__` entries are public). "
          "Imports count when they bind a public name through `import x as x`, `from m import n as n`, `from m import *`, appear in `__all__`, or the module is a package `__init__`. "
          "Assignments to public names count: annotated `NAME: ann` by annotation only (with the value for `Final` and for class-level annotated fields), unannotated `NAME = value` with the value tokens. "
          "`__all__` statements count whole. A final line `names|<sorted names bound at structural level, private included, plus * when the module cannot enumerate them>` lets importers check `from m import n`. "
          "A sibling `.pyi` replaces the module's source for the interface.")

doc = {
    "version": 1,
    "languages": ["typescript", "rust", "go", "python", "dotnet", "cpp", "wit", "tex"],
    "interfaceKinds": {
        "typescript-declarations": {"language": "typescript", "artifact": "declaration files (.d.ts) emitted for the unit", "digest": "SHA-256 over sorted rows of path and SHA-256 of bytes of the declaration tree", "closure": "transitive", "bazel": "artifact"},
        "rust-rmeta": {"language": "rust", "artifact": "crate metadata (--emit=metadata, .rmeta)", "digest": "SHA-256 of the .rmeta file; a body edit changes it (spans and inlinable bodies are inside), so this is a conservative API digest", "closure": "transitive", "bazel": "artifact"},
        "go-export-data": {"language": "go", "artifact": "export data member __.PKGDEF of the compiled package archive", "digest": "SHA-256 of __.PKGDEF without its `build id` line (the `go tool compile -o` export archive of a pure-Go package has none)", "closure": "transitive", "bazel": "artifact"},
        "python-api": {"language": "python", "artifact": "API text extracted from the module sources (or the sibling .pyi stub)", "digest": "SHA-256 of the API text (see python.api)", "closure": "transitive", "bazel": "artifact"},
        "dotnet-reference-assembly": {"language": "dotnet", "artifact": "reference assembly (metadata-only dll from /refout, deterministic build)", "digest": "SHA-256 of the reference assembly bytes", "closure": "transitive", "bazel": "sources"},
        "cpp-public-headers": {"language": "cpp", "artifact": "the declared public header set of the unit", "digest": "SHA-256 over sorted rows of path and SHA-256 of the comment- and whitespace-normalized text of the public headers", "closure": "transitive", "bazel": "artifact"},
        "wit-world": {"language": "wit", "artifact": "canonical text of the WIT package and world", "digest": "SHA-256 of the canonical text of the package: files merged, comments dropped, items of packages, interfaces and worlds sorted by text (order inside records, variants, enums, flags and signatures kept)", "closure": "transitive", "bazel": "artifact"},
        "tex-aux": {"language": "tex", "artifact": "auxiliary reference files of the final pass (.aux, .toc)", "digest": "SHA-256 of the auxiliary file reduced to sorted newlabel and bibcite lines, ordered toc/lof/lot writes and bibstyle/bibdata", "closure": "direct", "bazel": "artifact"},
    },
    "key": {"version": 1, "order": "code-point", "text": KEY_TEXT},
    "go": {
        "goos": sorted("aix android darwin dragonfly freebsd hurd illumos ios js linux nacl netbsd openbsd plan9 solaris wasip1 windows zos".split()),
        "goarch": sorted("386 amd64 amd64p32 arm armbe arm64 arm64be loong64 mips mipsle mips64 mips64le mips64p32 mips64p32le ppc ppc64 ppc64le riscv riscv64 s390 s390x sparc sparc64 wasm".split()),
        "unix": sorted("aix android darwin dragonfly freebsd hurd illumos ios linux netbsd openbsd solaris".split()),
        "auxiliaryExtensions": [".c", ".cc", ".cpp", ".cxx", ".h", ".hh", ".hpp", ".hxx", ".m", ".mm", ".s", ".S", ".syso"],
        "skipDirectories": ["testdata", "vendor", "node_modules"],
        "cgoDefault": True,
    },
    "python": {
        "stdlib": stdlib(),
        "skipDirectories": ["__pycache__", "node_modules", "site-packages", ".venv", "venv", "dist", "build"],
        "unannotatedReturns": "ignore",
        "api": PY_API,
    },
    "rust": {"skipDirectories": ["target", "node_modules"], "sourceExtensions": [".rs", ".toml", ".lock"]},
    "messages": {
        "languages": ["en", "de"],
        "texts": {
            "planning": {"en": "Planning {units} compilation units", "de": "Plane {units} Kompilationseinheiten"},
            "running": {"en": "Running {unit}", "de": "Führe {unit} aus"},
            "hit": {"en": "{unit}: cached", "de": "{unit}: aus dem Cache"},
            "ran": {"en": "{unit}: built in {milliseconds} ms", "de": "{unit}: gebaut in {milliseconds} ms"},
            "cutoff": {"en": "{unit}: rebuilt, interface unchanged (dependents keep their results)", "de": "{unit}: neu gebaut, Schnittstelle unverändert (Abhängige behalten ihre Ergebnisse)"},
            "failed": {"en": "{unit}: failed", "de": "{unit}: fehlgeschlagen"},
            "blocked": {"en": "{unit}: skipped, dependency {dependency} failed", "de": "{unit}: übersprungen, Abhängigkeit {dependency} fehlgeschlagen"},
            "done": {"en": "{ran} built, {hit} cached, {cutoff} cut off, {failed} failed", "de": "{ran} gebaut, {hit} aus dem Cache, {cutoff} abgeschnitten, {failed} fehlgeschlagen"},
            "cancelled": {"en": "Cancelled", "de": "Abgebrochen"},
        },
    },
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w", encoding="utf8") as handle:
    json.dump(doc, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
print("wrote", OUT, "stdlib names:", len(doc["python"]["stdlib"]))
