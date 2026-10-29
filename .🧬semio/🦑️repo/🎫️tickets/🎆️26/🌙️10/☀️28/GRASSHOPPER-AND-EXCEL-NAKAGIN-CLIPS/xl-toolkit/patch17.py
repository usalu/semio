import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:70]
    src = src.replace(old, new, 1)


swap('''_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
''', '''_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
_theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
''')
swap('''    kwargs = {key: value for key, value in (("weight", weight), ("slant", slant)) if value}
''', '''    kwargs = {key: value for key, value in (("weight", weight), ("slant", slant)) if value}
    if "(" in text or ")" in text:
        _fonts.register_fonts(_theme.FONT_SERIF)
        kwargs["t2f"] = {"(": _theme.FONT_SERIF, ")": _theme.FONT_SERIF}
''')
open(path, "w", encoding="utf-8").write(src)
