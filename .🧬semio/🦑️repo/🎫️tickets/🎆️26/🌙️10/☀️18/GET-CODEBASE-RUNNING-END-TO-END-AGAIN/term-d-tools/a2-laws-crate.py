import os
os.chdir('/Users/ueli/Documents/semio')
D='🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal'
F='🧰️framework/🔨️modules'
L=D+'/🧪️tests/🏛️laws/📦️packages/🦀️rust'
os.makedirs(L,exist_ok=True)
def rel(path): return os.path.relpath(path,L)
deps=[
 ('dashboard_terminal',D+'/📦️packages/🦀️rust','semio-framework-repo-dashboard-terminal','features = ["testkit"]'),
 ('dashboard_source_view','🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/📦️packages/🦀️rust','semio-framework-repo-dashboard-source-view','features = ["testkit"]'),
 ('dashboard_feedback',D+'/📣️feedback/📦️packages/🦀️rust','semio-framework-repo-dashboard-feedback',None),
 ('dashboard_keymap_editor',D+'/⚙️settings/⌨️keymap/📦️packages/🦀️rust','semio-framework-repo-dashboard-keymap-editor',None),
 ('dashboard_labels',D+'/🌐️labels/📦️packages/🦀️rust','semio-framework-repo-dashboard-labels',None),
 ('dashboard_launch','🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧾️launch/📦️packages/🦀️rust','semio-framework-repo-dashboard-launch',None),
 ('dashboard_launcher',D+'/🚀️launcher/📦️packages/🦀️rust','semio-framework-repo-dashboard-launcher',None),
 ('dashboard_preferences','🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/📦️packages/🦀️rust','semio-framework-repo-dashboard-preferences',None),
 ('dashboard_registry','🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/📦️packages/🦀️rust','semio-framework-repo-dashboard-registry',None),
 ('dashboard_wire','🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🧵️wire/📦️packages/🦀️rust','semio-framework-repo-dashboard-wire',None),
 ('ui_locale',F+'/🖱️ui/🌐️locale/📦️packages/🦀️rust','semio-framework-ui-locale',None),
 ('ui_styling',F+'/🖱️ui/🎨️styling/📦️packages/🦀️rust','semio-framework-ui-styling',None),
 ('semio-framework-filesystem-workspace',F+'/📁️filesystem/📝️workspace/📦️packages/🦀️rust',None,None),
 ('semio-framework-ui-tui-core',F+'/🖱️ui/⌨️tui/🧱️cell-model/📦️packages/🦀️rust',None,None),
 ('semio-framework-ui-tui-engine',F+'/🖱️ui/⌨️tui/⚙️engine/📦️packages/🦀️rust',None,None),
 ('semio-framework-ui-tui-frame',F+'/🖱️ui/⌨️tui/🏛️frame/📦️packages/🦀️rust',None,None),
 ('semio-framework-ui-tui-terminal',F+'/🖱️ui/⌨️tui/📺️terminal/📦️packages/🦀️rust',None,None),
 ('semio-framework-ui-tui-widgets',F+'/🖱️ui/⌨️tui/🧩️widgets/📦️packages/🦀️rust',None,None),
 ('semio-framework-test-fixtures',F+'/🧪️test/🧲️fixture-reader/📦️packages/🦀️rust',None,None),
]
lines=[]
for name,path,package,extra in deps:
    attrs=['path = "%s"'%rel(path)]
    if package: attrs.append('package = "%s"'%package)
    if extra: attrs.append(extra)
    lines.append('%s = { %s }'%(name,', '.join(attrs)))
lines.append('serde_json = { workspace = true }')
ups='../'*10
manifest='''[package]
workspace = "%s"
name = "semio-framework-repo-dashboard-terminal-laws"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "repo dashboard terminal laws: the native view driven through real keys, pointers, sessions and terminals"

[package.metadata.semio]
role = "tool"
id = "repo-dashboard-terminal-laws"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dev-dependencies]
%s
'''%(ups[:-1],'\n'.join(lines))
open(L+'/Cargo.toml','w',encoding='utf-8').write(manifest)
open(L+'/📋️project.json','w',encoding='utf-8').write('''{
  "name": "@semio-tech/repo-dashboard-terminal-laws-rs",
  "tags": [
    "scope:framework",
    "language:rust"
  ]
}
''')
open(D+'/🧪️tests/🏛️laws/🦀️.rs','w',encoding='utf-8').write('''//! 🏛️ Terminal laws: the native dashboard view driven through real keys, pointers, sessions and terminals.

#[cfg(test)]
#[path = "../🔬️unit/🦀️.rs"]
mod tests;
''')
print(manifest)
