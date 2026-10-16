#!/usr/bin/env python3
# 🧱️ The WP1 crate table: key -> package directory root, module names it owns (monolith hub names) and kind.
from lib import *

# module name (as the monolith knows it) -> crate key, for the WP1 units; the mapping of 12.5.3 / Appendix G.
OWNER = {
    "artifact_io": "abi", "operation_authority": "abi", "wasip2": "abi", "sqlite_wire": "abi",
    "host": "actor-host", "document_backbone_binding": "actor-host",
    "artifact_identity": "identity",
    "local_interaction": "interaction", "interaction_topology": "interaction",
    "tool_cancellation": "operation", "completion_effects": "operation", "completion_vacant": "operation",
    "bounded_initializer_laws": "operation",
    "window_config": "window-config",
    "window_surface": "window-transient", "window_mutation": "window-transient", "engagement": "window-transient", "window_transient": "window-transient",
    "window_kits": "window-kits", "media_kit": "window-kits", "text_kit": "window-kits", "image_kit": "window-kits", "mesh_kit": "window-kits",
    "tree_kit": "window-kits-rich", "table_kit": "window-kits-rich", "world3d_host": "window-kits-rich", "document_kit": "window-kits-rich",
}
ROOT = {  # crate key -> directory under PLUG that holds the crate root file
    "abi": "🚪️io", "actor-host": "🌐host", "identity": "🪪️identity", "interaction": "🕹️interaction", "operation": "♻️cancellation",
    "window-config": "🪟️window/🎚️config", "window-transient": "🪟️window", "window-kits": "🪟️window-kits", "window-kits-rich": "🪟️window-kits/🧱️rich",
}
ORDER = ["abi", "actor-host", "identity", "interaction", "operation", "window-config", "window-transient", "window-kits", "window-kits-rich"]

# WP1 files that are not hub units: crate key, parent module path, file (relative to PLUG)
NESTED = {
    "executor": ("actor-host", ["reactor"], "⚛️reactor/🧵️executor/🦀️.rs"),
    "operation_progress": ("operation", ["tool_jobs"], "⏳️operation-progress/🦀️.rs"),
    "context_menu": ("window-kits", ["window_kits"], "🖱️context-menu/🦀️.rs"),
    "original_catalog_constructor": ("operation", ["document_ingress"], "🌱️initialization/🏪️catalog/🦀️.rs"),
}
