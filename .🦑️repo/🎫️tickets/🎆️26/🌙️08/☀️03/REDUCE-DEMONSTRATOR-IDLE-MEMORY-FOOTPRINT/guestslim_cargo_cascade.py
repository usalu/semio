#!/usr/bin/env python3
"""One-off T1.3 GuestSlim Cargo.toml cascade: adds a forwarding `render` feature (default ON) to
the six infinite_board/flow_core kernel wrapper crates, and appends `default-features = false` to
every leaf guest-plugin crate's dependency line on infinite_canvas/flow_core/the board kernels —
so guest (wasm32-wasip2) builds drop typst-assets' embedded font bytes while host/engine
(wasm32-unknown-unknown wasm-pack + native) builds keep render=ON unaffected. Ticket-scoped,
throwaway; not a permanent project script.
"""
import re
import sys

ROOT = "/Users/ueli/Documents/semio/"

# Kernel wrapper crates: (file, own-canvas-dep-line-package, [render-forward targets])
KERNEL_WRAPPERS = {
    "🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🎲️board/⚡️implementation/🦀️rust/Cargo.toml": ["infinite_canvas/render"],
    "🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🎲️board/➕️normal/↔undirected/⚡️implementation/🦀️rust/Cargo.toml": ["infinite_board/render"],
    "🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🎲️board/🔌️port/➡️directed/⚡️implementation/🦀️rust/Cargo.toml": ["infinite_board/render", "infinite_board_normal_undirected/render", "infinite_canvas/render"],
    "🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🎲️board/🔌️port/➡️directed/➕️normal/⚡️implementation/🦀️rust/Cargo.toml": ["infinite_board_port_directed/render", "infinite_board_normal_undirected/render", "infinite_canvas/render"],
    "🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🎲️board/🔌️port/➡️directed/🕸️dag/⚡️implementation/🦀️rust/Cargo.toml": ["infinite_board_port_directed/render", "infinite_canvas/render"],
    "🧰️framework/🛍️product/💻️os/🔨️module/🌊️flow/🫀️core/⚡️implementation/🦀️rust/Cargo.toml": ["infinite_board_port_directed_dag/render", "infinite_canvas/render"],
}

# Known engine crates: never touched, must keep render=ON (default, unmodified).
ENGINE_SKIP = {
    "🧰️framework/🔨️module/✍️editor/⚡️implementation/🦀️rust/Cargo.toml",
    "🧰️framework/🔨️module/🗺️surface/🎨️paint/⚡️implementation/🦀️rust/Cargo.toml",
    "🧰️framework/🔨️module/🗺️surface/🕸️node-graph/⚡️implementation/🦀️rust/Cargo.toml",
    "🧰️framework/🔨️module/🗺️surface/🗺️tiled-map/⚡️implementation/🦀️rust/Cargo.toml",
    "🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementation/🦀️rust/Cargo.toml",
}

KERNEL_PACKAGE_NAMES = [
    "semio-framework-os-kernel-infinite-canvas",
    "semio-framework-os-kernel-flow-core",
    "semio-framework-os-kernel-infinite-board",
    "semio-framework-os-kernel-infinite-board-normal-undirected",
    "semio-framework-os-kernel-infinite-board-port-directed",
    "semio-framework-os-kernel-infinite-board-port-directed-normal",
    "semio-framework-os-kernel-infinite-board-port-directed-dag",
]

DEP_LINE_RE = re.compile(r'^(\w+)\s*=\s*\{([^}]*package\s*=\s*"(' + "|".join(KERNEL_PACKAGE_NAMES) + r')"[^}]*)\}\s*$', re.MULTILINE)


def add_default_features_false(text: str) -> tuple[str, int]:
    count = 0

    def repl(m: re.Match) -> str:
        nonlocal count
        inner = m.group(2)
        if "default-features" in inner:
            return m.group(0)
        count += 1
        return f'{m.group(1)} = {{{inner.rstrip()}, default-features = false }}'

    return DEP_LINE_RE.sub(repl, text), count


def find_files(names):
    return [ROOT + n for n in names]


def process_leaf(path: str) -> int:
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    new_text, count = add_default_features_false(text)
    if count:
        with open(path, "w", encoding="utf-8") as f:
            f.write(new_text)
    return count


def process_kernel_wrapper(path: str, forwards: list) -> int:
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    # Step 1: same treatment as a leaf for its OWN deps on other kernel crates (kernel wrappers
    # also declare normal dependency lines on their sibling kernel crates / infinite_canvas).
    text, count = add_default_features_false(text)
    # Step 2: inject/extend a [features] block with default=["render"] + render=[...forwards].
    render_line = f'render = [{", ".join(chr(34) + f + chr(34) for f in forwards)}]'
    if "[features]" in text:
        if re.search(r"^render\s*=", text, re.MULTILINE):
            print(f"  SKIP (already has render feature): {path}")
            return count
        text = text.replace("[features]\n", f"[features]\ndefault = [\"render\"]\n{render_line}\n", 1)
    else:
        text = text.replace("[lints]\nworkspace = true\n", f'[lints]\nworkspace = true\n[features]\ndefault = ["render"]\n{render_line}\n', 1)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    return count


def main():
    import subprocess

    result = subprocess.run(
        ["grep", "-rl", "-E", "|".join(f'package = "{p}"' for p in KERNEL_PACKAGE_NAMES), ROOT + "✏️s", ROOT + "🧰️framework"],
        capture_output=True, text=True,
    )
    all_files = [line for line in result.stdout.splitlines() if line.endswith("Cargo.toml") and "node_modules" not in line]
    all_rel = [f[len(ROOT):] for f in all_files]

    kernel_rel = set(KERNEL_WRAPPERS.keys())
    engine_rel = ENGINE_SKIP
    leaf_rel = [r for r in all_rel if r not in kernel_rel and r not in engine_rel]

    print(f"total consumer files found: {len(all_rel)}")
    print(f"kernel wrappers: {len(kernel_rel)}, engines skipped: {len(engine_rel & set(all_rel))}, leaves: {len(leaf_rel)}")

    total_edits = 0
    print("\n-- kernel wrappers --")
    for rel, forwards in KERNEL_WRAPPERS.items():
        c = process_kernel_wrapper(ROOT + rel, forwards)
        print(f"  {c} dep-line edit(s) + render feature: {rel}")
        total_edits += c

    print("\n-- leaves --")
    for rel in sorted(leaf_rel):
        c = process_leaf(ROOT + rel)
        if c == 0:
            print(f"  WARNING: no dep line matched in {rel}")
        else:
            print(f"  {c} dep-line edit(s): {rel}")
        total_edits += c

    missing_engine = engine_rel - set(all_rel)
    if missing_engine:
        print(f"\nNOTE: expected engine files not found in consumer set (fine if they don't depend on these kernels): {missing_engine}")

    print(f"\ntotal dependency-line edits: {total_edits}")


if __name__ == "__main__":
    main()
