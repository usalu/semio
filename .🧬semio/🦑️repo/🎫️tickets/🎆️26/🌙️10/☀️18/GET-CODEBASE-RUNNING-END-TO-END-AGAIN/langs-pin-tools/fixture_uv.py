import json
F = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🧪️tests"
p = F + "/🔣️.json"
d = json.load(open(p, encoding="utf8"))
hosts = ["darwin-arm64", "darwin-x64", "linux-arm64", "linux-x64", "win32-arm64", "win32-x64"]
root = "/work/a/.cache/b/t"
if not any(t["tool"] == "uv" for t in d["tools"]):
    d["tools"] += [{"name": "uv " + host, "tool": "uv", "host": host, "root": root} for host in hosts]
d["publisher"]["uv"] = {"source": "https://github.com/astral-sh/uv/releases/download/0.11.6/<archive>.sha256", "lines": [
 "bee7b25a7a999f17291810242b47565c3ef2b9205651a0fd02a086f261a7e167  uv-aarch64-pc-windows-msvc.zip",
 "4b69a4e366ec38cd5f305707de95e12951181c448679a00dce2a78868dfc9f5b  uv-aarch64-apple-darwin.tar.gz",
 "d5be4bf7015ea000378cb3c3aba53ba81a8673458ace9c7fa25a0be005b74802  uv-aarch64-unknown-linux-gnu.tar.gz",
 "99aa60edd017a256dbf378f372d1cff3292dbc6696e0ea01716d9158d773ab77  uv-x86_64-pc-windows-msvc.zip",
 "8e0ed5035eaa28c7c8cd2a46b5b9a05bfff1ef01dbdc090a010eb8fdf193a457  uv-x86_64-apple-darwin.tar.gz",
 "0c6bab77a67a445dc849ed5e8ee8d3cb333b6e2eba863643ce1e228075f27943  uv-x86_64-unknown-linux-gnu.tar.gz"]}
open(p, "w", encoding="utf8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")

o = F + "/🐍️oracle/🐍️.py"
s = open(o, encoding="utf8").read()
old = '    spec = policy["go"]["tool"] if tool == "go" else policy["cmake"][tool]\n'
assert old in s
s = s.replace(old, '    spec = policy["go"]["tool"] if tool == "go" else policy["python"]["tool"] if tool == "uv" else policy["cmake"][tool]\n')
open(o, "w", encoding="utf8").write(s)

t = F + "/🟦️.ts"
s = open(t, encoding="utf8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:80], s.count(old))
    s = s.replace(old, new)
rep('for (const tool of ["cmake", "ninja", "go"] as const) {\n      const rows = (tool === "go" ? policy.go.tool : policy.cmake[tool]).distributions;', 'for (const tool of ["cmake", "ninja", "go", "uv"] as const) {\n      const rows = (tool === "go" ? policy.go.tool : tool === "uv" ? policy.python.tool : policy.cmake[tool]).distributions;')
rep('const specs = {cmake: policy.cmake.cmake, ninja: policy.cmake.ninja, go: policy.go.tool};\n    for (const tool of ["cmake", "ninja", "go"] as const)', 'const specs = {cmake: policy.cmake.cmake, ninja: policy.cmake.ninja, go: policy.go.tool, uv: policy.python.tool};\n    for (const tool of ["cmake", "ninja", "go", "uv"] as const)')
rep('host: row.tool === "go" ? "dl.google.com" : "github.com"', 'host: row.tool === "go" ? "dl.google.com" : "github.com"')
open(t, "w", encoding="utf8").write(s)
