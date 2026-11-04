import glob, json, os, sys
root = sys.argv[1]
out = sys.argv[2]
roster = json.load(open(os.path.join(root, "🌎️hub/🧩️compositions/🗄️stdio/📇️catalog/🧪️tests/🧫️fixtures/📜️definition-roster/🔣️.json")))["identities"]
pending = ["idf", "idml"]
rows = []
for path in sorted(glob.glob(os.path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/📜️artifact-definition.json"))):
    definition = json.load(open(path))
    artifact = definition["artifact"]
    seen = {}
    for representation in definition["representations"]:
        key = (tuple(representation["mimes"]), tuple(representation["extensions"]))
        seen.setdefault(key, representation)
    for (mimes, extensions), representation in seen.items():
        rows.append({"artifact": artifact, "pending": artifact in pending, "mimes": list(mimes), "extensions": list(extensions)})
cases = []
for row in rows:
    for index, extension in enumerate(row["extensions"]):
        for spelling in (extension, extension.upper()):
            cases.append({"id": f"{row['artifact']}-{spelling}", "query": {"fileName": f"sample{spelling}"}, "artifact": row["artifact"], "pending": row["pending"], "basis": "extension"})
    for mime in row["mimes"]:
        cases.append({"id": f"{row['artifact']}-mime-{mime}", "query": {"mime": mime}, "artifact": row["artifact"], "pending": row["pending"], "basis": "mime"})
well_known = [
    {"id": "json", "query": {"fileName": "plan.json"}, "artifact": "json"},
    {"id": "xml-not-svg", "query": {"fileName": "drawing.xml", "mime": "image/svg+xml"}, "artifact": "xml"},
    {"id": "svg-not-xml", "query": {"fileName": "logo.svg", "mime": "application/xml"}, "artifact": "svg"},
    {"id": "htm", "query": {"fileName": "index.htm"}, "artifact": "html"},
    {"id": "docx-not-zip", "query": {"fileName": "letter.docx", "mime": "application/zip"}, "artifact": "docx"},
    {"id": "xlsx-not-zip", "query": {"fileName": "sheet.xlsx", "mime": "application/zip"}, "artifact": "xlsx"},
    {"id": "pptx-not-zip", "query": {"fileName": "deck.pptx", "mime": "application/zip"}, "artifact": "pptx"},
    {"id": "zip", "query": {"fileName": "bundle.zip", "mime": "application/zip"}, "artifact": "zip"},
    {"id": "jpeg", "query": {"fileName": "photo.JPEG", "mime": "image/jpeg"}, "artifact": "jpg"},
    {"id": "tif", "query": {"fileName": "scan.tif"}, "artifact": "tiff"},
    {"id": "markdown", "query": {"fileName": "notes.markdown", "mime": "text/plain"}, "artifact": "md"},
    {"id": "stp", "query": {"fileName": "part.stp"}, "artifact": "step"},
    {"id": "glb-octet-stream", "query": {"fileName": "scene.glb", "mime": "application/octet-stream"}, "artifact": "gltf"},
    {"id": "bin", "query": {"fileName": "dump.bin"}, "artifact": "binary"},
    {"id": "explicit-binary-over-name", "query": {"explicit": "bin", "fileName": "plan.json"}, "artifact": "binary"},
]
json.dump({"schema": "semio.stdio.format-resolution-law/v1", "rosterArtifacts": roster, "pendingArtifacts": pending, "wellKnown": well_known, "ownerCoordinates": {"pdf": {"coordinate": "s.stdio.pdf@1.7/*", "kindIdContains": "standard.1-7."}, "gif": {"coordinate": "s.stdio.gif@89a/*", "kindIdContains": "standard.89a."}, "dwg": {"coordinate": "s.stdio.dwg@ac1024/*", "kindIdContains": "standard.ac1024."}, "ifc": {"coordinate": "s.stdio.ifc@4/*", "kindIdContains": "standard.4."}}, "canonicalExportExtension": {"jpg": ".jpg", "tiff": ".tiff", "html": ".html", "md": ".md", "step": ".step"}, "formats": rows, "cases": cases}, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=2)
open(out, "a").write("\n")
