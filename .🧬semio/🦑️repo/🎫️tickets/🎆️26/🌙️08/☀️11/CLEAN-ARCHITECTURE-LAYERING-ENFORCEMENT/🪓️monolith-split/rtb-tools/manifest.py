"""📦️ Cargo manifests of the new rtb crates: dependency lines are copied verbatim from the manifests that already declare them (same depth, same paths)."""
import collections, os, re
from lib import *

DEP_TABLE = re.compile(r"^\[((?:target\.[^\]]+\.)?(?:dependencies|dev-dependencies|build-dependencies))\]\s*$")
FIRST_PARTY = PKG_PREFIX
SNAP_TOP = SNAP + "/E/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml"
TESTKIT_CRATES = None


def pool_manifests():
    out = []
    for suffix, d in sorted(crate_table().items()):
        if suffix.startswith("shell-tests-") or suffix.endswith("-tests") or suffix in ("test-support",) or "-tests-" in suffix:
            out.append(T + "/" + d + "/📦️packages/🦀️rust/Cargo.toml")
    return out


def parse_deps(path):
    """[(table, key, raw line)] of one manifest, comments dropped."""
    table, out = None, []
    for l in read(path).split("\n"):
        m = DEP_TABLE.match(l)
        if m:
            table = m.group(1)
            continue
        if l.startswith("["):
            table = None
            continue
        if table and l.strip() and not l.lstrip().startswith("#"):
            km = re.match(r"^([A-Za-z0-9_\-]+)\s*=", l)
            if km:
                out.append((table, km.group(1), l))
    return out


def pool():
    best = {}
    for mf in pool_manifests():
        for table, key, line in parse_deps(mf):
            best.setdefault((table, key), line)
    for table, key, line in parse_deps(SNAP_TOP):
        line = re.sub(r'path = "((?:\.\./)+)', lambda m: 'path = "../' + m.group(1), line.replace("={", " = {"))
        best.setdefault((table, key), line)
    return best


def has_testkit(suffix, crates):
    mf = T + "/" + crates[suffix] + "/📦️packages/🦀️rust/Cargo.toml"
    return os.path.exists(mf) and re.search(r"^\[features\][^\[]*\btestkit\b", read(mf), re.M | re.S) is not None


def ident_candidates(ident):
    return [ident, ident.replace("_", "-")]


def build_manifest(name, ident_id, description, idents, first_party_extra=(), force_testkit=True, dev_keys=()):
    """idents: crate identifiers used by the sources (`semio_framework_os_renderer_wgpu_x`, `ui_wgpu`, `serde_json`, …)."""
    crates = crate_table()
    p = pool()
    keys = collections.OrderedDict()
    miss = []
    for ident in sorted(idents):
        found = False
        for cand in ident_candidates(ident):
            for table in ("dependencies", "dev-dependencies"):
                if (table, cand) in p:
                    keys.setdefault(cand, True)
                    found = True
            for (table, key), line in p.items():
                if key == cand:
                    keys.setdefault(cand, True)
                    found = True
        if ident.startswith(PFX) and not found:
            suffix = ident[len(PFX):].replace("_", "-")
            if suffix in crates:
                keys.setdefault(PKG_PREFIX + suffix, True)
                found = True
        if not found:
            miss.append(ident)
    fp = []
    for k in first_party_extra:
        keys.setdefault(k, True)
    main, targets, dev = [], collections.OrderedDict(), []
    for (table, key), line in sorted(p.items(), key=lambda kv: kv[0]):
        if key not in keys or key.startswith(PKG_PREFIX):
            continue
        if table == "dev-dependencies" and key not in dev_keys:
            if not any(k2 == key and t2 != "dev-dependencies" for (t2, k2) in p):
                main.append(line)
            continue
        if table == "dependencies":
            main.append(line)
        elif table == "dev-dependencies":
            dev.append(line)
        else:
            targets.setdefault(table, []).append(line)
    for k in list(keys):
        if k.startswith(PKG_PREFIX):
            suffix = k[len(PKG_PREFIX):]
            if suffix in crates:
                feats = ', features = ["testkit"]' if force_testkit and has_testkit(suffix, crates) else ""
                main.append(f'{k} = {{ path = "../../../{crates[suffix]}/📦️packages/🦀️rust"{feats} }}')
    main = sorted(set(main), key=lambda l: (not l.startswith(PKG_PREFIX), l))
    head = f'''[package]
workspace = "../../../../../../../../../../.."
name = "{PKG_PREFIX}{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{description}"
publish = false

[package.metadata.semio]
role = "framework"
id = "os-renderer-wgpu-{name}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"
'''
    body = head + "\n[dependencies]\n" + "\n".join(main) + "\n"
    for tname, ls in targets.items():
        body += f"\n[{tname}]\n" + "\n".join(sorted(ls)) + "\n"
    if dev:
        body += "\n[dev-dependencies]\n" + "\n".join(sorted(dev)) + "\n"
    return body, miss


def read_manifest_deps(path):
    return {(t, k) for t, k, l in parse_deps(path)}


def add_deps(path, idents, table="dependencies", testkit_first_party=True):
    """Adds the dependency lines for the identifiers the sources use and the manifest lacks; returns the added keys."""
    text = read(path)
    have = {k for t, k, l in parse_deps(path)}
    crates = crate_table()
    p = pool()
    add = []
    for ident in sorted(idents):
        keys = ident_candidates(ident)
        if ident.startswith(PFX):
            suffix = ident[len(PFX):].replace("_", "-")
            key = PKG_PREFIX + suffix
            if suffix in crates and key not in have and os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(path)))) != crates[suffix]:
                feats = ', features = ["testkit"]' if testkit_first_party and has_testkit(suffix, crates) else ""
                add.append((key, f'{key} = {{ path = "../../../{crates[suffix]}/📦️packages/🦀️rust"{feats} }}'))
            continue
        for cand in keys:
            if cand in have:
                break
            hit = p.get(("dependencies", cand)) or p.get(("dev-dependencies", cand))
            if hit:
                add.append((cand, hit))
                break
    if not add:
        return []
    header = f"[{table}]"
    if header not in text:
        text = text.rstrip("\n") + f"\n\n{header}\n"
    i = text.index(header) + len(header)
    end = text.find("\n[", i)
    end = len(text) if end < 0 else end
    block = [l for l in text[i:end].split("\n") if l.strip()]
    block += [l for k, l in add]
    fp = sorted(l for l in block if l.startswith(PKG_PREFIX))
    rest = sorted(l for l in block if not l.startswith(PKG_PREFIX))
    text = text[:i] + "\n" + "\n".join(fp + rest) + "\n" + text[end:]
    edit(path, text)
    return [k for k, l in add]


def set_testkit_feature(path, members):
    text = read(path)
    line = "testkit = [" + ", ".join(f'"{m}"' for m in members) + "]"
    if re.search(r"(?m)^\[features\]", text):
        if re.search(r"(?m)^testkit\s*=", text):
            text = re.sub(r"(?m)^testkit\s*=\s*\[[^\]]*\]", line, text)
        else:
            text = re.sub(r"(?m)^\[features\]\n", "[features]\n" + line + "\n", text, count=1)
    else:
        text = text.replace("\n[dependencies]\n", f"\n[features]\n{line}\n\n[dependencies]\n", 1)
    edit(path, text)


def first_party_testkit_members(path):
    """`dep/testkit` for every first-party dependency of the manifest that declares a testkit feature."""
    crates = crate_table()
    out = []
    for t, k, l in parse_deps(path):
        if t in ("dependencies",) or t.startswith("target"):
            if k.startswith(PKG_PREFIX):
                suffix = k[len(PKG_PREFIX):]
                if suffix in crates and has_testkit(suffix, crates):
                    out.append(k + "/testkit")
    return sorted(set(out))
