"""🧬️ Mutation checks of the layered overview tests (ticket QUIZ-PRODUCT-AND-TEACHING-PROCTOR): each mutant breaks one specified
behaviour of `🥞️layered-overview-geometry` or `🥞️LayeredOverview`; the fixture/component tests must fail for every mutant. Every file is
restored right after its run. Usage: python layered_overview_mutations.py  → 🗑️generated/layered-final/mutations.json"""

import json
import pathlib
import subprocess
import time

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parents[6]
UI = REPO / "🧰️framework" / "🔨️modules" / "🖱️ui"
GEOMETRY = UI / "🔨️modules" / "🥞️layered-overview-geometry" / "🟦️.ts"
ELEMENT = UI / "🧱️elements" / "🥞️LayeredOverview" / "🟦️.tsx"
PACKAGE = UI / "🎯️targets" / "⚛️react" / "📦️packages" / "🟦️typescript"
OUT = HERE / "🗑️generated" / "layered-final" / "mutations.json"

MUTANTS = [
    ("veil hole one tenth short", GEOMETRY, "const end = Math.min(1, start + size);", "const end = Math.min(1, start + size * 0.9);"),
    ("follow lerp 0.2", GEOMETRY, "export const LAYERED_FOLLOW_LERP = 0.12;", "export const LAYERED_FOLLOW_LERP = 0.2;"),
    ("ease-in only", GEOMETRY, "return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;", "return t * t * t;"),
    ("warm queue past the budget", GEOMETRY, "if (liveCount >= budget || booted.size >= budget)", "if (liveCount >= budget)"),
    ("budget ignores kept panes", GEOMETRY, "liveByRecency.filter((id) => !keep.has(id)).slice(0, excess)", "liveByRecency.slice(0, excess)"),
    ("centred last row left-aligned", GEOMETRY, "const first = Math.floor((columns - inRow) / 2);", "const first = 0;"),
    ("no reveal on focus", ELEMENT, "onFocus={(event) => focusVisible(event.target) && onReveal(pane.id)}", "onFocus={() => undefined}"),
    ("no conceal on blur", ELEMENT, "onBlur={(event) => !event.currentTarget.contains(event.relatedTarget as Node | null) && onConceal(pane.id)}", "onBlur={() => undefined}"),
    ("touch reveals", ELEMENT, "onPointerEnter={(event) => event.pointerType === \"mouse\" && onReveal(pane.id)}", "onPointerEnter={() => onReveal(pane.id)}"),
    ("focus not returned to the card", ELEMENT, "cardElements.current.get(previous)?.querySelector<HTMLElement>(FOCUSABLE)?.focus({ preventScroll: true });", "void FOCUSABLE;"),
    ("opened page not focused", ELEMENT, "if (region && !region.contains(document.activeElement)) region.focus({ preventScroll: true });", "void region;"),
    ("panes not inert", ELEMENT, "inert={!opened}", "inert={false}"),
    ("no budget release", ELEMENT, "release(panesOverBudget(byRecency(live), keep(false), lifecycle.budget));", "void byRecency;"),
    ("cards not memoized", ELEMENT, "const LayeredCardHost = React.memo(function LayeredCardHost(", "const LayeredCardHost = (function LayeredCardHost("),
    ("veil without level", ELEMENT, "<div ref={veilCallback} data-layered-veil=\"\" data-level=\"dialog\"", "<div ref={veilCallback} data-layered-veil=\"\""),
    ("ids by identity", ELEMENT, "const ids = useStableByKey(freshIds, freshIds.join(\"\\n\"));", "const ids = freshIds;"),
    ("cells by identity", ELEMENT, "const cellList = useStableByKey(freshCells, freshCells.map((cell) => `${cell.column}:${cell.row}`).join(\" \"));", "const cellList = freshCells;"),
    ("list veils everywhere", ELEMENT, "{Math.abs(index - listIndex) <= radius ? (", "{true ? ("),
    ("grid rest letterboxes instead of covering", GEOMETRY, "const scale = Math.max(view.width, view.height);", "const scale = Math.min(view.width, view.height);"),
    ("grid rest ignores track weights", GEOMETRY, "const sizes = Array.from({ length: count }, (_, index) => weights?.[index] ?? 1);", "const sizes = Array.from({ length: count }, () => 1);"),
    ("grid rest pans with the pointer", ELEMENT, "if (mode !== \"strip\" || gridRest || pan !== \"pointer\"", "if (mode !== \"strip\" || pan !== \"pointer\""),
    ("grid rest boots on demand", ELEMENT, "gridRest ? \"all\" : \"warm\"", "gridRest ? \"warm\" : \"warm\""),
    ("grid zoom always snaps", ELEMENT, "if (latest.current.reduced || (from.x === to.x && from.y === to.y && from.width === to.width", "if (true || (from.x === to.x && from.y === to.y && from.width === to.width"),
    ("hole painted only on the next frame", ELEMENT, "      focusView(id);\n      repaint();", "      focusView(id);"),
    ("grid rest never zooms back out", ELEMENT, "      if (latest.current.gridRest && openedRef.current === null) focusView(null);", ""),
    ("Escape ignored", ELEMENT, "if (event.key === \"Escape\" && !event.defaultPrevented) requestOpen(null);", "void event;"),
]


def write(path: pathlib.Path, text: str) -> None:
    for attempt in range(20):
        try:
            path.write_bytes(text.encode("utf-8"))
            return
        except OSError:
            time.sleep(0.5)
    raise OSError(f"cannot write {path}")


def run() -> bool:
    result = subprocess.run(["bun", "./📜️script.ts", "test", "quick", "layered"], cwd=PACKAGE, capture_output=True, text=True, encoding="utf-8", errors="replace")
    return result.returncode == 0


results = []
for name, path, old, new in MUTANTS:
    original = path.read_bytes().decode("utf-8")
    if original.count(old) != 1:
        results.append({"mutant": name, "status": "anchor-missing"})
        continue
    try:
        write(path, original.replace(old, new))
        passed = run()
    finally:
        write(path, original)
    results.append({"mutant": name, "status": "survived" if passed else "killed"})
    print(f"[DEBUG] {name}: {results[-1]['status']}", flush=True)

OUT.write_text(json.dumps({"baselineGreen": run(), "results": results}, indent=2, ensure_ascii=False), encoding="utf-8")
print(f"[DEBUG] killed {sum(r['status'] == 'killed' for r in results)}/{len(results)}")
