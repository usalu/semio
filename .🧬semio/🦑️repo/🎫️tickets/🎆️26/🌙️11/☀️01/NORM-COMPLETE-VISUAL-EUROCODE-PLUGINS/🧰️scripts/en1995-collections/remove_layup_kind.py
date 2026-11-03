#!/usr/bin/env python3
"""Removes the retired `change-panel-layup` mutation kind (the layup string is replaced by typed `panels[].layers[]` rows) from every aggregate of the EN 1995 artifact, renumbers the binary protocol tags and the proto oneof, and deletes the kind crate directory and its fixtures. Idempotent.
Usage: python3 -I remove_layup_kind.py [--dry]"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import spec as S

DRY = "--dry" in sys.argv
ANY = S.ANY
MUT = ANY + "🧬️schema/🧬️mutations"
H = ANY + "🧪️tests/🪵️mutate-en1995-1"
TAX = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"


def rd(p):
    return open(p, encoding="utf-8").read()


def edit(p, fn):
    t = rd(p)
    n = fn(t)
    if n != t:
        print("  edit", p.replace(ANY, "")[:140])
        if not DRY:
            open(p, "w", encoding="utf-8").write(n)


def drop(t, pat, flags=0):
    return re.sub(pat, "", t, flags=flags)


def renumber(t, pat, base):
    def f(m):
        n = int(m.group(2))
        return m.group(1) + str(n - 1 if n > base else n) + m.group(3)
    return re.sub(pat, f, t)


def main():
    edit(MUT + "/🦀️.rs", lambda t: drop(drop(drop(t, r"use semio_s_artifact_norm_en1995_mut_panels::change_panel_layup;\n"), r"    ChangePanelLayup\(change_panel_layup::ChangePanelLayup\),\n"), r'    "change-panel-layup",\n'))
    edit(MUT + "/🟦️.ts", lambda t: drop(drop(drop(t, r'import \{ type ChangePanelLayup, parseChangePanelLayup \} from [^\n]*\n'), r"  \| \{ ChangePanelLayup: ChangePanelLayup \}\n"), r"  ChangePanelLayup: parseChangePanelLayup,\n"))

    def js(t):
        d = json.loads(t)
        before = len(d["oneOf"])
        d["oneOf"] = [x for x in d["oneOf"] if "ChangePanelLayup" not in x.get("required", [])]
        if len(d["oneOf"]) == before:
            return t
        c = dict(separators=(",", ":"), ensure_ascii=False)
        rest = {k: v for k, v in d.items() if k != "oneOf"}
        head = json.dumps(rest, indent=2, ensure_ascii=False)[:-2]
        return head + ',\n  "oneOf": [\n' + ",\n".join("    " + json.dumps(x, **c) for x in d["oneOf"]) + "\n  ]\n}\n"
    edit(MUT + "/🔣️.json", js)
    edit(MUT + "/🔗️.graphql", lambda t: drop(drop(t, r"  \| ChangePanelLayup\n"), r"type ChangePanelLayup \{\n(?:[^\n]*\n)*?\}\n\n?"))

    def proto(t):
        t = drop(t, r"message ChangePanelLayup \{\n(?:[^\n]*\n)*?\}\n\n")
        if "ChangePanelLayup change_panel_layup = 121;" not in t:
            return t
        t = t.replace("    ChangePanelLayup change_panel_layup = 121;\n", "")
        start = t.index("  oneof payload {")
        end = t.index("  }", start)
        body = renumber(t[start:end], r"(    \w+ \w+ = )(\d+)(;)", 121)
        return t[:start] + body + t[end:]
    edit(MUT + "/🛰️.proto", proto)

    def protocol(t):
        if "change-panel-layup" not in t:
            return t
        t = t.replace("record change-panel-layup tag=120\nfield payload bytes\n", "")
        return renumber(t, r"(record [\w-]+ tag=)(\d+)(\n)", 120)
    edit(ANY + "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", protocol)
    edit(MUT + "/🪵️panels/🦀️.rs", lambda t: drop(t, r'#\[path = "\.\./🧱️change-panel-layup/🦀️\.rs"\]\npub mod change_panel_layup;\n'))
    for p in (MUT + "/🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️op-round-trip/🦀️.rs"):
        edit(p, lambda t: drop(t, r"[^\n]*En1995Mutation::ChangePanelLayup\([^\n]*\n"))
    edit(MUT + "/🧭️edit-rules/🦀️.rs", lambda t: drop(t, r'[^\n]*kind: "change-panel-layup"[^\n]*\n'))
    edit(H + "/🦀️.rs", lambda t: drop(drop(t, r'    "change-panel-layup",\n'), r'    \("change-panel-layup", \(\n(?:        include_str![^\n]*\n){4}    \)\),\n'))
    edit(H + "/🐍️.py", lambda t: drop(drop(t, r'    "change-panel-layup",\n'), r'    "change-panel-layup": \("🧱️change-panel-layup", "✅apply"\),\n'))
    edit(H + "/🥒️.feature", lambda t: drop(t, r"[^\n]*\| change-panel-layup \|[^\n]*\n"))

    def oracle(t):
        d = json.loads(t)
        n = 0
        for cat in d.get("mutationCatalogs", []):
            before = len(cat["vectors"])
            cat["vectors"] = [v for v in cat["vectors"] if v["mutationId"] != "change-panel-layup"]
            cat["kinds"] = [k for k in cat["kinds"] if k != "change-panel-layup"]
            n += before - len(cat["vectors"])
        for man in d.get("mutationManifests", []):
            before = len(man["mutations"])
            man["mutations"] = [m for m in man["mutations"] if m["id"] != "change-panel-layup"]
            n += before - len(man["mutations"])
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n" if n else t
    edit(ANY + "🔮️oracles/🔣️.json", oracle)

    def tax(t):
        d = json.loads(t)
        before = len(d["rows"])
        d["rows"] = [r for r in d["rows"] if not (r["artifact"] == "🪵️en1995" and r["kind"] == "change-panel-layup")]
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n" if len(d["rows"]) != before else t
    edit(TAX, tax)
    for d in (MUT + "/🧱️change-panel-layup", ANY + "🧪️tests/🧫️fixtures/🧬️mutations/🧱️change-panel-layup"):
        if os.path.isdir(d):
            print("  rmtree", d.replace(ANY, "")[:140])
            if not DRY:
                shutil.rmtree(d)


if __name__ == "__main__":
    main()
