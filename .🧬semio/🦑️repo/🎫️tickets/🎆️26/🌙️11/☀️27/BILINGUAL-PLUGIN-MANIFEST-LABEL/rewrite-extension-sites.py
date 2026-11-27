"""🧩️ Rewrites every `ExtensionBundle::new(id, "…", v)` label to a bilingual `LocalizedLabel::native(en, de)`; `EXTENSION_LABEL` consts keep their English text and gain the German one inline."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from labels import DE

NEW = re.compile(r'(?P<head>ExtensionBundle::new\(\s*[^,]+?,\s*)(?P<arg>"(?:[^"\\]|\\.)*"|EXTENSION_LABEL|id)(?P<tail>\s*,)')
CONST = re.compile(r'const EXTENSION_LABEL: &str = "([^"]*)";')
missing, changed = set(), 0


def qualifier(text):
    if re.search(r'(?<![:\w])LocalizedLabel::', text) or re.search(r'use [^;]*\bLocalizedLabel\b[^;]*;', text):
        return "LocalizedLabel"
    if "semio_plugin_extension::extension_runtime_lift" in text:
        return "semio_plugin_extension::extension_runtime_lift::LocalizedLabel"
    if "crate::extension_runtime_lift" in text:
        return "crate::extension_runtime_lift::LocalizedLabel"
    return "semio_plugin::LocalizedLabel"


for directory, dirs, files in os.walk("."):
    dirs[:] = [d for d in dirs if d not in {"node_modules", "target"} and not d.startswith((".", "🗑️"))]
    for name in files:
        if not name.endswith(".rs"):
            continue
        path = os.path.join(directory, name)
        text = open(path, encoding="utf-8").read()
        if "ExtensionBundle::new(" not in text:
            continue
        const = CONST.search(text)
        def repl(m):
            arg = m.group("arg")
            if arg == "id":
                call = "id, id"
            else:
                en = const.group(1) if arg == "EXTENSION_LABEL" else arg[1:-1]
                if en not in DE:
                    missing.add(en)
                    return m.group(0)
                call = f'{arg if arg == "EXTENSION_LABEL" else chr(34) + en + chr(34)}, "{DE[en]}"'
            return f'{m.group("head")}{qualifier(text)}::native({call}){m.group("tail")}'
        new = NEW.sub(repl, text)
        if new != text:
            changed += 1
            print("rewrote", path)
            open(path, "w", encoding="utf-8").write(new)
print(changed, "files; missing:", sorted(missing))
