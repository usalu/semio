"""🧮️ Counts the tests in the cfg-active closure of every test unit read from real-closure-probe.mjs output (stdin): test functions by name and test-generating macro invocations.
usage: real-closure-tests.py [names-file]   (appends the sorted test names and macro invocations of each crate to names-file)"""
import json, re, sys
attr = re.compile(r"#\[\s*(?:[A-Za-z0-9_]+::)*(?:[A-Za-z0-9_]*test|rstest|test_case)\s*(?:\(.*?\))?\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)")
macro = re.compile(r"^\s*((?:[A-Za-z0-9_]+::)*[A-Za-z0-9_]*(?:law|accept|bench|test|proptest)[A-Za-z0-9_]*!\s*[\{(][^\n]{0,100})", re.M | re.I)
out = open(sys.argv[1], "a", encoding="utf8") if len(sys.argv) > 1 else None
for r in json.load(sys.stdin):
    if "+all" in r["unit"] or "inputs" not in r:
        continue
    if not (r["unit"].startswith("lib[cfg(test)]") or r["unit"].startswith("test ") or r["unit"].startswith("bin[cfg(test)]")):
        continue
    names, macros = [], []
    for path, size, lines, kind in r["inputs"]:
        if kind != "source":
            continue
        text = open(path, encoding="utf8").read()
        names += attr.findall(text)
        macros += [m.strip() for m in macro.findall(text) if not m.strip().startswith(("macro_rules", "assert", "debug_assert"))]
    print(f"{r['crate']:<62}{r['unit']:<30}test-fns={len(names)} test-macro-invocations={len(macros)} files={r['files']} bytes={r['bytes']} lines={r['lines']}")
    if out:
        out.write(f"## {r['crate']} {r['unit']}\n" + "".join("fn " + n + "\n" for n in sorted(names)) + "".join("macro " + m + "\n" for m in sorted(macros)))
