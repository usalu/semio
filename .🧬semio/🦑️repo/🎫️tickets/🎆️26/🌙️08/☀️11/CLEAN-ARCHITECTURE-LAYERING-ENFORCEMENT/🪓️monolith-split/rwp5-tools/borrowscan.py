# 🔎️ Borrow-hazard scan: bridged calls `self.as_x().m(args)` whose arguments use a local that was bound from a borrow of self (two-phase borrows no longer apply to the bridge receiver).
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, ctx, deps
from rsedit import Doc
from blank import blank

REFRET = ['action_session','active_plugin_manifest','active_tab','active_utility_for_window','anchor_state','attached_document_id','command_session','context_window_instance_id','engine_host_id_for_window','host_app','history_time_travel','node_at','page','palette_query','scope','transient_notice','window_document_session','window_icon_override','window_title_override','world3d_host_id_for_window','world3d_state_for_window','world3d_status_rows','world_projection_template_id','space_id','tabs','options','head','result','live_window_kind_id','window_kind_of','active_utility_bar_window_kind']
KW = set("let mut ref as if else match for in while loop return break continue fn move async await true false None Some Ok Err Self self crate super pub use mod impl struct enum const static where dyn unsafe type".split())

def match_close(b, i):
    d = 0
    while i < len(b):
        if b[i] in "([{": d += 1
        elif b[i] in ")]}":
            d -= 1
            if d == 0: return i
        i += 1
    return -1

def scan(root):
    out = []
    for crate in ctx.BRIDGE:
        for slug, p in ctx.crate_files(root, crate):
            doc = Doc(open(p, encoding="utf-8").read())
            for it in doc.items():
                if it["kind"] != "impl" or "Ctx" not in it["head"]: continue
                for m in doc.members(it):
                    body = "\n".join(doc.blank[m["s"]:m["e"] + 1])
                    for mm in re.finditer(r"\bself\s*\.\s*(as_\w+)\s*\(\s*\)\s*\.\s*(\w+)\s*(?:::\s*<[^>]*>)?\s*\(", body):
                        lp = mm.end() - 1; rp = match_close(body, lp)
                        args = body[lp + 1:rp]
                        ids = {x for x in re.findall(r"(?<![\w.:])([a-z_][a-z_0-9]*)\b(?!\s*\()(?!\s*!)", args) if x not in KW}
                        before = body[:mm.start()]
                        for v in ids:
                            binds = list(re.finditer(r"(?:\blet\s+(?:mut\s+)?(?:ref\s+)?|\bSome\(|\bOk\(|\bfor\s+|\|)" + re.escape(v) + r"\b[^;=]*?(?:=|\bin\b)\s*([^;{]*)", before))
                            if not binds: continue
                            init = binds[-1].group(1)
                            if re.search(r"\bself\b", init) or re.search(r"\bself\b", before[binds[-1].start():binds[-1].start() + 200].split("{")[0]):
                                refy = re.match(r"\s*&", init) or re.search(r"\.(as_ref|as_deref|get|get_mut|iter|first|last|find|values|as_str|as_slice)\(", init) or re.search(r"\b(" + "|".join(REFRET) + r")\(", init)
                                owned = re.search(r"\.(clone|to_string|to_owned|len|is_empty|cloned|copied|unwrap_or_default|collect|into_iter)\(\)|\.cloned\(\)|\.copied\(\)|\.map\(String::|\.to_string\(\)|\.clone\(\)", init)
                                if refy and not owned:
                                    out.append((crate, m["name"], mm.group(2), v, init.strip()[:80].replace("\n", " ")))
    return out

if __name__ == "__main__":
    r = scan(sys.argv[1] if len(sys.argv) > 1 else cfg.NEW)
    for x in r: print(x)
    print(len(r))
