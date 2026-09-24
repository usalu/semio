"""🧹️ After outline registration, deletes every `KINDS`/`SUBJECT_KINDS` const that no code reads any more (with its
doc comment and attributes), strips it from `use` lists, and removes a `🔖️Kinds` region left empty.

Usage: drop-dead-kinds.py [--write] <adapter.rs>..."""
import re, sys
write = "--write" in sys.argv

def code_uses(text, name):
    code = "\n".join(line for line in text.split("\n") if not line.lstrip().startswith("//"))
    return len(re.findall(rf"\b{name}\b", code))

def drop(text, name):
    definition = re.search(rf"(?:[ \t]*///[^\n]*\n|[ \t]*#\[[^\n]*\n)*[ \t]*(?:pub )?const {name}: [^=]+= [^;]*;\n", text)
    uses = re.findall(rf"(?m)^[ \t]*(?:pub )?use [^;]*\b{name}\b[^;]*;", text)
    if code_uses(text, name) != (1 if definition else 0) + len(uses):
        return text
    if definition is not None:
        text = text.replace(definition.group(0), "", 1)
    for use in uses:
        fixed = re.sub(rf",\s*{name}\b|\b{name},\s*", "", use)
        fixed = re.sub(r"\{\s*([\w:]+)\s*\}", r"\1", fixed)
        text = text.replace(use, fixed, 1)
    return text

for path in [a for a in sys.argv[1:] if not a.startswith("--")]:
    before = open(path, encoding="utf-8").read()
    text = before
    for _ in range(3):
        text = drop(drop(drop(text, "SUBJECT_KINDS"), "INVERSE_KINDS"), "KINDS")
    text = re.sub(r"\n//#region 🔖️Kinds\n(?:\s*\n)*//#endregion 🔖️Kinds\n", "\n", text)
    left = code_uses(text, "KINDS")
    print(f"{'changed' if text != before else 'same':7} KINDS-uses={left} {path.split('/🧪️tests/')[-1]}")
    if write and text != before:
        open(path, "w", encoding="utf-8").write(text)
