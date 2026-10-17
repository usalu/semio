"""Rewrites `serde_json::from_str(include_str!("REL")).unwrap()` into `crate::fixture_json("BASE-RELATIVE")` and records sha256 of each fixture file.
usage: rt_include.py <base dir> <test file...>; REL resolves against the test file dir; BASE-RELATIVE against <base dir>."""
import os, re, sys, hashlib, json
base = os.path.abspath(sys.argv[1]); files = sys.argv[2:]
rx = re.compile(r'serde_json::from_str\(include_str!\("([^"]+)"\s*\)\s*\)\.unwrap\(\)')
MODE = os.environ.get("MODE", "base")
log = os.path.join(os.path.dirname(os.path.abspath(__file__)), "rt_include.log.tsv")
for f in files:
    f = os.path.abspath(f); d = os.path.dirname(f)
    s = open(f, encoding="utf-8").read(); n = 0
    def sub(m):
        global n
        target = os.path.normpath(os.path.join(d, m.group(1)))
        assert os.path.isfile(target), target
        rel = os.path.relpath(target, base)
        sha = hashlib.sha256(open(target, "rb").read()).hexdigest()
        open(log, "a").write(f"{f}\t{rel}\t{sha}\n")
        n += 1
        if MODE == "file":
            return f'crate::fixture_json(file!(), "{m.group(1)}")'
        return f'crate::fixture_json("{rel}")'
    s2 = rx.sub(sub, s)
    left = len(re.findall(r'include_str!\(', s2))
    print("remaining include_str:", left, f[-40:])
    open(f, "w", encoding="utf-8").write(s2)
    print(n, f[-50:])
