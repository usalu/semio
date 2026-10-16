#!/usr/bin/env python3
"""🧱️ Spec of group G8: runtime-ops, extension, reactor-surfaces (WP4 ranks 30 to 33); reactor-surfaces owns the cold-pair statics of the reactor root block."""
from lib import *
import groupdrv as G
import statics as ST
import translate as TR

SURF = ["COLD_PAIR_INGRESS", "COLD_PAIR_DOCUMENT_LOAD", "RESTORED_DOCUMENT_LOADS"]
TR.STATIC_OWNER.update({n: "reactor-surfaces" for n in SURF})
USES = "use semio_framework_plugin_reactor_tasks::tasks_fixed_slots::PLUGIN_REACTOR_INSTANCE_SLOTS;\n"


def host_imports(text):
    for a, b in [("crate::plugin_runtime::extension_runtime_lift::extension_retirement::ExtensionResourceOwner", "semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner"),
                 ("crate::plugin_runtime::ops_ingress::PluginExchangeOutput", "semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput"),
                 ("crate::plugin_runtime::ops_typed_operations::TypedOperationGrant", "semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant")]:
        text = text.replace(a, b)
    return text


class S(G.Spec):
    name = "g8"
    group = ["runtime-ops", "extension", "reactor-surfaces"]
    roots_b = {"extension": PLUG + "/🧩️extension/🦀️.rs", "reactor-surfaces": PLUG + "/⚛️reactor/📥️cold-pair/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/⚛️wp4-reactor/🦀️.rs"
    hub_units = {}
    root_extra = {"runtime-ops": "\n#[cfg(test)]\nextern crate semio_framework_os_kernel as dsl;\n", "reactor-surfaces": ST.owned_block(SURF, USES, rewrite=[("cold_pair::", "")])}
    extra_edits = {
        PLUG + "/🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/📦️runtime-instances/🦀️.rs": host_imports,
        ST.ROOT: ST.leave_root({"semio_framework_plugin_reactor_surfaces": SURF}),
        PLUG + "/⚛️reactor/📥️cold-pair/🦀️.rs": ST.widen_types(["ColdPairDocumentLoad", "ColdDocumentPairIngressRegistry"]),
    }


if __name__ == "__main__":
    G.main(S)
