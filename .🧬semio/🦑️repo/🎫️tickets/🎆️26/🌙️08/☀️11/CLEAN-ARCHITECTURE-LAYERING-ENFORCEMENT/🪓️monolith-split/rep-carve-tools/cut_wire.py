"""✂️ Splits `📡️wire/🦀️.rs` into frames (stays), interaction and presence roots; hands the six wire combinators to wire-core."""
import re
from rlib import *
from cut_mutation import register_moved

WIRE = R + "📡️wire/🦀️.rs"
INTERACTION = R + "📡️wire/🕹️interaction/🦀️.rs"
PRESENCE = R + "📡️wire/👥️presence/🦀️.rs"
COMBINATORS = ["malformed", "write_opt_str", "read_opt_str", "write_opt_bytes", "read_opt_bytes", "write_vec_bytes", "read_vec_bytes"]
PRESENCE_REEXPORT = "pub use crate::io::presence::{borrow_presence_metadata, BorrowedPresenceMetadata, BorrowedPresenceInteraction, BorrowedPresenceDomain, BorrowedPresenceStrings};\n"

MOUNT_RX = re.compile(r'#\[cfg\(test\)\]\n#\[path = "🧪️tests/([^"]+)"\]\nmod (\w+);\n?')

def cut_mount(text, mod):
    ms = [m for m in MOUNT_RX.finditer(text) if m.group(2) == mod]
    assert len(ms) == 1, (mod, len(ms))
    m = ms[0]
    return text[:m.start()] + text[m.end():], '#[cfg(test)]\n#[path = "../🧪️tests/' + m.group(1) + '"]\nmod ' + mod + ';\n'

def run():
    t = read_base(WIRE)
    assert t.count(PRESENCE_REEXPORT) == 1
    t = t.replace(PRESENCE_REEXPORT, "")
    rest, presence_interaction = cut_region(t, "🔖️PresenceInteraction")
    rest, presence = cut_region(rest, "🔖️Presence")
    rest, interaction = cut_region(rest, "🔖️Interaction")
    rest, ty_mount = cut_mount(rest, "typed_field_order_tests")
    presence, iw_mount = cut_mount(presence, "interaction_wire_tests")
    interaction = interaction.rstrip("\n")

    presence = re.sub(r'#\[path = "🧪️tests/', '#[path = "../🧪️tests/', presence)
    presence_interaction = re.sub(r'#\[path = "🧪️tests/', '#[path = "../🧪️tests/', presence_interaction)
    interaction = re.sub(r'#\[path = "🧪️tests/', '#[path = "../🧪️tests/', interaction)

    frames = rest
    frames, comb = cut_items(frames, COMBINATORS)
    comb_pub = []
    for c in comb:
        m = re.search(r"(^|\n)async fn ", c)
        assert m, c[:80]
        comb_pub.append(c.replace("async fn ", "pub async fn ", 1))

    uses_map = lambda s: "BTreeMap" in bare_ids(s)
    uses = {"frames": uses_map(frames), "presence": uses_map(presence + presence_interaction), "interaction": uses_map(interaction)}
    btree = "use std::collections::BTreeMap;\n"
    if not uses["frames"]:
        frames = frames.replace(btree + "\n", "", 1) if (btree + "\n") in frames else frames.replace(btree, "", 1)
    frames = re.sub(r"\n{3,}", "\n\n", frames)

    pres_doc = ("//! 👥️ Peer presence wire contract: window views, typing, tool runs, history edits and the typed interaction roster, with the borrowed reader and the peer overlay.\n"
                "//!\n"
                "//! Presence travels inside the lane-tagged frames only as opaque blobs; this crate owns what the blobs contain.\n\n")
    inter_doc = ("//! 🕹️ Interaction vocabulary shared by every pointer and selection surface: hover and selection specs, the topology, the selection and hover machines and the state validation.\n\n")
    pres_imports = ""
    if uses["presence"]: pres_imports += btree + "\n"
    inter_imports = ""
    if uses["interaction"]: inter_imports += btree + "\n"
    pres_text = pres_doc + pres_imports + presence.rstrip("\n") + "\n\n" + presence_interaction.rstrip("\n") + "\n"
    inter_text = inter_doc + inter_imports + interaction + "\n\n" + iw_mount + "\n" + ty_mount

    register_moved(WIRE, "presence", presence)
    register_moved(WIRE, "presence", presence_interaction)
    register_moved(WIRE, "interaction", interaction)
    register_moved(WIRE, "wire-core", "\n".join(comb_pub))
    write(WIRE, frames)
    write(PRESENCE, pres_text)
    write(INTERACTION, inter_text)
    return comb_pub

if __name__ == "__main__":
    c = run()
    print(len(c), "combinators")
