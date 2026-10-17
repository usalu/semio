#!/usr/bin/env python3
# ✂️ Splits the FlowSnapshot-dependent functions out of the artifact root's content regions into a new 🔖️SnapshotBridge region, so the content core no longer depends on the snapshot type.
ROOT="/Users/ueli/Documents/semio"
p=ROOT+"/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs"
t=open(p,encoding="utf8").read()
def cut(t,start,end):
    assert t.count(start)==1,start
    i=t.index(start); j=t.index(end,i)+len(end)
    return t[:i]+t[j:],t[i:j]
t,genesis=cut(t,"/// 🌱️ The initial pack for the `content` child","    Some(<SemioFlowSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&flow_content_snapshot_from_working(&scene.widgets, &scene.synapses, &scene.layout)))\n}\n\n")
t,composed=cut(t,"/// 🪆️ The flow document every reader and writer works on","    Ok(FlowSnapshot { schema: snapshot.schema.clone(), content })\n}\n\n")
t,scene=cut(t,"/// 🔎 Reads the current document's live widgets/synapses/layout off its `content` child handle","    flow_working_scene_for_handle(&snapshot.content)\n}\n\n")
anchor="//#endregion 🔖️WorkingScene\n"
assert t.count(anchor)==1
bridge="\n//#region 🔖️SnapshotBridge\n"+scene+genesis+composed.rstrip("\n")+"\n//#endregion 🔖️SnapshotBridge\n"
t=t.replace(anchor,anchor+bridge)
t=t.replace("pub(crate) fn widget_from_node(","pub fn widget_from_node(").replace("pub(crate) fn flow_content_child_from_digest(","pub fn flow_content_child_from_digest(")
open(p,"w",encoding="utf8").write(t)
