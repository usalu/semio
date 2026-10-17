#!/usr/bin/env python3
# 🧩️ Writes the staged editor root (after extract-base.py): adds the base/host imports, drops the window-id qualifiers, mounts the two new modules in the artifact nest.
import os,re
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT="/Users/ueli/Documents/semio"
A=ROOT+"/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
ED=A+"/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
t=open(HERE+"/stage-editor-root.rs",encoding="utf8").read()
anchor="use crate::editor::flow::terminology::{flow_play_labels, FlowPlayLabels};\n"
assert t.count(anchor)==1
imports=anchor+'''use crate::editor::flow::base::{flow_graph_edge_target_id, flow_graph_node_target_id, flow_graph_selection_domains, FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_INTERACTION_GRAPH, FLOW_PLAY_APP_ID, FLOW_PLAY_WINDOW_MAIN, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_SCENE_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
#[cfg(test)]
use crate::editor::flow::base::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
use crate::editor::flow::host::{cold_grant, flow_content_leaves_emit, flow_removal_leaves, focus_selection_camera, seed_host_catalogue, value_fault};
#[cfg(test)]
use crate::editor::flow::host::{host_from_snapshot, sync_host_selection_domains};
'''
t=t.replace(anchor,imports)
n=t.count("main::FLOW_PLAY_WINDOW_MAIN")
t=t.replace("main::FLOW_PLAY_WINDOW_MAIN","FLOW_PLAY_WINDOW_MAIN")
open(ED+"/🦀️.rs","w",encoding="utf8").write(t)
print("replaced qualifiers",n)
rp=A+"/🦀️.rs"
r=open(rp,encoding="utf8").read()
a='''        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
'''
assert r.count(a)==1
b=a+'''
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧱️base/🦀️.rs"]
        pub mod base;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖥️host/🦀️.rs"]
        pub mod host;
'''
open(rp,"w",encoding="utf8").write(r.replace(a,b))
