"""🔁️ Rewrites one registry's serde_json carrier oracle onto json-rust: ids, package facts and prose naming the reader."""
import json, re, sys
path = sys.argv[1]
raw = open(path, encoding="utf-8").read()
class Lexeme(str):
    pass
lexemes = []
def keep(text):
    lexemes.append(text)
    return Lexeme(f"\u0000number{len(lexemes) - 1}\u0000")
data = json.loads(raw, parse_float=keep)
ENGINE = {"family": "json-rust", "implementation": "json-rust 0.12 value tree", "version": "0.12"}
PROSE = [
    (re.compile(r'`\[dependencies\] serde_json = "1"`'), '`[dependencies] json = "0.12"`'),
    (re.compile(r"`📐️cad/🏭generator/🦀️json-engine`"), "`📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust`"),
    (re.compile(r"serde-json-([a-z0-9-]+)-carrier-reader"), r"json-rust-\1-carrier-reader"),
    (re.compile(r"`serde_json`"), "`json` (json-rust)"),
    (re.compile(r"serde_json 1 is vendored in the cargo registry cache"), "json 0.12 is vendored in the cargo registry cache"),
    (re.compile(r"serde_json@1"), "json@0.12"),
    (re.compile(r"\bserde_json\b(?! is a production)(?!::)"), "json-rust"),
]
def prose(text):
    for pattern, replacement in PROSE: text = pattern.sub(replacement, text)
    return text
def walk(value):
    if isinstance(value, dict):
        if value.get("package") == "serde_json":
            value["package"] = "json"
            if value.get("version") == "1": value["version"] = "0.12"
            if "engine" in value: value["engine"] = dict(ENGINE)
            if "homepage" in value: value["homepage"] = "https://docs.rs/json"
            if isinstance(value.get("source"), dict): value["source"] = {"repository": "https://github.com/maciejhirsz/json-rust", "license": "MIT OR Apache-2.0"}
        if value.get("engineFamily") == "serde-json":
            value["engineFamily"] = "json-rust"
            if value.get("packageVersion") == "1": value["packageVersion"] = "0.12"
            if value.get("engineVersion") == "1": value["engineVersion"] = "0.12"
        if value.get("family") == "serde-json" and value.get("implementation", "").startswith("serde_json"):
            value.clear(); value.update(ENGINE)
        for key in list(value): value[key] = walk(value[key])
        return value
    if isinstance(value, list): return [walk(item) for item in value]
    if isinstance(value, Lexeme): return value
    if isinstance(value, str): return prose(value)
    return value
data = walk(data)
indent = 2
out = json.dumps(data, ensure_ascii=False, indent=indent) + "\n"
out = re.sub(r'"\\u0000number(\d+)\\u0000"', lambda match: lexemes[int(match.group(1))], out)
open(path, "w", encoding="utf-8").write(out)
