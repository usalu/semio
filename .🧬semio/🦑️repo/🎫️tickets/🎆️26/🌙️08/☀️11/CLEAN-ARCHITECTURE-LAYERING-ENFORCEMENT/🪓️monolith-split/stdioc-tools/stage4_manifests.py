#!/usr/bin/env python3
# 📦️ Stage 4: derives each producer's dependency tables from its source closure and writes the manifests, project files and the workspace member rows.
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(__file__))
from lib import *  # noqa: E402,F401,F403

BASE_MANIFEST = open(os.environ["SCRATCH"] + "/baseline/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml", encoding="utf8").read()
CONTRACT_PKG = C + "/📦️packages/🦀️rust"


def section(text, name):
    match = re.search(r"(?ms)^\[" + re.escape(name) + r"\]\n(.*?)(?=^\[|\Z)", text)
    return match.group(1) if match else ""


def parse_specs(body):
    out = {}
    for line in body.split("\n"):
        match = re.match(r'^([A-Za-z0-9_.\-]+?)(?:\.workspace)?\s*=\s*(.+)$', line)
        if not match:
            continue
        key, rest = match.group(1), match.group(2).strip()
        workspace = line.startswith(key + ".workspace") or "workspace = true" in rest
        spec = {"workspace": workspace, "raw": rest, "key": key}
        path = re.search(r'path = "([^"]+)"', rest)
        if path:
            spec["target"] = os.path.normpath(os.path.join(CONTRACT_PKG, path.group(1)))
        features = re.search(r"features = \[([^\]]*)\]", rest)
        if features:
            spec["features"] = features.group(1).strip()
        version = re.match(r'^"([^"]+)"$', rest)
        if version:
            spec["version"] = version.group(1)
        out[key] = spec
    return out


DEPS = parse_specs(section(BASE_MANIFEST, "dependencies"))
DEV = parse_specs(section(BASE_MANIFEST, "dev-dependencies"))
DEV_ONLY = {"semio-framework-trace", "base64", "json-patch", "semio-framework-ui-scene", "serde_json"}


def spec_line(owner_pkgdir, key, spec, features=None):
    if spec.get("workspace"):
        return f"{key} = {{ workspace = true }}"
    if "target" in spec:
        rel = os.path.relpath(spec["target"], owner_pkgdir)
        extra = f", features = [{features}]" if features else ""
        return f'{key} = {{ path = "{rel}"{extra} }}'
    return f'{key} = "{spec["version"]}"'


def sibling_line(owner_pkgdir, crate):
    return f'{crate["package"]} = {{ path = "{os.path.relpath(crate["pkgdir"], owner_pkgdir)}" }}'


def collect(crate):
    files = closure(crate["root"])
    normal, dev, siblings_normal, siblings_dev = set(), set(), set(), set()
    flagged = []
    for path, is_test in files.items():
        text = read(path)
        gated = re.findall(r"#\[cfg\(test\)\]\s*\n#\[global_allocator\]\s*\nstatic[^\n]*\n", text)
        for block in gated:
            dev |= external_tokens(block)
            text = text.replace(block, "")
        tokens = external_tokens(text)
        sibs = sibling_tokens(text)
        sibs.discard(crate["package"])
        for include in INCLUDE.findall(strip_comments(text)):
            pass
        if is_test:
            dev |= tokens
            siblings_dev |= sibs
        else:
            normal |= tokens
            siblings_normal |= sibs
            for name in tokens & DEV_ONLY:
                flagged.append((os.path.relpath(path, C), name))
    return files, normal, dev - normal, siblings_normal, siblings_dev - siblings_normal, flagged


def write_manifest(crate):
    files, normal, dev, sib_normal, sib_dev, flagged = collect(crate)
    pkgdir = crate["pkgdir"]
    lines = ["[package]", f'workspace = "{os.path.relpath(STDIO, pkgdir)}"', f'name = "{crate["package"]}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", f'description = "{crate["description"]}"', "", "[package.metadata.semio]", 'role = "s-module"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', ""]
    dep_lines = []
    for token in sorted(normal):
        if token in DEPS:
            dep_lines.append((token, spec_line(pkgdir, token, DEPS[token])))
        elif token in DEV:
            dep_lines.append((token, spec_line(pkgdir, token, DEV[token])))
        else:
            raise SystemExit(f"{crate['key']}: no spec for normal dependency {token}")
    for package in sorted(sib_normal):
        dep_lines.append((package, sibling_line(pkgdir, next(c for c in CRATES if c["package"] == package))))
    dep_lines.sort(key=lambda row: row[0])
    dev_lines = []
    for token in sorted(dev):
        spec = DEV.get(token) or DEPS.get(token)
        assert spec, f"{crate['key']}: no spec for dev dependency {token}"
        dev_lines.append((token, spec_line(pkgdir, token, spec, spec.get("features"))))
    kernel_dev = any(is_test and re.search(r"(?<![A-Za-z0-9_])kernel::|semio_framework_os_kernel", strip_comments(read(path))) for path, is_test in files.items())
    if kernel_dev:
        spec = DEV["semio-framework-os-kernel"]
        dev_lines.append(("semio-framework-os-kernel", spec_line(pkgdir, "semio-framework-os-kernel", spec, spec.get("features"))))
    for package in sorted(sib_dev):
        dev_lines.append((package, sibling_line(pkgdir, next(c for c in CRATES if c["package"] == package))))
    dev_lines.sort(key=lambda row: row[0])
    seen = set()
    final_dev = []
    for key, line in dev_lines:
        if (key, line) not in seen:
            seen.add((key, line))
            final_dev.append(line)
    lines.append("[dependencies]")
    lines.extend(line for _, line in dep_lines)
    lines.append("")
    lines.append("[dev-dependencies]")
    lines.extend(final_dev)
    lines.append("")
    write(pkgdir + "/Cargo.toml", "\n".join(lines))
    schema = os.path.relpath(ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json", pkgdir)
    project = {"name": crate["project"], "$schema": schema, "projectType": "library", "tags": ["language:rust", "role:contract", "family:stdio"]}
    write(pkgdir + "/📋️project.json", json.dumps(project, ensure_ascii=False, indent=2) + "\n")
    return files, normal, dev, sib_normal, sib_dev, flagged


if __name__ == "__main__":
    for crate in CRATES:
        files, normal, dev, sib_normal, sib_dev, flagged = write_manifest(crate)
        print(crate["package"], "files", len(files), "normal", len(normal), "dev", len(dev), "sib", sorted(c.replace(FACADE + "-", "") for c in sib_normal | sib_dev), "FLAG" if flagged else "", flagged[:4])
