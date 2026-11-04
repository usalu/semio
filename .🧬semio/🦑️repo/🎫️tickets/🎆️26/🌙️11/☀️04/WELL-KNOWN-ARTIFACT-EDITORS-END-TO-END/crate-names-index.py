#!/usr/bin/env python3
# 📦️ Reads Cargo.toml [package] names (TOML only, no cargo) for the stdio artifacts, the hub stdio composition,
# the OS plugin tests, os-io, semio, the wgpu Shell test crate and the energy model. Writes the TSV index and the markdown.
import glob
import os
import sys
import tomllib

ROOT = "/home/deck/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END")
GENERATED = os.path.join(TICKET, "🗑️generated")
ARTIFACTS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
HUB_STDIO = "🌎️hub/🧩️compositions/🗄️stdio"
ENERGY = "✏️s/🔌️plugins/🔋️energy"
OS = "🧰️framework/🛍️products/💻️os/🔨️modules"
SHELL = OS + "/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell"


def cargo_files(*dirs):
    found = set()
    for d in dirs:
        base = os.path.join(ROOT, d)
        if os.path.isfile(base):
            found.add(base)
            continue
        for dirpath, dirnames, filenames in os.walk(base):
            dirnames[:] = [n for n in dirnames if n not in ("node_modules", "target", "🗑️generated")]
            if "Cargo.toml" in filenames:
                found.add(os.path.join(dirpath, "Cargo.toml"))
    return sorted(found)


def package_name(path):
    try:
        with open(path, "rb") as handle:
            data = tomllib.load(handle)
    except Exception as error:  # noqa: BLE001 - reported as data
        return f"ERR {error}"[:80], False
    return data.get("package", {}).get("name", "(workspace-only)"), "workspace" in data


def role_of(rel):
    if "🧪️tests" in rel:
        return "test"
    if "🔮️oracles" in rel:
        return "oracle"
    if "✏️editor" in rel:
        return "editor"
    if "🧬️mutations" in rel:
        return "mutations"
    if "📸️snapshot" in rel:
        return "snapshot"
    if "🚪️io" in rel:
        return "io"
    if "🧬️schema" in rel:
        return "schema"
    return "other"


def artifact_of(rel):
    if rel.startswith(ARTIFACTS + "/"):
        return rel[len(ARTIFACTS) + 1 :].split("/")[0]
    return None


def main():
    rows = []
    for path in cargo_files(ARTIFACTS, HUB_STDIO, OS + "/🔌️plugin/🧪️tests", OS + "/🚪️io", OS + "/🧬️semio", ENERGY, SHELL):
        rel = os.path.relpath(path, ROOT)
        name, workspace = package_name(path)
        art = artifact_of(rel)
        if art is None:
            if rel.startswith(HUB_STDIO):
                group = "hub-stdio"
            elif rel.startswith(ENERGY):
                group = "energy"
            elif rel.startswith(SHELL):
                group = "wgpu-shell"
            elif rel.startswith(OS + "/🔌️plugin/🧪️tests"):
                group = "os-plugin-tests"
            elif rel.startswith(OS + "/🚪️io"):
                group = "os-io"
            elif rel.startswith(OS + "/🧬️semio"):
                group = "os-semio"
            else:
                group = "other"
        else:
            group = art
        rows.append((group, role_of(rel), name, os.path.relpath(os.path.dirname(path), ROOT), workspace))
    rows.sort(key=lambda r: (r[0], r[1], r[2], r[3]))
    os.makedirs(GENERATED, exist_ok=True)
    with open(os.path.join(GENERATED, "crate-names-index.tsv"), "w", encoding="utf-8") as tsv:
        tsv.write("group\trole\tpackage\tdirectory\tworkspace\n")
        for r in rows:
            tsv.write("\t".join([r[0], r[1], r[2], r[3], str(r[4]).lower()]) + "\n")
    print(len(rows), "packages", file=sys.stderr)


if __name__ == "__main__":
    main()
