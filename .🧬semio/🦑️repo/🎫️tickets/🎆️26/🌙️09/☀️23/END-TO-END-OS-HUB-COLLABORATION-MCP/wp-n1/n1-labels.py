"""N1 one-off codemod: replaces every norm mutation leaf `label()` that returns its kebab id (or a half-translated
`Ändern: <field>` / `<field> ändern`) with the proper en + de engineering label from `n1-label-glossary.json`."""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
DIRS = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "din4108": "🧱️din4108", "en1994": "🧩️en1994", "en1997": "🌍️en1997", "en1998": "🫨️en1998", "en1999": "🪶️en1999", "din16798": "🌬️din16798", "din18599": "⚡️din18599"}
VERBS = {"insert": ("Insert", "einfügen"), "remove": ("Remove", "entfernen"), "add": ("Add", "hinzufügen")}
glossary = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "n1-label-glossary.json"), encoding="utf-8"))
write = "--write" in sys.argv


def label_for(family, kind):
    fixed = glossary["labels"].get(family, {}).get(kind)
    if fixed:
        return fixed
    verb, _, noun = kind.partition("-")
    if verb in VERBS and noun in glossary["nouns"].get(family, {}):
        en, de = glossary["nouns"][family][noun]
        return [f"{VERBS[verb][0]} {en}", f"{de} {VERBS[verb][1]}"]
    return None


def rust(text):
    return json.dumps(text, ensure_ascii=False)


changed, missing = 0, []
for family, directory in DIRS.items():
    root = f"{ROOT}/{directory}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
    for leaf in sorted(os.listdir(root)):
        source, descriptor = f"{root}/{leaf}/🦀️.rs", f"{root}/{leaf}/🔣️.json"
        if not (os.path.isfile(source) and os.path.isfile(descriptor)):
            continue
        kind = json.load(open(descriptor, encoding="utf-8")).get("semanticKind")
        text = open(source, encoding="utf-8").read()
        head = re.search(r"fn label\(&self\) -> protocol::LocalizedLabel \{", text)
        if not kind or not head:
            continue
        depth, i = 1, head.end()
        while depth:
            depth += {"{": 1, "}": -1}.get(text[i], 0)
            i += 1
        label = label_for(family, kind)
        if label is None:
            missing.append(f"{family}/{kind}")
            continue
        body = f"fn label(&self) -> protocol::LocalizedLabel {{\n        protocol::LocalizedLabel::native({rust(label[0])}, {rust(label[1])})\n    }}"
        replaced = text[:head.start()] + body + text[i:]
        if replaced != text:
            changed += 1
            if write:
                open(source, "w", encoding="utf-8").write(replaced)
print(f"{changed} labels {'rewritten' if write else 'to rewrite'}; no glossary row: {missing}")
