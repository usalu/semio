"""🎞️ Moves the presentation react in-source vitest suites into one canonical case that receives the module's exports through a
dependency bag, exporting the few internals the suite reads."""
import re, pathlib
mod = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx")
case_dir = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎞️presentation-react-deck")
REL = "../../📦️packages/🟦️typescript/🎯️targets/⚛️react"
s = mod.read_text()
lines = s.split("\n")

def block(start_marker_line):
    start = lines.index(start_marker_line)
    end = next(i for i in range(start + 1, len(lines)) if lines[i] == "}")
    return start, end

a0 = lines.index("//#region 🧪️Tests")
assert lines[a0 + 1] == "if (import.meta.vitest) {"
a1 = next(i for i in range(a0 + 2, len(lines)) if lines[i] == "}")
assert lines[a1 + 1] == "//#endregion 🧪️Tests"
b0 = next(i for i in range(a1 + 2, len(lines)) if lines[i] == "//#region 🧪️Tests")
assert lines[b0 + 1] == "if (import.meta.vitest) {"
b1 = next(i for i in range(b0 + 2, len(lines)) if lines[i] == "}")
assert lines[b1 + 1] == "//#endregion 🧪️Tests"
body_a = "\n".join(lines[a0 + 2:a1])
body_b = "\n".join(lines[b0 + 2:b1])
body_a = body_a.replace("  const { describe, expect, it, beforeEach, afterEach } = import.meta.vitest;\n", "", 1)
body_b = body_b.replace("  const { describe, expect, it } = import.meta.vitest;\n", "", 1)
body = (body_a + "\n\n" + body_b).replace("import.meta.url", "source.url")
assert "import.meta" not in body, "unexpected import.meta usage left in the suite"
rest_lines = lines[:a0] + lines[a1 + 2:b0] + lines[b1 + 2:]
rest = "\n".join(rest_lines)

unexported = ["AutoAnimateMatcherHost", "elementIsInteractiveFigureDisposition", "elementIsSourceGhostAnchor", "elementIsTargetGhostAnchor", "transformFrameStyle"]
for name in unexported:
    pat = re.compile(r"^(?!export )((?:async\s+)?(?:function|const|let|class|type|interface)\s+" + re.escape(name) + r"\b)", re.M)
    rest, n = pat.subn(r"export \1", rest)
    assert n == 1, (name, n)

decl_kind = {}
for m in re.finditer(r"^export\s+(?:declare\s+)?(async\s+)?(function|const|let|class|type|interface|enum)\s+([A-Za-z_$][\w$]*)", rest, re.M):
    decl_kind[m.group(3)] = m.group(2)
for m in re.finditer(r"^export\s+(type\s+)?\{([^}]*)\}(?:\s+from\s+\"[^\"]+\")?", rest, re.M):
    for n in m.group(2).split(","):
        n = n.strip()
        if not n: continue
        is_type = bool(m.group(1)) or n.startswith("type ")
        n = n.replace("type ", "").split(" as ")[-1].strip()
        decl_kind.setdefault(n, "type" if is_type else "const")

def used(name):
    return re.search(r"(?<![\w$.])" + re.escape(name) + r"(?![\w$])", body) is not None

values = sorted(n for n, k in decl_kind.items() if k not in ("type", "interface") and used(n))
types = sorted(n for n, k in decl_kind.items() if k in ("type", "interface") and used(n))

imports = []
for m in re.finditer(r"^import\s+(type\s+)?\{([^}]*)\}\s+from\s+\"([^\"]+)\";", rest, re.M):
    spec = m.group(3)
    if spec.startswith("./"): spec = REL + spec[1:]
    names = []
    for n in m.group(2).split(","):
        n = n.strip()
        if not n: continue
        local = n.replace("type ", "").split(" as ")[-1].strip()
        if used(local) and local not in values and local not in types: names.append(n)
    if names: imports.append(f'import {m.group(1) or ""}{{ {", ".join(names)} }} from "{spec}";')
m = re.search(r'^import Reveal from "reveal.js";', rest, re.M)
if used("Reveal"): imports.append('import Reveal from "reveal.js";')

bag = " | ".join(f'"{n}"' for n in values)
header = "\n".join(imports)
case = f'''{header}
import type {{ {", ".join(types)} }} from "{REL}/🟦️.tsx";

type TestSource = {{ readonly directory: string; readonly url: string }};

/** 🎞️ The presentation react deck: mounting, reveal lifecycle, auto-animate matching, ghost anchors, surface chrome, the
 * projektetage deck contract and the JSON preview renderer, driven through the module's own exports. */
export async function registerPresentationReactDeckTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("{REL}/🟦️.tsx"), {bag}>, source: TestSource): Promise<void> {{
  const {{ {", ".join(values)} }} = dependencies;
  const {{ describe, expect, it, beforeEach, afterEach }} = vitest;

{body}
}}
'''
case_dir.mkdir(parents=True, exist_ok=True)
(case_dir / "🟦️.tsx").write_text(case)
wiring = f'''//#region 🧪️Tests
if (import.meta.vitest) {{
  const {{ registerPresentationReactDeckTests }} = await import("../../../../🧪️tests/🎞️presentation-react-deck/🟦️.tsx");
  await registerPresentationReactDeckTests(import.meta.vitest, {{ {", ".join(values)} }}, {{ directory: import.meta.dir, url: import.meta.url }});
}}
//#endregion 🧪️Tests'''
out = rest.rstrip("\n") + "\n\n" + wiring + "\n"
mod.write_text(out)
print(len(values), "values;", len(types), "types;", len(imports), "import lines")
