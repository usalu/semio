#!/usr/bin/env python3
"""🗂️ WG11 session 15 — authors `🏛️ShellHost/🗂️local-catalog/🔣️.json` (the schema-first local-catalog vocabulary both shells read):
static parts + shared vectors with hand-authored expectations; the archive bytes (`archiveHex`) are filled by `lc-archive-hex.ts`
(the TS encoders — the byte authority both shells' codecs are pinned to). Writes `vocabulary.json` next to this script."""
import json
from pathlib import Path

NOTICES = {
    "keeping": ("Keeping “{name}” on this device…", "„{name}“ wird auf diesem Gerät gespeichert…"),
    "kept": ("“{name}” is kept on this device.", "„{name}“ ist auf diesem Gerät gespeichert."),
    "retired": ("“{name}” is no longer listed on this device; its files stay where they are.", "„{name}“ wird auf diesem Gerät nicht mehr aufgeführt; seine Dateien bleiben erhalten."),
    "no-data-folder": ("This shell has no data folder, so it cannot keep documents on this device.", "Diese Shell hat keinen Datenordner und kann daher keine Dokumente auf diesem Gerät speichern."),
    "invalid-request": ("That request to keep a document was incomplete and was not run.", "Diese Anfrage zum Speichern eines Dokuments war unvollständig und wurde nicht ausgeführt."),
    "document-unknown": ("That document is not kept on this device.", "Dieses Dokument ist auf diesem Gerät nicht gespeichert."),
    "write-failed": ("“{name}” could not be written to this device.", "„{name}“ konnte nicht auf diesem Gerät gespeichert werden."),
    "cancelled": ("Keeping “{name}” on this device was cancelled; nothing was written.", "Das Speichern von „{name}“ auf diesem Gerät wurde abgebrochen; nichts wurde geschrieben."),
}
PACK, SPR, NOW = "AQID", "BAU=", 1727600000123


def studio(**over):
    args = {"documentId": "studio-1", "schema": "s.space", "name": "Studio", "storage": "folder", "pack": PACK, "spr": SPR}
    args.update(over)
    return {key: value for key, value in args.items() if value is not None}


def kept(name, args, data_dir, target, folder, storage="folder", display=None, schema="s.space", document_id="studio-1"):
    document = {"documentId": document_id, "schema": schema, "name": display if display is not None else args.get("name", ""), "storage": storage, "target": target, "admittedAtMs": NOW}
    return {"name": name, "args": args, "dataDir": data_dir, "nowMs": NOW, "expect": {"document": document, "folder": folder, "archiveHex": ""}}


def refused(name, args, data_dir, refusal):
    return {"name": name, "args": args, "dataDir": data_dir, "nowMs": NOW, "expect": {"refusal": refusal}}


ADMISSIONS = [
    kept("an empty folder target lands in the data folder", studio(), "/data/", "/data/os/local-documents/studio-1", "/data/os/local-documents/studio-1"),
    kept("a file target keeps its events in the file's folder", studio(storage="file", target="/home/ada/plans/studio.semio", name=""), None, "/home/ada/plans/studio.semio", "/home/ada/plans", storage="file", display=""),
    kept("a file at the root keeps its events in the root", studio(storage="file", target="/studio.semio"), None, "/studio.semio", "/", storage="file"),
    kept("an explicit folder target needs no data folder", studio(target="/srv/studios/a"), None, "/srv/studios/a", "/srv/studios/a"),
    refused("an empty folder target without a data folder", studio(), None, "no-data-folder"),
    refused("a data folder that is only the root is none", studio(), "/", "no-data-folder"),
    refused("a relative file target", studio(storage="file", target="plans/studio.semio"), "/data", "invalid-request"),
    refused("a file without a target", studio(storage="file"), "/data", "invalid-request"),
    refused("a relative folder target", studio(target="studios/a"), "/data", "invalid-request"),
    refused("an empty pack", studio(pack=""), "/data", "invalid-request"),
    refused("a pack that is not base64", studio(pack="AQI"), "/data", "invalid-request"),
    refused("an id with a slash", studio(documentId="a/b"), "/data", "invalid-request"),
    refused("an id that is the parent folder", studio(documentId=".."), "/data", "invalid-request"),
    refused("an id that is the folder itself", studio(documentId="."), "/data", "invalid-request"),
    refused("a name with a control character", studio(name="Stu\u0007dio"), "/data", "invalid-request"),
    refused("a schema with surrounding space", studio(schema=" s.space"), "/data", "invalid-request"),
    refused("an unknown storage", studio(storage="cloud"), "/data", "invalid-request"),
    refused("an id longer than its bound", studio(documentId="a" * 257), "/data", "invalid-request"),
    refused("no arguments at all", None, "/data", "invalid-request"),
]

CATALOGS = [
    {"catalog": {"documents": []}, "archiveHex": ""},
    {"catalog": {"documents": [
        {"documentId": "plan-a", "schema": "s.space", "name": "Plan A", "storage": "file", "target": "/home/ada/plan-a.semio", "admittedAtMs": 1727600000000},
        {"documentId": "studio-1", "schema": "s.space", "name": "Studio", "storage": "folder", "target": "/data/os/local-documents/studio-1", "admittedAtMs": NOW},
    ]}, "archiveHex": ""},
]

VOCABULARY = {
    "schema": "semio.os.shell-local-catalog/v1",
    "description": "The host-owned local document catalog lane (`os.config.local-catalog`, persisted local-only in `<S_DATA_DIR>/os`) both shells serve: the guest commands, the landing app's re-hydration action, the bounds, what the lane tells the user (en, de; any other locale reads en; `{name}` is the document's name) under which fault code, and the shared vectors both shells' laws answer — an admission request resolves to its catalog entry, event folder and exact archive bytes or to a refusal; a catalog persists as exactly its archive bytes.",
    "actions": {"admit": "os.local-catalog.admit", "retire": "os.local-catalog.retire", "rehydrate": "applyLocalCatalogDocument"},
    "codePrefix": "shell.localCatalog.",
    "bounds": {"textMaximumBytes": 256, "targetMaximumBytes": 4096, "documentMaximumBase64Bytes": 8 * 1024 * 1024, "writeDeadlineMs": 30000, "readDeadlineMs": 10000},
    "notices": {key: {"en": en, "de": de} for key, (en, de) in NOTICES.items()},
    "admissions": ADMISSIONS,
    "catalogArchives": CATALOGS,
}
Path(__file__).with_name("vocabulary.json").write_text(json.dumps(VOCABULARY, ensure_ascii=False, indent=2) + "\n")
print(f"vocabulary: {len(ADMISSIONS)} admissions, {len(CATALOGS)} catalog archives")
