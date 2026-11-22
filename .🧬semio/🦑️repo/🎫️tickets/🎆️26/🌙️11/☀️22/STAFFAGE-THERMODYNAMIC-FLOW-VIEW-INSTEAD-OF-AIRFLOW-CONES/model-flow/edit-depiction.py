"""One-shot editor of the two `emitters` spots of the depiction module."""
import sys, pathlib
path = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts")
text = path.read_text(encoding="utf-8")
pairs = [
    ("    const emitters = thermal !== undefined && state.asset.thermal.emitters.length > 0 && (options.flow?.length ?? 0) === 0 ? (analysis.transport ?? solveTransport(state.asset, state.query, thermal.environment, { maxAgeS: 0.5, pathlines: false, streamlines: false })).emitters : undefined;\n", ""),
    ("{ ...(thermal ? { thermal } : {}), ...(structure ? { structure } : {}), ...(emitters ? { emitters } : {}) }, { groundOverhangM: options.annotations.ground", "{ ...(thermal ? { thermal } : {}), ...(structure ? { structure } : {}) }, { groundOverhangM: options.annotations.ground"),
    ("  const emitters = undefined;\n", ""),
    ("{ ...(result ? { structure: result } : {}), ...(emitters ? { emitters } : {}) }, { glyphEdges: true,", "{ ...(result ? { structure: result } : {}) }, { glyphEdges: true,"),
]
for old, new in pairs:
    if text.count(old) != 1:
        sys.exit(f"expected one match of {old[:80]!r}, found {text.count(old)}")
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
print("ok")
