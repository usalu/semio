#!/usr/bin/env python3
"""🧰️ Builds `v50triage-tools/staged/fix` from the CURRENT live files (re-run right before landing; asserts every anchor is unique).
rows: witness test (the two plan refusals match the token), lru test (members/scan lang6 token), worker test (token/escape expectations follow the runner's machine tokens), witness publisher (4 bilingual one-string refusals -> machine tokens, 2 duplicate docstring emojis made unique), language allowlist (witness entry removed, ceiling 9570 -> 9569), scheduling table (bootstrap texts catalog joins progress-reporting, its module is the port)."""
import hashlib, json, os, shutil, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.environ.get("SEMIO_REPO") or os.path.abspath(os.path.join(C, *[".."] * 7))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B = L + "🏗️build/🔧️tools/🏗️bazel/"
WIT = B + "🚦️test-classes/🧾️witness/🟦️.ts"
TEST = B + "🏃️execution/🧪️tests/👷️worker/🟦️.ts"
ALLOW = L + "⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json"
SCHED = L + "🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"
WTEST = B + "🚦️test-classes/🧾️witness/🧪️tests/🟦️.ts"
LRU = L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/♻️lru/🟦️.ts"
OUT = os.path.join(C, "v50triage-tools", "staged", "fix")
def read(rel): return open(os.path.join(R, rel), encoding="utf8").read()
def once(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:80]); return text.replace(old, new)
def sha(data): return hashlib.sha256(data).hexdigest()
rows = {}
t = read(TEST); rows[TEST] = once(t, "mode===\"token\"?/token refused/u:/escapes the execution root/u", "mode===\"token\"?/token-refused/u:/path-escapes-root/u")
rows[LRU] = once(read(LRU), 'toThrow("Exact optional graph target bytes required")', 'toThrow("exact-optional-graph-target-bytes-required")')
wt = read(WTEST); assert wt.count('.toThrow("witness plan")') == 2
rows[WTEST] = wt.replace('.toThrow("witness plan")', '.toThrow("bazel-witness:plan-closed-required")')
w = read(WIT)
w = w.replace('throw Error("Closed Bazel witness plan required / Geschlossener Bazel-Zeugenplan erforderlich")', 'throw Error("bazel-witness:plan-closed-required")')
w = once(w, 'throw Error("Bounded Bazel witness plan required / Begrenzter Bazel-Zeugenplan erforderlich")', 'throw Error("bazel-witness:plan-bounded-required")')
w = once(w, 'throw Error(`Captured witness input required: ${path} / Erfasste Zeugeneingabe erforderlich: ${path}`)', 'throw Error(`bazel-witness:input-required:${path}`)')
w = once(w, "/** \U0001F9FE\ufe0f The content of one witness file. */", "/** \U0001F4C4\ufe0f The content of one witness file. */")
w = once(w, "/** \U0001F5D2\ufe0f The plan text the planner writes", "/** \U0001F4DD\ufe0f The plan text the planner writes")
assert "Closed Bazel" not in w and "erforderlich" not in w
rows[WIT] = w
a = json.loads(read(ALLOW)); raw = read(ALLOW)
assert a["files"].pop(WIT) == 1 and a["ceiling"] == 9570 and sum(a["files"].values()) == 9569
a["ceiling"] = 9569
indent = 1
dumped = json.dumps(a, ensure_ascii=False, indent=indent) + ("\n" if raw.endswith("\n") else "")
if json.dumps(json.loads(raw), ensure_ascii=False, indent=indent) + ("\n" if raw.endswith("\n") else "") != raw: sys.exit("allowlist serialisation differs from the live bytes")
rows[ALLOW] = dumped
s = read(SCHED)
BOOT = L + "⚡️caching/🚀️bootstrap/📖️texts/"
CACH = L + "⚡️caching/📖️texts/"
s = once(s, '        "' + CACH + '"\n      ],\n      "ports": [\n        "' + CACH + '🟦️.ts",', '        "' + CACH + '",\n        "' + BOOT + '"\n      ],\n      "ports": [\n        "' + CACH + '🟦️.ts",\n        "' + BOOT + '🟦️.ts",')
rows[SCHED] = s
shutil.rmtree(OUT, ignore_errors=True); os.makedirs(OUT + "/files"); os.makedirs(OUT + "/base")
manifest = {}
for i, (rel, new) in enumerate(rows.items()):
    ext = os.path.splitext(rel)[1]; name = "%02d%s" % (i, ext)
    live = open(os.path.join(R, rel), "rb").read()
    open(os.path.join(OUT, "base", name), "wb").write(live); open(os.path.join(OUT, "files", name), "wb").write(new.encode())
    manifest[rel] = {"file": name, "base": sha(live), "new": sha(new.encode())}
json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
shutil.copyfile(os.path.join(C, "xplat5-tools", "stage.template.py"), os.path.join(OUT, "stage.py"))
print("staged", len(rows), "rows ->", OUT)
