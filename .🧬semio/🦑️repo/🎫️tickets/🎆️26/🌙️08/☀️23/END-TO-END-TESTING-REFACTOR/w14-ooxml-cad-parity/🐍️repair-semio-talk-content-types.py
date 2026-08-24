#!/usr/bin/env python3
"""🩹 Restore the eight `<Default>` content-type entries wave 7's derivation silently dropped.

`w7-pptx-ecma-376-mutate/derive-semio-talk-subset.py` says "keep every Default" and then reads
them with `re.findall(r"<Default [^/]+/>", ct_xml)` — a character class that cannot span the `/`
in `application/vnd.openxmlformats-...`, so it matched NOTHING and the derived fixture shipped a
`[Content_Types].xml` with Overrides only.  Every `.rels`, `.png` and `.jpeg` part in the package
was therefore left with no resolvable content type at all, which ECMA-376 Part 2 §10.1.2.2.1
forbids outright, and `decode_pptx` rightly refused the file:

    decode_pptx failed: pptx: opc: part docProps/thumbnail.jpeg has no resolvable content type

This rewrites ONLY the `[Content_Types].xml` entry, splicing the eight `<Default>` elements back
in from the real source deck ahead of the Overrides the derivation already kept.  Every other
part keeps its exact bytes, its exact order and its exact zip timestamp, so the repair is
auditable as "the eight lines the regex lost" and nothing else.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14.
"""
import re
import shutil
import zipfile

REPO = "/Users/ueli/Documents/semio"
SRC = f"{REPO}/temp/domai-specific-programmaning-language-for-architects.pptx"
FIXTURE = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx"
CT = "[Content_Types].xml"

with zipfile.ZipFile(SRC) as source:
    defaults = re.findall(r"<Default\b[^>]*/>", source.read(CT).decode("utf-8"))
assert len(defaults) == 8, defaults

with zipfile.ZipFile(FIXTURE) as current:
    infos = current.infolist()
    payload = {info.filename: current.read(info.filename) for info in infos}

old_ct = payload[CT].decode("utf-8")
assert "<Default" not in old_ct, "the fixture already carries Default entries — nothing to repair"
new_ct = old_ct.replace("<Override", "".join(defaults) + "<Override", 1)
assert new_ct != old_ct
payload[CT] = new_ct.encode("utf-8")

shutil.copy2(FIXTURE, f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w14-ooxml-cad-parity/📦️semio-talk.pptx.before-repair")
with zipfile.ZipFile(FIXTURE, "w", zipfile.ZIP_DEFLATED) as out:
    for info in infos:
        kept = zipfile.ZipInfo(info.filename, date_time=info.date_time)
        kept.compress_type = info.compress_type
        kept.external_attr = info.external_attr
        kept.create_system = info.create_system
        out.writestr(kept, payload[info.filename])

with zipfile.ZipFile(FIXTURE) as check:
    ct = check.read(CT).decode("utf-8")
    extensions = set(re.findall(r'<Default Extension="([^"]+)"', ct))
    overrides = {name.lstrip("/") for name in re.findall(r'<Override PartName="([^"]+)"', ct)}
    for name in check.namelist():
        if name == CT:
            continue
        assert name in overrides or name.rsplit(".", 1)[-1].lower() in extensions, f"{name} still has no content type"
print(f"repaired {FIXTURE}: {len(defaults)} Default entries restored, {len(infos)} parts preserved")
