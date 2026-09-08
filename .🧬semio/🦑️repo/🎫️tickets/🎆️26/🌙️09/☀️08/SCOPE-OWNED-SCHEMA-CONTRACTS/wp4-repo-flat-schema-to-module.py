import json, os, collections

DRAFT = "http://json-schema.org/draft-07/schema#"

def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle, object_pairs_hook=collections.OrderedDict)

def write(path, document):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")

def module(schema_id, title, description, defs, root):
    return collections.OrderedDict([
        ("$schema", DRAFT),
        ("$id", schema_id),
        ("title", title),
        ("description", description),
        ("$ref", root),
        ("$defs", collections.OrderedDict(defs)),
    ])

def strip(document):
    document = collections.OrderedDict(document)
    document.pop("$schema", None)
    return document

jobs = [
    # (source, target, id, title, description, root export, item export, item pointer)
    ("🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🧬️schema.json",
     "🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🧬️schema/🔣️.json",
     "https://semio.tech/schema/print/print-font-catalog/schema.json",
     "PrintFontCatalog", "Fonts published into every print document build.",
     "PrintFontCatalog", "PrintFontDescriptor", "items"),
    ("🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧬️schema.json",
     "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧬️schema/🔣️.json",
     "https://semio.tech/schema/print/tectonic-template-compilation/catalog/schema.json",
     "PrintDocumentCatalog", "Every print document owned by the tectonic template compilation module.",
     "PrintDocumentCatalog", "PrintDocumentEntry", "properties.documents.items"),
    ("🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧬️schema.json",
     "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧬️schema/🔣️.json",
     "https://semio.tech/schema/print/tectonic-template-compilation/bundle/schema.json",
     "PrintBundleManifest", "Locked TeX bundle ranges fetched for reproducible tectonic runs.",
     "PrintBundleManifest", "PrintBundleFile", "properties.files.items"),
    ("🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema.json",
     "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema/🔣️.json",
     "https://semio.tech/schema/print/tectonic-template-compilation/toolchain/schema.json",
     "PrintToolchainManifest", "Pinned tectonic release downloaded per platform.",
     "PrintToolchainManifest", "PrintToolchainPlatform", "properties.platforms.items"),
    ("♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema.json",
     "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema/🔣️.json",
     "https://semio.tech/schema/mit-bestand/bericht/documents/schema.json",
     "ReportDocumentCatalog", "Every report document owned by the mit-bestand report product.",
     "ReportDocumentCatalog", "ReportDocumentEntry", "properties.documents.items"),
    ("♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema.json",
     "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema/🔣️.json",
     "https://semio.tech/schema/mit-bestand/demonstrator/runtime/schema.json",
     "DemonstratorRuntimeCatalog", "Panes and runtime variants the mit-bestand demonstrator boots.",
     "DemonstratorRuntimeCatalog", "DemonstratorRuntimePane", "properties.panes.items"),
]

for source, target, schema_id, title, description, root_export, item_export, pointer in jobs:
    document = strip(load(source))
    node, parent, key = document, None, None
    for step in pointer.split("."):
        parent, key, node = node, step, node[step]
    item = node
    parent[key] = collections.OrderedDict([("$ref", f"#/$defs/{item_export}")])
    defs = [(root_export, document), (item_export, item)]
    write(target, module(schema_id, title, description, defs, f"#/$defs/{root_export}"))
    os.remove(source)
    print("converted", target)
