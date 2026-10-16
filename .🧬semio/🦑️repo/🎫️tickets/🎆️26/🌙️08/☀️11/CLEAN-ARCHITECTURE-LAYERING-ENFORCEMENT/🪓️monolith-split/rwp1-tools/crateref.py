#!/usr/bin/env python3
# 🔎️ Which `crate::<module>` roots and `super::` names each WP1 source file mentions.
import collections
import re
import sys
from common import *

FILES = {
    "root": R + "/🦀️.rs",
    "registry": R + "/📇️registry/🦀️.rs",
    "catalog": T + "/📇️catalog/🦀️.rs",
    "runtime-assets": T + "/🔤️runtime-assets/🦀️.rs",
    "frame-latency": T + "/⏱️frame-latency/🦀️.rs",
    "mesh": REPO + "/🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🦀️.rs",
    "element-contracts": T + "/🧬️element-contracts/🦀️.rs",
    "media-slots": T + "/🎬️media-slots/🦀️.rs",
    "media-slots-slot": T + "/🎬️media-slots/📤️slot/🦀️.rs",
    "media-slots-props": T + "/🎬️media-slots/📥️props/🦀️.rs",
    "media-slots-collect": T + "/🎬️media-slots/🎟️collect/🦀️.rs",
    "native-temporal": T + "/🕰️native-temporal/🦀️.rs",
    "native-accessibility": T + "/♿️native-accessibility/🦀️.rs",
    "mailbox-core": T + "/📮️runtime-mailbox-core/🦀️.rs",
    "asset-authority": R + "/🗂️asset-authority/🦀️.rs",
    "glb-decode": R + "/🦴️glb-decode/🦀️.rs",
    "asset-decode-jobs": R + "/🔁️asset-decode-jobs/🦀️.rs",
    "socket-door": T + "/🔌️socket-door/🦀️.rs",
    "directory-door": T + "/📇️directory-door/🦀️.rs",
    "kernel-runtime": R + "/📦️kernel-runtime/🦀️.rs",
    "kernel-core": R + "/📦️kernel-runtime/🧮️kernel-core/🦀️.rs",
    "kernel-replay-client": R + "/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs",
    "kernel-pool": R + "/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs",
    "kernel-retained-state": R + "/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs",
    "scale-bench": R + "/📊️scale-bench/🦀️.rs",
    "parallel-runtime": T + "/🎠️runtime/🦀️.rs",
    "dock": E + "/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs",
    "hub-sign-in": E + "/🧱️elements/🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs",
    "space-browser": E + "/🧱️elements/🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs",
    "hub-connection": E + "/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs",
    "space-administration": E + "/🧱️elements/🛂️SpaceAdministration/🎯️targets/🧊️wgpu/🦀️.rs",
    "canvas-presence": E + "/🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu/🦀️.rs",
    "agent-bridge": E + "/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs",
    "agent-presence": E + "/🧱️elements/🚦️AgentPresence/🎯️targets/🧊️wgpu/🦀️.rs",
    "agent-approvals": E + "/🧱️elements/🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs",
    "agent-chat-panel": E + "/🧱️elements/💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs",
    "program-bridge": E + "/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs",
    "agent-bridge-door": T + "/🌉️agent-bridge-door/🦀️.rs",
    "native-runtime-modules": T + "/⌨️native-entrypoint/📦️modules/🦀️.rs",
    "canvas2d-paint": E + "/🧱️elements/📐️Canvas2dHost/🎨️paint/🦀️.rs",
}

if __name__ == "__main__":
    for name, path in FILES.items():
        try:
            text = read(path)
        except FileNotFoundError:
            print(f"{name:24s} MISSING")
            continue
        mods = collections.Counter(re.findall(r"(?<![A-Za-z0-9_])crate::([A-Za-z_][A-Za-z0-9_]*)", text))
        sup = collections.Counter(re.findall(r"(?<![A-Za-z0-9_])super::([A-Za-z_*][A-Za-z0-9_]*)", text))
        glob = "use super::*" in text
        print(f"{name:24s} glob={glob!s:5s} crate::{dict(mods)}  super::{dict(sup)}")
