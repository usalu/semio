#!/usr/bin/env python3
"""🧲️ Moves the mounted instance's inherent `impl VcsArtifactApp` blocks out of facet modules (wave E2, step 1).

An inherent impl must live in the crate that defines its type, so every `impl … VcsArtifactApp …` block found in
a facet module (jobs, publication, window) is the instance's own turn code. This tool cuts those blocks (with
their docs and attributes) out of each listed source file and appends them, in order, to the instance child
module given for it, which `artifact_instance` declares; the child starts from the old `app` preamble, which the
import pipeline (`🧹️imports.py`) then narrows. It compiles nothing.
Usage: extract-impls.py [--write]
"""
import importlib.util, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file)); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m
ri = load("rust_items", "🧱️rust_items.py")
dc = load("decompose", "🪚️decompose.py")
PLUGIN = dc.PLUGIN
INSTANCE = os.path.join(PLUGIN, "🧵️instance/🦀️.rs")
MOVES = [
    ("⏪️time-travel/🦀️.rs", "🧵️instance/⏪️time-travel", "time_travel_turns", "⏪️ The instance's time-travel turns: history editing sessions, replay and supersession."),
    ("⏯️tool-run/🦀️.rs", "🧵️instance/⏯️tool-run", "tool_run_turns", "⏯️ The instance's tool-run turns: ledger driving, tick folding and finalize publication."),
    ("🛠️tool-machine/🦀️.rs", "🧵️instance/🛠️tool-machine", "tool_machine_turns", "🛠️ The instance's tool-machine turns: gesture, scrub and typing dispatch."),
    ("⏳️operation-progress/🦀️.rs", "🧵️instance/⏳️operation-progress", "operation_progress_turns", "⏳️ The instance's operation-progress publication."),
    ("🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs", "🧵️instance/🔄️full-operation", "full_operation_turns", "🔄️ The instance's mounted typed-operation frontier turns."),
    ("🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs", "🧵️instance/🤝️group", "child_group_turns", "🤝️ The instance's private child group publication and mounted command log turns."),
    ("🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🦀️.rs", "🧵️instance/🤝️group", "child_group_turns", None),
]


def main():
    write = "--write" in sys.argv
    out = {}
    docs = {}
    edits = {}
    for src, dst, mod, doc in MOVES:
        f = os.path.join(PLUGIN, src)
        raw = edits.get(f) or open(f, encoding="utf-8").read()
        cl = ri.clean(raw)
        mask = dc.literal_mask(raw)
        its, _ = ri.items(raw, cl, 0, len(cl))
        cut = [it for it in its if it["kind"] == "impl" and it.get("self") == "VcsArtifactApp" and not it.get("trait")]
        if not cut: print("[extract] nothing in", src); continue
        blocks = []
        for it in sorted(cut, key=lambda x: x["start"], reverse=True):
            blocks.append(raw[it["start"]:it["end"]].strip("\n"))
            raw = raw[:it["start"]] + raw[it["end"]:]
        blocks.reverse()
        edits[f] = re.sub(r"\n{3,}", "\n\n", raw)
        key = (dst, mod)
        out.setdefault(key, []).extend(blocks)
        if doc: docs[key] = doc
        print(f"[extract] {src}: {len(cut)} impl blocks, {sum(b.count(chr(10)) + 1 for b in blocks)} lines -> {dst}")
    decls = []
    for (dst, mod), blocks in out.items():
        content = f"//! {docs[(dst, mod)]}\nuse super::*;\n\n" + "\n\n".join(blocks) + "\n"
        target = os.path.join(PLUGIN, dst, "🦀️.rs")
        decls.append(f'#[path = "{os.path.relpath(target, os.path.dirname(INSTANCE))}"]\nmod {mod};')
        if write:
            if os.path.exists(target): raise SystemExit(f"[extract] exists: {target}")
            os.makedirs(os.path.dirname(target), exist_ok=True)
            open(target, "w", encoding="utf-8").write(content)
    if write:
        for f, raw in edits.items(): open(f, "w", encoding="utf-8").write(raw)
        inst = open(INSTANCE, encoding="utf-8").read().rstrip("\n")
        open(INSTANCE, "w", encoding="utf-8").write(inst + "\n\n" + "\n".join(decls) + "\n")
    print("\n".join(decls))


if __name__ == "__main__":
    main()
