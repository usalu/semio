import sys,re
S=sys.argv[1]; E=S+'/✏️editor/'
P='crate::editor::puzzle5d::'
M='crate::standards::v1::subsets::any::schema::mutations::'
jobs={
 '🖌️session/📋️clipboard':['semio_framework_job::InteractiveJob','semio_framework_plugin::ArtifactReservedJob'],
 '🎮️commands/🔁️cycle-brush-candidate':[P+'commands::add_brush_part::puzzle5d_brush_source_grip'],
 '🎮️commands/🎯️focus-selection':[P+'view::gumball_target_world'],
 '📌️panels/🗿️artifact':[P+'catalog::puzzle5d_part_display_label',P+'interaction::ui_label'],
 '🎭️modes/✏️edit':[P+'entities::puzzle5d_action'],
 '🎭️modes/✏️edit/🛠️tools/🪣️fill':[P+'modes::edit::options::brush::distribution_children'],
 '🧠️precompute':[P+'view::engine_grip_kind'],
 '🧰️kit/➕️add-node-and-brush-part':[M+'PUZZLE5D_FLAT_TO_WORLD'],
 '🎮️commands/🧭️set-suggestion-offset':[P+'entities::puzzle5d_absolute_or_delta'],
 '🎮️commands/🌍️world-relocate':[P+'modes::edit::windows::world3d::utilities::transform::puzzle5d_relocate_record'],
 '🎮️commands/✅️accept-suggestion':[P+'precompute::brush::puzzle5d_brush_placement'],
 '🎮️commands/🗑️delete-selection':[P+'entities::puzzle5d_grip_full_id'],
 '🎮️commands/👯️duplicate-selection':[M+'create_part'],
 '🎮️commands/🛑️engagement-abort':[P+'view::puzzle5d_fill_tool_active'],
 '🎮️commands/📽️set-projection':[P+'config::puzzle5d_camera3d_distance'],
 '🎮️commands/📡️proximity-connect':[P+'precompute::geometry::distance_squared'],
 '🎮️commands/⚖️set-kind-weight':[P+'catalog::{puzzle5d_ensure_catalog_kind_weights, puzzle5d_kind_ids, puzzle5d_normalize_kind_weight_group}'],
 '🧠️precompute/🪣️fill/🧪️tests/🔬️unit':[P+'Puzzle5dPlayApp'], '🖌️session/📌️paste':['semio_framework_job::InteractiveJob','semio_framework_plugin::ArtifactReservedJob'],
 '🖌️session/📥️snapshot-import':['semio_framework_job::InteractiveJob','semio_framework_plugin::ArtifactReservedJob'],
 '📌️panels/🔍️inspection':[P+'interaction::ui_label',P+'catalog::puzzle5d_part_display_label'],
 '📌️panels/⚙️settings':[P+'interaction::ui_label'],
 '📌️panels/🛍️catalogue':[P+'interaction::ui_label'],
 '🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/🎥️projection':[P+'entities::puzzle5d_action'],
 '🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/☀️sun':[P+'entities::puzzle5d_action'],
 '🎮️commands/🚧️set-brush-placement-contact-tolerance':[P+'entities::puzzle5d_absolute_or_delta'],
 '🎮️commands/📡️proximity-connect/🧪️tests/🔬️unit':[P+'view::engine_grip_kind'],
 '🧪️tests/🧪️transform-tool':[P+'modes::edit::windows::world3d::utilities::transform::puzzle5d_relocate_record'],
 '🧪️tests/🔬️target-volumes':[M+'PUZZLE5D_FLAT_TO_WORLD'],
 '🧪️tests/🔬️unit/☑️option-groups-and-edits':[M+'PUZZLE5D_FLAT_TO_WORLD'],
 '🧪️tests/🔬️unit/🗂️manifest-and-window-actions':[M+'PUZZLE5D_FLAT_TO_WORLD'],
 '🧪️tests/🔬️unit/📤️import-export-and-menus':[P+'entities::puzzle5d_grip_full_id'],
 '🧪️tests/🔬️unit/🔌️in-port-and-lane-verbs':[P+'catalog::puzzle5d_normalize_kind_weight_group'],
 '🧪️tests/🔬️unit/⚖️laws-and-rendering':[P+'catalog::puzzle5d_normalize_kind_weight_group'],
}
for d,uses in jobs.items():
    p=E+d+'/🦀️.rs'; t=open(p,encoding='utf8').read()
    new=[]
    for u in uses:
        leaf=u.split('::')[-1]
        names=[n.strip() for n in leaf.strip('{}').split(',')]
        if all(re.search(r'\buse\b[^;]*\b'+re.escape(n)+r'\b',t) for n in names): continue
        new.append('use '+u+';\n')
    if not new: print('skip',d); continue
    m=re.match(r'((?:\s*//![^\n]*\n)*(?:\s*#!\[[^\n]*\n)*)',t)
    h=m.group(1)
    open(p,'w',encoding='utf8').write(h+''.join(new)+t[len(h):]); print('ok',d,len(new))
