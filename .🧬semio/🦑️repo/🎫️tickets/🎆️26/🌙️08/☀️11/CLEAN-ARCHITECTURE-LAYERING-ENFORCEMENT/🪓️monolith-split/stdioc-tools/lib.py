#!/usr/bin/env python3
# 🧰️ Shared model of the stdio-contract carve: the crate table, module-closure walker, token scanner and public-item extractor.
import os
import re

ROOT = "/Users/ueli/Documents/semio"
STDIO = ROOT + "/✏️s/🔌️plugins/🗄️stdio"
C = STDIO + "/📇️registry/🧬️contract"
FACADE = "semio-s-artifact-stdio-contract"
SCRATCH = os.environ.get("SCRATCH", "")

CRATES = [
    {"key": "wire", "node": "🧾️payload-wire", "suffix": "wire", "description": "Wire helpers every stdio artifact shares: operation payload JSON, draft splices, ordered rows, Base64, fingerprints and the serde operation codec"},
    {"key": "window-kit", "node": "🪟️window-kit", "suffix": "window-kit", "description": "Window-kit conventions of the stdio artifacts: example switching, argument readers, addressed and structural tables, revisions and media import"},
    {"key": "assembly", "node": "🏗️assembly", "suffix": "assembly", "description": "Assembly of stdio artifact contributions: schema definitions, native codec receipts, capability ledgers and the contribution registry"},
    {"key": "editing", "node": "✏️editing", "suffix": "editing", "description": "Schema-erased, typed snapshot editing shared by every stdio artifact editor"},
    {"key": "editing-patch", "node": "✏️editing/🩹️patch", "suffix": "editing-patch", "description": "Snapshot patch wire, application, inversion and input schemas of the stdio editors"},
    {"key": "editing-details", "node": "✏️editing/🪟️details", "suffix": "editing-details", "description": "Snapshot details window: schema-driven detail presentation and its controls"},
    {"key": "part21", "node": "📐️part21", "suffix": "part21", "description": "The canonical ISO 10303-21 Part-21 codec of the stdio contract"},
    {"key": "list-delta", "node": "🪡️list-delta", "suffix": "list-delta", "description": "Wire types of the stdio positional list deltas"},
    {"key": "media-export", "node": "🎬️media-export", "suffix": "media-export", "description": "Format-neutral lifecycle for a truly incremental stdio media serializer"},
    {"key": "native-print", "node": "🖨️native-print", "suffix": "native-print", "description": "Original-caller authority for complete native text printing of a snapshot"},
    {"key": "testing", "node": "🧰️testing", "suffix": "testing", "description": "Original caller authorities for stdio artifact tests"},
]
for crate in CRATES:
    crate["package"] = FACADE + "-" + crate["suffix"]
    crate["lib"] = crate["package"].replace("-", "_")
    crate["dir"] = C + "/" + crate["node"]
    crate["root"] = crate["dir"] + "/🦀️.rs"
    crate["pkgdir"] = crate["dir"] + "/📦️packages/🦀️rust"
    crate["project"] = "@semio-tech/stdio-artifact-contract-" + crate["suffix"] + "-rs"
BY_KEY = {crate["key"]: crate for crate in CRATES}


def read(path):
    return open(path, encoding="utf8").read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(text)


MOUNT = re.compile(r'((?:#\[[^\]\n]*\]\s*\n\s*)*)#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*\n\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_0-9]+)\s*;')
INCLUDE = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)')


def closure(root, test=False, seen=None, test_only=None):
    seen = {} if seen is None else seen
    if root in seen:
        return seen
    seen[root] = test
    text = read(root)
    for match in MOUNT.finditer(text):
        attrs, rel, _ = match.groups()
        is_test = test or "cfg(test)" in attrs
        target = os.path.normpath(os.path.join(os.path.dirname(root), rel))
        if target in seen:
            seen[target] = seen[target] and is_test
            continue
        closure(target, is_test, seen)
    return seen


def strip_comments(text):
    return "\n".join(line for line in text.split("\n") if not line.lstrip().startswith("//"))


PUB_ITEM = re.compile(r"(?m)^pub\s+(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(fn|struct|enum|trait|type|static|const|union|mod)\s+([A-Za-z_][A-Za-z0-9_]*)")
MACRO = re.compile(r"(?m)^#\[macro_export\]\s*\nmacro_rules!\s+([A-Za-z_][A-Za-z0-9_]*)")
PUB_USE = re.compile(r"(?m)^pub\s+use\s+([^;]+);", re.S)


def use_names(body):
    body = re.sub(r"\s+", " ", body).strip()
    out = []
    if "{" in body:
        head, rest = body.split("{", 1)
        inner = rest.rsplit("}", 1)[0]
        depth, current, parts = 0, "", []
        for ch in inner:
            depth += ch == "{"
            depth -= ch == "}"
            if ch == "," and depth == 0:
                parts.append(current)
                current = ""
            else:
                current += ch
        if current.strip():
            parts.append(current)
        for part in parts:
            out.extend(use_names(head + part.strip()) if "{" not in part else use_names(head.strip() + part.strip()))
        return out
    name = body.split(" as ")[-1].strip() if " as " in body else body.split("::")[-1].strip()
    if name and name != "*":
        out.append(name)
    return out


def public_items(path):
    text = strip_comments(read(path))
    items = {"fn": [], "struct": [], "enum": [], "trait": [], "type": [], "static": [], "const": [], "union": [], "mod": [], "use": [], "macro": []}
    for kind, name in PUB_ITEM.findall(text):
        items[kind].append(name)
    items["macro"] = MACRO.findall(text)
    for body in PUB_USE.findall(text):
        items["use"].extend(use_names(body))
    return items


def all_public_names(path):
    items = public_items(path)
    names = []
    for kind in ("fn", "struct", "enum", "trait", "type", "static", "const", "union", "use"):
        names.extend(items[kind])
    return names, items


EXTERNALS = {
    "pack": "semio-framework-pack",
    "base64": "base64",
    "json_patch": "json-patch",
    "serde_json": "serde_json",
}


def external_tokens(text):
    stripped = strip_comments(text)
    found = set(re.findall(r"(?<![A-Za-z0-9_])(semio_framework_[a-z0-9_]+)\b", stripped))
    out = {name.replace("_", "-") for name in found}
    for token, package in EXTERNALS.items():
        if re.search(r"(?<![A-Za-z0-9_])" + token + r"::", stripped) or re.search(r"\buse " + token + r"\b", stripped):
            out.add(package)
    return out


def sibling_tokens(text):
    stripped = strip_comments(text)
    return {crate["package"] for crate in CRATES if re.search(r"(?<![A-Za-z0-9_])" + crate["lib"] + r"\b", stripped)}
