"""🧪️ W2-W-norm-3: regenerates the norm mutation-leaf taxonomy fixture
(`✏️s/🔌️plugins/📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`) while the norm package's `📜️script.ts`
(`mutation-leaf-taxonomy-generate`) is mid-move to `🌎️hub/🧩️compositions/📕️norm` and cannot reach the artifacts — the
same reading as its `taxonomy()` (leaf `🦀️.rs` with a `MutationKind` impl, `SemanticDescriptor`, matching descriptor,
payload schema and `MutationLeaf` contract), rows ordered by the UTF-8 bytes of their source path.

`python3 🧪️w2-w-norm-3-leaf-taxonomy.py generate|check` from the repository root.
"""
import json
import os
import re
import sys

NORM = "✏️s/🔌️plugins/📕️norm"
ARTIFACTS = f"{NORM}/🗿️artifacts"
FIXTURE = f"{NORM}/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
SEMANTICS = re.compile(r'const SEMANTICS:\s*protocol::SemanticDescriptor\s*=\s*protocol::SemanticDescriptor\s*\{\s*verb:\s*"([^"]+)",\s*entity:\s*"([^"]+)",\s*kind:\s*"([^"]+)",\s*record:\s*"([^"]+)",?\s*\}', re.S)


def files_below(root):
    for entry in sorted(os.scandir(root), key=lambda entry: entry.name):
        if entry.is_dir(follow_symlinks=False):
            yield from files_below(entry.path)
        else:
            yield entry.path


def taxonomy():
    rows = []
    for source in files_below(ARTIFACTS):
        if not source.endswith("/🦀️.rs") or "/🧬️schema/🧬️mutations/" not in source:
            continue
        rust = open(source, encoding="utf-8").read()
        if "impl protocol::MutationKind<" not in rust:
            continue
        declared, semantics = re.search(r"pub struct\s+([A-Za-z0-9_]+)", rust), SEMANTICS.search(rust)
        if not declared or not semantics:
            raise SystemExit(f"mutation leaf metadata is unreadable: {source}")
        segments = source.split("/")
        artifact = segments.index("🗿️artifacts")
        standard = segments.index("🏅️standards", artifact + 1)
        subset = segments.index("🪆️subsets", standard + 1)
        mutations = segments.index("🧬️mutations", subset + 1)
        layout = "split" if source.endswith("/🦠️mutation/🦀️.rs") else "direct"
        owner = os.path.dirname(os.path.dirname(source)) if layout == "split" else os.path.dirname(source)
        if not os.path.exists(f"{owner}/🔣️.json"):
            continue
        descriptor = json.load(open(f"{owner}/🔣️.json", encoding="utf-8"))
        if descriptor.get("owner") != owner or descriptor.get("semanticKind") != semantics.group(3) or descriptor.get("aggregateVariant") != declared.group(1):
            continue
        if not isinstance(descriptor.get("payloadSchema"), str) or not os.path.exists(f"{owner}/{descriptor['payloadSchema']}"):
            continue
        if "dsl::MutationLeaf" not in rust or "#[mutation_leaf(contract = ::protocol)]" not in rust:
            continue
        rows.append({
            "aggregateVariant": declared.group(1),
            "artifact": segments[artifact + 1],
            "entity": semantics.group(2),
            "kind": semantics.group(3),
            "module": segments[mutations + 1],
            "physicalLayout": layout,
            "record": semantics.group(4),
            "source": os.path.relpath(source, ARTIFACTS),
            "standard": segments[standard + 1],
            "subset": segments[subset + 1],
            "type": declared.group(1),
            "verb": semantics.group(1),
        })
    rows.sort(key=lambda row: row["source"].encode("utf-8"))
    return {"contractId": "semio.norm.mutation-leaf-taxonomy/v1", "rows": rows, "schemaVersion": 1}


def text(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


if __name__ == "__main__":
    fresh = text(taxonomy())
    if sys.argv[1] == "generate":
        open(FIXTURE, "w", encoding="utf-8").write(fresh)
        print(f"norm mutation-leaf taxonomy generated: {len(json.loads(fresh)['rows'])} payloads")
    else:
        print("fresh" if open(FIXTURE, encoding="utf-8").read() == fresh else "stale")
