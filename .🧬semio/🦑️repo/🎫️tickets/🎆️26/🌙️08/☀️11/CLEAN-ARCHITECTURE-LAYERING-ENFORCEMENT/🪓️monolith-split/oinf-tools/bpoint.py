"""🧭 Re-points the TypeScript and script readers of the board Rust sources at the producer crate roots."""
import sys, os
R = "/Users/ueli/Documents/semio/"
INF = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/"
DIR = INF + "🎲️board/🔌️ports/➡️directed/"
DAG = ["🗃️model", "🕸️graph", "🧭️host"]
NORMAL = ["🗃️model", "🖼️scene", "🎯️focus", "🕹️gestures", "🪣️fill", "🧭️host"]
RE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/"

def dag_join(call, root_expr):
    return '["' + '", "'.join(DAG) + f'"].map((node) => {call}({root_expr})).join("\\n")'

def tpl(prefix_dir):
    return "`" + prefix_dir + "${node}/🦀️.rs`"

EDITS = []
def add(path, old, new, count=1): EDITS.append((path, old, new, count))

add("📜️script.ts", f'policyReadModuleTreeSource(this.root, "{DIR}🕸️dag/🦀️.rs"),', dag_join("policyReadModuleTreeSource", f"this.root, {tpl(DIR + '🕸️dag/')}") + ",")

SC = INF + "🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts"
SC = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts"
add(SC, 'new URL("../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs", import.meta.url)', 'new URL("../../../../../♾️infinite/🎲️board/⚙️engine/🦀️.rs", import.meta.url)')
add(SC, 'new URL("../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs", import.meta.url)', 'new URL("../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🕹️gestures/🦀️.rs", import.meta.url)')
add(SC, 'new URL("../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs", import.meta.url)', 'new URL("../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🧪️tests/🔬️board-host/🦀️.rs", import.meta.url)')

LIFE = RE + "🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts"
add(LIFE, f'read("{DIR}🕸️dag/🦀️.rs"),', dag_join("read", tpl(DIR + '🕸️dag/')) + ",")

READ = lambda: dag_join("readModuleTreeSource", "resolve(repoRoot, " + tpl(DIR + '🕸️dag/') + ")")
OLD = f'readModuleTreeSource(resolve(repoRoot, "{DIR}🕸️dag/🦀️.rs"))'
add(RE + "🧪️tests/🔗️node-graph-wire-edit/🟦️.ts", OLD, READ())
add(RE + "🧪️tests/🧪️node-graph-edit-rows/🟦️.ts", OLD, READ())
G = RE + "🧪️tests/🫳️node-graph-gestures/🟦️.ts"
add(G, OLD, READ(), 3)
add(G, 'expect(begins.slice(0, begins.indexOf("\\n    }\\n"))).toContain("self.screen_hit(sx, sy).is_screen_path()");', 'expect(begins.slice(0, begins.indexOf("\\n}\\n"))).toContain("screen_hit(viewport, graph, minimap, sx, sy).is_screen_path()");')
add(G, f'resolve(repoRoot, "{INF}🎲️board/🧬️schema/🎯️dag-input/🦀️.rs")', f'resolve(repoRoot, "{INF}🎲️board/🧬️schema/🦀️.rs")')

SA = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts"
add(SA, "  `${OS_INFINITE_DIRECTED}/➕️normal/🦀️.rs`,\n  `${OS_INFINITE_DIRECTED}/🕸️dag/🦀️.rs`,\n", "".join(f"  `${{OS_INFINITE_DIRECTED}}/➕️normal/{n}/🦀️.rs`,\n" for n in NORMAL) + "".join(f"  `${{OS_INFINITE_DIRECTED}}/🕸️dag/{n}/🦀️.rs`,\n" for n in DAG))

def main(dry):
    out = []
    for path, old, new, count in EDITS:
        t = open(R + path, encoding='utf-8').read()
        c = t.count(old)
        if c != count: out.append(('/'.join(path.split('/')[-2:]), 'SKIPPED', c, count)); continue
        if not dry: open(R + path, 'w', encoding='utf-8').write(t.replace(old, new))
        out.append(('/'.join(path.split('/')[-2:]), 'ok', c, count))
    return out

if __name__ == '__main__':
    for r in main(len(sys.argv) > 1 and sys.argv[1] == 'dry'): print(r)
