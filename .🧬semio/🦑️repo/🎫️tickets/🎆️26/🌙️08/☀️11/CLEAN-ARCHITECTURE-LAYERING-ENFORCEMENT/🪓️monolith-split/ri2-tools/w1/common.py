#!/usr/bin/env python3
# 🧭️ Shared constants of the RENDERER-WP1 tools: repo paths and the WP1 source file table.
import os
import re

REPO = "/Users/ueli/Documents/semio"
E = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T = E + "/🎯️targets/🧊️wgpu"
R = T + "/🧊️renderer"
SPLIT = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = SPLIT + "/🗑️generated"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def write(path, text):
    os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
    with open(path, "w", encoding="utf8") as handle:
        handle.write(text)


def rs_files(root):
    for base, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "dist", "target")]
        for name in files:
            if name.endswith(".rs"):
                yield os.path.join(base, name)

ENG_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T_REL = ENG_REL + "/🎯️targets/🧊️wgpu"
PKG_REL = T_REL + "/📦️packages/🦀️rust"
LIB_REL = PKG_REL + "/📚️library/🦀️.rs"
PREFIX = "semio_framework_os_renderer_wgpu_"
PKG_PREFIX = "semio-framework-os-renderer-wgpu-"
RS = SCRATCH + "/rwp1"
SNAP = RS + "/snap0"

# WP1 crate suffix -> folder under T
CRATES = {
    "contracts": "🤝️contracts", "host-base": "🏛️host-base", "native-platform": "🖥️native-platform", "media-slots": "🎥️media-slots",
    "canvas2d-paint": "🖌️canvas2d-paint", "asset-authority": "🗂️asset-authority", "asset-decode": "🦴️asset-decode", "doors": "🛂️doors",
    "kernel-protocol": "📡️kernel-protocol", "kernel-retained": "🗃️kernel-retained", "kernel-pool": "🏊️kernel-pool", "kernel-runtime": "🧠️kernel-runtime",
    "hub-contracts": "🔐️hub-contracts", "hub-workspace": "🏘️hub-workspace", "agent-elements": "🙋️agent-elements", "dock": "🛰️dock",
    "program-bridge": "🪢️program-bridge", "mailbox-core": "📬️mailbox-core",
}

# directories (relative to the engine folder E) that WP1 owns; the baseline sandbox restores them from the pre-carve snapshot
OWNED = [
    "🎯️targets/🧊️wgpu/" + d for d in [
        "📇️catalog", "🔤️runtime-assets", "⏱️frame-latency", "🧬️element-contracts", "🎬️media-slots", "🕰️native-temporal", "♿️native-accessibility",
        "📮️runtime-mailbox-core", "🔌️socket-door", "📇️directory-door", "🎠️runtime", "🌉️agent-bridge-door", "⌨️native-entrypoint/📦️modules",
        "🧊️renderer/📇️registry", "🧊️renderer/🗂️asset-authority", "🧊️renderer/🦴️glb-decode", "🧊️renderer/🔁️asset-decode-jobs", "🧊️renderer/📦️kernel-runtime",
        "🧊️renderer/📊️scale-bench", "🧊️renderer/📥️mounts/🌱️wp1-foundation",
    ] + list(CRATES.values())
] + ["🧱️elements/" + d for d in ["🛰️Dock", "🔐️HubSignIn", "🏘️SpaceBrowser", "🔗️HubConnection", "🛂️SpaceAdministration", "👕️canvas-presence", "🔗️AgentBridge", "🚦️AgentPresence", "🤖️AgentApprovals", "💬️AgentChatPanel", "🌉️ProgramBridge", "📐️Canvas2dHost/🎨️paint"]]
OWNED_FILES = ["🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs", "🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/🦀️.rs"]


def lib_name(suffix):
    return PREFIX + suffix.replace("-", "_")


def _all_crates():
    import re
    out = {}
    for d in sorted(os.listdir(T)):
        mf = os.path.join(T, d, "📦️packages/🦀️rust/Cargo.toml")
        if os.path.isfile(mf):
            m = re.search(r'^name = "' + re.escape(PKG_PREFIX) + r'([a-z0-9-]+)"', read(mf), re.M)
            if m:
                out[m.group(1)] = d
    return out


CRATES.update(_all_crates())
