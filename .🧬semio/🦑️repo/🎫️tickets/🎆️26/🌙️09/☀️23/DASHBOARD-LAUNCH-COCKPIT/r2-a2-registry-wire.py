import re,sys
p=r"C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🎛️dashboard\🎮️registry\🦀️.rs"
s=open(p,encoding="utf8",newline="").read()
start=s.index("// #region 🔖️Wire")
end=s.index("// #endregion 🔖️Wire")+len("// #endregion 🔖️Wire")
new="// #region 🔖️Wire\n/// 🏷️ The wire types of a launch: the daemon schema owns them, the registry produces them.\n///\n/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs\npub use crate::daemon::ipc::{Ready, TaskLabel};\n// #endregion 🔖️Wire"
s=s[:start]+new+s[end:]
open(p,"w",encoding="utf8",newline="").write(s)
print("ok")
