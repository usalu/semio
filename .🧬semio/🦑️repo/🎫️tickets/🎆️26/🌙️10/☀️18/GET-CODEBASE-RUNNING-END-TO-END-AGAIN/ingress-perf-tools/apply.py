#!/usr/bin/env python3
"""🧩 Anchored apply of the ingress-perf staged set: every `old` anchor must occur exactly once, new files must not exist, `--dry-run` writes nothing, `--root` targets an overlay copy."""
import json, os, sys, hashlib

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from patches import EDITS, NEW_FILES

TOKENS = {"@JSON@": chr(0x1F523) + "\ufe0f.json", "@TS@": chr(0x1F7E6) + "\ufe0f.ts", "@INFORMATION@": chr(0x1F4E5) + "\ufe0finformation", "@SCHEMA@": chr(0x1F9EC) + "\ufe0fschema", "@OWNED@": chr(0x1F39B) + "\ufe0fowned-execution", "@TESTS@": chr(0x1F9EA) + "\ufe0ftests", "@ROSTER@": chr(0x1F4E1) + "\ufe0froster", "@JOURNAL@": chr(0x1F4DC) + "\ufe0fjournal", "@REVISION@": chr(0x1F516) + "\ufe0frevision", "@BUDGET@": chr(0x1F4E6) + "\ufe0fbudget", "@OPENING@": chr(0x1F4E5) + "\ufe0fopening", "@STORE@": chr(0x1F3EA) + "\ufe0fstore", "@COMPILER@": chr(0x1F3D7) + "\ufe0fcompiler", "@POOL@": chr(0x1F3CA) + "\ufe0fpool", "@SESSION@": chr(0x1F9F5) + "\ufe0fsession", "@CACHING@": chr(0x26A1) + "\ufe0fcaching", "@STORAGE@": chr(0x1F4BE) + "\ufe0fstorage", "@ENFORCEMENT@": chr(0x1F512) + "\ufe0fenforcement", "@SCHEDULER@": chr(0x1F516) + "\ufe0fscheduler", "@FETCHED@": chr(0x1F9FE) + "\ufe0ffetched", "@IMAGE@": chr(0x1F34E) + "\ufe0fimage"}

def spec_token(root, keywords):
    parts, current = [], root
    for part in keywords.split("/"):
        key, _, code = part.partition("@")
        matches = [e for e in os.listdir(current) if ascii_key(e) == key and (not code or ord(e[0]) == int(code, 16))]
        if len(matches) != 1:
            raise SystemExit(f"cannot resolve validator segment {key!r}: {matches}")
        current = os.path.join(current, matches[0])
        parts.append(matches[0])
    return "/".join(parts[1:])

def expand(text):
    for token, value in TOKENS.items():
        text = text.replace(token, value)
    return text

def ascii_key(name):
    return "".join(ch for ch in name if ord(ch) < 128)

def resolve(root, rel):
    """🔎 Resolves an ASCII keyword path (`framework/modules/process`) to the emoji-named entries; `name=1f501` spells a not-yet-existing entry."""
    current = root
    for part in rel.split("/"):
        if "=" in part:
            name, code = part.split("=")
            candidate = os.path.join(current, chr(int(code, 16)) + "\ufe0f" + name)
            if os.path.isdir(current):
                matches = [e for e in os.listdir(current) if ascii_key(e) == name]
                if matches:
                    candidate = os.path.join(current, matches[0])
            current = candidate
            continue
        key, _, code = part.partition("@")
        matches = [e for e in os.listdir(current) if ascii_key(e) == key and (not code or ord(e[0]) == int(code, 16))] if os.path.isdir(current) else []
        if len(matches) != 1:
            raise SystemExit(f"cannot resolve {part!r} under {current}: {matches}")
        current = os.path.join(current, matches[0])
    return current

def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")

def main():
    args = sys.argv[1:]
    dry = "--dry-run" in args
    root = find_root()
    if "--root" in args:
        root = os.path.abspath(args[args.index("--root") + 1])
    TOKENS["@VALIDATOR@"] = spec_token(root, "framework/modules/schema/validator/packages/typescript/.ts")
    for name, path in {"@ROUTING@": "routing", "@LEASES@": "leases", "@MODE@": "build/mode", "@EXECUTION@": "execution"}.items():
        TOKENS[name] = spec_token(root, "framework/modules/process/packages/typescript/" + path + "/.ts")
    TOKENS["@ROSTERPATH@"] = spec_token(root, "framework/modules/process/artifacts/native-build/storage/image@1f34e") + "/" + TOKENS["@ROSTER@"] + "/" + TOKENS["@TS@"]
    state = {}
    problems = []
    for path, old, new, label in EDITS:
        old, new = expand(old), expand(new)
        target = resolve(root, path)
        if path not in state:
            if not os.path.isfile(target):
                problems.append(f"missing target {path}")
                continue
            state[path] = open(target, encoding="utf-8").read()
        count = state[path].count(old)
        if count != 1:
            problems.append(f"anchor {label} in {path}: {count} occurrences")
            continue
        state[path] = state[path].replace(old, new)
    for path, source in NEW_FILES:
        target = resolve(root, path)
        if os.path.exists(target):
            problems.append(f"new file exists {path}")
    if problems:
        print("\n".join(problems))
        raise SystemExit(1)
    if dry:
        print(f"dry-run ok: {len(EDITS)} anchored edits over {len(state)} files, {len(NEW_FILES)} new files")
        return
    for path, text in state.items():
        open(resolve(root, path), "w", encoding="utf-8").write(text)
    for path, source in NEW_FILES:
        target = resolve(root, path)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(expand(open(os.path.join(HERE, "staged", source), encoding="utf-8").read()))
    print(f"applied: {len(EDITS)} anchored edits over {len(state)} files, {len(NEW_FILES)} new files")

main()
