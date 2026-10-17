"""✂️ F-04(a) facade self-edit: deletes the aliases `ui`, `abi`, `platform` and `tool_run` from the facade root and re-points the facade's own uses (the platform item re-export, the reserved-contribution constant, the mounted reserved-contribution test). Each file is re-read and every anchor must occur exactly once. usage: facade_self.py [--write]"""
import os, shutil, sys
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
PRE="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rsl-pre/"
write="--write" in sys.argv
def edit(path, pairs):
    t=open(path,encoding="utf8").read(); o=t
    for a,b in pairs:
        assert t.count(a)==1,(path,a[:60],t.count(a))
        t=t.replace(a,b)
    if write and t!=o:
        d=PRE+path
        if not os.path.exists(d): os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copy2(path,d)
        assert open(path,encoding="utf8").read()==o
        open(path,"w",encoding="utf8").write(t)
    print(("WROTE " if write else "DRY ")+path[-60:],len(o)-len(t),"bytes removed")
FAC="🧰️framework/📦️packages/🦀️rust/🦀️.rs"
edit(FAC,[
 ("(`kernel`, `manifest`, `ui`, `abi`, `action_bus`, `platform`, `interaction`, `source_projection`).","(`kernel`, `manifest`, `action_bus`, `interaction`, `onboarding`, `history_edit`, `io_schema`, `sqlite_snapshot`, `mesh_io`)."),
 ("pub use manifest as ui;\npub use semio_framework_abi as abi;\n",""),
 ("pub use semio_framework_action_bus as action_bus;\npub use semio_framework_action_bus::platform;\n","pub use semio_framework_action_bus as action_bus;\n"),
 ("pub use semio_framework_tool_run_actions as tool_run;\n",""),
 ("= [history_edit::RESERVED, tool_run::RESERVED];","= [semio_framework_time_travel_actions::RESERVED, semio_framework_tool_run_actions::RESERVED];"),
 ("pub use platform::{PanelVisibility, Platform, PlatformSpec};","pub use semio_framework_action_bus::platform::{PanelVisibility, Platform, PlatformSpec};"),
])
edit("🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧩️framework-reserved/🦀️.rs",[
 ("contribution.id == history_edit::RESERVED.id","contribution.id == semio_framework_time_travel_actions::RESERVED.id"),
 ("contribution.id == tool_run::RESERVED.id","contribution.id == semio_framework_tool_run_actions::RESERVED.id"),
])
