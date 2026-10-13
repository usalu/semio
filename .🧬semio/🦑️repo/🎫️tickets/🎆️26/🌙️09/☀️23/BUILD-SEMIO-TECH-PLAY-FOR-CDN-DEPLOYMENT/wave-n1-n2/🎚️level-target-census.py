"""🎚️ Census (and with --apply, removal) of explicit `test-<level>` targets: a level target identical to `test` plus the level argument is redundant now
that levels are arguments (`nx run X:test <level>`). REVIEWED lists the differing ones whose `test <level>` was checked by hand to be the same run (a level
target that lacked `forwardAllArgs` or a dependency `test` declares, or a Cargo test that ignores the level); every other differing one is migrated by hand."""
import json, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[8]
LEVELS = ("quick", "long", "exhaustive")
REVIEWED = {
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/📋️project.json",
    "🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📋️project.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json",
    "✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/📋️project.json",
    "🌎️hub/🧩️compositions/🌊️flow/📦️packages/🦀️rust/📋️project.json",
    "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📋️project.json",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📋️project.json",
}
apply = "--apply" in sys.argv


def skip_value(text, index):
    """⏭️ The index just past the JSON value starting at `index`."""
    depth, string = 0, False
    while True:
        char = text[index]
        if string:
            if char == "\\":
                index += 1
            elif char == '"':
                string = False
                if depth == 0:
                    return index + 1
        elif char == '"':
            string = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            if depth == 0:
                return index
            depth -= 1
            if depth == 0:
                return index + 1
        elif depth == 0 and char in ", \t\r\n":
            return index
        index += 1


def member_spans(text, start):
    """📍️ (key, key start, value end) of every member of the object opening at `start`."""
    spans, index = [], start + 1
    while True:
        while text[index] in " \t\r\n,":
            index += 1
        if text[index] == "}":
            return spans
        key_end = skip_value(text, index)
        key = json.loads(text[index:key_end])
        colon = text.index(":", key_end)
        value = colon + 1
        while text[value] in " \t\r\n":
            value += 1
        end = skip_value(text, value)
        spans.append((key, index, end))
        index = end


def remove_members(text, names):
    """✂️ Removes the named members of the top-level `targets` object, keeping every other byte."""
    top = member_spans(text, text.index("{"))
    key_start = next(begin for key, begin, _ in top if key == "targets")
    targets = text.index("{", text.index(":", key_start))
    spans = member_spans(text, targets)
    kept = [position for position, (key, _, _) in enumerate(spans) if key not in names]
    last = kept[-1] if kept else None
    cuts = []
    for position, (key, begin, end) in enumerate(spans):
        if key in names and last is not None and position < last:
            cuts.append((begin, spans[position + 1][1]))
    if spans and spans[-1][0] in names:
        tail_start = spans[last][2] if last is not None else targets + 1
        cuts.append((tail_start, spans[-1][2]))
    for begin, end in sorted(cuts, reverse=True):
        text = text[:begin] + text[end:]
    return text
files = [p for p in subprocess.run(["git", "ls-files", "-z", "--", "*project.json"], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0") if p.endswith("📋️project.json")]
identical, differing, edited = [], [], 0
for rel in files:
    path = ROOT / rel
    if not path.exists():
        continue
    text = path.read_text(encoding="utf8")
    data = json.loads(text)
    targets = data.get("targets") or {}
    base = targets.get("test")
    remove = []
    for level in LEVELS:
        name = f"test-{level}"
        if name not in targets:
            continue
        target = targets[name]
        expected = None
        if base is not None and isinstance(base.get("options", {}).get("command"), str):
            expected = json.loads(json.dumps(base))
            expected["options"]["command"] = f"{base['options']['command']} {level}"
        if expected is not None and (target == expected or rel in REVIEWED):
            identical.append((rel, name)); remove.append(name)
        else:
            keys = sorted(k for k in set(target) | set(expected or {}) if target.get(k) != (expected or {}).get(k))
            detail = {k: [target.get(k), (expected or {}).get(k)] for k in keys if k != "inputs"}
            differing.append((rel, name, keys, json.dumps(detail, ensure_ascii=False)[:400]))
    if apply and remove:
        updated = remove_members(text, set(remove))
        expected = {k: v for k, v in targets.items() if k not in remove}
        if json.loads(updated).get("targets") != expected or {k: v for k, v in json.loads(updated).items() if k != "targets"} != {k: v for k, v in data.items() if k != "targets"}:
            raise SystemExit(f"span removal changed more than {remove} in {rel}")
        path.write_text(updated, encoding="utf8")
        edited += 1
print(f"identical {len(identical)} in {len({r for r, _ in identical})} file(s); differing {len(differing)}; edited {edited}")
for rel, name, keys, detail in differing:
    print(f"DIFF {rel} {name} keys={keys}\n  {detail}")
