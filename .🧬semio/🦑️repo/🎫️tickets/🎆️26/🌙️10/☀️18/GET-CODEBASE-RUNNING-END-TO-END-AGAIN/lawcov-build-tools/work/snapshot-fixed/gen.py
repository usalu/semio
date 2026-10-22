"""📝 Authoring aid: writes the authorizer snapshot corpus (corpus.json): faults injected into a valid snapshot, and a small workspace whose import closure is captured."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def s(path, text="x\n", **extra):
    return {"path": path, "content": {"text": text}, **extra}


def big(path, size):
    return {"path": path, "content": {"repeat": "ab", "times": size // 2}}


MANIFEST = "Original captured authorizer manifest required"
PATH = "Original captured authorizer path required"
CLOSED = "Closed captured authorizer bytes required"
c = {"version": 1}
c["snapshots"] = [
    {"name": "three files in byte order", "sources": [s("A.ts"), s("a.ts", "import './b.ts'\n"), s("é.ts")], "entry": "a.ts", "patch": {}, "error": ""},
    {"name": "a single entry file", "sources": [s("a.ts")], "entry": "a.ts", "patch": {}, "error": ""},
    {"name": "a file of three megabytes", "sources": [big("a.ts", 3000000)], "entry": "a.ts", "patch": {}, "error": ""},
    {"name": "a file of exactly the 4 MiB file maximum", "sources": [big("a.ts", 4194304)], "entry": "a.ts", "patch": {}, "error": ""},
    {"name": "a file of one byte above the file maximum is refused by its byte identity", "sources": [big("a.ts", 4194306)], "entry": "a.ts", "patch": {}, "error": "Original captured authorizer byte identity required"},
    {"name": "a manifest that is not in byte order is refused", "sources": [s("b.ts"), s("a.ts")], "entry": "a.ts", "patch": {}, "error": "Canonical captured authorizer order required"},
    {"name": "a repeated path is refused", "sources": [s("a.ts"), s("a.ts")], "entry": "a.ts", "patch": {}, "error": PATH},
    {"name": "a path leaving the workspace is refused", "sources": [s("../x.ts"), s("z.ts")], "entry": "z.ts", "patch": {}, "error": PATH},
    {"name": "an absolute path is refused", "sources": [s("/etc/x"), s("z.ts")], "entry": "z.ts", "patch": {}, "error": PATH},
    {"name": "a backslash is refused", "sources": [s("a\\b.ts"), s("z.ts")], "entry": "z.ts", "patch": {}, "error": PATH},
    {"name": "a colon is refused", "sources": [s("a:b.ts"), s("z.ts")], "entry": "z.ts", "patch": {}, "error": PATH},
    {"name": "an empty segment is refused", "sources": [s("a//b.ts"), s("z.ts")], "entry": "z.ts", "patch": {}, "error": PATH},
    {"name": "a dot segment is refused", "sources": [s("a/./b.ts"), s("z.ts")], "entry": "z.ts", "patch": {}, "error": PATH},
    {"name": "bytes that do not match their digest are refused", "sources": [s("a.ts", "x\n", sha256="0" * 64)], "entry": "a.ts", "patch": {}, "error": "Original captured authorizer byte identity required"},
        {"name": "five files of three and a half megabytes exceed the source maximum", "sources": [big("a.ts", 3500000), big("b.ts", 3500000), big("c.ts", 3500000), big("d.ts", 3500000), big("e.ts", 3500000)], "entry": "a.ts", "patch": {}, "error": "Captured authorizer byte capacity exceeded"},
    {"name": "base64 without its padding is not canonical", "sources": [s("a.ts", "a", base64="YQ")], "entry": "a.ts", "patch": {}, "error": "Original captured authorizer byte identity required"},
    {"name": "base64 with a character outside the alphabet is not closed", "sources": [s("a.ts", "a", base64="YQ?=")], "entry": "a.ts", "patch": {}, "error": CLOSED},
    {"name": "base64 with padding in the middle is not closed", "sources": [s("a.ts", "ab", base64="YQ==YQ==")], "entry": "a.ts", "patch": {}, "error": CLOSED},
    {"name": "an entry that is not captured is refused", "sources": [s("a.ts")], "entry": "nope.ts", "patch": {}, "error": "Captured authorizer snapshot capacity exceeded"},
    {"name": "a wrong digest is refused", "sources": [s("a.ts")], "entry": "a.ts", "patch": {"digest": "1" * 64}, "error": MANIFEST},
    {"name": "an additional field is not closed", "sources": [s("a.ts")], "entry": "a.ts", "patch": {"extra": 1}, "error": CLOSED},
    {"name": "another version is not closed", "sources": [s("a.ts")], "entry": "a.ts", "patch": {"version": 2}, "error": CLOSED},
    {"name": "a missing file is refused", "sources": [s("a.ts"), s("b.ts")], "entry": "a.ts", "patch": {"dropFile": 1}, "error": MANIFEST},
    {"name": "an extra file is refused", "sources": [s("a.ts")], "entry": "a.ts", "patch": {"addFile": "b.ts"}, "error": MANIFEST},
    {"name": "a manifest that is not for release is refused", "sources": [s("a.ts")], "entry": "a.ts", "manifest": {"mode": "debug"}, "patch": {}, "error": MANIFEST},
    {"name": "a resolution without a captured workspace manifest is refused", "sources": [s("a.ts"), s("y.ts")], "entry": "a.ts", "manifest": {"resolutions": [{"specifier": "@semio-tech/y", "path": "y.ts"}]}, "patch": {}, "error": "Captured root workspace manifest required"},
]
c["workspace"] = {
    "files": [s("a.ts", "import fs from 'node:fs';\nimport {b} from './b.ts';\nimport data from './c.json';\nexport const a=[fs,b,data];\n"), s("b.ts", "import {d} from './sub/d.ts';\nexport const b=d;\n"), s("c.json", "{\"k\":1}\n"), s("sub/d.ts", "export const d=1;\n"), s("unused.ts", "export const u=2;\n")],
    "entry": "a.ts",
    "closure": ["a.ts", "b.ts", "c.json", "sub/d.ts"],
}
c["roles"] = {"valid": ["module", "program"], "refused": [None, "auto", "bun", True, {}, 0], "targets": {"valid": ["node", "bun"], "refused": ["deno", "", None]}}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
