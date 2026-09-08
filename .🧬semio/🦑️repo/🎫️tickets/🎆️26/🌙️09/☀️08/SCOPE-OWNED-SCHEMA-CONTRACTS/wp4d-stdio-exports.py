#!/usr/bin/env python3
"""🧾 Brings the `🗄️stdio` artifact-subset schema modules onto the export half of the execution
contract (§A/§B, ledger rows 99/111/123): one export id per scope resolves to exactly one document,
cross-document references address an export by absolute `$id`, module-internal helpers live in
`definitions`, and an export that a format genuinely does not carry says so with `x-semio-formats`.

The mutation trees (`🧬️schema/🧬️mutations/**`) are `wp4-stdio-schemas.py`'s and are never touched
here; the codec facet documents under them are, because they are facets of the subset scope.

Subcommands
  survey    — per-scope inventory: documents, exports, duplicates, references, format coverage
  titles    — PascalCase / de-collide the facet document titles (`<Owner><Text|Binary>`)
  refs      — path-style `$ref` → `<target $id>#/$defs/<Export>`, promoting the named helper
  defkeys   — non-PascalCase `$defs` keys → the crate spelling, or `definitions` when internal
  dedupe    — a facet document never restates the owning document's export; it references it
  formats   — `x-semio-formats` per export, from what the module's format files actually declare
"""

from __future__ import annotations

import json
import os
import re
import sys
from collections import Counter, OrderedDict, defaultdict

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
ARTIFACTS = os.path.join(REPO, "✏️s", "🔌️plugins", "🗄️stdio", "🗿️artifacts")
DIALECT = "http://json-schema.org/draft-07/schema#"
ID_BASE = "https://semio.tech/schema/"
SCHEMA_DIR = "🧬️schema"
MUTATIONS_DIR = "🧬️mutations"
LEAF = "🔣️.json"
PASCAL = re.compile(r"^[A-Z][A-Za-z0-9]*$")

# 🔣 taxonomy `schemaFormats` → the module leaf each format is written in, and the declaration
# keywords `📚️library/🔍️discovery/🟦️.ts:3079 schemaFormatDeclaresExport` accepts for it.
FORMAT_FILES = {"🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🛰️protobuf": "🛰️.proto"}
FORMAT_KEYWORDS = {
    "🦀️rust": ["struct", "enum", "type", "trait", "fn"],
    "🟦️typescript": ["interface", "type", "class", "enum", "const", "function"],
    "🔗️graphql": ["type", "input", "enum", "union", "scalar", "interface"],
    "🛰️protobuf": ["message", "enum", "service"],
}
NORMATIVE = "🔣️jsonschema"


def declares(format_id: str, source: str, exported: str) -> bool:
    """🔎 The presence rule of one format, mirroring the two instruments that measure it: the root
    `schema check` (`schemaFormatDeclaresExport`) and the harness (`declaresSchemaExport` +
    `declaresSchemaExportParser`). A name is present only where a same-named entity is declared."""
    name = re.escape(exported)
    if re.search(rf"\b(?:{'|'.join(FORMAT_KEYWORDS[format_id])})\s+{name}\b", source):
        return True
    if format_id == "🦀️rust":
        return bool(re.search(rf"\bpub\s+use\b[^;]*\b{name}\b", source))
    if format_id != "🟦️typescript":
        return False
    return bool(re.search(rf"\bparse{name}\b", source)) or bool(re.search(rf"\bexport\b(?:\s+type)?\s*\{{[^}}]*\b{name}\b[^}}]*\}}", source))


def read(path: str) -> str | None:
    try:
        with open(path, encoding="utf8") as handle:
            return handle.read()
    except OSError:
        return None


def load(path: str) -> "OrderedDict[str, object]":
    with open(path, encoding="utf8") as handle:
        return json.load(handle, object_pairs_hook=OrderedDict)


def dump(path: str, document) -> None:
    with open(path, "w", encoding="utf8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def modules() -> list[str]:
    """🪆 Every artifact-subset `🧬️schema` module of the partition."""
    found: list[str] = []
    for artifact in sorted(os.listdir(ARTIFACTS)):
        standards = os.path.join(ARTIFACTS, artifact, "🏅️standards")
        if not os.path.isdir(standards):
            continue
        for standard in sorted(os.listdir(standards)):
            subsets = os.path.join(standards, standard, "🪆️subsets")
            if not os.path.isdir(subsets):
                continue
            for subset in sorted(os.listdir(subsets)):
                module = os.path.join(subsets, subset, SCHEMA_DIR)
                if os.path.isdir(module):
                    found.append(module)
    return found


def documents(module: str) -> list[dict]:
    """🧾 The module's own schema documents: the root and every facet child, excluding the mutation
    tree (another partition) and any test/fixture collection."""
    found: list[dict] = []
    for directory, subdirectories, files in os.walk(module):
        subdirectories[:] = [name for name in subdirectories if name not in ("target", "node_modules") and not name.startswith(("🧪️", "🧫️"))]
        if LEAF not in files:
            continue
        facet = os.path.relpath(directory, module)
        facet = "" if facet == "." else facet
        if facet.split(os.sep)[0] == MUTATIONS_DIR:
            continue
        path = os.path.join(directory, LEAF)
        try:
            document = load(path)
        except Exception as error:  # noqa: BLE001 — an unreadable document is reported, never skipped
            print(f"   UNREADABLE {os.path.relpath(path, REPO)}: {error}")
            continue
        found.append({"path": path, "rel": os.path.relpath(path, REPO), "dir": directory, "facet": facet, "document": document})
    return sorted(found, key=lambda entry: entry["rel"])


def root_is_object(document) -> bool:
    return document.get("type") == "object" or (document.get("type") is None and isinstance(document.get("properties"), dict))


def exports_of(document) -> list[str]:
    found: list[str] = []
    title = document.get("title")
    if isinstance(title, str) and root_is_object(document):
        found.append(title)
    for key in document.get("$defs", {}):
        if key not in found:
            found.append(key)
    return found


def sibling_source(entry: dict, format_id: str) -> str | None:
    return read(os.path.join(entry["dir"], FORMAT_FILES[format_id]))


def module_source(module: str, format_id: str) -> str | None:
    return read(os.path.join(module, FORMAT_FILES[format_id]))


def module_formats(module: str) -> list[str]:
    return [format_id for format_id, leaf in FORMAT_FILES.items() if os.path.isfile(os.path.join(module, leaf))]


def command_survey() -> None:
    scopes = duplicates = pathrefs = badkeys = 0
    coverage: Counter = Counter()
    for module in modules():
        entries = documents(module)
        if not entries:
            continue
        scopes += 1
        seen: dict[str, str] = {}
        for entry in entries:
            for exported in exports_of(entry["document"]):
                if exported in seen:
                    duplicates += 1
                else:
                    seen[exported] = entry["facet"] or "<root>"
                if not PASCAL.match(exported):
                    badkeys += 1
            body = json.dumps(entry["document"], ensure_ascii=False)
            for ref in re.findall(r'"\$ref"\s*:\s*"([^"]+)"', body):
                if not ref.startswith("#") and not ref.startswith(ID_BASE):
                    pathrefs += 1
        for exported, facet in seen.items():
            entry = next(one for one in entries if (one["facet"] or "<root>") == facet)
            for format_id in module_formats(module):
                source = sibling_source(entry, format_id)
                coverage[(format_id, "sibling-missing" if source is None else ("declares" if declares(format_id, source, exported) else "absent"))] += 1
    print(f"scopes={scopes} duplicate-export-declarations={duplicates} path-style-refs={pathrefs} non-pascal-export-ids={badkeys}")
    for key, value in sorted(coverage.items()):
        print(f"   {key[0]:14s} {key[1]:16s} {value}")


def module_id_base(module: str) -> str | None:
    """🆔 The scope path every facet document of a module varies only the filename of."""
    root = os.path.join(module, LEAF)
    if not os.path.isfile(root):
        return None
    identity = load(root).get("$id")
    if not isinstance(identity, str) or not identity.startswith(ID_BASE):
        return None
    return identity[: identity.rfind("/") + 1]


def rewrite_refs(document, mapping: dict[str, str]) -> tuple[int, "OrderedDict[str, object]"]:
    """🔁 Textual `$ref` substitution over one document, counting the references it moved."""
    body = json.dumps(document, ensure_ascii=False)
    moved = 0
    for old, new in mapping.items():
        needle = f'"$ref": "{old}"'
        moved += body.count(needle)
        body = body.replace(needle, f'"$ref": "{new}"')
    return moved, json.loads(body, object_pairs_hook=OrderedDict)


def command_refs(write: bool) -> None:
    """🔗 A cross-document reference names the target `$id` and addresses an export (contract §A). The
    stdio facet documents still spell filesystem paths (`snapshot.json#/definitions/LasHeader`), which
    resolve for nobody: the checker rejects the form and ajv resolves it against the `$id` base, where
    no such document is registered. The target document is found by facet filename inside the same
    module, and the member the reference names is promoted out of `definitions` into `$defs` — a
    member another document references IS an export of this scope, which is what promotes it."""
    rewritten = promoted = unresolved = 0
    problems: list[str] = []
    for module in modules():
        base = module_id_base(module)
        entries = documents(module)
        if base is None or not entries:
            continue
        by_facet_filename: dict[str, dict] = {}
        for entry in entries:
            identity = entry["document"].get("$id")
            if isinstance(identity, str) and identity.startswith(base):
                by_facet_filename[identity[len(base) :]] = entry
        for entry in entries:
            body = json.dumps(entry["document"], ensure_ascii=False)
            mapping: dict[str, str] = {}
            for ref in sorted(set(re.findall(r'"\$ref"\s*:\s*"([^"]+)"', body))):
                if ref.startswith("#") or ref.startswith(ID_BASE):
                    continue
                target_name, _, fragment = ref.partition("#")
                target = by_facet_filename.get(target_name)
                for start in (entry["dir"], module):
                    if target is not None:
                        break
                    candidate = os.path.normpath(os.path.join(start, target_name))
                    target = next((one for one in entries if one["path"] == candidate), None)
                if target is None:
                    problems.append(f"{entry['rel']}: {ref} names no document of this module")
                    unresolved += 1
                    continue
                identity = target["document"]["$id"]
                if not fragment:
                    mapping[ref] = identity
                    continue
                if not fragment.startswith(("/$defs/", "/definitions/")) or fragment.count("/") != 2:
                    problems.append(f"{entry['rel']}: {ref} addresses a pointer, not an export — hand-decide the export it means")
                    unresolved += 1
                    continue
                member = fragment.rsplit("/", 1)[-1]
                if member not in target["document"].get("$defs", {}):
                    helpers = target["document"].get("definitions", {})
                    if member not in helpers:
                        problems.append(f"{entry['rel']}: {ref} names {member}, which {target['rel']} declares nowhere")
                        unresolved += 1
                        continue
                    target["document"].setdefault("$defs", OrderedDict())[member] = helpers.pop(member)
                    if not helpers:
                        target["document"].pop("definitions", None)
                    target["dirty"] = True
                    promoted += 1
                mapping[ref] = f"{identity}#/$defs/{member}"
            if mapping:
                moved, entry["document"] = rewrite_refs(entry["document"], mapping)
                rewritten += moved
                entry["dirty"] = True
        for entry in entries:
            if entry.get("dirty") and write:
                dump(entry["path"], entry["document"])
    print(f"path-style-refs-rewritten={rewritten} helpers-promoted-to-exports={promoted} unresolved={unresolved} (write={write})")
    for problem in problems[:40]:
        print("   UNRESOLVED", problem)


def crate_spelling(module: str, entry: dict, key: str) -> str | None:
    """🐫 The PascalCase name the crate gives a camelCase `$defs` key, confirmed by a real declaration
    in one of the module's format files. Without a declaration the key names no type and is a helper."""
    bare = key[0].upper() + key[1:]
    if not PASCAL.match(bare):
        bare = re.sub(r"[^A-Za-z0-9]", "", bare.title())
    if not PASCAL.match(bare):
        return None
    # 🏷 The crate qualifies its types with the artifact's own prefix (`shape` is `PptxShape`,
    # `point2` is `SemioPoint2`), so the prefixes this module's own titles begin with are tried too —
    # each only ever accepted against a real declaration, never as a naming convention.
    prefixes = [""]
    for other in documents(module):
        title = other["document"].get("title")
        if not isinstance(title, str) or not PASCAL.match(title):
            continue
        stem = re.sub(r"(Artifact|Snapshot|Diff|Inference|Mutation)(Text|Binary)?$", "", title)
        words = re.findall(r"[A-Z][a-z0-9]*", stem)
        for start in range(len(words)):
            for end in range(start + 1, len(words) + 1):
                head = "".join(words[start:end])
                if head and head not in prefixes:
                    prefixes.append(head)
    for candidate in [f"{prefix}{bare}" for prefix in prefixes]:
        for format_id in module_formats(module):
            for source in (sibling_source(entry, format_id), module_source(module, format_id)):
                if source is not None and declares(format_id, source, candidate):
                    return candidate
    return None


def external_defs_references(entries: list[dict]) -> dict[str, set[str]]:
    """🔗 Per document `$id`, the `$defs` members OTHER documents of the scope address. A member another
    document references cannot be demoted to `definitions`: a cross-document reference must address an
    export (contract §A), so the reference decides that the member is one."""
    referenced: dict[str, set[str]] = defaultdict(set)
    for entry in entries:
        body = json.dumps(entry["document"], ensure_ascii=False)
        for ref in re.findall(r'"\$ref"\s*:\s*"([^"#]+)#/\$defs/([A-Za-z0-9_]+)"', body):
            if ref[0] != entry["document"].get("$id"):
                referenced[ref[0]].add(ref[1])
    return referenced


def command_defkeys(write: bool) -> None:
    """🔠 An export id is PascalCase and names the same entity in every format (contract §A). A
    camelCase `$defs` key is one of two things and the crate decides which: the module declares the
    PascalCase type, so the key is a misspelling of a real export — or it declares nothing, and the
    key is a module-internal helper, which `definitions` is for (ledger row 77)."""
    renamed = demoted = stuck = 0
    problems: list[str] = []
    for module in modules():
        entries = documents(module)
        addressed = external_defs_references(entries)
        for entry in entries:
            document = entry["document"]
            defs = document.get("$defs")
            if not isinstance(defs, dict):
                continue
            offenders = [key for key in defs if not PASCAL.match(key)]
            if not offenders:
                continue
            outside = addressed.get(document.get("$id"), set())
            local: dict[str, str] = {}
            for key in offenders:
                spelled = crate_spelling(module, entry, key)
                if spelled is not None and spelled not in defs:
                    local[f"#/$defs/{key}"] = f"#/$defs/{spelled}"
                    defs[spelled] = defs.pop(key)
                    renamed += 1
                elif key in outside:
                    problems.append(f"{entry['rel']}: $defs.{key} is addressed by another document and no format declares a PascalCase spelling of it")
                    stuck += 1
                else:
                    local[f"#/$defs/{key}"] = f"#/definitions/{key}"
                    document.setdefault("definitions", OrderedDict())[key] = defs.pop(key)
                    demoted += 1
            # 🔁 Reordering `$defs` keeps the file readable and the rename deterministic.
            document["$defs"] = OrderedDict(sorted(defs.items()))
            if not document["$defs"]:
                document.pop("$defs")
            moved, entry["document"] = rewrite_refs(document, local)
            entry["dirty"] = True
            identity = entry["document"].get("$id")
            for other in entries:
                if other is entry or not isinstance(identity, str):
                    continue
                mapping = {f"{identity}{old[1:]}": f"{identity}{new[1:]}" for old, new in local.items()}
                count, other["document"] = rewrite_refs(other["document"], mapping)
                if count:
                    other["dirty"] = True
        for entry in entries:
            if entry.get("dirty") and write:
                dump(entry["path"], entry["document"])
    for problem in problems[:20]:
        print("   ", problem)
    print(f"defs-keys-renamed-to-crate-spelling={renamed} demoted-to-definitions={demoted} unresolvable={stuck} (write={write})")


def command_dedupe(write: bool) -> None:
    """♻️ One scope resolves each export id to exactly one file (contract §A). Where a facet document
    restates a member the module's owning document already declares, the restatement is deleted and
    every reference to it is re-pointed at the owner's `$id` — the row-111 decision, applied by
    document order: the root owns before `📸️snapshot`, which owns before `🔺️diff`/`💡️inferences`,
    and a `📝️text`/`💾️binary` codec child never owns what its parent facet declares."""
    order = ["", "📸️snapshot", "🔺️diff", "💡️inferences"]

    def rank(entry: dict) -> tuple:
        head = entry["facet"].split(os.sep)[0] if entry["facet"] else ""
        return (entry["facet"].count(os.sep) + (0 if entry["facet"] == "" else 1), order.index(head) if head in order else len(order), entry["facet"])

    removed = repointed = 0
    for module in modules():
        entries = sorted(documents(module), key=rank)
        owner: dict[str, dict] = {}
        for entry in entries:
            document = entry["document"]
            title = document.get("title")
            if isinstance(title, str) and root_is_object(document) and title not in owner:
                owner[title] = entry
            defs = document.get("$defs")
            if not isinstance(defs, dict):
                continue
            drop = [key for key in defs if key in owner and owner[key] is not entry]
            for key in drop:
                target = owner[key]["document"].get("$id")
                if not isinstance(target, str):
                    continue
                defs.pop(key)
                removed += 1
                mapping = {f"#/$defs/{key}": f"{target}#/$defs/{key}"}
                count, entry["document"] = rewrite_refs(entry["document"], mapping)
                repointed += count
                document = entry["document"]
                defs = document.get("$defs", {})
                entry["dirty"] = True
            if not defs:
                document.pop("$defs", None)
                entry["document"] = document
            for key in defs:
                owner.setdefault(key, entry)
        for entry in entries:
            if entry.get("dirty") and write:
                dump(entry["path"], entry["document"])
    print(f"restated-exports-removed={removed} references-repointed={repointed} (write={write})")


def command_titles(write: bool) -> None:
    """🏷 A codec facet document (`…/📝️text`, `…/💾️binary`) describes the WIRE FORM of the facet it
    sits under, so its export is that facet's export plus the representation — `<Owner>Text` /
    `<Owner>Binary`, the spelling row 81 settled for the mutation facets. A title that repeats the
    parent facet's export id makes two documents claim one export id; a title that is not PascalCase
    is not an export id at all."""
    fixed = 0
    for module in modules():
        entries = {entry["facet"]: entry for entry in documents(module)}
        for facet, entry in sorted(entries.items()):
            parts = facet.split(os.sep) if facet else []
            if not parts or parts[-1] not in ("📝️text", "💾️binary"):
                continue
            parent = entries.get(os.sep.join(parts[:-1]))
            if parent is None:
                continue
            owner_title = parent["document"].get("title")
            if not isinstance(owner_title, str) or not PASCAL.match(owner_title):
                continue
            expected = f"{owner_title}{'Text' if parts[-1] == '📝️text' else 'Binary'}"
            title = entry["document"].get("title")
            # 🏷 A title that is already a distinct PascalCase name of its own (`SemioKitSnapshotPackHeader`
            # describes the pack header, not the whole snapshot) is left alone: only a repeat of the
            # parent's export id, or a string that is no export id at all, is a defect.
            if title == owner_title or not isinstance(title, str) or not PASCAL.match(title):
                entry["document"]["title"] = expected
                fixed += 1
                if write:
                    dump(entry["path"], entry["document"])
    print(f"codec-facet-titles-fixed={fixed} (write={write})")


def command_formats(write: bool) -> None:
    """🏷 `x-semio-formats` states, per export, the formats that carry it — and is held to the
    statement in both directions (contract §B, ledger rows 56/98). It is written only where the truth
    is not "all of them": a codec facet document describes an encoding, not a transportable type, so
    its export exists in JSON Schema (and in whichever format file really declares it) and nowhere
    else. The set is measured against the export's OWN facet files, never asserted."""
    annotated = cleared = 0
    tally: Counter = Counter()
    for module in modules():
        provided = module_formats(module)
        for entry in documents(module):
            document = entry["document"]
            touched = False
            for exported in exports_of(document):
                subject = document if document.get("title") == exported and root_is_object(document) else document.get("$defs", {}).get(exported)
                if not isinstance(subject, dict):
                    continue
                carried = [format_id for format_id in provided if (source := sibling_source(entry, format_id)) is not None and declares(format_id, source, exported)]
                complete = len(carried) == len(provided)
                if complete:
                    if subject.pop("x-semio-formats", None) is not None:
                        cleared += 1
                        touched = True
                    continue
                declared = sorted([NORMATIVE] + carried)
                if subject.get("x-semio-formats") != declared:
                    subject["x-semio-formats"] = declared
                    annotated += 1
                    touched = True
                tally[tuple(declared)] += 1
            if touched and write:
                dump(entry["path"], document)
    print(f"exports-annotated={annotated} annotations-cleared={cleared} (write={write})")
    for key, value in tally.most_common():
        print(f"   {value:5d}  {list(key)}")


def main() -> int:
    command = sys.argv[1] if len(sys.argv) > 1 else "survey"
    write = "--write" in sys.argv
    table = {"survey": lambda: command_survey(), "titles": lambda: command_titles(write), "refs": lambda: command_refs(write), "defkeys": lambda: command_defkeys(write), "dedupe": lambda: command_dedupe(write), "formats": lambda: command_formats(write)}
    if command not in table:
        print(__doc__)
        return 2
    table[command]()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
