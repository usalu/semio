# 🪝 Text surgery on authored project manifests: removes the authored `cargoToolchain` named input (the build generates it per Rust project) and checks, by parsing both texts, that exactly that key disappeared.
import json, re

BLOCK = re.compile(r'"cargoToolchain": \[[^\]]*\]')
RAW = ["{workspaceRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock", "{workspaceRoot}/.cargo/config.toml", "{workspaceRoot}/rust-toolchain.toml", "{workspaceRoot}/rustfmt.toml"]

def drop_cargo_toolchain(text):
    if '"cargoToolchain": [' not in text: return text, False
    data = json.loads(text)
    named = data.get("namedInputs") or {}
    if named.get("cargoToolchain") != RAW: raise SystemExit("cargoToolchain definition is not the known raw list")
    matches = list(BLOCK.finditer(text))
    if len(matches) != 1: raise SystemExit("cargoToolchain block occurs %d times" % len(matches))
    m = matches[0]; start, end = m.start(), m.end()
    line_start = text.rfind("\n", 0, start) + 1
    if text[line_start:start].strip():
        after = re.match(r"\s*,\s*", text[end:]); before = re.search(r",\s*$", text[:start])
        new = text[:start] + text[end + after.end():] if after else text[:before.start()] + text[end:] if before else text[:start] + text[end:]
    elif text[end:].startswith(",\n"):
        new = text[:line_start] + text[end + 2:]
    else:
        comma = text.rfind(",", 0, line_start)
        if comma < 0 or text[comma + 1:line_start].strip(): raise SystemExit("cargoToolchain block has no separator")
        new = text[:comma] + text[end:]
    expected = json.loads(text); del expected["namedInputs"]["cargoToolchain"]
    if not expected["namedInputs"]: del expected["namedInputs"]
    try: got = json.loads(new)
    except Exception as error: raise SystemExit("removal produced invalid JSON: %s" % error)
    if got != expected: raise SystemExit("removal changed more than cargoToolchain")
    return new, True

def replace_all(text, old, new, count, label):
    if text.count(old) == 0 and new in text: return text, False
    if text.count(old) != count: raise SystemExit("anchor %s: %d occurrence(s), expected %d" % (label, text.count(old), count))
    out = text.replace(old, new); json.loads(out); return out, True

# 🌊️ The browser package's three source groups hash the root manifest and the whole root lock next to the crates they compile; the generated resolution identity of the project replaces both.
def flow_browser(text):
    return replace_all(text, '      "{workspaceRoot}/Cargo.lock",\n      "{workspaceRoot}/Cargo.toml",\n', '      "cargoToolchain",\n', 3, "flow-browser")

# 🌎️ The hub's two source groups hash the whole root lock; the generated resolution identity of the hub crate replaces it.
def os_hub(text):
    return replace_all(text, '      "{workspaceRoot}/Cargo.lock",\n', '      "cargoToolchain",\n', 2, "os-hub")
