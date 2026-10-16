#!/usr/bin/env python3
"""📦️ Manifest edits of the flipped crates: `[lib] path`, first-party and external dependency lines derived from what the crate's sources name."""
import json, os, re
from lib import *
from rs import blank
import world as Wd

CR = Wd.O.CR
PKG_FACADE = REPO + "/" + PLUG_REL + "/" + PKGDIR


def pkg_dir(K):
    return REPO + "/" + CR[K]["pkg"]


def sections(text):
    """[(header, start, end)] byte spans of the top-level sections of a manifest"""
    out = []
    heads = [(m.start(), m.group(0).strip()) for m in re.finditer(r"(?m)^\[[^\]]+\]\s*$", text)]
    for i, (pos, h) in enumerate(heads):
        end = heads[i + 1][0] if i + 1 < len(heads) else len(text)
        out.append((h, pos, end))
    return out


def dep_lines(text, header_rx=r"\[(?:target\.[^\]]*\.)?(?:dev-)?dependencies\]"):
    """{key: (header, line)} of dependency lines in all dependency tables"""
    out = {}
    for h, a, b in sections(text):
        if re.fullmatch(header_rx, h):
            for line in text[a:b].split("\n")[1:]:
                m = re.match(r"^([A-Za-z0-9_\-]+)\s*=", line)
                if m:
                    out[m.group(1)] = (h, line)
    return out


def norm(key):
    return key.replace("-", "_")


def facade_deps():
    return dep_lines(read(PKG_FACADE + "/Cargo.toml"))


def relative_spec(line, from_dir, to_dir):
    """rewrite the path = ".." of a dependency line written for from_dir so it is valid from to_dir"""
    m = re.search(r'path\s*=\s*"([^"]+)"', line)
    if not m:
        return line
    absolute = os.path.normpath(os.path.join(from_dir, m.group(1)))
    return line[:m.start(1)] + os.path.relpath(absolute, to_dir) + line[m.end(1):]


def used_crate_idents(texts):
    ids = set()
    for t in texts:
        c = blank(t)
        for m in re.finditer(r"(?<![A-Za-z0-9_:$.])([a-z][a-z0-9_]*)::", c):
            ids.add(m.group(1))
        for m in re.finditer(r"\buse\s+([a-z][a-z0-9_]*)\b", c):
            ids.add(m.group(1))
        for m in re.finditer(r"\buse\s*\{\s*([a-z][a-z0-9_]*)::", c):
            ids.add(m.group(1))
    return ids


def macro_derive_hint(texts):
    """derive macros and attribute macros bring crate dependencies without a path"""
    need = set()
    joined = "\n".join(blank(t) for t in texts)
    return need, joined


def plan_manifest(K, prod_texts, test_texts, flip_lib=True):
    """(new manifest text, report dict): first-party and external dependencies the sources name; test-only ones go to [dev-dependencies]"""
    man = pkg_dir(K) + "/Cargo.toml"
    text = read(man)
    new = text
    report = dict(added=[], up=[])
    if flip_lib:
        new = re.sub(r'(?m)^(\[lib\]\s*\npath\s*=\s*)"[^"]+"', r'\1"../../🦀️.rs"', new)
    have = dep_lines(new)
    haveidents = {norm(k): k for k in have}
    fac = facade_deps()
    facidents = {norm(k): k for k in fac}
    fp = {PFX_ + k.replace("-", "_"): k for k in CR if CR[k].get("kind") in ("lower", "instance")}
    prod = used_crate_idents(prod_texts)
    test = used_crate_idents(test_texts) - prod
    rank = CR[K]["rank"]
    adds = {"[dependencies]": [], "[dev-dependencies]": []}
    for ident, table in [(i, "[dependencies]") for i in sorted(prod)] + [(i, "[dev-dependencies]") for i in sorted(test)]:
        if ident in haveidents:
            continue
        if ident in fp:
            k2 = fp[ident]
            if k2 == K:
                continue
            if CR[k2]["rank"] > rank and CR[k2].get("kind") != "support":
                report["up"].append((ident, table))
                continue
            adds[table].append((ident, f'semio-framework-plugin-{k2} = {{ path = "{os.path.relpath(pkg_dir(k2), pkg_dir(K))}" }}'))
        elif ident in facidents:
            h, line = fac[facidents[ident]]
            adds[table].append((ident, relative_spec(line, PKG_FACADE, pkg_dir(K))))
    for table, rows in adds.items():
        if not rows:
            continue
        secs = [s for s in sections(new) if s[0] == table]
        lines = "".join(l + "\n" for i, l in rows)
        if not secs:
            new = new.rstrip("\n") + "\n\n" + table + "\n" + lines
        else:
            h, a, b = secs[0]
            body = new[a:b].rstrip("\n")
            tail = new[b:]
            new = new[:a] + body + "\n" + lines + ("\n" + tail.lstrip("\n") if tail.strip() else "")
        for i, l in rows:
            report["added"].append((i, table))
    feats = used_features(list(prod_texts) + list(test_texts))
    new, fadd = ensure_features(new, feats)
    report["features"] = fadd
    return new, report

def used_features(texts):
    out = set()
    for t in texts:
        for mm in re.finditer(r'feature\s*=\s*"([A-Za-z0-9_\-]+)"', blank(t) if False else t):
            out.add(mm.group(1))
    return out


def ensure_features(text, feats):
    """(new manifest text, features added): every feature the sources are gated by is declared in [features]"""
    secs = [s for s in sections(text) if s[0] == "[features]"]
    declared = set()
    if secs:
        h, a, b = secs[0]
        declared = set(re.findall(r"(?m)^([A-Za-z0-9_\-]+)\s*=", text[a:b]))
    add = sorted(f for f in feats if f not in declared)
    if not add:
        return text, []
    lines = "".join(f"{f} = []\n" for f in add)
    if secs:
        h, a, b = secs[0]
        body = text[a:b].rstrip("\n")
        tail = text[b:]
        text = text[:a] + body + "\n" + lines + ("\n" + tail.lstrip("\n") if tail.strip() else "")
    else:
        text = text.rstrip("\n") + "\n\n[features]\n" + lines
    return text, add


def facade_forward(text, feature, key):
    """facade manifest text with `semio-framework-plugin-<key>/<feature>` added to the forwarding list of `feature`"""
    m = re.search(r"(?m)^" + re.escape(feature) + r"\s*=\s*\[(.*)\]\s*$", text)
    if not m:
        return text
    entry = f'"semio-framework-plugin-{key}/{feature}"'
    if entry in m.group(1):
        return text
    body = m.group(1).rstrip()
    new = f"{feature} = [{body}, {entry}]" if body.strip() else f"{feature} = [{entry}]"
    return text[:m.start()] + new + text[m.end():]


PFX_ = "semio_framework_plugin_"
