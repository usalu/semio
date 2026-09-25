#!/usr/bin/env python3
"""🧭️ Moves each plugin's story support modules out of `📖️stories/🧭️coordination/🧫️fixtures/<m>/🟦️.ts` into the ONE
coordination module the taxonomy already uses elsewhere (`📖️stories/🧭️coordination/🟦️.ts[x]`, e.g. os and ui styling):
executable story support is not a fixture, so a story importing it read as production code depending on test data.
The dependency module (the DSL reader or the model) comes first, the scene module second; relative imports are
re-rooted two levels up, the sibling import is dropped, identical imports are kept once, and every importer (the
stories and fem's viewport test) is re-pointed.
Usage: story-coordination-merge.py [--write]"""
import re
import sys
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
write = "--write" in sys.argv
PLANS = {
    "🏗️fem": (["🧫️dsl", "🧫️scene"], "The `🏗️fem` scope's story coordination: the story-local reader of the two fem DSL dialects and the scene projections, bilingual labels and command emulators built on it — everything the `🎭️*` stories need that is not itself a story."),
    "🧱️block": (["🧫️dsl", "🧫️scene"], "The `🧱️block` scope's story coordination: the story-local block DSL reader and the scene projections, window-render text and command emulators built on it — everything the `🎭️*` stories need that is not itself a story."),
    "📸️remodel": (["🧫️model", "🧫️scene"], "The `📸️remodel` scope's story coordination: the typed story model and labels and the scene, panel and report projections built on it — everything the `🎭️*` stories need that is not itself a story."),
}
IMPORT = re.compile(r'^import [^;]*?from "([^"]+)";\n', re.M | re.S)
HEADER = re.compile(r"(?:/// <reference [^\n]*\n)?// #region 🧲️Header\n(.*?)// #endregion 🧲️Header\n", re.S)


def rerooted(specifier):
    return specifier[len("../../"):] if specifier.startswith("../../../") else specifier


def merged_imports(source):
    """🔗️ Folds every import of the one coordination module into a single statement."""
    pattern = re.compile(r'^import (type )?\{([^}]*)\} from "([^"]*🧭️coordination/🟦️\.ts)";\n', re.M)
    found = list(pattern.finditer(source))
    if len(found) < 2:
        return source
    names = []
    for match in found:
        for name in (part.strip() for part in match.group(2).split(",")):
            if name:
                name = f"type {name}" if match.group(1) and not name.startswith("type ") else name
                if name not in names:
                    names.append(name)
    statement = f'import {{ {", ".join(names)} }} from "{found[0].group(3)}";\n'
    for match in reversed(found[1:]):
        source = source[: match.start()] + source[match.end():]
    return source[: found[0].start()] + statement + source[found[0].end():]


for plugin, (modules, summary) in PLANS.items():
    stories = root / "✏️s/🔌️plugins" / plugin / "📖️stories"
    coordination = stories / "🧭️coordination"
    target = coordination / "🟦️.ts"
    imports, bodies, reference, histories = [], [], False, []
    for module in modules:
        source = (coordination / "🧫️fixtures" / module / "🟦️.ts").read_text(encoding="utf-8")
        reference = reference or source.startswith("/// <reference")
        header = HEADER.search(source)
        histories.append("\n".join(line for line in header.group(1).splitlines() if not line.startswith("// 💻️") and "Ueli Saluz" not in line))
        body = source[header.end():]
        for match in IMPORT.finditer(body):
            specifier = match.group(1)
            if specifier.startswith("../🧫️"):
                continue
            statement = match.group(0).replace(f'"{specifier}"', f'"{rerooted(specifier)}"')
            if statement not in imports:
                imports.append(statement)
        bodies.append(IMPORT.sub("", body).strip("\n"))
    text = ("/// <reference types=\"vite/client\" />\n" if reference else "") + "// #region 🧲️Header\n"
    text += f"// 💻️ {target.relative_to(root)}\n// Specs: {summary}\n"
    text += "\n//\n".join(histories) + "\n// 2026 Ueli Saluz <ueli@semio-tech.com>\n// #endregion 🧲️Header\n\n"
    text += "".join(imports) + "\n" + "\n\n".join(bodies) + "\n"
    print(f"{plugin}: {len(imports)} imports, {sum(body.count(chr(10)) for body in bodies)} body lines -> {target.relative_to(root)}")
    importers = [path for path in stories.rglob("*.ts*") if path.is_file() and "🧫️fixtures" not in path.parts and path != target]
    for path in importers:
        source = path.read_text(encoding="utf-8")
        updated = re.sub(r'"((?:\.\./)+)🧭️coordination/🧫️fixtures/🧫️[a-z]+/🟦️\.ts"', r'"\g<1>🧭️coordination/🟦️.ts"', source)
        updated = re.sub(r'"\.\./\.\./🧫️fixtures/🧫️[a-z]+/🟦️\.ts"', '"../../🟦️.ts"', updated) if "🧭️coordination" in path.parts else updated
        updated = merged_imports(updated)
        if updated != source:
            print(f"  importer {path.relative_to(stories)}")
            if write:
                path.write_text(updated, encoding="utf-8")
    if write:
        target.write_text(text, encoding="utf-8")
        for module in modules:
            (coordination / "🧫️fixtures" / module / "🟦️.ts").unlink()
            (coordination / "🧫️fixtures" / module).rmdir()
        (coordination / "🧫️fixtures").rmdir()
