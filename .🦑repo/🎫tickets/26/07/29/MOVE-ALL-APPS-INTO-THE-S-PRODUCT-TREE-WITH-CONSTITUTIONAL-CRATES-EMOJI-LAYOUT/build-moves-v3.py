#!/usr/bin/env python3
"""B0 (round 3): build the crate/package move map for leaf-level ⚡️implementation.
For every crate/package dir at <scope>/⚡️implementation/<lang>/<subpath> with
non-empty <subpath>, the new location is <scope>/<subpath>/⚡️implementation/<lang>.
Driven by root Cargo.toml members (Rust) + root package.json workspaces (JS/TS,
excluding entries whose dir already has a sibling Cargo.toml -- those ride along
with their Rust crate's own move, no separate JS move needed)."""
import re
import os
import json

LANG_MARKER = "⚡️implementation"


def rust_members():
    content = open("Cargo.toml", encoding="utf-8").read()
    block = re.search(r"members = \[(.*?)\]", content, re.S).group(1)
    return re.findall(r'"([^"]+)"', block)


def js_workspaces():
    d = json.load(open("package.json", encoding="utf-8"))
    out = []
    for w in d["workspaces"]:
        if w.startswith("compose/") or "*" in w:
            continue
        out.append(w)
    return out


def compute_new(old):
    parts = old.split("/")
    if LANG_MARKER not in parts:
        return None
    i = parts.index(LANG_MARKER)
    lang = parts[i + 1] if i + 1 < len(parts) else None
    if lang is None:
        return None
    sub = parts[i + 2 :]
    if not sub:
        return None
    new_parts = parts[:i] + sub + [LANG_MARKER, lang]
    return "/".join(new_parts)


def main():
    moves = {}  # old -> new
    for m in rust_members():
        new = compute_new(m)
        if new:
            moves[m] = new

    for w in js_workspaces():
        if w in moves:
            continue
        if not os.path.isdir(w):
            continue
        if os.path.exists(os.path.join(w, "Cargo.toml")):
            continue  # co-located wasm-pack wrapper, rides along with the Rust move
        new = compute_new(w)
        if new:
            moves[w] = new

    entries = [{"old": old, "new": new} for old, new in moves.items()]
    entries.sort(key=lambda e: -e["old"].count("/"))

    # sanity: no duplicate news, every old exists, no old is a prefix of another old
    # (which would indicate a crate nested inside another *moving* crate -- must be
    # handled by depth-order, not a data bug, but flag anyway for visibility)
    news = [e["new"] for e in entries]
    dup_news = len(news) - len(set(news))
    missing_olds = [e["old"] for e in entries if not os.path.isdir(e["old"])]

    print(f"total moves: {len(entries)}")
    print(f"duplicate targets: {dup_news}")
    print(f"missing sources: {len(missing_olds)}")
    for m in missing_olds:
        print("  MISSING:", m)

    with open(
        ".repo/🎫/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/moves-v3.json",
        "w",
        encoding="utf-8",
    ) as f:
        json.dump(entries, f, ensure_ascii=False, indent=2)


if __name__ == "__main__":
    main()
