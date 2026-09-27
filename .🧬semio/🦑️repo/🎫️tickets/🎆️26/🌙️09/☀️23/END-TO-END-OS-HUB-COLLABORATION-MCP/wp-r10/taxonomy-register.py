#!/usr/bin/env python3
"""🗂️ R10 item 1: builds the candidate taxonomy that registers every unresolved directory the kinds probe found.

Each unresolved directory's name joins the member kind its parent's resolved kind owns (`members-of-tests`,
`members-of-fixtures`, `members-of-schema`, `members-of-<kind>` …); a parent kind that owns no `members-of-<kind>` gets one
(`ownerKindIds: [<kind>]`, `source: registry`), inserted right after the nearest existing `members-of-*` entry. Names that
are also handed over by slices as not-yet-created directories come from `window3-spec.json` `newDirectories`.
Rows whose parent is itself unresolved are left for the next pass (re-run the kinds probe against the candidate); names
without a leading emoji (runtime data leaked into the tree) and names whose emoji lacks its variation selector (a rename,
not a registration) are reported, never registered.
Usage: python3 taxonomy-register.py <kinds.json>... [--base <taxonomy.json>] --out <candidate.json> [--report <out.json>]
       [--exclude <member name>]... (names the taxonomy validator refuses: a rename, reported with the no-VS16 ones)
The live taxonomy is never written by this script.
"""
import json
import sys
import unicodedata

ROOT = "/Users/ueli/Documents/semio"
LIVE = f"{ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
SPEC = f"{ROOT}/.tmp-ticket/wp-r10/window3-spec.json"
TEST_OWNER = "tests"


def member_kind_for(parent_kind, member_kinds):
    preferred = f"members-of-{parent_kind}"
    if preferred in member_kinds:
        return preferred, False
    owned = [kid for kid, spec in member_kinds.items() if spec["ownerKindIds"] == [parent_kind]]
    if len(owned) == 1 and owned[0].startswith("members-of-"):
        return owned[0], False
    return preferred, True


def main():
    args = sys.argv[1:]
    def option(name, default=None):
        if name in args:
            index = args.index(name)
            value = args[index + 1]
            del args[index:index + 2]
            return value
        return default
    excluded = set()
    while "--exclude" in args:
        excluded.add(unicodedata.normalize("NFC", option("--exclude")))
    base = option("--base", LIVE)
    out = option("--out")
    report_path = option("--report")
    taxonomy = json.load(open(base, encoding="utf-8"))
    member_kinds = taxonomy["semanticDirectoryMemberKinds"]
    rows = []
    for path in args:
        rows.extend(json.load(open(path, encoding="utf-8")))
    spec = json.load(open(SPEC, encoding="utf-8"))
    for entry in spec.get("newDirectories", []):
        rows.append({"path": entry["path"], "name": entry["path"].split("/")[-1], "parentKind": entry.get("parentKind") or (TEST_OWNER if entry["path"].split("/")[-2] == "🧪️tests" else None), "code": "planned"})
    added, created, deferred, missing_vs16, unregistrable = {}, [], [], [], []
    for row in rows:
        name = unicodedata.normalize("NFC", row["name"])
        if row["parentKind"] is None:
            deferred.append(row["path"])
            continue
        if name in excluded:
            missing_vs16.append(row["path"])
            continue
        if not name or not (ord(name[0]) >= 0x1F000 or unicodedata.category(name[0]) == "So"):
            unregistrable.append(row["path"])
            continue
        if "\ufe0f" not in name[:4] and not name[0].isdigit():
            missing_vs16.append(row["path"])
            continue
        kind, create = member_kind_for(row["parentKind"], member_kinds)
        if create:
            items = list(member_kinds.items())
            anchor = max((index for index, (kid, _) in enumerate(items) if kid.startswith("members-of-")), default=len(items) - 1)
            items.insert(anchor + 1, (kind, {"ownerKindIds": [row["parentKind"]], "memberNames": [], "source": "registry"}))
            member_kinds.clear()
            member_kinds.update(items)
            created.append(kind)
        if name not in member_kinds[kind]["memberNames"]:
            member_kinds[kind]["memberNames"].append(name)
            added.setdefault(kind, []).append(name)
    text = json.dumps(taxonomy, ensure_ascii=False, indent=2) + "\n"
    open(out, "w", encoding="utf-8").write(text)
    report = {"added": added, "created": created, "deferred": deferred, "namesWithoutVariationSelector": missing_vs16, "unregistrable": unregistrable, "addedCount": sum(len(v) for v in added.values())}
    if report_path:
        json.dump(report, open(report_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps({k: (len(v) if isinstance(v, (list, dict)) else v) for k, v in report.items()}))
    for kind, names in added.items():
        print(f"{kind}: {len(names)}")


main()
