"""🔁 Rewrites every `PluginBuilder::new(..).label("…")` and `Plugin::new(id, "…", v)` to a bilingual `LocalizedLabel::native(en, de)`.

Run from the repo root: `python3 <ticket>/rewrite-call-sites.py [--dry]`. Prints every site it could not translate.
"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from labels import DE

SKIP = {".git", "node_modules", "target", ".🧬semio"}
BUILDER = re.compile(r'(?P<prefix>[\w:]*?)PluginBuilder(?:::<[^>]*>)?::new\s*\([^)]*\)\s*(?:\.package_id\([^)]*\)\s*)?\.label\(\s*(?P<arg>"(?:[^"\\]|\\.)*")\s*\)', re.S)
PLUGIN_NEW = re.compile(r'(?P<prefix>[\w:]*?)\bPlugin(?:::<[^>]*>)?::new\s*\(\s*(?P<id>"[^"]*"|[\w.()]+)\s*,\s*(?P<arg>"(?:[^"\\]|\\.)*")\s*,', re.S)
BARE = re.compile(r'(?<![:\w])LocalizedLabel::')
dry = "--dry" in sys.argv
missing, changed = set(), 0


def qualifier(text, prefix):
    if BARE.search(text) or re.search(r'use [^;]*\bLocalizedLabel\b[^;]*;', text):
        return "LocalizedLabel"
    if prefix.startswith("semio_plugin_builder::") or re.search(r'use semio_plugin_builder::[^;]*PluginBuilder', text):
        return "semio_plugin_builder::LocalizedLabel"
    return "semio_plugin::LocalizedLabel"


def replacement(text, match):
    en = match.group("arg")[1:-1]
    if en not in DE:
        missing.add(en)
        return match.group(0)
    call = f'{qualifier(text, match.group("prefix"))}::native("{en}", "{DE[en]}")'
    start, end = match.span("arg")
    return match.group(0)[: start - match.start()] + call + match.group(0)[end - match.start():]


for directory, dirs, files in os.walk("."):
    dirs[:] = [d for d in dirs if d not in SKIP and not d.startswith((".", "🗑️"))]
    for name in files:
        if not name.endswith(".rs"):
            continue
        path = os.path.join(directory, name)
        text = open(path, encoding="utf-8").read()
        if "PluginBuilder" not in text and "Plugin::" not in text and "Plugin::<" not in text:
            continue
        new = BUILDER.sub(lambda m: replacement(text, m), text)
        new = PLUGIN_NEW.sub(lambda m: replacement(text, m), new)
        if new != text:
            changed += 1
            print("rewrote", path)
            if not dry:
                open(path, "w", encoding="utf-8").write(new)
print(changed, "files;", "missing:", sorted(missing))
