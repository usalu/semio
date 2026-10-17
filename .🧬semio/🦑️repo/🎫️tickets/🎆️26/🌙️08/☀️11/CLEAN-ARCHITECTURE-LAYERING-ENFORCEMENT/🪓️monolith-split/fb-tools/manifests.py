"""📦 FB carve, stage 2: Cargo.toml, project.json and the engine manifest edit for the four new crates."""
import os, re, sys
R = "/Users/ueli/Documents/semio"
E = f"{R}/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine"
PKG = "📦️packages/🦀️rust"

NEW = {
    "kernel-contract": dict(dir="🧬️kernel-contract", pkg="semio-framework-3d-brep-kernel-contract", id="3d-brep-kernel-contract",
        desc="Contract of the 3D kernel's B-Rep interface - the geometry handle, kind, topology and error vocabulary and the model-free synchronous BrepKernel trait the arena-backed engine implements",
        deps=["semio-framework-3d-brep-representation.workspace = true", "semio-framework-mesh-engine.workspace = true", "semio-framework-value.workspace = true", "semio-framework-value-derive.workspace = true"], dev=[]),
    "mesh-io": dict(dir="📦️mesh-io", pkg="semio-framework-3d-brep-mesh-io", id="3d-brep-mesh-io",
        desc="Triangle-mesh exchange of the 3D kernel - STL, OBJ and GLB import and export over native B-Rep bodies, the bounded retained mesh-import cursor and the analytic preview mesh conversion",
        deps=["semio-framework-3d-brep-kernel-contract.workspace = true", "semio-framework-3d-brep-construction.workspace = true", "semio-framework-3d-brep-curve.workspace = true", "semio-framework-3d-brep-representation.workspace = true", "semio-framework-3d-brep-surface.workspace = true", "semio-framework-3d-brep-tessellation.workspace = true", "semio-framework-3d-brep-tolerance.workspace = true", "semio-framework-3d-brep-vector.workspace = true", "@MCR@", "semio-framework-mesh-engine.workspace = true", "semio-framework-value.workspace = true"],
        dev=["semio-framework-async-macros.workspace = true", "@PACKJSON@", "parry3d = \"0.17\"", "serde_json.workspace = true"]),
    "shape-value": dict(dir="🪅️shape-value", pkg="semio-framework-3d-brep-shape-value", id="3d-brep-shape-value",
        desc="Owned standalone B-Rep geometry value of the 3D kernel - a minimal body holding exactly the entity closure of one shape's roots, its root reference, every persistent label and a session-free tessellation job",
        deps=["semio-framework-3d-brep-kernel-contract.workspace = true", "semio-framework-3d-brep-curve.workspace = true", "semio-framework-3d-brep-representation.workspace = true", "semio-framework-3d-brep-surface.workspace = true", "semio-framework-3d-brep-tessellation.workspace = true", "semio-framework-3d-brep-vector.workspace = true", "semio-framework-hash.workspace = true", "@PACKJSON@", "semio-framework-value.workspace = true", "semio-framework-value-derive.workspace = true"], dev=[]),
    "tests": dict(dir="🧪️tests", pkg="semio-framework-3d-brep-engine-tests", id="3d-brep-engine-tests", role="test",
        desc="Black-box laws of the arena-backed B-Rep engine through its public surface - the sync API, the operation jobs, the shape value and the session analysis",
        deps=[], dev=["semio-framework-3d-brep-engine.workspace = true", "semio-framework-3d-brep-analysis.workspace = true", "semio-framework-3d-brep-boolean.workspace = true", "semio-framework-3d-brep-queries.workspace = true", "semio-framework-3d-brep-representation.workspace = true", "semio-framework-3d-brep-tessellation.workspace = true", "semio-framework-async-macros.workspace = true", "@PACKJSON@", "semio-framework-value.workspace = true", "parry3d = \"0.17\"", "serde_json.workspace = true"]),
}


def rel_path(from_dir, to_dir):
    return os.path.relpath(to_dir, from_dir)


def manifest_text(key):
    n = NEW[key]
    pkg_dir = f"{E}/{n['dir']}/{PKG}"
    depth = os.path.relpath(pkg_dir, f"{R}/🧰️framework").count("/") + 1
    ws = "/".join([".."] * depth)
    mcr = f"semio-framework-mesh-component-references = {{ path = \"{rel_path(pkg_dir, R + '/🧰️framework/🔨️modules/🏗️mesh-engine/🎯️component-references/' + PKG)}\" }}"
    pj = f"semio-framework-pack-json = {{ path = \"{rel_path(pkg_dir, R + '/🧰️framework/🔨️modules/🎒️pack/🔤️json/' + PKG)}\" }}"
    deps = [mcr if d == "@MCR@" else pj if d == "@PACKJSON@" else d for d in n["deps"]]
    dev = [mcr if d == "@MCR@" else pj if d == "@PACKJSON@" else d for d in n["dev"]]
    role = n.get("role", "framework")
    return (
        "[package]\n"
        f"workspace = \"{ws}\"\n"
        f"name = \"{n['pkg']}\"\n"
        "version.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n"
        f"description = \"{n['desc']}\"\n\n"
        "[package.metadata.semio]\n"
        f"role = \"{role}\"\nid = \"{n['id']}\"\n\n"
        "[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n"
        "[lints]\nworkspace = true\n\n"
        "[lib]\npath = \"../../🦀️.rs\"\n\n"
        "[dependencies]\n" + "".join(d + "\n" for d in sorted(deps, key=lambda s: s.split(" ")[0].split(".")[0])) + "\n"
        "[dev-dependencies]\n" + "".join(d + "\n" for d in sorted(dev, key=lambda s: s.split(" ")[0].split(".")[0]))
    )


def project_json(key):
    n = NEW[key]
    return '{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n' % n["id"]


if __name__ == "__main__":
    write = "--write" in sys.argv
    for key in NEW:
        n = NEW[key]
        d = f"{E}/{n['dir']}/{PKG}"
        mt = manifest_text(key)
        pj = project_json(key)
        print("==", key, d.replace(R + "/", "")[-90:], len(d.replace(R + "/", "").encode()) + 40)
        print(mt)
        if write:
            os.makedirs(d, exist_ok=True)
            for name, text in (("Cargo.toml", mt), ("📋️project.json", pj)):
                p = f"{d}/{name}"
                assert not os.path.exists(p), p
                with open(p, "w", encoding="utf-8") as f:
                    f.write(text)
