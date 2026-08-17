from pathlib import Path
import glob, json

PUZZLE = Path(glob.glob("✏️s/🔌️plugins/*puzzle")[0]).resolve()
TICKET = Path(glob.glob(".🦑️repo/**/EXAMPLE-SHAPE*/", recursive=True)[0]).resolve()

arts = {}
apps = {}
for a in (PUZZLE / "🗿️artifacts").iterdir():
    if "2d" in a.name and "3d" not in a.name and "5d" not in a.name:
        arts["2d"] = a.name
    elif "5d" in a.name:
        arts["5d"] = a.name
    else:
        arts["3d"] = a.name
for a in (PUZZLE / "🎛️apps").iterdir():
    if "2d" in a.name and "3d" not in a.name and "5d" not in a.name:
        apps["2d"] = a.name
    elif "5d" in a.name:
        apps["5d"] = a.name
    else:
        apps["3d"] = a.name

TESTS = next(
    p.name
    for p in PUZZLE.rglob("*")
    if p.is_dir() and p.name.endswith("tests") and "examples" in str(p)
)

parts = ["/** puzzle facet WASM facades */"]
for dim in ["3d", "5d", "2d"]:
    art = arts[dim]
    for facet, name in [
        ("🔺️diff", "diff"),
        ("🗣️dsl", "dsl"),
        ("🎒️pack", "pack"),
        ("🔧️op", "op"),
        ("📡️spr", "spr"),
    ]:
        parts.append(
            f'export * as {dim}_{name} from "../../🗿️artifacts/{art}/{facet}/🟦️component.ts";'
        )
parts.append("")
parts.append("/** 📚️ Example definition leaves */")
for dim in ["2d", "3d", "5d"]:
    art = arts[dim]
    parts.append(
        f'export * as examples_{dim}_concrete_forest from "../../🗿️artifacts/{art}/📚️examples/🌲️concrete-forest/🟦️component.ts";'
    )
    parts.append(
        f'export * as examples_{dim}_nakagin from "../../🗿️artifacts/{art}/📚️examples/🏗️nakagin-capsule-tower/🟦️component.ts";'
    )
for dim in ["2d", "3d", "5d"]:
    app = apps[dim]
    parts.append(
        f'export * as examples_app_{dim}_demo_session from "../../🎛️apps/{app}/📚️examples/🎬️demo-session/🟦️component.ts";'
    )
parts.append("")
(PUZZLE / "📦️packages/🟦️typescript/📦️index.ts").write_text("\n".join(parts) + "\n")
print("index ok")

includes = []
for dim in ["3d", "5d", "2d"]:
    art = arts[dim]
    for slug in ["🌲️concrete-forest", "🏗️nakagin-capsule-tower"]:
        includes.append(f"../../🗿️artifacts/{art}/📚️examples/{slug}/{TESTS}/🟦️test.ts")
for dim in ["2d", "3d", "5d"]:
    app = apps[dim]
    includes.append(f"../../🎛️apps/{app}/📚️examples/🎬️demo-session/{TESTS}/🟦️test.ts")

vpath = next((PUZZLE / "📦️packages/🟦️typescript").glob("*vitest*"))
vitest = (
    'import { defineConfig } from "vitest/config";\n\n'
    "/** @emoji ️ Vitest for puzzle example definition leaves. */\n"
    "export default defineConfig({\n"
    "  test: {\n"
    '    name: "@semio-tech/puzzle-js",\n'
    '    environment: "node",\n'
    f"    include: {json.dumps(includes, ensure_ascii=False)},\n"
    "  },\n"
    "});\n"
)
vpath.write_text(vitest)
print("vitest ok", vpath.name)

# How other plugins run vitest from script.ts
sample = Path("✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts")
print("flow script:", sample.read_text()[:800] if sample.exists() else "missing")
