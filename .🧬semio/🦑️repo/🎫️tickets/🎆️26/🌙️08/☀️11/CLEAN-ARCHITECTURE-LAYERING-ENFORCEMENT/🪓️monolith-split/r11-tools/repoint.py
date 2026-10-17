#!/usr/bin/env python3
# 🧭️ Re-points callers of the editor items moved to 🧱️base and 🖥️host; usage: repoint.py [--dry]. Scans every .rs file of the flow artifact except the editor root.
import re,os,sys
ROOT="/Users/ueli/Documents/semio"
A=ROOT+"/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
HERE=os.path.dirname(os.path.abspath(__file__))
BASE="FLOW_PLAY_APP_ID FLOW_PLAY_WINDOW_MAIN FLOW_PLAY_WINDOW_GENERATIONS FLOW_PLAY_WINDOW_GENERATE_FORM FLOW_PLAY_WINDOW_GENERATE_PREVIEW flow_action ui_value_text ui_value_bool ui_value_number ui_value_list ui_value_map ui_node_list pick_item FLOW_INTERACTION_GRAPH FLOW_GRAPH_NODE_TARGET_PREFIX FLOW_GRAPH_EDGE_TARGET_PREFIX FLOW_GRAPH_HANDLE_TARGET_PREFIX flow_graph_node_target_id flow_graph_edge_target_id flow_graph_selection_domains FLOW_STORE_MAX_SCENE_ITEMS FLOW_STORE_MAX_TEXT_BYTES FLOW_STORE_MAX_MUTATION_ITEMS FLOW_GRAPH_OPERATION_RAW_BYTES".split()
HOST="seed_host_catalogue apply_canvas_options cold_grant host_with_session host_from_snapshot with_host_from_snapshot with_live_host_snapshot flow_composed_content flow_content_leaves_emit flow_removal_leaves sync_host_selection sync_host_selection_domains focus_selection_camera value_fault".split()
MOD={**{n:"base" for n in BASE},**{n:"host" for n in HOST}}
dry="--dry" in sys.argv
ROOTFILE=A+"/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
files=[]
for dp,dn,fn in os.walk(A):
    dn[:]=[d for d in dn if d not in("node_modules","target")]
    for f in fn:
        p=os.path.join(dp,f)
        if f.endswith(".rs") and p!=ROOTFILE and not p.endswith("/🧱️base/🦀️.rs") and not p.endswith("/🖥️host/🦀️.rs"): files.append(p)
def split_top(s):
    out=[];d=0;cur=""
    for ch in s:
        if ch in "{(": d+=1
        if ch in "})": d-=1
        if ch=="," and d==0: out.append(cur.strip()); cur=""
        else: cur+=ch
    if cur.strip(): out.append(cur.strip())
    return out
PFX="crate::editor::flow::"
changed=[]
for p in files:
    t=open(p,encoding="utf8").read(); o=t
    # group / single use statements
    def use_sub(m):
        body=m.group(2)
        if m.group(1)=="{":
            items=split_top(body)
        else:
            items=[body.strip()]
        keep=[];mods={}
        for it in items:
            name=re.match(r'[A-Za-z_][A-Za-z0-9_]*',it)
            nm=name.group(0) if name else ""
            if nm in MOD and "::" not in it.split(" as ")[0]:
                mods.setdefault(MOD[nm],[]).append(it)
            else: keep.append(it)
        if not mods: return m.group(0)
        lines=[]
        if keep: lines.append("use "+PFX+("{"+", ".join(keep)+"}" if len(keep)>1 or m.group(1)=="{" and False else keep[0])+";" if len(keep)==1 else "use "+PFX+"{"+", ".join(keep)+"};")
        for mod,its in mods.items():
            lines.append("use "+PFX+mod+"::"+(its[0] if len(its)==1 else "{"+", ".join(its)+"}")+";")
        return "\n".join(lines)
    t=re.sub(r'use '+re.escape(PFX)+r'(\{)([^;]*?)\};',use_sub,t,flags=re.S)
    t=re.sub(r'use '+re.escape(PFX)+r'()([A-Za-z_][A-Za-z0-9_]*(?: as [A-Za-z_][A-Za-z0-9_]*)?);',use_sub,t)
    # inline paths
    def path_sub(m):
        return PFX+MOD[m.group(1)]+"::"+m.group(1)
    t=re.sub(re.escape(PFX)+r'('+"|".join(MOD)+r')(?![A-Za-z0-9_])',path_sub,t)
    if t!=o:
        changed.append(p)
        if not dry: open(p,"w",encoding="utf8").write(t)
for p in changed: print("changed",p.replace(A+"/",""))
