"""📝 Authoring aid: writes the unit cache key corpus (corpus.json): hand-written rustc command lines, seeded pseudo-random ones, lane texts, environments, dependency files, build script output, key inputs and response files."""
import json, os, random

W = os.path.dirname(os.path.abspath(__file__)) + "/"
rng = random.Random(20261021)
pools = {
    "name": [[], ["--crate-name", "foo"], ["--crate-name=bar"], ["--crate-name", ""]],
    "type": [[], ["--crate-type", "lib"], ["--crate-type=rlib"], ["--crate-type", "bin"], ["--crate-type", "cdylib", "--crate-type", "rlib"], ["--crate-type", "lib,proc-macro"], ["--crate-type", "staticlib"], ["--crate-type", "wasm"], ["--crate-type=lib,bin"]],
    "emit": [[], ["--emit", "link"], ["--emit=metadata"], ["--emit", "dep-info,metadata,link"], ["--emit=dep-info"], ["--emit", "link=out.rlib,metadata"], ["--emit", "asm"]],
    "out": [[], ["--out-dir", "/o/deps"], ["--out-dir=rel/deps"], ["-o", "/o/x"]],
    "target": [[], ["--target", "aarch64-apple-darwin"], ["--target=x86_64-unknown-linux-gnu"]],
    "extern": [[], ["--extern", "dep=/o/deps/libdep-1.rlib"], ["--extern=noprelude:dep=/o/deps/libdep-2.rlib"], ["--extern", "bare"], ["--extern", "a=/o/deps/liba.rlib", "--extern", "b=/o/deps/libb.rlib"], ["--extern", "priv:p=/o/deps/libp.rlib"]],
    "search": [[], ["-L", "dependency=/o/deps"], ["-Ldependency=/o/deps"], ["-L", "native=/n"], ["-Lall=/n"], ["-L", "/plain"], ["-L", "dependency=/o/deps", "-L", "native=/n"]],
    "link": [[], ["-l", "z"], ["-lstatic=foo"], ["-l", "dylib=m"]],
    "codegen": [[], ["-C", "incremental=/o/incr"], ["-Cincremental=/o/incr"], ["-C", "opt-level=3"], ["-Copt-level=2", "-C", "extra-filename=-abc"], ["-C", "extra-filename=-def", "-C", "incremental=/o/i"]],
    "json": [[], ["--json=diagnostic-rendered-ansi,artifacts,future-incompat"], ["--json", "artifacts"], ["--json=diagnostic-short"], ["--error-format=json", "--json=artifacts"]],
    "misc": [[], ["--edition", "2021"], ["--cfg", "feature=\"x\""], ["--check-cfg=cfg(a)"], ["-D", "warnings"], ["-Awarnings"], ["--cap-lints", "allow"], ["-Zunstable-options"], ["--remap-path-prefix=/a=/b"], ["-O"], ["-g"]],
    "source": [["src/lib.rs"], ["/abs/main.rs"], ["-"], [], ["notrust.c"], ["build.rs"]],
    "query": [[], [], [], [], ["--print", "cfg"], ["--print=sysroot"], ["-vV"], ["--version"], ["-V"], ["--explain", "E0001"]],
}
vectors = [
    ["--crate-name", "foo", "--edition=2021", "src/lib.rs", "--crate-type", "lib", "--emit=dep-info,metadata,link", "--out-dir", "/o/deps", "--json=artifacts", "-C", "incremental=/o/incr", "-L", "dependency=/o/deps", "--extern", "dep=/o/deps/libdep-1.rlib"],
    ["--crate-name", "bin", "src/main.rs", "--crate-type", "bin", "--emit=link", "--out-dir=/o", "--json=artifacts", "-l", "z"],
    ["--crate-name", "bin", "src/main.rs", "--crate-type", "bin", "--emit=link", "--out-dir=/o", "--json=artifacts", "-Lnative=/n"],
    ["--crate-name", "pm", "src/lib.rs", "--crate-type", "proc-macro", "--emit=link", "--out-dir=/o", "--json=artifacts", "-l", "z"],
    ["--crate-name", "lib", "src/lib.rs", "--crate-type", "lib", "--emit=link", "--out-dir=/o", "--json=artifacts", "-l", "z", "-Lnative=/n"],
    ["--crate-name", "w", "src/lib.rs", "--crate-type", "wasm", "--emit=link", "--out-dir=/o", "--json=artifacts"],
    ["--crate-name", "w", "src/lib.rs", "--crate-type", "lib", "--crate-type", "wasm", "--emit=metadata", "--out-dir=/o", "--json=artifacts"],
]
for _ in range(360):
    row = []
    for key in ("name", "type", "emit", "out", "target", "extern", "search", "link", "codegen", "json", "misc"):
        if key in ("name", "out", "emit", "json") or rng.random() < 0.55:
            row += rng.choice(pools[key])
    row += rng.choice(pools["query"])
    source = rng.choice(pools["source"])
    row = row + source if rng.random() < 0.5 else source + row
    vectors.append(row)
for _ in range(60):
    vectors.append(["--crate-name", "foo", "--crate-type", "lib", "--emit=link", "--out-dir=/o", "--json=artifacts", "src/lib.rs"] + rng.choice(pools["extern"]) + rng.choice(pools["search"]) + rng.choice(pools["codegen"]))
vectors += [["--extern"], ["-C"], ["-L"], ["--crate-name"], ["--emit"], ["-Cincremental"], ["--extern=a="], ["--extern", "x=y=z"], ["--cfg"], ["@file"], ["@"], ["-"], ["--out-dir"]]
c = {"version": 1, "cwd": "/work", "laneRoot": "/lane/root", "externDigests": {"/o/deps/libdep-1.rlib": "d1", "/o/deps/libdep-2.rlib": "d2", "/o/deps/liba.rlib": "da", "/o/deps/libp.rlib": "dp"}, "vectors": vectors}
c["lanes"] = [
    {"text": "path=/lane/root/a and /lane/root", "root": "/lane/root"},
    {"text": "path=C:\\lane\\root\\a, C:/lane/root/a and C:\\\\lane\\\\root\\\\a", "root": "C:\\lane\\root"},
    {"text": "/lane/root/sub/lane/root", "root": "/lane/root"},
    {"text": "nothing here", "root": "/lane/root"},
    {"text": "/lane/root-longer /lane/root", "root": "/lane/root"},
    {"text": "@@semio-lane@@/x", "root": "/lane/root"},
    {"text": "/lane/rootx", "root": None}, {"text": "/a/a/a", "root": "/a"}, {"text": "", "root": "/lane/root"},
]
c["environments"] = [
    {"CARGO_PKG_NAME": "foo", "CARGO_PKG_VERSION": "1.0.0", "CARGO_MANIFEST_DIR": "/lane/root/foo", "PATH": "/bin", "RUSTC_BOOTSTRAP": "1", "SDKROOT": "/sdk", "CARGO_PRIMARY_PACKAGE": "1", "OUT_DIR": "/lane/root/out"},
    {"CARGO_CRATE_NAME": "foo", "CARGO_BIN_NAME": "foo", "HOME": "/h", "CARGO_PKG_AUTHORS": "a:b", "MACOSX_DEPLOYMENT_TARGET": "11.0"},
    {"TARGET": "t", "HOST": "h", "PROFILE": "release", "NUM_JOBS": "8", "CARGO_FEATURE_X": "1", "CARGO_CFG_TARGET_OS": "macos", "DEP_Z_ROOT": "/lane/root/z", "CARGO_MAKEFLAGS": "j8", "CARGO_ENCODED_RUSTFLAGS": "-Cx"},
    {}, {"CARGO_PKG_NAME": "é🧪", "SDKROOT": "/lane/root"},
]
c["depinfo"] = [
    "foo.d: src/lib.rs src/a.rs\n\nsrc/lib.rs:\nsrc/a.rs:\n# env-dep:A=1\n# env-dep:B\n# env-dep:C=line\\nbreak\\\\x\n",
    "out/foo.rmeta out/libfoo.rlib: /abs/dir\\ with\\ space/a.rs src/b.rs\r\n# env-dep:X=\r\n",
    "foo.d: a.rs\nbar.d: b.rs\n",
    "# only a comment\n\n   \n",
    "no rule here\n",
    "foo.d:\n",
    "foo.d: ",
    "a\\: b: c.rs\n",
    "dir/foo.d: c:\\x\\y.rs\n",
    "foo.d:  a.rs   b.rs \n",
    "# env-dep:K=a\\\\nb\\r\n",
]
c["directives"] = [
    "cargo:rerun-if-changed=build.rs\ncargo::rerun-if-env-changed=FOO\ncargo:rustc-link-lib=z\r\ncargo:rerun-if-changed=src/x.c\n",
    "", "cargo:rerun-if-changed=\ncargo:rerun-if-env-changed=\n", " cargo:rerun-if-changed=a\ncargo:rerun-if-changed = a\ncargo:warning=x\n", "cargo:::rerun-if-changed=a\ncargo::rerun-if-changed=a=b\n",
]
c["keys"] = [
    {"toolchain": "rustc 1.80\nhost: aarch64", "normalized": ["--crate-name", "foo"], "sourceSha256": "a" * 64, "externs": [{"name": "dep", "sha256": "b" * 64}], "environment": [["CARGO_PKG_NAME", "foo"]]},
    {"toolchain": "", "normalized": [], "sourceSha256": "", "externs": [], "environment": []},
    {"toolchain": "é🧪", "normalized": ["a\"b", "c\\d", "e\nf"], "sourceSha256": "c" * 64, "externs": [{"name": "x", "sha256": "d" * 64}, {"name": "y", "sha256": "e" * 64}], "environment": [["K", "v"], ["L", "w"]]},
]
c["hashes"] = ["", "abc", "é🧪", "a\u0000b", "x" * 100000]
c["responses"] = [
    {"name": "a response file expands to one argument per line", "files": {"args.rsp": "--crate-name\nfoo\n\n--crate-type=lib\nsrc/lib.rs\n"}, "args": ["@args.rsp", "--emit=link"]},
    {"name": "a response file with windows line ends", "files": {"w.rsp": "a\r\nb\r\n"}, "args": ["x", "@w.rsp"]},
    {"name": "a missing response file stays an argument", "files": {}, "args": ["@missing.rsp", "y"]},
    {"name": "a lone at sign stays an argument", "files": {}, "args": ["@", "@@"]},
    {"name": "response files are not expanded recursively", "files": {"o.rsp": "@inner.rsp\nz\n", "inner.rsp": "q\n"}, "args": ["@o.rsp"]},
    {"name": "an empty response file contributes nothing", "files": {"e.rsp": "\n\n"}, "args": ["a", "@e.rsp", "b"]},
]
c["admit"] = [
    {"args": ["--crate-name", "foo", "--crate-type", "lib", "--out-dir", "rel/o", "src/lib.rs", "--extern", "a=rel/liba.rlib", "--extern", "b", "-C", "extra-filename=-xyz"], "environment": {"CARGO_PKG_NAME": "foo", "CARGO_PKG_VERSION": "1.2.3", "CARGO_MANIFEST_DIR": "/m", "CARGO_PRIMARY_PACKAGE": "1", "OUT_DIR": "/out"}},
    {"args": ["--crate-name", "bar", "src/main.rs", "--target=t"], "environment": {}},
    {"args": ["--crate-name", "bar", "-C", "extra-filename=-one", "-Cextra-filename=-two", "src/x.rs"], "environment": {}},
    {"args": ["-vV"], "environment": {}}, {"args": ["--crate-name", "x", "--print", "cfg", "-"], "environment": {}}, {"args": ["src/x.rs"], "environment": {}},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False)
print(len(vectors))
