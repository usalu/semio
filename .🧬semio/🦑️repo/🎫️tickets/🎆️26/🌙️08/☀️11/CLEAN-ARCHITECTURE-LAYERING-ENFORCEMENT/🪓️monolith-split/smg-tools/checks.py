#!/usr/bin/env python3
"""✅️ Static gates for the touched files of this slice: rustfmt parse (stdin, no child traversal), path literals, path length, TOML and JSON parse.

usage: checks.py <list file with one repo-relative path per line> [--lock]
"""
import json, os, re, subprocess, sys

R = "/Users/ueli/Documents/semio"
TOOLCHAIN = "+nightly-2026-07-20"
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'\b(include_str|include_bytes|include)!\(\s*"([^"]+)"\s*\)')


def strip_comments(text):
    text = re.sub(r"/\*.*?\*/", lambda m: "\n" * m.group(0).count("\n"), text, flags=re.S)
    return re.sub(r"(?m)(^|[^:\"'])//[^\n]*", lambda m: m.group(1), text)


def main():
    paths = [line.strip() for line in open(sys.argv[1], encoding="utf8") if line.strip()]
    rs = [p for p in paths if p.endswith(".rs") and os.path.exists(f"{R}/{p}")]
    toml = [p for p in paths if p.endswith(".toml") and os.path.exists(f"{R}/{p}")]
    locks = [p for p in paths if p.endswith(".lock") and os.path.exists(f"{R}/{p}")]
    js = [p for p in paths if p.endswith((".json", ".jsonc")) and os.path.exists(f"{R}/{p}")]
    report = {"files": len(paths), "rs": len(rs), "toml": len(toml), "lock": len(locks), "json": len(js)}
    fmt_fail = []
    for p in rs:
        r = subprocess.run(["rustfmt", TOOLCHAIN, "--edition", "2021", "--emit", "stdout"], stdin=open(f"{R}/{p}", "rb"), capture_output=True, cwd=R)
        if r.returncode != 0:
            fmt_fail.append((p, r.stderr.decode("utf8", "replace")[:300]))
    report["rustfmt_failures"] = len(fmt_fail)
    missing = []
    checked = 0
    for p in rs:
        text = strip_comments(open(f"{R}/{p}", encoding="utf8").read())
        base = os.path.dirname(f"{R}/{p}")
        for m in PATH_ATTR.finditer(text):
            checked += 1
            if not os.path.exists(os.path.normpath(os.path.join(base, m.group(1)))):
                missing.append((p, "path", m.group(1)))
        for m in INCLUDE.finditer(text):
            checked += 1
            if not os.path.exists(os.path.normpath(os.path.join(base, m.group(2)))):
                missing.append((p, m.group(1), m.group(2)))
    report["path_literals_checked"] = checked
    report["path_literals_missing"] = len(missing)
    longest = max(((len(p.encode("utf8")), len(p), p) for p in paths), default=(0, 0, ""))
    over = [(len(p.encode("utf8")), p) for p in paths if len(p.encode("utf8")) > 240]
    report["longest_path_bytes"] = longest[0]
    report["longest_path_chars"] = max((len(p) for p in paths), default=0)
    report["paths_over_240_bytes"] = len(over)
    script = '''
import {readFileSync} from "node:fs";
import {parse} from "jsonc-parser";
const input = JSON.parse(readFileSync(process.argv[2], "utf8"));
const bad: string[] = [];
for (const p of input.toml) { try { Bun.TOML.parse(readFileSync("/Users/ueli/Documents/semio/" + p, "utf8")); } catch (e) { bad.push("toml " + p + " " + String(e).slice(0, 120)); } }
for (const p of input.lock) { try { Bun.TOML.parse(readFileSync("/Users/ueli/Documents/semio/" + p, "utf8")); } catch (e) { bad.push("lock " + p + " " + String(e).slice(0, 120)); } }
for (const p of input.js) {
  const text = readFileSync("/Users/ueli/Documents/semio/" + p, "utf8");
  if (p.endsWith(".jsonc")) { const errors: any[] = []; parse(text, errors, {allowTrailingComma: true}); if (errors.length) bad.push("jsonc " + p + " errors=" + errors.length); }
  else { try { JSON.parse(text); } catch (e) { const errors: any[] = []; parse(text, errors, {allowTrailingComma: true}); if (errors.length) bad.push("json " + p + " " + String(e).slice(0, 100)); } }
}
console.log(JSON.stringify(bad));
'''
    tmp = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/checks-input.json"
    open(tmp, "w", encoding="utf8").write(json.dumps({"toml": toml, "lock": locks, "js": js}, ensure_ascii=False))
    scr = tmp.replace(".json", ".ts")
    open(scr, "w", encoding="utf8").write(script)
    r = subprocess.run(["bun", scr, tmp], capture_output=True, text=True, cwd=R)
    bad = json.loads(r.stdout.strip().splitlines()[-1]) if r.stdout.strip() else [r.stderr[-300:]]
    report["toml_json_failures"] = len(bad)
    print(json.dumps(report, ensure_ascii=False))
    for row in fmt_fail:
        print("RUSTFMT", row)
    for row in missing:
        print("MISSING", row)
    for row in over:
        print("TOO-LONG", row)
    for row in bad:
        print("PARSE", row)


main()
