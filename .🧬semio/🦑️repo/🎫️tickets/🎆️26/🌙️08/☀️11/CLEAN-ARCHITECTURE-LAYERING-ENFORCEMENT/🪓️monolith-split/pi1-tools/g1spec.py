#!/usr/bin/env python3
"""🧱️ Spec of group G1: child, composition-emission, composition-publication, publication-mounted."""
from lib import *
import groupdrv as G

CHILD_CONTENT = """pub(crate) mod child_content {
    pub use semio_framework_plugin_child::*;
    pub use semio_framework_plugin_composition_emission::{metadata::*, root::*};
    pub use semio_framework_plugin_composition_publication::group::*;
    pub use semio_framework_plugin_publication_mounted::{command_mounted::*, owner::*, mounted_child_group_identity, mounted_child_group_receipt, mounted_child_group_receipt_group};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_identity::{MountedChildGroupIdentityIssuer, MountedChildGroupIdentitySource};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_receipt_group::command::PagedCommandLog;
}"""


class S(G.Spec):
    name = "g1"
    group = ["child", "composition-emission", "composition-publication", "publication-mounted"]
    roots_b = {"child": PLUG + "/🪆️child/🦀️.rs", "composition-publication": PLUG + "/🧩️composition/📬️publication/🦀️.rs"}
    promote = {PLUG + "/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🦀️.rs": "command_mounted"}
    skip_files = {
        PLUG + "/🧩️composition/📨️emission/📦️preparation/🎟️receipt/🧪️tests/🦀️.rs",
        PLUG + "/🧩️composition/📨️emission/📦️preparation/🦀️.rs",
    }
    skip_mounts = {"original_tests", "child_emit_preparation"}
    hub_file = PLUG + "/🪝️mounts/👥️wp3-owners/🦀️.rs"
    hub_units = {
        "child_emit_preparation": "pub(crate) use semio_framework_plugin_decl_emission::child_emit_preparation;",
        "child_content": CHILD_CONTENT,
        "private_child_input": "pub(crate) use semio_framework_plugin_composition_emission::private_child_input;",
        "store_publication": "pub(crate) extern crate semio_framework_plugin_composition_publication as store_publication;",
    }
    delete = [PLUG + "/🧩️composition/📨️emission/📦️preparation/🦀️.rs"]


if __name__ == "__main__":
    G.main(S)
