#!/usr/bin/env python3
"""🔎️ U7 census: every example loader (the `setActiveExample` reduction and the helpers it calls) in the guest tree,
classified by how it resolves, decodes and seats an example and by every silent fallback it takes.

Usage: python3 u7-census.py [--json OUT] [--md OUT]
"""
import json, re, subprocess, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SWITCH = re.compile(r'SET_ACTIVE_EXAMPLE_ACTION_ID|"setActiveExample"|keyword = "(set-)?active-example"|SetActiveExample')
FN = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)\s*(?:<[^{;]*?>)?\s*\(', re.M)
EXAMPLE_REF = re.compile(r'example_id|examples::|SetActiveExample|PRIMARY_TEXT|examples\(\)|example_snapshot|example_document|ExampleSource')


def tracked(paths):
    out = subprocess.run(["git", "grep", "-l", "-E", SWITCH.pattern, "--", *paths, ":!*/🧪️tests/*", ":!*.ts", ":!*.tsx", ":!*.json", ":!*.md"], cwd=ROOT, capture_output=True, text=True).stdout
    return [line for line in out.splitlines() if line]


def functions(text):
    for match in FN.finditer(text):
        start = match.start()
        brace = text.find("{", match.end())
        semi = text.find(";", match.end())
        if brace < 0 or (0 <= semi < brace):
            continue
        depth, index = 0, brace
        while index < len(text):
            char = text[index]
            if char == "{":
                depth += 1
            elif char == "}":
                depth -= 1
                if depth == 0:
                    break
            elif char == '"':
                index += 1
                while index < len(text) and text[index] != '"':
                    index += 2 if text[index] == "\\" else 1
            elif char == "/" and text.startswith("//", index):
                index = text.find("\n", index)
                if index < 0:
                    index = len(text)
                continue
            index += 1
        yield match.group(1), start, index + 1, text[start:index + 1]


def artifact_of(path):
    match = re.search(r'🔌️plugins/([^/]+)/(?:🗿️artifacts/([^/]+)/(?:🏅️standards/([^/]+)/🪆️subsets/([^/]+))?)?', path)
    if not match:
        return path, "", ""
    plugin, artifact, standard, subset = match.groups()
    return plugin, artifact or "", "/".join(part for part in (standard, subset) if part)


def classify(body):
    tags = []
    if re.search(r'(parse_dsl|from_pack|decode[a-z_]*|from_json[a-z_]*|from_str)\([^;]*\)\s*\.unwrap_or_default\(\)', body, re.S) or re.search(r'PRIMARY_TEXT\)\s*\.unwrap_or_default\(\)', body):
        tags.append("decode→default")
    if re.search(r'(parse_dsl|from_pack|decode[a-z_]*|from_json[a-z_]*)\([^;]*\)\s*\.ok\(\)', body, re.S):
        tags.append("decode→ok()")
    if re.search(r'(parse_dsl|from_pack|decode[a-z_]*)\([^;]*\)\s*\.unwrap_or_else\(\|_\|', body, re.S):
        tags.append("decode→else")
    if re.search(r'if let Ok\([^)]*\)\s*=\s*[^{]*(parse_dsl|from_pack|decode)', body):
        tags.append("decode→if-let-ok")
    if re.search(r'else\s*\{\s*[A-Za-z0-9_:<>]*::default\(\)\s*\}', body) and re.search(r'example', body):
        tags.append("unknown→default")
    if re.search(r'else\s*\{\s*return Ok\(Emit::default\(\)\);?\s*\}', body) or re.search(r'None\s*=>\s*(return\s+)?Ok\(Emit::default\(\)\)', body):
        tags.append("unknown→noop")
    if re.search(r'\.unwrap_or\([A-Za-z0-9_:]*::default\(\)\)', body):
        tags.append("unwrap_or(default)")
    if re.search(r'(parse_dsl|from_pack|decode[a-z_]*)\([^;]*\)\s*\.(expect|unwrap)\(', body, re.S):
        tags.append("decode→panic")
    if re.search(r'Fault::(new|from)\(', body):
        tags.append("fault")
    if re.search(r'Effect::LoadDocument|load_example_effect|reset_document_effect|LoadDocument', body):
        tags.append("seat:LoadDocument")
    if re.search(r'artifact_mutations', body):
        tags.append("seat:mutations")
    return tags


def main():
    paths = tracked(["✏️s/🔌️plugins"])
    rows = []
    for rel in paths:
        text = (ROOT / rel).read_text()
        plugin, artifact, dialect = artifact_of(rel)
        for name, start, end, body in functions(text):
            if not EXAMPLE_REF.search(body) or name.startswith("test_"):
                continue
            if not re.search(r'example', name, re.I) and not re.search(r'SetActiveExample|PRIMARY_TEXT|examples::', body):
                continue
            line = text.count("\n", 0, start) + 1
            tags = classify(body)
            if not tags:
                continue
            rows.append({"plugin": plugin, "artifact": artifact, "dialect": dialect, "file": rel, "fn": name, "line": line, "tags": tags})
    args = sys.argv[1:]
    out_json = args[args.index("--json") + 1] if "--json" in args else None
    out_md = args[args.index("--md") + 1] if "--md" in args else None
    if out_json:
        Path(out_json).write_text(json.dumps(rows, ensure_ascii=False, indent=1))
    lines = ["| plugin | artifact | dialect | fn | line | tags |", "|---|---|---|---|---|---|"]
    for row in rows:
        lines.append(f"| {row['plugin']} | {row['artifact']} | {row['dialect']} | `{row['fn']}` | {row['line']} | {', '.join(row['tags'])} |")
    if out_md:
        Path(out_md).write_text("\n".join(lines) + "\n")
    silent = [row for row in rows if any(tag in row["tags"] for tag in ("decode→default", "decode→ok()", "decode→else", "decode→if-let-ok", "unknown→default", "unknown→noop", "unwrap_or(default)"))]
    print(f"files={len(paths)} loader-fns={len(rows)} silent={len(silent)} loaddocument={sum('seat:LoadDocument' in row['tags'] for row in rows)}")


if __name__ == "__main__":
    main()
