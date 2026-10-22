"""📝 Authoring aid: writes the native manifests corpus (corpus.json): Cargo path dependencies, Cargo workspace discovery and Go module files."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
D = lambda name, path, workspace=False, kind="dependencies": {"name": name, "path": path, "workspace": workspace, "kind": kind}
deps = [
    {"name": "a path dependency", "toml": '[dependencies]\nb = { path = "../b" }\n', "workspace": {}, "expected": [D("b", "../b")]},
    {"name": "a registry dependency has no path", "toml": '[dependencies]\nserde = "1"\nrand = { version = "0.8" }\n', "workspace": {}, "expected": []},
    {"name": "a renamed dependency reports the package name", "toml": '[dependencies]\nalias = { path = "../real", package = "real-crate" }\n', "workspace": {}, "expected": [D("real-crate", "../real")]},
    {"name": "build and dev dependencies are reported by kind in table order", "toml": '[dev-dependencies]\nd = { path = "../d" }\n[build-dependencies]\nb = { path = "../b" }\n[dependencies]\na = { path = "../a" }\n', "workspace": {}, "expected": [D("a", "../a"), D("b", "../b", False, "build-dependencies"), D("d", "../d", False, "dev-dependencies")]},
    {"name": "target tables are searched after the manifest tables", "toml": "[target.'cfg(unix)'.dependencies]\nu = { path = \"../u\" }\n[dependencies]\na = { path = \"../a\" }\n[target.'cfg(windows)'.dev-dependencies]\nw = { path = \"../w\" }\n", "workspace": {}, "expected": [D("a", "../a"), D("u", "../u"), D("w", "../w", False, "dev-dependencies")]},
    {"name": "an inherited dependency takes its path from the workspace", "toml": "[dependencies]\nx = { workspace = true }\n", "workspace": {"dependencies": {"x": {"path": "crates/x"}}}, "expected": [D("x", "crates/x", True)]},
    {"name": "an inherited dependency uses the workspace package name", "toml": "[dependencies]\nx = { workspace = true }\n", "workspace": {"dependencies": {"x": {"path": "crates/x", "package": "real-x"}}}, "expected": [D("real-x", "crates/x", True)]},
    {"name": "an inherited dependency without a workspace path is dropped", "toml": "[dependencies]\nx = { workspace = true }\ny = { workspace = true }\nz = { workspace = true }\n", "workspace": {"dependencies": {"x": "1.0", "y": {"version": "2"}}}, "expected": []},
    {"name": "a workspace flag that is not true is a plain declaration", "toml": '[dependencies]\nx = { workspace = false, path = "../x" }\n', "workspace": {"dependencies": {"x": {"path": "elsewhere"}}}, "expected": [D("x", "../x")]},
    {"name": "a dependency with an inline table and features", "toml": '[dependencies]\nx = { path = "../x", features = ["a", "b"], default-features = false, optional = true }\n', "workspace": {}, "expected": [D("x", "../x")]},
    {"name": "a dependency written as its own table", "toml": '[dependencies.x]\npath = "../x"\nversion = "0.1"\n', "workspace": {}, "expected": [D("x", "../x")]},
    {"name": "several dependencies keep declaration order", "toml": '[dependencies]\nz = { path = "../z" }\na = { path = "../a" }\nm = { path = "../m" }\n', "workspace": {}, "expected": [D("z", "../z"), D("a", "../a"), D("m", "../m")]},
    {"name": "a manifest without dependency tables", "toml": '[package]\nname = "p"\n', "workspace": {}, "expected": []},
    {"name": "an empty path is not a path dependency", "toml": '[dependencies]\nx = { path = "" }\n', "workspace": {}, "expected": []},
    {"name": "several target tables in order of appearance", "toml": "[target.'cfg(a)'.dependencies]\na = { path = \"../a\" }\n[target.'cfg(b)'.dependencies]\nb = { path = \"../b\" }\n[target.'cfg(a)'.build-dependencies]\nc = { path = \"../c\" }\n", "workspace": {}, "expected": [D("a", "../a"), D("c", "../c", False, "build-dependencies"), D("b", "../b")]},
]
W1 = '[workspace]\nmembers = ["a"]\n'
P = '[package]\nname = "p"\n'
wk = lambda name, files, directory, expected, error=None, **kw: {"name": name, "files": files, "directory": directory, **({"error": error} if error else {"root": expected[0], "declaration": expected[1]}), **kw}
workspaces = [
    wk("a crate that is itself a workspace root", {"c/Cargo.toml": '[package]\nname = "c"\n[workspace]\nmembers = ["x"]\n'}, "c", ("c", {"members": ["x"]})),
    wk("the nearest ancestor workspace", {"Cargo.toml": W1, "crates/a/Cargo.toml": P}, "crates/a", (".", {"members": ["a"]})),
    wk("a nested workspace wins over the outer one", {"Cargo.toml": W1, "crates/Cargo.toml": '[workspace]\nmembers = ["inner"]\n', "crates/inner/Cargo.toml": P}, "crates/inner", ("crates", {"members": ["inner"]})),
    wk("ancestors without a workspace table are skipped", {"Cargo.toml": W1, "crates/Cargo.toml": P, "crates/a/Cargo.toml": P}, "crates/a", (".", {"members": ["a"]})),
    wk("no workspace anywhere ends at the repository root", {"crates/a/Cargo.toml": P}, "crates/a", (".", {})),
    wk("an explicit workspace key points at its root", {"Cargo.toml": W1, "other/ws/Cargo.toml": '[workspace]\nmembers = ["m"]\n', "crates/a/Cargo.toml": '[package]\nname = "a"\nworkspace = "../../other/ws"\n'}, "crates/a", ("other/ws", {"members": ["m"]})),
    wk("an explicit workspace key to a directory without a workspace walks up from there", {"Cargo.toml": W1, "other/deep/Cargo.toml": P, "crates/a/Cargo.toml": '[package]\nname = "a"\nworkspace = "../../other/deep"\n'}, "crates/a", (".", {"members": ["a"]})),
    wk("a workspace key that leaves the repository is refused", {"crates/a/Cargo.toml": '[package]\nname = "a"\nworkspace = "../../../outside"\n'}, "crates/a", None, "Native workspace escapes repository: {directory}"),
    wk("a workspace key to the parent of the repository is refused", {"Cargo.toml": W1, "a/Cargo.toml": '[package]\nname = "a"\nworkspace = "../.."\n'}, "a", None, "Native workspace escapes repository: {directory}"),
    wk("the repository root itself needs no manifest above it", {"Cargo.toml": P}, ".", (".", {})),
    wk("a declaration carries the whole workspace table", {"Cargo.toml": '[workspace]\nmembers = ["a", "b/*"]\nexclude = ["c"]\nresolver = "2"\n[workspace.dependencies]\nx = { path = "crates/x" }\n', "a/Cargo.toml": P}, "a", (".", {"members": ["a", "b/*"], "exclude": ["c"], "resolver": "2", "dependencies": {"x": {"path": "crates/x"}}})),
    wk("a crate without a manifest cannot be read", {"Cargo.toml": W1}, "missing", None, "ENOENT"),
    wk("a workspace key with a trailing slash resolves like the plain path", {"ws/Cargo.toml": W1, "ws/a/Cargo.toml": '[package]\nname = "a"\nworkspace = "../"\n'}, "ws/a", ("ws", {"members": ["a"]})),
]
GO = lambda text, module, requires, replacements, **kw: {"text": text, "module": module, "requires": requires, "replacements": replacements, **kw}
go = [
    {"name": "a module with single and block requirements and replacements", **GO('module example.com/m\n\ngo 1.22\n\nrequire example.com/a v1.0.0\nrequire (\n\texample.com/b v1.2.3\n\texample.com/c v0.1.0 // indirect\n)\n\nreplace example.com/a => ../a\nreplace (\n\texample.com/b v1.2.3 => ./b\n\texample.com/c => example.com/fork v1.0.0\n)\n',
        "example.com/m", ["example.com/a", "example.com/b", "example.com/c"], [{"module": "example.com/a", "path": "../a"}, {"module": "example.com/b", "path": "./b"}, {"module": "example.com/c", "path": "example.com/fork"}])},
    {"name": "a quoted module path", **GO('module "example.com/q"\n\ngo 1.21\n', "example.com/q", [], [])},
    {"name": "comments and blank lines", **GO('// header\nmodule example.com/m // trailing\n\n// note\nrequire example.com/a v1.0.0 // indirect\n\nrequire (\n\t// inside\n\texample.com/b v1.0.0\n\n)\n', "example.com/m", ["example.com/a", "example.com/b"], [])},
    {"name": "a replacement with versions on both sides", **GO("module m\n\nrequire example.com/a v1.0.0\nreplace example.com/a v1.0.0 => example.com/f v2.0.0\n", "m", ["example.com/a"], [{"module": "example.com/a", "path": "example.com/f"}])},
    {"name": "exclude, retract and toolchain lines are ignored", **GO("module m\n\ngo 1.22.0\ntoolchain go1.22.3\n\nexclude example.com/z v1.0.0\nexclude (\n\texample.com/y v1.0.0\n)\nretract v1.0.0\nretract (\n\tv1.1.0\n\t[v1.2.0, v1.3.0]\n)\nrequire example.com/a v1.0.0\n", "m", ["example.com/a"], [])},
    {"name": "Windows line ends", **GO('module example.com/m\r\n\r\nrequire (\r\n\texample.com/a v1.0.0\r\n)\r\nreplace example.com/a => ../a\r\n', "example.com/m", ["example.com/a"], [{"module": "example.com/a", "path": "../a"}])},
    {"name": "a replacement path with a comment", **GO("module m\nreplace example.com/a => ./a // local\n", "m", [], [{"module": "example.com/a", "path": "./a"}])},
    {"name": "quoted requirements and replacements", **GO('module m\nrequire "example.com/a" v1.0.0\nreplace "example.com/a" => "../a b"\n', "m", ["example.com/a"], [{"module": "example.com/a", "path": "../a b"}])},
    {"name": "tools and godebug lines are ignored", **GO("module m\ngo 1.24\ngodebug default=go1.21\ntool example.com/cmd/x\nrequire example.com/a v1.0.0\n", "m", ["example.com/a"], [])},
    {"name": "a file without a module line", **GO("go 1.22\nrequire example.com/a v1.0.0\n", "", ["example.com/a"], [], noGo=True)},
    {"name": "an empty file", **GO("", "", [], [], noGo=True)},
]
json.dump({"version": 1, "workspaceRoot": "ws", "dependencies": deps, "workspaces": workspaces, "go": go}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(deps), len(workspaces), len(go))
