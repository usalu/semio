"""🩹 tc2: idempotent hand fixes that a retransform of the companion files would otherwise undo (white-box module visibility, bare child-module paths, dead unresolved imports)."""
import re
R="/Users/ueli/Documents/semio/"
A=R+"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
EDIT=[
 (A+"📌️panels/🔍️properties/🦀️.rs",'#[path="🎬️actions/🦀️.rs"]\nmod selection_actions;','#[path="🎬️actions/🦀️.rs"]\npub mod selection_actions;'),
 (A+"📌️panels/🔍️properties/🧪️tests/🎛️selection/🦀️.rs","let eligibility=selection_actions::","let eligibility=semio_s_artifact_draw_drawing_editor_interaction::panels::properties::selection_actions::"),
 (A+"🎮️commands/🎛️edit-selection/🦀️.rs",'#[path="🧩️ungroup/🦀️.rs"]\nmod ungroup;','#[path="🧩️ungroup/🦀️.rs"]\npub mod ungroup;'),
 (A+"🎮️commands/🎛️edit-selection/🧪️tests/🔬️unit/🦀️.rs","let result=ungroup::plan(","let result=semio_s_artifact_draw_drawing_editor_commands::edit_selection::ungroup::plan("),
 (A+"🕹️interaction/🧪️tests/🔬️unit/🦀️.rs","#[allow(unused_imports)] use export_job::DrawingExportCommandJobFactory;\n",""),
 (A+"🕹️interaction/🧪️tests/🔬️unit/🦀️.rs","#[allow(unused_imports)] use image_job::DrawingImageCommandJobFactory;\n",""),
 (A+"🕹️interaction/🧪️tests/🔬️unit/🦀️.rs","#[allow(unused_imports)] use path_job::DrawingPathCommandJobFactory;\n",""),
]
for p,old,new in EDIT:
    t=open(p,encoding="utf-8").read()
    if old in t:
        assert t.count(old)==1,(p,old)
        open(p,"w",encoding="utf-8").write(t.replace(old,new)); print("fixed",p.split("/")[-3:-1])
