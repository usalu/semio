"""Makes the cfg(test) items that test files in other crates name visible through `feature = "testkit"` (cfg(test) -> cfg(any(test, feature = "testkit")))."""
import sys
from lib import *
import modtree, manifest
APPLY = "--apply" in sys.argv
BARE_TEST = re.compile(r"#\[cfg\(test\)\]")
ALL_TEST = re.compile(r"#\[cfg\(all\(test,\s*(.*)\)\)\]")
TK = 'any(test, feature = "testkit")'
SPEC = {
    "engine-attach": ["board2d_tool_run_trace_state"],
    "engine-graph": ["component_scene_begin_note_edit_fixture"],
    "engine-registry": ["EngineCanvasPacketDestination"],
    "kernel-protocol": ["CANCELLED_KERNEL_TURNS"],
    "kernel-runtime": ["CANCELLED_KERNEL_TURNS"],
    "program-bridge": ["ProgramFixtureDocument"],
    "scene-ink": ["ink_pointer_state_is_clear"],
    "scene-tables": ["PAINT2D_NAVIGATOR_PADDING", "Paint2dDocSyncJson", "Paint2dFlatLayer"],
}


def retag(line):
    s = line.strip()
    if BARE_TEST.fullmatch(s):
        return line.replace(s, f"#[cfg({TK})]")
    m = ALL_TEST.fullmatch(s)
    if m:
        return line.replace(s, f"#[cfg(all({TK}, {m.group(1)}))]")
    return line


def main():
    crates = crate_table()
    for suffix, names in SPEC.items():
        for f in modtree.tree(T + "/" + crates[suffix] + "/🦀️.rs"):
            if "/🧪️tests/" in f:
                continue
            text = read(f)
            lines = text.split("\n")
            changed = False
            for it in itm.table(text):
                if it["name"] in names and it["kind"] not in ("impl", "mod"):
                    for k in range(it["start"] - 1, it["head"] - 1):
                        if lines[k].lstrip().startswith("#[cfg(") and "testkit" not in lines[k]:
                            n = retag(lines[k])
                            if n != lines[k]:
                                lines[k] = n
                                changed = True
            # re-export lines
            for m in re.finditer(r"(?m)^(#\[cfg\((?:test|all\(test,[^\n]*\))\)\]\n)((?:pub(?:\([^)]*\))? )use [^;]*;)", text):
                if any(re.search(r"\b" + re.escape(n) + r"\b", m.group(2)) for n in names):
                    idx = text[:m.start()].count("\n")
                    n = retag(lines[idx])
                    if n != lines[idx]:
                        lines[idx] = n
                        changed = True
            if changed:
                print(suffix, os.path.relpath(f, E)[-70:])
                if APPLY:
                    edit(f, "\n".join(lines))
        mp = T + "/" + crates[suffix] + "/📦️packages/🦀️rust/Cargo.toml"
        if APPLY and not manifest.has_testkit(suffix, crates):
            t = read(mp)
            edit(mp, t.replace("\n[dependencies]\n", "\n[features]\ntestkit = []\n\n[dependencies]\n", 1) if "[features]" not in t else re.sub(r"(?m)^\[features\]\n", "[features]\ntestkit = []\n", t, count=1))


if __name__ == "__main__":
    main()
