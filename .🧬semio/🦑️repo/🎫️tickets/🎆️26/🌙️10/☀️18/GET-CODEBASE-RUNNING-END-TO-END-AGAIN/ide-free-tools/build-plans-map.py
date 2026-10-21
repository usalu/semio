#!/usr/bin/env python3
"""🗺️ Derives staged/plans-map.json: where every tracked .cursor/plans/*.plan.md (and .codex/plans/*.md) goes. Read only; standard library only.
Evidence tiers (strongest first): T1 ticket.json plan.id/plan_id; T2 the plan's own todo/ticket section names an existing ticket slug (curated below); T3 ticket.md/phase_progress.md of one ticket names the plan id. A plan with no evidence gets its own closed archive ticket dated by the day it entered version control."""
import json, os, re, subprocess, sys, collections, datetime

HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(C, *[".."] * 7))
TICKETS = ".🧬semio/🦑️repo/🎫️tickets"
GOAL = "AI-OPTIMIZED-REPO"
git = lambda *a: subprocess.run(["git", "-C", ROOT, *a], capture_output=True, text=True).stdout

ACCEPT = {  # plan filename prefix -> ticket slug (curated: the plan's own ticket)
 "default_driver_hotkeys": "DEFAULT-DRIVER-AFFORDANCES", "dissolve_cores_plugin_contract": "DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT",
 "domain-driven_artifact_specs": "HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT", "generalize_jack_graph_dsl": "GENERALIZE-JACK-GRAPH-DSL",
 "generalize_wires_cross-window_sync": "WIRES-NORMAL-GRAPH", "icon_hover_tooltips": "UNLABELED-CONTROL-HOVER-TOOLTIPS",
 "runtime_installable_extensions": "RUNTIME-INSTALLABLE-EXTENSIONS", "spatial_query_raw_vs_analytic": "BREPJS-CELLCOMPLEX-PARTS-SURFACES",
 "subset_conformance_roundtrips": "SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS", "smooth_projection_transitions": "COMPLETE-3D-PROJECTION-GIZMO",
 "note_app_feature_completion": "NOTE-INFINITE-CANVAS-APP", "fix_muted_chip_fill_overshoot": "FIX-PRINT-PARAGRAPH-CHIP-ALIGNMENT",
 "fill_target_volumes": "PUZZLE-3D-FILL-TARGET-VOLUMES", "map_vector_tile_labels": "MAP-VECTOR-TILES",
 "print_spacing_single_unit": "PRINT-SINGLE-SPACING-UNIT", "procedural-3d-hardening": "FEATURE-COMPLETE-AND-BATTLE-TESTED-PROCEDURAL-3D",
 "sketchpad_platform_refactor": "SKETCHPAD-PLATFORM-REFACTOR", "2d_references_on_grid": "INFINITE-WORLD-2D-REFERENCES",
 "full_virtual_file_system": "FULL-VIRTUAL-FILE-SYSTEM", "fix_board_wasm_reentrant_borrow": "BOARD-WASM-GPUREADY-REENTRANCY",
 "rs_kit_store_materialization": "REFACTOR-RS-KIT-STORE-MATERIALIZED-READS", "infinite_world_hide_lock": "INFINITE-WORLD-HIDE-LOCK",
 "puzzle_5d_unified_tools": "PUZZLE-5D-UNIFIED-TOOLS",
}
REJECT = (  # plans whose only evidence names another ticket as context, not as their own
 "auto-derive_playgrounds", "celebrate_conic_content", "composable_window_option_trees", "dev_build_opt_profiles", "enforce_styling_by_api",
 "flip_vcs_history", "flow_wgpu_rich_rendering_parity", "generalize_commit_metrics", "gis_2d_map_parity_restore", "lowpoly_editor_parity_restoration",
 "os_framework_and_composable_apps", "post_plan_to_issue", "procedural_3d_eval_session", "procedural_feature_complete", "puzzle_3d_selection_overhaul",
 "restore_playgrounds_app_split", "rust-only_apps_migration", "window-local_utilities", "wires_terminology_and_ui", "playground_chrome_reliability",
 "gis_2d", "building_bim_import", "fix_3d_matrix_chain_perf", "kit_wires_typology_parity", "puzzle3d_selection_context_menu", "theme_consistent_readable_ui",
 "fix_vortex_pick_perf", "per-kind_object_styling",
)
DUPLICATE_OF_TICKET_COPY = {"f692e604"}  # the ticket already owns the swept version of this plan; the .cursor copy is the stale spelling
STRONG = re.compile(r"(reopen|re-open|open|close|ticket_reopen|ticket_open|continues? (?:under|inside|in)|ticket:|under (?:the )?(?:open )?ticket|inside (?:the )?(?:open )?ticket|existing open ticket|work in)", re.I)
WEAKNEG = re.compile(r"(prior|sibling|same class|different|supersed|duplicate|modelled|mirrors|see |previous|related|follow-up to|class as)", re.I)
TOK = re.compile(r"[A-Z0-9][A-Z0-9]*(?:-[A-Z0-9]+)+")
PID = re.compile(r"_([0-9a-f]{8})\.plan\.md$")

tickets = [l for l in git("ls-files", TICKETS).split("\n") if l.endswith("/🎫️ticket.json")]
slugdirs = collections.defaultdict(list)
for t in tickets: slugdirs[os.path.basename(os.path.dirname(t))].append(os.path.dirname(t))
plans = [l for l in git("ls-files", ".cursor/plans").split("\n") if l]
ignored = [".cursor/plans/" + n for n in sorted(os.listdir(os.path.join(ROOT, ".cursor", "plans"))) if ".cursor/plans/" + n not in set(plans)] if os.path.isdir(os.path.join(ROOT, ".cursor", "plans")) else []
allplans = sorted(plans + ignored)
byid = {PID.search(f).group(1): f for f in allplans if PID.search(f)}

ticket_doc = {}
t1 = collections.defaultdict(set)
for t in tickets:
    try: j = json.load(open(os.path.join(ROOT, t), encoding="utf-8"), strict=False)
    except ValueError: continue
    ticket_doc[os.path.dirname(t)] = j
    p = j.get("plan")
    for pid in ((p or {}).get("id"), j.get("plan_id")):
        if pid in byid: t1[byid[pid]].add(os.path.dirname(t))

addlog = {}
cur = None
for l in git("log", "--diff-filter=A", "--name-only", "--format=@@%ad", "--date=short", "--", ".cursor/plans").split("\n"):
    if l.startswith("@@"): cur = l[2:]
    elif l and cur and l not in addlog: addlog[l] = cur

def strong_slug(f):
    txt = open(os.path.join(ROOT, f), encoding="utf-8", errors="replace").read()
    found = []
    for m in TOK.findall(txt):
        if m in slugdirs and m not in found: found.append(m)
    ok = []
    for s in found:
        for m in re.finditer(re.escape(s), txt):
            pre = txt[max(0, m.start() - 90):m.start()]
            if STRONG.search(pre) and not WEAKNEG.search(pre): ok.append((txt.find(s), s)); break
    return [s for _, s in sorted(ok)]

def titleize(name):
    small = {"a", "an", "and", "as", "at", "by", "for", "in", "of", "on", "or", "the", "to", "vs", "with"}
    words = re.sub(r"[_\s]+", " ", name.strip()).split(" ")
    return " ".join(w if (w[:1].isupper() or any(c.isdigit() for c in w[:2]) and w[:2].isupper()) else (w.lower() if i and w.lower() in small else w[:1].upper() + w[1:]) for i, w in enumerate(words))

def archive_slug(name):
    s = re.sub(r"[^A-Z0-9]+", "-", name.upper()).strip("-")
    if len(s) > 60: s = s[:60].rsplit("-", 1)[0]
    return s

def fm(f):
    raw = open(os.path.join(ROOT, f), encoding="utf-8", errors="replace").read()
    m = re.match(r"---\n(.*?)\n---", raw, re.S); head = m.group(1) if m else ""
    name = re.search(r"^name:\s*(.*)$", head, re.M); ov = re.search(r"^overview:\s*(.*)$", head, re.M)
    return ((name.group(1) if name else "").strip().strip('"'), (ov.group(1) if ov else "").strip().strip('"'))

TODAY = datetime.date.today()
ARCHIVE_DIR = f"{TICKETS}/🎆️{TODAY:%y}/🌙️{TODAY:%m}/☀️{TODAY:%d}/ARCHIVED-CURSOR-PLANS-WITHOUT-TICKET"
entries, archives, edits = [], {}, []
used = collections.defaultdict(set)
for f in allplans:
    base = os.path.basename(f); pid = PID.search(base).group(1); prefix = base[:-len(".plan.md")]
    e = {"src": f, "id": pid, "tracked": f in set(plans)}
    if pid in DUPLICATE_OF_TICKET_COPY:
        owner = [d for d in slugdirs.get("NEURAL-SCHEMAS-AND-OPERATORS", [])][0]
        e.update(kind="duplicate", ticket=owner, evidence="T1 (the ticket already owns the swept copy " + base + ")"); entries.append(e); continue
    dirs = sorted(t1.get(f, [])); ev = "T1 ticket.json plan.id"
    if not dirs:
        acc = [s for k, s in ACCEPT.items() if base.startswith(k)]
        if acc and len(slugdirs.get(acc[0], [])) == 1: dirs = slugdirs[acc[0]]; ev = "T2/T3 curated: the plan names its own ticket"
        elif not any(base.startswith(k) for k in REJECT):
            st = strong_slug(f)
            if len(st) >= 1 and len(slugdirs[st[0]]) == 1: dirs = slugdirs[st[0]]; ev = "T2 the plan's ticket todo/section names " + st[0]
    if dirs:
        if len(dirs) > 1: raise SystemExit("ambiguous " + f + " " + str(dirs))
        e.update(kind="move", ticket=dirs[0], dest=dirs[0] + "/" + base, evidence=ev)
    else:
        e.update(kind="archive", ticket=ARCHIVE_DIR, dest=ARCHIVE_DIR + "/" + base, evidence="none: the one archive ticket")
    entries.append(e)
for f in sorted(os.listdir(os.path.join(ROOT, ".codex", "plans"))) if os.path.isdir(os.path.join(ROOT, ".codex", "plans")) else []:
    d = slugdirs["DECLARATIVE-SPATIAL-ACTIONS"][0]
    entries.append({"src": ".codex/plans/" + f, "id": "", "tracked": True, "kind": "move", "ticket": d, "dest": d + "/" + f.replace(".md", ".plan.md"), "evidence": "curated: the plan is the declarative spatial actions plan"})
for e in entries:
    if e["kind"] != "move": continue
    j = ticket_doc.get(e["ticket"]) or {}
    p = j.get("plan")
    if isinstance(p, dict) and p.get("source") and p.get("id") == e["id"]:
        text = open(os.path.join(ROOT, e["ticket"], "🎫️ticket.json"), encoding="utf-8", newline="").read()
        m = re.search(r'^(\s*)"source":\s*"((?:[^"\\]|\\.)*)"', text, re.M)
        edits.append({"path": e["ticket"] + "/🎫️ticket.json", "old": m.group(0), "new": m.group(1) + '"local": ' + json.dumps(os.path.basename(e["dest"]), ensure_ascii=False)})
n_arch = sum(1 for e in entries if e["kind"] == "archive")
if n_arch:
    if ARCHIVE_DIR.rsplit("/", 1)[1] in slugdirs: raise SystemExit("archive slug exists")
    archives[ARCHIVE_DIR] = {"title": "Archived Cursor Plans Without Ticket", "emoji": "🗺️", "status": "closed",
        "description": f"The {n_arch} Cursor plans that sat in the retired .cursor/plans directory without any ticket evidence (no ticket.json plan reference, no ticket named in the plan). Each stays a ticket-owned file of this folder under its original name, <name>_<id>.plan.md.",
        "summary": f"Archived {n_arch} plans out of the retired .cursor/plans source into this ticket when the directory was removed from the repository.",
        "goal": GOAL}
out = {"version": 1, "goal": GOAL, "entries": entries, "archiveTickets": archives, "ticketEdits": edits}
json.dump(out, open(os.path.join(HERE, "staged", "plans-map.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
k = collections.Counter(e["kind"] for e in entries)
print(json.dumps({"entries": len(entries), **k, "archiveTickets": len(archives), "ticketEdits": len(edits), "ignoredUntracked": len(ignored)}))
