"""🧪 Builds the pre-optimisation copy of the thermal solver inside the generated folder (A/B timing): the per-frame history integration with a plain `temps`/`bodyStates` pair and no session.

@see ./thermal_orig_block.txt — the original integration block
"""

import pathlib

HERE = pathlib.Path(__file__).resolve().parent
TARGET = next((HERE.parent / "🗑️generated" / "pf" / "orig" / "🔨️modules" / "🌡️thermal").glob("🟦️.ts"))
text = TARGET.read_text(encoding="utf-8")
a = text.index("    const session = options.session ?? thermalSession();")
end_marker = "    return assemble(asset, sys, instantAt(t), final.temps, steadyTemps, taus, [...final.bodies], input, true);\n  }\n"
b = text.index(end_marker) + len(end_marker)
text = text[:a] + (HERE / "thermal_orig_block.txt").read_text(encoding="utf-8") + text[b:]
text = text.replace("  const temps = [...steadyTemps];", "  let temps = [...steadyTemps];").replace("  const bodyStates: BodyState[] = steadyBodies", "  let bodyStates: BodyState[] = steadyBodies")
TARGET.write_text(text, encoding="utf-8")
print("ok")
