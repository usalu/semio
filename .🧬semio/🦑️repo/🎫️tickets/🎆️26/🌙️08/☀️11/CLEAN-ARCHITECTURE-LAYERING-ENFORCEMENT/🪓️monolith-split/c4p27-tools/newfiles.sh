#!/bin/zsh
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p27-tools/env.sh
cd "$E"
for d in $(cat /dev/null); do :; done
python3 - <<'PY'
import os
E=os.environ["E"]
spec={
 "🔗️AgentBridge/🧪️tests":["🔭️config-discovery","🧬️frame-codec","🛠️tool-call-lifecycle","🚏️artifact-routes","🤝️handshake-ladder"],
 "📐️Canvas2dHost":["📷️camera-math","🖍️layer-paint","👆️pointer-lane","🗂️layers-session"],
 "🔗️HubConnection":["🌐️ui-labels","🔌️connection-contract","🛰️fetch-port"],
 "🐚️Shell":["📦️program-sessions","🔒️boot-and-locks","👥️peer-interaction","🗃️state-shapes","🎛️shell-actions","♻️slice-reducers","🕹️shell-store"],
 "📌️ChromePanels":["🆔️panel-ids","🖥️display-panel","⚙️settings-general","🎨️settings-theme","⌨️settings-keybindings","📱️settings-defaults-and-conflicts","🏪️marketplace-panel"],
 "🌐️World3dHost/🧱️instance-layers":["🎛️scene-gumball","🧊️instance-node","☁️point-cloud-layer","🌪️vortex-markers","🪟️instance-chrome-store"],
 "🌐️World3dHost":["🛤️tool-run-trace-lane","🖱️interaction-domain-args","🌍️scene-lanes","🧭️fit-and-presence","🏗️brush-mesh-and-menus","👇️hover-and-placement","🔀️paint-and-projection","🎛️marquee-and-gumball","👆️pointer-and-drop"],
}
for parent,names in spec.items():
    for n in names:
        d=os.path.join(E,parent,n)
        for f in sorted(os.listdir(d)): print(os.path.join(d,f))
PY
