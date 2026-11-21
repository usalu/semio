import json
import sys
from pathlib import Path


def units_in(build):
    for stamp in build.glob("*/*/fingerprint/*.json"):
        unit = stamp.parent.parent
        marker = stamp.with_suffix("")
        if marker.is_file():
            yield marker.read_text(), (unit, stamp, json.loads(stamp.read_text()))


def admitted(root, inventory):
    pending = [root]
    seen = set()
    while pending:
        unit, stamp, entry = pending.pop()
        if stamp in seen:
            continue
        seen.add(stamp)
        yield unit, stamp, entry
        for _, _, _, fingerprint in entry.get("deps", []):
            marker = fingerprint.to_bytes(8, "little").hex()
            if marker in inventory:
                pending.append(inventory[marker])


def prepare(build, hub_hash, generated):
    inventory = dict(units_in(build))
    stamp = build / "semio-hub-fem" / hub_hash / "fingerprint" / "lib-semio_hub_fem.json"
    root = inventory[stamp.with_suffix("").read_text()]
    closure = list(admitted(root, inventory))
    args = ["--crate-name=fem3d_native_original_actor_probe", "--edition=2024", "-Cdebuginfo=0"]
    for directory in sorted({unit / "out" for unit, _, _ in closure}):
        if directory.is_dir():
            args.append(f"-Ldependency={directory}")
            if any(directory.glob("*.lib")):
                args.append(f"-Lnative={directory}")
    context = next(unit for unit in closure if unit[1].stem == "lib-semio_plugin_reactor_context")
    direct = {
        name: inventory[fingerprint.to_bytes(8, "little").hex()][0]
        for anchor in (root, context)
        for _, name, _, fingerprint in anchor[2]["deps"]
        if fingerprint.to_bytes(8, "little").hex() in inventory
    }
    packages = ("semio_hub_fem", "semio_plugin", "semio_framework", "semio_framework_actor",
                "semio_framework_value", "semio_os_kernel", "serde_json")
    for name in packages:
        pools = [direct[name]] if name in direct else [
            unit for unit, stamp, _ in closure if stamp.stem == f"lib-{name}"
        ]
        libraries = [library for pool in pools for library in (pool / "out").glob(f"lib{name}-*.rlib")]
        if name == "serde_json":
            libraries = libraries[:1]
        assert len(libraries) == 1, (name, libraries)
        library = libraries[0]
        metadata = library.with_suffix(".rmeta")
        args.append(f"--extern={name}={library}")
        if metadata.is_file():
            args.append(f"--extern={name}={metadata}")
    args.extend([str(Path(__file__).with_name("probe.rs")), "-o", str(generated / "fem3d-native-probe.exe")])
    (generated / "native-probe-rustc.rsp").write_text("\n".join(args) + "\n", encoding="utf-8")


if __name__ == "__main__":
    prepare(Path(sys.argv[1]), sys.argv[2], Path(sys.argv[3]))
