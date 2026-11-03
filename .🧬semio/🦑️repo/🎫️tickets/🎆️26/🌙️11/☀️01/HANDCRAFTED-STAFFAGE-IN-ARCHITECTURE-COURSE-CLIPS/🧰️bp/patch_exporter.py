"""📤 bp: exact-text edits: figure-only bounds in the cast index of transport casts."""
from pathlib import Path

root = Path(__file__).resolve().parents[8]
arch = root / "🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast"


def edit(path: Path, pairs):
    text = path.read_bytes().decode("utf8")
    for old, new in pairs:
        assert text.count(old) == 1, (path.name, old, text.count(old))
        text = text.replace(old, new)
    path.write_bytes(text.encode("utf8"))


edit(arch / "🟦️.ts", [
    ("    heightM = Math.max(heightM, state.bounds.maxY - state.bounds.minY);\n",
     "    heightM = Math.max(heightM, state.bounds.maxY - state.bounds.minY);\n    figure = { minX: Math.min(figure.minX, state.bounds.minX), minY: Math.min(figure.minY, state.bounds.minY), maxX: Math.max(figure.maxX, state.bounds.maxX), maxY: Math.max(figure.maxY, state.bounds.maxY) };\n"),
    ("  let heightM = 0;\n", "  let heightM = 0;\n  let figure = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };\n"),
    ("    heightM: round(viewpointOf(cast) === \"elevation\"",
     "    ...((cast.transport?.length ?? 0) > 0 ? { figureBoundsM: cast.mirror === true ? { minX: round(-figure.maxX), minY: round(figure.minY), maxX: round(-figure.minX), maxY: round(figure.maxY) } : { minX: round(figure.minX), minY: round(figure.minY), maxX: round(figure.maxX), maxY: round(figure.maxY) } } : {}),\n    heightM: round(viewpointOf(cast) === \"elevation\""),
])
edit(arch / "🧬️schema/🟦️.ts", [
    ("  readonly heightM: number;\n};", "  readonly figureBoundsM?: { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };\n  readonly heightM: number;\n};"),
])
