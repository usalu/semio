#!/usr/bin/env python3
"""📏️ Fast partition scan of the `schema-*` invariants the W6d partition owns.

Mirrors `inventorySchemaScopes` (📚️library/🔍️discovery/🟦️.ts) and
`schemaExportCompletenessDiagnostics` (🧪️test/📦️packages/🟦️typescript/🟦️.ts) for `✏️s/**` outside
`🧬️schema/🧬️mutations/**`. It exists because a full `bun … test schema` walk takes tens of minutes on
a loaded box; the authoritative numbers still come from that command.

Usage: python3 wp4c-scan.py [--json OUT] [--plugin EMOJINAME]
"""
from __future__ import annotations
import json, os, re, subprocess, sys, collections

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
TAX = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")
LEAF = {"🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🔣️jsonschema": "🔣️.json", "🛰️protobuf": "🛰️.proto", "📜️wit": "📜️.wit"}
NORMATIVE = "🔣️jsonschema"
FACET_DIR = "🧬️schema"
MUT = "/🧬️mutations/"
GRAPHQL_KEYWORDS = ("type", "input", "enum", "interface", "union", "scalar")

taxonomy = json.load(open(TAX, encoding="utf-8"))
CHILD_DIRS = set(taxonomy["schemaChildDirs"]) | set(taxonomy["representationDirs"])
DIALECT = taxonomy["schemaJsonDialect"]
EXPORT_RE = re.compile(taxonomy["schemaExportResolution"]["exportIdPattern"])
ID_BASE = taxonomy["schemaExportResolution"]["idBase"]
CANONICAL = set(LEAF.values())


def repo_files() -> list[str]:
    out = subprocess.run(["git", "ls-files", "-z", "✏️s"], cwd=REPO, capture_output=True, check=True).stdout
    tracked = [entry.decode() for entry in out.split(b"\0") if entry]
    return sorted({path for path in tracked if os.path.exists(os.path.join(REPO, path))})


def module_of(path: str) -> str | None:
    parts = path.split("/")
    for index in range(len(parts) - 1, -1, -1):
        if parts[index] == FACET_DIR:
            return "/".join(parts[: index + 1])
    return None


def declares(fmt: str, source: str, name: str) -> bool:
    escaped = re.escape(name)
    if fmt == "🛰️protobuf":
        return re.search(rf"^\s*message\s+{escaped}\b", source, re.M) is not None
    if fmt == "🔗️graphql":
        return re.search(rf"^\s*(?:{'|'.join(GRAPHQL_KEYWORDS)})\s+{escaped}\b", source, re.M) is not None
    if fmt == "🦀️rust":
        return re.search(rf"^\s*pub\s+(?:struct|enum|type)\s+{escaped}\b", source, re.M) is not None
    if fmt == "🟦️typescript":
        return re.search(rf"^\s*export\s+(?:interface|type|const|class)\s+{escaped}\b", source, re.M) is not None
    return False


def declares_parser(fmt: str, source: str, name: str) -> bool:
    if fmt != "🟦️typescript":
        return True
    return re.search(rf"^\s*export\s+(?:(?:async\s+)?function|const|let|declare\s+function)\s+parse{re.escape(name)}\b", source, re.M) is not None


def document_exports(document: dict) -> tuple[list[str], dict[str, list[str]]]:
    exports: list[str] = []
    restricted: dict[str, list[str]] = {}
    root = document.get("title")
    root_object = document.get("type") == "object" or ("type" not in document and isinstance(document.get("properties"), dict))
    if isinstance(root, str) and root_object and EXPORT_RE.match(root):
        exports.append(root)
        if isinstance(document.get("x-semio-formats"), list):
            restricted[root] = document["x-semio-formats"]
    for key, value in (document.get("$defs") or {}).items():
        if EXPORT_RE.match(key) and key not in exports:
            exports.append(key)
            if isinstance(value, dict) and isinstance(value.get("x-semio-formats"), list):
                restricted[key] = value["x-semio-formats"]
    return exports, restricted


def scan(files: list[str]):
    modules: dict[str, list[str]] = collections.defaultdict(list)
    for path in files:
        module = module_of(path)
        if module is not None:
            modules[module].append(path)
    module_paths = sorted(modules)
    nested = set(module_paths)

    scopes: dict[str, dict] = {}
    findings: list[dict] = []
    for module in module_paths:
        if MUT in module + "/" or not module.startswith("✏️s/"):
            continue
        owned = []
        for path in modules[module]:
            relative = path[len(module) + 1 :]
            segments = relative.split("/")
            if segments[-1] not in CANONICAL or any(segment not in CHILD_DIRS for segment in segments[:-1]):
                continue
            if any(f"{module}/{'/'.join(segments[:index])}" in nested for index in range(1, len(segments))):
                continue
            owned.append(path)
        formats = {fmt: leaf for fmt, leaf in LEAF.items() if f"{module}/{leaf}" in set(modules[module])}
        if NORMATIVE not in formats:
            continue
        documents = []
        for path in sorted(owned):
            if not path.endswith(LEAF[NORMATIVE]):
                continue
            try:
                parsed = json.load(open(os.path.join(REPO, path), encoding="utf-8"))
            except Exception as error:  # noqa: BLE001
                findings.append({"code": "schema-document-unparseable", "path": path, "detail": str(error)})
                continue
            documents.append((path, parsed))
        root_document = next((parsed for path, parsed in documents if path == f"{module}/{LEAF[NORMATIVE]}"), None)
        if root_document is None:
            continue
        scope_id = None
        module_id = root_document.get("$id")
        if isinstance(module_id, str) and module_id.startswith(ID_BASE):
            scope_id = ".".join(module_id[len(ID_BASE) :].split("/")[:-1])
        if not scope_id:
            findings.append({"code": "schema-module-id-missing", "path": f"{module}/{LEAF[NORMATIVE]}", "detail": "no addressable $id"})
            continue
        exports: dict[str, dict] = {}
        restrictions: dict[str, list[str]] = {}
        for path, parsed in documents:
            if parsed.get("$schema") != DIALECT:
                findings.append({"code": "schema-dialect-not-draft-07", "path": path, "scope": scope_id, "detail": f"$schema is {parsed.get('$schema')!r}"})
            names, restricted = document_exports(parsed)
            restrictions.update(restricted)
            relative = path[len(module) + 1 :]
            for name in names:
                if name in exports:
                    if exports[name]["file"] != relative:
                        findings.append({"code": "schema-export-id-duplicate", "path": path, "scope": scope_id, "export": name, "detail": f"already declared by {exports[name]['file']}"})
                    continue
                exports[name] = {"file": relative}
        scopes[scope_id] = {"path": module, "formats": formats, "exports": exports, "restrictions": restrictions, "documents": documents}

    sources: dict[str, str] = {}

    def source_of(path: str) -> str | None:
        if path not in sources:
            absolute = os.path.join(REPO, path)
            sources[path] = open(absolute, encoding="utf-8").read() if os.path.exists(absolute) else None
        return sources[path]

    for scope_id, scope in scopes.items():
        for name, row in scope["exports"].items():
            restricted = scope["restrictions"].get(name)
            if restricted is not None:
                for fmt in restricted:
                    if fmt not in scope["formats"]:
                        findings.append({"code": "schema-export-incomplete", "scope": scope_id, "export": name, "format": fmt, "path": scope["path"], "detail": "declared format the scope does not implement"})
            facet_dir = row["file"].rsplit("/", 1)[0] if "/" in row["file"] else ""
            for fmt in scope["formats"]:
                supported = restricted is None or fmt in restricted
                if fmt == NORMATIVE:
                    if not supported:
                        findings.append({"code": "schema-export-format-undeclared", "scope": scope_id, "export": name, "format": fmt, "path": f"{scope['path']}/{row['file']}"})
                    continue
                file = f"{scope['path']}/{facet_dir}/{LEAF[fmt]}" if facet_dir else f"{scope['path']}/{LEAF[fmt]}"
                source = source_of(file)
                if source is None:
                    if supported:
                        findings.append({"code": "schema-file-missing", "scope": scope_id, "export": name, "format": fmt, "path": file})
                    continue
                present = declares(fmt, source, name)
                if supported and not present:
                    findings.append({"code": "schema-export-incomplete", "scope": scope_id, "export": name, "format": fmt, "path": file})
                if supported and present and not declares_parser(fmt, source, name):
                    findings.append({"code": "schema-export-parser-missing", "scope": scope_id, "export": name, "format": fmt, "path": file})
                if not supported and present:
                    findings.append({"code": "schema-export-format-undeclared", "scope": scope_id, "export": name, "format": fmt, "path": file})
    return scopes, findings


def main() -> None:
    files = repo_files()
    scopes, findings = scan(files)
    plugin = None
    if "--plugin" in sys.argv:
        plugin = sys.argv[sys.argv.index("--plugin") + 1]
        findings = [row for row in findings if plugin in (row.get("path") or "")]
    by_code = collections.Counter(row["code"] for row in findings)
    by_plugin = collections.Counter((row.get("path") or "/").split("/")[2] if (row.get("path") or "").startswith("✏️s/🔌️plugins/") else "?" for row in findings)
    summary = {"scopes": len(scopes), "findings": len(findings), "byCode": dict(by_code.most_common()), "byPlugin": dict(by_plugin.most_common())}
    if "--json" in sys.argv:
        json.dump({"summary": summary, "findings": findings}, open(sys.argv[sys.argv.index("--json") + 1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps(summary, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
