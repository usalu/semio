#!/usr/bin/env python3
"""🔍 Norm artifact editor/viewer audit matrix (read-only): windows, legacy, DE labels, JSON bodies, live wiring, macros, keybindings, incremental units."""
import os, re, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
ALPHA = lambda s: re.sub(r'[^A-Za-z]', '', s)
def rs_files(base):
    out = []
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in ("target", "node_modules")]
        for f in fn:
            if f.endswith((".rs", ".ts")):
                p = os.path.join(dp, f)
                out.append(p)
    return out
def is_test(p):
    return "🧪️tests" in p or "/tests/" in p or "🧪️tests" in p
def read(p):
    return open(p, encoding="utf-8", errors="replace").read()
def windows(base):
    names = set()
    for dp, dn, fn in os.walk(base):
        if os.path.basename(os.path.dirname(dp)) == "🪟️windows" or "🪟️windows" in dp.split(os.sep)[-2:-1]:
            names.add(ALPHA(os.path.basename(dp)))
    return names
def grep(files, pat, nontest=True):
    rx = re.compile(pat)
    hits = []
    for p in files:
        if nontest and is_test(p):
            continue
        for i, l in enumerate(read(p).split("\n"), 1):
            if rx.search(l):
                hits.append((p, i, l.strip()[:160]))
    return hits
rows = []
for art in sorted(os.listdir(ROOT)):
    S = os.path.join(ROOT, art, "🏅️standards/🔖️1/🪆️subsets/✳️any")
    if not os.path.isdir(S):
        continue
    E = os.path.join(S, "✏️editor"); V = os.path.join(S, "👁️viewer")
    ef = rs_files(E); vf = rs_files(V); sf = rs_files(S)
    ew = windows(os.path.join(E, "🎭️modes")); vw = windows(os.path.join(V, "🎭️modes"))
    ev = [w for w in ("viewport", "diagrams", "properties", "checks", "outliner") if w not in ew]
    vv = [w for w in ("viewport", "diagrams", "checks") if w not in vw]
    legacy_e = sorted(w for w in ew if w in ("inputs", "results"))
    legacy_v = sorted(w for w in vw if w in ("report",))
    rd = grep(ef + vf, r'render_document_editor|render_document_json')
    js = grep(ef + vf, r'to_string_pretty|serde_json::to_string|json!\(|<pre|dump_json|\.json_body')
    native = len(grep(ef + vf, r'LocalizedLabel::native\('))
    single = grep(ef + vf, r'LocalizedLabel::(en|de|text|single|plain|new)\(')
    bare_def = grep(ef + vf, r'(window_definition|action_describe|action_with|ActionArgDef::\w+)\((?!.*LocalizedLabel)(?!.*\n)')
    acc_kit = len(grep(sf, r'accessible_name', nontest=True))
    aria = len(grep(ef + vf, r'accessib|aria', nontest=True))
    live_use = len(grep(ef + vf, r'inferences::live'))
    eval_direct = grep(ef + vf, r'_inferences::evaluate\(|inferences::evaluate\(|\bevaluate\(&?\w*doc')
    live_macro = len(grep(S and [os.path.join(S, "🧬️schema/💡️inferences/🦀️.rs")], r'live_fold_source!|live_report!'))
    live_report_fn = len(grep(ef, r'fn live_report\('))
    macros = {m: len(grep(ef + vf, m)) for m in ("norm_live_editor!", "norm_live_acceptance!", "norm_live_commands!", "norm_exact_store_ownership!", "norm_lane_stores!", "norm_command_from_action!", "norm_viewer_surface!")}
    keyb = len(grep(ef, r'\.keybinding\('))
    escape = len(grep(ef, r'keybinding\("escape"'))
    incr_dir = os.path.isdir(os.path.join(E, "🧮️incremental"))
    scope = len(grep(sf, r'struct Scope\b|fn check_scoped\b'))
    units = len(grep(sf, r'UnitSource for|impl UnitSource'))
    fm = os.path.isdir(os.path.join(E, "🏷️field-meta"))
    outliner_layout = len(grep([os.path.join(E, "🎭️modes/✏️edit/🦀️.rs")], r'outliner', nontest=True))
    rows.append(dict(art=art, ew=sorted(ew), vw=sorted(vw), ev=ev, vv=vv, legacy_e=legacy_e, legacy_v=legacy_v,
        rd=sorted({os.path.relpath(h[0], S) for h in rd}), js=sorted({os.path.relpath(h[0], S) for h in js}),
        native=native, single=[os.path.relpath(h[0], S) + ":" + str(h[1]) for h in single][:6],
        bare=len(bare_def), acc_kit=acc_kit, aria=aria, live_use=live_use, eval_direct=[os.path.relpath(h[0], S) + ":" + str(h[1]) for h in eval_direct][:6],
        live_macro=live_macro, live_report_fn=live_report_fn, macros=macros, keyb=keyb, escape=escape, incr=incr_dir, scope=scope, units=units, fm=fm, outliner_layout=outliner_layout))
for r in rows:
    print("==", r["art"])
    for k, v in r.items():
        if k != "art":
            print("   ", k, ":", v)
