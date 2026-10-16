"""Companion crate file generation: Cargo.toml / project.json with dependency lines re-rooted from the production manifest."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E
import extcrates

ROOT = E.ROOT


def reroot(line, extra):
    return re.sub(r'path = "((?:\.\./)+)', lambda m: 'path = "' + "../" * extra + m.group(1), line)


def dep_specs(manifest_text):
    """{ident: [(section, line)]} for every dependency line of the manifest."""
    out = {}
    sec = None
    for line in manifest_text.splitlines():
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1)
            continue
        if sec and sec.endswith("dependencies") and "=" in line and not line.lstrip().startswith("#"):
            key = line.split("=", 1)[0].strip()
            if re.match(r"^[A-Za-z0-9_\-]+$", key):
                out.setdefault(key.replace("-", "_"), []).append((sec, line.strip(), key))
    return out


def merged_line(entries):
    """(section, line): the normal dependency (any target table) wins over the dev one; features of all entries are united; `optional` is dropped."""
    normal = [e for e in entries if e[0] != "dev-dependencies" and not e[0].endswith(".dev-dependencies")]
    dev = [e for e in entries if e not in normal]
    first = (normal or dev)[0]
    if len(entries) == 1 and "optional" not in first[1]:
        sec0 = first[0] if not first[0].endswith("dev-dependencies") else "dependencies"
        return sec0, first[1]
    sec, base = first[0], first[1]
    if sec.endswith("dev-dependencies"):
        sec = "dependencies"
    feats = set()
    for e in entries:
        for m in re.finditer(r'features\s*=\s*\[([^\]]*)\]', e[1]):
            feats |= {x.strip().strip('"') for x in m.group(1).split(",") if x.strip()}
    base = re.sub(r",\s*optional\s*=\s*true", "", base)
    base = re.sub(r"optional\s*=\s*true\s*,\s*", "", base)
    if feats:
        fl = ", ".join(f'"{x}"' for x in sorted(feats))
        if "features" in base:
            base = re.sub(r'features\s*=\s*\[[^\]]*\]', f"features = [{fl}]", base)
        else:
            base = base.rstrip("}").rstrip() + f", features = [{fl}] }}"
    return sec, base


def cargo_text(name, cid, desc, depth, deps, p_name, p_rel, p_features=(), extra_sections=""):
    ws = "/".join([".."] * depth)
    pf = f", features = [{', '.join(chr(34) + f + chr(34) for f in p_features)}]" if p_features else ""
    body = f'''[package]
workspace = "{ws}"
name = "{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{desc}"
publish = false

[package.metadata.semio]
role = "framework"
id = "{cid}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
{p_name} = {{ path = "{p_rel}"{pf} }}
'''
    by = {}
    for sec, line in deps:
        by.setdefault(sec, []).append(line)
    for line in by.get("dependencies", []):
        body += line + "\n"
    for sec in by:
        if sec != "dependencies":
            body += f"\n[{sec}]\n" + "\n".join(by[sec]) + "\n"
    body += extra_sections
    return body


def project_json(project):
    return '{\n  "name": "%s",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n' % project
