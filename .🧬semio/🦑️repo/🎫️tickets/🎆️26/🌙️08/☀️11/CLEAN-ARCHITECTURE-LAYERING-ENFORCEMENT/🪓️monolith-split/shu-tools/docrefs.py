import os,sys
R="/Users/ueli/Documents/semio"
E=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
EL=E+"/🧱️elements"
TW="🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx"
UR="🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts"
RC="🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx"
TT="🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts"
LB="🛠️ShellHelpers/🏷️labels/🟦️.tsx"
rows=[
 (R+"/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts","`🛠️ShellHelpers/🟦️.tsx`): a node's AUTHORED","`"+TW+"`): a node's AUTHORED"),
 (R+"/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🟦️.ts","`buildUiRefreshRequest` (`🛠️ShellHelpers/🟦️.tsx`)","`buildUiRefreshRequest` (`"+UR+"`)"),
 (R+"/🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts","`🛠️ShellHelpers/🟦️.tsx`). */","`"+TT+"`). */"),
 (R+"/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts","(`🛠️ShellHelpers/🟦️.tsx`'s `loadPluginModuleResilient`)","(`"+RC+"`'s `loadPluginModuleResilient`)"),
 (R+"/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts","`ShellHelpers/🟦️.tsx`'s `resolveManifestLabel`","`"+LB+"`'s `resolveManifestLabel`"),
 (E+"/🧪️tests/🎛️retained-control-commit/🟦️.ts","`🛠️ShellHelpers/🟦️.tsx`'s `uiInputField`","`"+TW+"`'s `uiInputField`"),
 (E+"/🧪️tests/🎛️retained-control-commit/🟦️.ts","`🛠️ShellHelpers/🟦️.tsx`'s `uiIntentPayload`","`"+TW+"`'s `uiIntentPayload`"),
 (E+"/🧪️tests/🫀️settle-pump/🟦️.ts","(`🛠️ShellHelpers/🟦️.tsx`'s `createUiRefreshCoalescerV1`)","(`"+UR+"`'s `createUiRefreshCoalescerV1`)"),
 (EL+"/🏛️ShellHost/🗄️shell-uri-documents/🟦️.ts","`ShellHelpers/🟦️.tsx`, outside this lane's lease","`"+RC+"`, outside this lane's lease"),
 (EL+"/🔌️PluginRuntime/🔗️channel-adapter/🟦️.ts","`🐚️🌿️ShellHelpers/🟦️.tsx`","`"+RC+"`"),
 (EL+"/🔌️PluginRuntime/🫀️load-progress/🟦️.ts","(`🛠️ShellHelpers/🟦️.tsx`) while","(`"+RC+"`) while"),
 (EL+"/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx","Split out of `🛠️ShellHelpers/🟦️.tsx` so a law can mount them without importing that module's\n * whole shell surface (which cycles through `🏛️ShellHost`/`🐚️Shell`).","Split out of the `🛠️ShellHelpers` shell surface so a law can mount them without importing that surface\n * (which cycles through `🏛️ShellHost`/`🐚️Shell`)."),
 (EL+"/🛠️ShellHelpers/📌️panel/🟦️.ts","importing `🛠️ShellHelpers/🟦️.tsx` from a test hits the\n * `ShellHelpers → Shell → ShellHost` cycle","importing the shell-surface modules of `🛠️ShellHelpers` from a test hits the\n * `ShellHelpers → Shell → ShellHost` cycle"),
]
for p,old,new in rows:
    s=open(p,encoding="utf8").read()
    c=s.count(old)
    if c!=1: print("SKIP",c,p.split("/")[-2],old[:50]); continue
    open(p,"w",encoding="utf8").write(s.replace(old,new)); print("ok",p.split("/")[-2])
