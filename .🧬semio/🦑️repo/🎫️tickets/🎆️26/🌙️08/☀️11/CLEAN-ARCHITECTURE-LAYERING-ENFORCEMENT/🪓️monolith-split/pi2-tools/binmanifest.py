#!/usr/bin/env python3
"""🧳️ Manifest of a companion test bin: every dependency the mounted sources name is a [dev-dependencies] line (the bin's library is empty outside `cfg(test)`); first-party crates carry the test features they declare."""
import json, os, re
from lib import *
from rs import blank
import manifest as MF
import world as Wd

CR = Wd.O.CR
TEST_FEATS = ["artifact-app-testing", "media-turn-testing", "reserved-producer-testing", "original-actor-context-testing"]
_FEATS = {}


def declared_features(k2):
    if k2 not in _FEATS:
        try:
            t = read(REPO + "/" + CR[k2]["pkg"] + "/Cargo.toml")
        except OSError:
            t = ""
        m = re.search(r"(?ms)^\[features\]\n(.*?)(?=^\[|\Z)", t)
        _FEATS[k2] = set(re.findall(r"(?m)^([A-Za-z0-9_\-]+)\s*=", m.group(1))) if m else set()
    return _FEATS[k2]


def idents(texts):
    ids = MF.used_crate_idents(texts)
    for t in texts:
        for m in re.finditer(r"\bextern\s+crate\s+([a-z][a-z0-9_]*)", blank(t)):
            ids.add(m.group(1))
    return ids


def plan(K, texts, provider=False):
    """(manifest text, report)"""
    man = MF.pkg_dir(K) + "/Cargo.toml"
    text = read(man)
    secs = MF.sections(text)
    cut = next((a for h, a, b in secs if h in ("[dependencies]", "[dev-dependencies]", "[features]")), len(text))
    head = text[:cut].rstrip("\n") + "\n"
    skeleton = MF.dep_lines(text)
    skel = {MF.norm(k): v for k, v in skeleton.items()}
    fac = MF.facade_deps()
    facn = {MF.norm(k): v for k, v in fac.items()}
    fp = {MF.PFX_ + k.replace("-", "_"): k for k in CR if CR[k].get("kind") in ("lower", "instance", "companion")}
    fp["semio_framework_plugin_test_support"] = "test-support"
    used = idents(texts)
    rows, missing = [], []
    if "semio_framework_plugin" in used:
        feats = ", ".join(f'"{f}"' for f in TEST_FEATS)
        rows.append(("semio-framework-plugin", f'semio-framework-plugin = {{ path = "{os.path.relpath(MF.PKG_FACADE, MF.pkg_dir(K))}", features = [{feats}] }}'))
    for ident in sorted(used):
        if ident in fp:
            k2 = fp[ident]
            if k2 == K:
                continue
            feats = [f for f in TEST_FEATS if f in declared_features(k2)]
            spec = f'path = "{os.path.relpath(MF.pkg_dir(k2), MF.pkg_dir(K))}"'
            if feats:
                spec += ", features = [" + ", ".join(f'"{f}"' for f in feats) + "]"
            rows.append((f"semio-framework-plugin-{k2}", f"semio-framework-plugin-{k2} = {{ {spec} }}"))
        elif ident in skel:
            h, line = skel[ident]
            rows.append((ident, MF.relative_spec(line, MF.pkg_dir(K), MF.pkg_dir(K))))
        elif ident in facn:
            h, line = facn[ident]
            line = re.sub(r",\s*optional\s*=\s*true", "", line)
            line = re.sub(r"optional\s*=\s*true\s*,\s*", "", line)
            rows.append((ident, MF.relative_spec(line, MF.PKG_FACADE, MF.pkg_dir(K))))
    body = "\n".join(l for _, l in sorted(rows, key=lambda r: r[0]))
    new = head + ("\n[dependencies]\n" + body + "\n\n[dev-dependencies]\n" if provider else "\n[dependencies]\n\n[dev-dependencies]\n" + body + "\n")
    feats = MF.used_features(texts)
    new, fadd = MF.ensure_features(new, feats)
    rep = dict(added=[(k, "[dev-dependencies]") for k, _ in rows], up=[], features=fadd, missing=missing)
    return new, rep
