#!/usr/bin/env python3
# 🪟️ Moves the four window-id constants into 🧱️base: removes their definitions and re-points users.
import os,re
ROOT="/Users/ueli/Documents/semio"
A=ROOT+"/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
ED=A+"/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
def edit(rel,fn):
    p=ED+"/"+rel; t=open(p,encoding="utf8").read(); n=fn(t); assert n!=t,rel; open(p,"w",encoding="utf8").write(n)
def sub1(t,a,b,count=1):
    assert t.count(a)==count,(a,t.count(a)); return t.replace(a,b)
W="🎭️modes/✏️edit/🪟️windows"
G="🎭️modes/🧬️generate"
# main window
edit(W+"/🌊️main/🦀️.rs",lambda t:sub1(sub1(t,'pub const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";\n',''),'use crate::editor::flow::base::{FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_HANDLE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_INTERACTION_GRAPH};','use crate::editor::flow::base::{FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_HANDLE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_INTERACTION_GRAPH, FLOW_PLAY_WINDOW_MAIN};'))
# config + transient
edit(W+"/🌊️main/🎚️config/🦀️.rs",lambda t:sub1(t,"super::FLOW_PLAY_WINDOW_MAIN","crate::editor::flow::base::FLOW_PLAY_WINDOW_MAIN",2))
def tr(t):
    t=sub1(t,"super::FLOW_PLAY_WINDOW_MAIN","crate::editor::flow::base::FLOW_PLAY_WINDOW_MAIN")
    t=sub1(t,"crate::editor::flow::modes::generate::windows::generations::FLOW_PLAY_WINDOW_GENERATIONS","crate::editor::flow::base::FLOW_PLAY_WINDOW_GENERATIONS")
    t=sub1(t,"crate::editor::flow::modes::generate::windows::form::FLOW_PLAY_WINDOW_GENERATE_FORM","crate::editor::flow::base::FLOW_PLAY_WINDOW_GENERATE_FORM")
    t=sub1(t,"crate::editor::flow::modes::generate::windows::preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW","crate::editor::flow::base::FLOW_PLAY_WINDOW_GENERATE_PREVIEW")
    return t
edit(W+"/🌊️main/🫧️transient/🦀️.rs",tr)
# generate windows: definitions
for rel,name,val in ((G+"/🪟️windows/🗂️generations/🦀️.rs","FLOW_PLAY_WINDOW_GENERATIONS","flow-generations"),(G+"/🪟️windows/📝️form/🦀️.rs","FLOW_PLAY_WINDOW_GENERATE_FORM","flow-generate-form"),(G+"/🪟️windows/👁️preview/🦀️.rs","FLOW_PLAY_WINDOW_GENERATE_PREVIEW","flow-generate-preview")):
    def f(t,name=name,val=val):
        t=sub1(t,f'pub const {name}: &str = "{val}";\n','')
        i=t.index("\nuse ")+1
        return t[:i]+f"use crate::editor::flow::base::{name};\n"+t[i:]
    edit(rel,f)
# users
edit("🎭️modes/✏️edit/🦀️.rs",lambda t:sub1(sub1(t,"main::FLOW_PLAY_WINDOW_MAIN.into()","FLOW_PLAY_WINDOW_MAIN.into()"),"use crate::editor::flow::modes::edit::windows::{compiled, main};","use crate::editor::flow::base::FLOW_PLAY_WINDOW_MAIN;\nuse crate::editor::flow::modes::edit::windows::{compiled, main};"))
edit(G+"/🦀️.rs",lambda t:sub1(sub1(sub1(sub1(t,"generations::FLOW_PLAY_WINDOW_GENERATIONS.into()","FLOW_PLAY_WINDOW_GENERATIONS.into()"),"form::FLOW_PLAY_WINDOW_GENERATE_FORM.into()","FLOW_PLAY_WINDOW_GENERATE_FORM.into()"),"preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW.into()","FLOW_PLAY_WINDOW_GENERATE_PREVIEW.into()"),"use crate::editor::flow::modes::generate::windows::{form, generations, preview};","use crate::editor::flow::base::{FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW, FLOW_PLAY_WINDOW_GENERATIONS};\nuse crate::editor::flow::modes::generate::windows::{form, generations, preview};"))
# commands
for c in ("⏱️flow-eval-tick/🧪️tests/🔬️unit","▶️run-extension-action","🏁️flow-eval-resolve","🧮️evaluate"):
    edit("🎮️commands/"+c+"/🦀️.rs",lambda t:sub1(t,"use crate::editor::flow::modes::edit::windows::main::FLOW_PLAY_WINDOW_MAIN;","use crate::editor::flow::base::FLOW_PLAY_WINDOW_MAIN;"))
