"""🗺️ The ui-scene crate model: crate table, external name catalog, import synthesis and dependency inference."""
import re
from lib import *

PFX = "semio_framework_ui_scene_"

CRATES = {}


def crate(key, **kw):
    kw.setdefault("deps", [])
    kw["key"] = key
    kw["pkg"] = "semio-framework-ui-scene-" + key
    kw["ident"] = PFX + key.replace("-", "_")
    CRATES[key] = kw
    return kw


def ident(key):
    return CRATES[key]["ident"]


EXTERNAL = {
    "semio_framework_geometry": "semio-framework-geometry",
    "serde": "serde",
    "serde_json": "serde_json",
    "semio_framework_mesh_engine": "semio-framework-mesh-engine",
    "protocol": "protocol",
    "semio_framework_ui_viewport": "semio-framework-ui-viewport",
    "semio_framework_pack_json": "semio-framework-pack-json",
    "semio_framework_value": "semio-framework-value",
    "ui_contract": "ui_contract",
    "fnv": "fnv",
}

VEC3_METHODS = ["add_m", "sub_m", "scale_m", "dot_m", "cross_m", "length_m", "normalize_m", "to_array_m"]
MAT4_METHODS = ["mul_m", "transform_point_m", "inverse_m", "to_cols_array_m"]


def parse_catalog(header_text):
    cat = {}
    for m in re.finditer(r"^\s*(?:#\[cfg\(test\)\]\s*)?(?:pub\s+)?use\s+([^;]+);", header_text, re.M):
        stmt = "use " + m.group(1)
        if m.group(1).startswith("mesh3d_retirement"):
            continue
        for n, full in use_names(stmt):
            cat[n] = full
    return cat


TRAIT_METHODS = {}
for _m in VEC3_METHODS:
    TRAIT_METHODS[_m] = "Vec3Math"
for _m in MAT4_METHODS:
    TRAIT_METHODS[_m] = "Mat4Math"
TRAIT_METHODS.update({"to_value": "ToValue", "from_value": "FromValue", "encode_pack": "SceneDoc", "decode_pack": "SceneDoc"})


def used_trait_imports(text):
    need = []
    for m, tr in TRAIT_METHODS.items():
        if tr in need:
            continue
        if re.search(r"(?:\.|::)\s*" + m + r"\b", text):
            need.append(tr)
    return need


crate("algebra", node="📐️math/🧮️algebra")
crate("camera", node="📐️math/🎥️camera")
crate("mesh", node="📐️math/🕸️mesh")
crate("pass", node="📐️math/🎞️pass")
crate("picking", node="📐️math/🎯️picking")
crate("pack", node="📦️pack")
crate("doc", node="🎬️scenes/📜️doc")
crate("modelling", node="📏️world3d-modelling")
crate("world3d", node="🎬️scenes/🌍️world3d")
crate("node-graph", node="🎬️scenes/🕸️node-graph")
crate("plane", node="🎬️scenes/🖼️plane")
crate("views", node="🎬️scenes/📄️views")
crate("surface", node="🌉️surface")
