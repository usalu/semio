import io
import json

VS = chr(0xFE0F)
PATH = "🧰" + VS + "framework/🛍" + VS + "products/🦑" + VS + "repo/🔨" + VS + "modules/📚" + VS + "library/🔣" + VS + "taxonomy.json"
AREA = "👴" + VS + "leutwiler"
PROOF = "💤" + VS + "realparts-of-powers-z-n"
NOTES, PAPER, LEAN = "📯" + VS + "notes", "🏆" + VS + "proof", "🧘" + VS + "lean"
LEAN_ROOT = AREA + "/*/" + LEAN

text = io.open(PATH, encoding="utf-8").read()


def body(entries, indent):
    dumped = json.dumps(entries, ensure_ascii=False, indent=2)
    lines = dumped.split("\n")[1:-1]
    return "\n".join(" " * (indent - 2) + line for line in lines)


def insert_after_block(text, section, key, entries):
    start = text.index('\n  "' + section + '": {')
    anchor = text.index('\n    "' + key + '": {', start)
    end = text.index("\n    }", anchor) + len("\n    }")
    assert text[end] == ","
    return text[: end + 1] + "\n" + body(entries, 4) + "," + text[end + 1 :]


def insert_map_line(text, section, after_key, key, value):
    start = text.index('\n  "' + section + '": {')
    needle = '\n    "' + after_key + '": '
    anchor = text.index(needle, start)
    end = text.index("\n", anchor + 1)
    line = text[anchor:end]
    assert line.endswith(","), line
    return text[:end] + '\n    "' + key + '": "' + value + '",' + text[end:]


def contract(pattern, reason, verification):
    return {
        "pathPattern": pattern,
        "authority": "Lake",
        "reason": reason,
        "configurability": "unconfigurable",
        "scope": {"kind": "path-pattern"},
        "verification": verification,
        "expires": None,
    }


def kind(emoji, slug):
    return {"emoji": emoji + VS, "slugPattern": "^" + slug + "$", "allowEmojiOnly": False}


def members(owner, names):
    return {"ownerKindIds": [owner], "memberNames": names, "source": "registry"}


assert AREA not in text
text = insert_after_block(text, "fileKinds", "toml", {
    "lean-source": {"emoji": "🧘" + VS, "extensionChains": [".lean"], "role": "source"},
})
text = insert_after_block(text, "fileKindResolutionRules", "physical-toml-toml", {
    "physical-lean-source-lean": {"extensionChain": ".lean", "fileKindId": "lean-source", "priority": 0},
})
text = insert_after_block(text, "semanticDirectoryKinds", "semio-tech", {
    "leutwiler": kind("👴", "leutwiler"),
    "leutwiler-realparts-of-powers-z-n": kind("💤", "realparts-of-powers-z-n"),
    "leutwiler-notes": kind("📯", "notes"),
    "leutwiler-proof": kind("🏆", "proof"),
    "leutwiler-lean": kind("🧘", "lean"),
})
text = insert_after_block(text, "semanticDirectoryMemberKinds", "members-of-semio-tech", {
    "members-of-leutwiler": members("leutwiler", [PROOF]),
    "members-of-leutwiler-realparts-of-powers-z-n": members("leutwiler-realparts-of-powers-z-n", [NOTES, PAPER, LEAN]),
})
text = insert_after_block(text, "fixedFilenameContracts", "cargo-manifest", {
    "lake-manifest": contract(LEAN_ROOT + "/lakefile.toml", "Lake package configuration discovery", "lake build"),
    "lake-lock": contract(LEAN_ROOT + "/lake-manifest.json", "Lake resolved dependency manifest discovery", "lake build"),
    "lean-toolchain": contract(LEAN_ROOT + "/lean-toolchain", "elan toolchain discovery", "lean --version"),
    "lean-module-source": contract(LEAN_ROOT + "/**/*.lean", "Lean derives the module name from the source basename", "lake build"),
})
text = insert_map_line(text, "areas", "🏢" + VS + "semio-tech", AREA, "clean")
start = text.index('\n  "areaLayers": {')
anchor = text.index('\n    "🏢' + VS + 'semio-tech": "implementation"', start)
end = text.index("\n", anchor + 1)
text = text[:end] + ',\n    "' + AREA + '": "implementation"' + text[end:]

json.loads(text)
io.open(PATH, "w", encoding="utf-8", newline="\n").write(text)
print("ok")
