#!/usr/bin/env python3
"""Lays out one sample project per tool family inside the scratch overlay and writes the build graph that selects them.

Each project runs the family's real tool; the print sample is a stand-in (no TeX engine is installed on this host) and the wasm sample transpiles a hand-assembled empty component with the real jco.
usage: fixtures.py [--mutate <file under fx> [text with \\n escapes]]   (default: lay out; --mutate appends a line to a declared input to force exactly one re-run)"""
import json, os, sys

OV = "/private/tmp/bzlcov-ov"
FX = os.path.join(OV, "fx")
NM = "/Users/ueli/Documents/semio/node_modules/.bin"
CHROMIUM = os.path.expanduser("~/Library/Caches/ms-playwright/chromium_headless_shell-1234/chrome-headless-shell-mac-arm64/chrome-headless-shell")
PW = "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs"
E = lambda c: chr(c) + "\ufe0f"
PY_DIR = E(0x1F40D) + "python"
GO_DIR = E(0x1F439) + "go"
PRINT_DIR = E(0x1F4D3) + "print"
W = {
    "bun/gen.ts": 'import {readFileSync,writeFileSync,mkdirSync} from "node:fs";\nconst input=JSON.parse(readFileSync("input.json","utf8"));\nmkdirSync("out",{recursive:true});\nwriteFileSync("out/gen.json",JSON.stringify({squares:input.values.map((v:number)=>v*v)}));\nconsole.log("generated",input.values.length);\n',
    "bun/input.json": '{"values":[1,2,3,4]}\n',
    "bun/publish.ts": 'import {appendFileSync} from "node:fs";\nappendFileSync("/private/tmp/bzlcov/publish.log",new Date().toISOString()+"\\n");\nconsole.log("published");\n',
    "tsc/tsconfig.json": '{"compilerOptions":{"strict":true,"noEmit":true,"target":"ES2022","module":"ES2022","moduleResolution":"bundler","types":[]},"include":["*.ts"]}\n',
    "tsc/a.ts": 'export const add=(a:number,b:number):number=>a+b;\nexport const value:number=add(1,2);\n',
    "vitest/a.test.js": 'it("adds",()=>{expect(1+2).toBe(3);});\nit("strings",()=>{expect("a"+"b").toBe("ab");});\n',
    "buntest/a.test.ts": 'import {expect,test} from "bun:test";\ntest("adds",()=>{expect(1+2).toBe(3);});\n',
    "buntest/fail.test.ts": 'import {expect,test} from "bun:test";\ntest("fails",()=>{expect(1).toBe(2);});\n',
    "vite/index.html": '<!doctype html><html><body><div id="app"></div><script type="module" src="/main.js"></script></body></html>\n',
    "vite/main.js": 'import {greet} from "./lib.js";\ndocument.getElementById("app").textContent=greet("bazel");\n',
    "vite/lib.js": 'export const greet=name=>"hello "+name;\n',
    "pw/e2e.mjs": 'import {chromium} from "' + PW + '";\nconst browser=await chromium.launch({headless:true,executablePath:process.env.BZLCOV_CHROMIUM});\nconst page=await browser.newPage();\nawait page.setContent("<html><body><h1 id=t>hello</h1></body></html>");\nconst text=await page.textContent("#t");\nawait browser.close();\nif(text!=="hello")throw new Error("unexpected "+text);\nconsole.log("playwright-ok",text);\n',
    GO_DIR + "/go.mod": "module fx/hello\n\ngo 1.25\n",
    GO_DIR + "/hello.go": 'package main\n\nimport "fmt"\n\nfunc Hello() string { return "hello" }\n\nfunc main() { fmt.Println(Hello()) }\n',
    GO_DIR + "/hello_test.go": 'package main\n\nimport "testing"\n\nfunc TestHello(t *testing.T) {\n\tif Hello() != "hello" {\n\t\tt.Fatal("unexpected")\n\t}\n}\n',
    PY_DIR + "/gen.py": 'import json,os\nos.makedirs("out",exist_ok=True)\nopen("out/py.json","w").write(json.dumps({"sum":sum(range(10))}))\nprint("py-generated")\n',
    PY_DIR + "/test_gen.py": 'import unittest\n\nclass T(unittest.TestCase):\n    def test_sum(self):\n        self.assertEqual(sum(range(10)),45)\n\nunittest.main()\n',
    PRINT_DIR + "/mkpdf.py": 'import os\nos.makedirs("out",exist_ok=True)\nbody=b"%PDF-1.1\\n1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\\n2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\\n3 0 obj<</Type/Page/Parent 2 0 R/MediaBox[0 0 72 72]>>endobj\\ntrailer<</Root 1 0 R>>\\n%%EOF\\n"\nopen("out/doc.pdf","wb").write(body)\nprint("stand-in pdf written")\n',
    PRINT_DIR + "/doc.tex": "\\documentclass{article}\\begin{document}stand-in\\end{document}\n",
    "wasm/empty.wasm": None,
    "dev/serve.ts": 'console.log("[bazel] task-started fx-dev:dev");\nsetInterval(()=>{},1000);\n',
}


def lay_out():
    for rel, text in W.items():
        path = os.path.join(FX, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        if text is None:
            open(path, "wb").write(b"\x00asm\x0d\x00\x01\x00")
        else:
            open(path, "w", encoding="utf8").write(text)


def target(command, root, outputs, cache=True, env=None, runtime=None, depends=None, continuous=False):
    inputs = ["{projectRoot}/**/*", "!{projectRoot}/out/**", "!{projectRoot}/dist/**", "!{projectRoot}/bin/**", "!{projectRoot}/node_modules/**"]
    if runtime:
        inputs.append({"runtime": runtime})
    t = {"execution": {"kind": "command", "command": command, "cwd": root}, "inputs": inputs}
    if env:
        t["execution"]["env"] = env
    if outputs is not None:
        t["outputs"] = outputs
    t["cache"] = cache
    if depends:
        t["dependsOn"] = depends
    if continuous:
        t["continuous"] = True
    return t


def project(name, root, targets):
    return {"name": name, "root": root, "targets": targets}


def graph():
    out = lambda d: ["{projectRoot}/" + d]
    projects = [
        project("fx-bun", "fx/bun", {"generate": target("bun ./gen.ts", "fx/bun", out("out")), "publish": target("bun ./publish.ts", "fx/bun", [], cache=False)}),
        project("fx-tsc", "fx/tsc", {"typecheck": target(NM + "/tsc -p tsconfig.json --noEmit", "fx/tsc", [], runtime=NM + "/tsc --version")}),
        project("fx-vitest", "fx/vitest", {"test": target(NM + "/vitest run --globals --root .", "fx/vitest", [], runtime=NM + "/vitest --version")}),
        project("fx-buntest", "fx/buntest", {"test": target("bun test ./a.test.ts", "fx/buntest", [], depends=["fx-bun:generate"]), "test-fail": target("bun test ./fail.test.ts", "fx/buntest", [])}),
        project("fx-vite", "fx/vite", {"build": target(NM + "/vite build --outDir dist --emptyOutDir", "fx/vite", out("dist"), runtime=NM + "/vite --version", depends=["fx-bun:generate"])}),
        project("fx-pw", "fx/pw", {"test-e2e": target("node ./e2e.mjs", "fx/pw", [], env={"BZLCOV_CHROMIUM": CHROMIUM}, runtime="node --version")}),
        project("fx-go", "fx/" + GO_DIR, {"test": target("go test ./...", "fx/" + GO_DIR, [], runtime="go version"), "build": target("go build -o bin/hello .", "fx/" + GO_DIR, out("bin"), runtime="go version")}),
        project("fx-py", "fx/" + PY_DIR, {"generate": target("python3 gen.py", "fx/" + PY_DIR, out("out"), runtime="python3 --version"), "test": target("python3 test_gen.py", "fx/" + PY_DIR, [], runtime="python3 --version")}),
        project("fx-latex", "fx/" + PRINT_DIR, {"build": target("python3 mkpdf.py", "fx/" + PRINT_DIR, out("out"), runtime="python3 --version")}),
        project("fx-wasm", "fx/wasm", {"wasm": target(NM + "/jco transpile empty.wasm -o out", "fx/wasm", out("out"), runtime=NM + "/jco --version")}),
        project("fx-dev", "fx/dev", {"dev": target("bun ./serve.ts", "fx/dev", None, cache=False, continuous=True)}),
    ]
    return {"version": 1, "projects": {p["name"]: p for p in projects}, "dependencies": [], "namedInputs": {}}


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--mutate":
        path = os.path.join(FX, sys.argv[2])
        with open(path, "a", encoding="utf8") as handle:
            handle.write((sys.argv[3] if len(sys.argv) > 3 else "// mutated\n").encode("utf8").decode("unicode_escape"))
        print("mutated", path)
        return
    lay_out()
    json.dump(graph(), open("/private/tmp/bzlcov/graph.json", "w"), ensure_ascii=False)
    print("fixtures laid out:", len(W), "files;", len(graph()["projects"]), "projects")


main()
