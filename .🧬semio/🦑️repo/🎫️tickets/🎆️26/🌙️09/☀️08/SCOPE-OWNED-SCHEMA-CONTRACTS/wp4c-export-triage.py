#!/usr/bin/env python3
"""🩺 Triage of the `schema-export-incomplete` / `schema-export-parser-missing` rows of `✏️s/**`.

Every row names a JSON `$defs` export that a format the scope provides does not declare. Execution
contract §A/§B admit exactly four honest answers, and this script decides which one a row gets from
evidence rather than from taste:

- `reexport`  — a same-named Rust entity exists elsewhere in the plugin, so the module `pub use`s it.
- `project`   — the export is addressed from outside (a cross-document `$ref`, a `schema://` fixture
                binding, or another format already declares it), so the missing format must state it.
- `helper`    — nothing outside the document addresses it and it is referenced from inside it: a
                module-internal helper, which contract §A puts in `definitions`, not in `$defs`.
- `dead`      — nothing addresses it at all, inside or out: contract §B deletes it with its data.

Usage: python3 wp4c-export-triage.py <findings.json> [--json OUT]
"""
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import collections

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
CODES = ("schema-export-incomplete", "schema-export-parser-missing")


def plugin_of(path: str) -> str | None:
    parts = path.split("/")
    return "/".join(parts[:3]) if len(parts) > 3 and parts[0] == "✏️s" and parts[1] == "🔌️plugins" else None


def module_of(path: str) -> str:
    parts = path.split("/")
    for index in range(len(parts) - 1, -1, -1):
        if parts[index] == "🧬️schema":
            return "/".join(parts[: index + 1])
    return os.path.dirname(path)


def json_documents() -> list[str]:
    out = subprocess.run(["git", "ls-files", "-z"], cwd=REPO, capture_output=True, check=True).stdout
    return [entry.decode() for entry in out.split(b"\0") if entry.endswith(b".json")]


def main() -> None:
    findings = json.load(open(sys.argv[1], encoding="utf-8"))
    rows = [row for row in (findings["diagnostics"] if isinstance(findings, dict) else findings)
            if row.get("code") in CODES and (row.get("path") or "").startswith("✏️s/") and "/🧬️mutations/" not in (row.get("path") or "")]

    # 📇 Every cross-document reference and fixture binding in the repository, once.
    referenced: set[tuple[str, str]] = set()          # (target $id, export)
    uris: set[str] = set()                            # schema://scope/Export
    text_by_file: dict[str, str] = {}
    for path in json_documents():
        try:
            with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                text = handle.read()
        except OSError:
            continue
        if "https://semio.tech/schema/" in text:
            for target, export in re.findall(r'"(https://semio\.tech/schema/[^"#]+)#/\$defs/([A-Za-z0-9]+)"', text):
                referenced.add((target, export))
        for uri in re.findall(r'"schema://([a-z0-9.\-]+)/([A-Za-z0-9]+)"', text):
            uris.add(f"{uri[0]}/{uri[1]}")
        text_by_file[path] = text

    # 🧬️Which other schema documents declare the same export id, so a restatement is told from a helper.
    declared_by: dict[str, list[str]] = {}
    for path, text in text_by_file.items():
        if "/🧬️schema/" not in f"/{path}" or not path.endswith("🔣️.json"):
            continue
        try:
            document = json.loads(text)
        except Exception:  # noqa: BLE001
            continue
        names = set((document.get("$defs") or {}).keys()) if isinstance(document, dict) else set()
        title = document.get("title") if isinstance(document, dict) else None
        if isinstance(title, str):
            names.add(title)
        for name in names:
            declared_by.setdefault(name, []).append(path)

    rust_index: dict[str, set[str]] = {}
    for base, dirs, files in os.walk(os.path.join(REPO, "✏️s")):
        dirs[:] = [name for name in dirs if name not in ("node_modules", ".venv", "target", "dist")]
        for name in files:
            if name != "🦀️.rs":
                continue
            path = os.path.relpath(os.path.join(base, name), REPO)
            plugin = plugin_of(path)
            if plugin is None:
                continue
            with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                source = handle.read()
            rust_index.setdefault(plugin, set()).update(re.findall(r"^\s*pub\s+(?:struct|enum|type)\s+([A-Za-z0-9_]+)", source, re.M))

    decided: list[dict] = []
    for row in rows:
        path = row["path"]
        export = row.get("export")
        if not export:
            continue
        module = module_of(path)
        plugin = plugin_of(path)
        document = f"{module}/{os.path.dirname(path[len(module) + 1:])}/🔣️.json".replace("//", "/") if os.path.dirname(path[len(module) + 1:]) else f"{module}/🔣️.json"
        text = text_by_file.get(document, "")
        document_id = None
        match = re.search(r'"\$id"\s*:\s*"([^"]+)"', text)
        if match:
            document_id = match.group(1)
        outside = any(target == document_id and name == export for target, name in referenced) if document_id else False
        bound = f"{row.get('scope')}/{export}" in uris
        local = len(re.findall(rf'"#/\$defs/{re.escape(export)}"', text)) > 0
        in_rust = plugin is not None and export in rust_index.get(plugin, set())
        others = [other for other in declared_by.get(export, []) if other != document]
        is_title = False
        try:
            is_title = json.loads(text_by_file.get(document, "{}")).get("title") == export
        except Exception:  # noqa: BLE001
            pass
        if row["code"] == "schema-export-parser-missing":
            decision = "parser"
        elif others and not is_title:
            decision = "restatement"
        elif row.get("format") == "🦀️rust" and in_rust:
            decision = "reexport"
        elif outside or bound:
            decision = "project"
        elif local and not is_title:
            decision = "helper"
        elif row.get("format") == "🦀️rust":
            decision = "rust-absent"
        else:
            decision = "project"
        decided.append({**row, "decision": decision, "document": document, "documentId": document_id,
                        "outside": outside, "bound": bound, "localRef": local, "rustElsewhere": in_rust,
                        "title": is_title, "alsoDeclaredBy": others[:4], "module": module, "plugin": plugin})

    counts = collections.Counter(row["decision"] for row in decided)
    per_plugin = collections.Counter((row["plugin"], row["decision"]) for row in decided)
    print(json.dumps({"rows": len(decided), "byDecision": dict(counts)}, ensure_ascii=False, indent=1))
    plugins = sorted({row["plugin"] for row in decided if row["plugin"]})
    for plugin in plugins:
        line = {decision: per_plugin[(plugin, decision)] for decision in sorted(counts) if per_plugin[(plugin, decision)]}
        print(f"  {plugin}: {json.dumps(line, ensure_ascii=False)}")
    if "--json" in sys.argv:
        with open(sys.argv[sys.argv.index("--json") + 1], "w", encoding="utf-8") as handle:
            json.dump(decided, handle, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
