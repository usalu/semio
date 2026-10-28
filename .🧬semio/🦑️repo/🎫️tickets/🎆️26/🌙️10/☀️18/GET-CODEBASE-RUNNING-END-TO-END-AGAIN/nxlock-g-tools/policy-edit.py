"""policy-edit.py <policy.json>: nxlock-g's rank declarations on top of the nxfix-g `core` policy (put-gate/put-slot are already `image`).
1. `nx:daemon-startup:` leaves `tool-singleton` (rank 2) for the new class `daemon-startup` (rank 8, no edge out): the start lease is taken last, once every image, tool and graph lease of the start is open.
2. `build-graph:` (pins of immutable graph snapshots, graph 4) and `nx:daemon-slot:` (resident daemon slots, tool-singleton 2) become two INERT classes: every request for them is a non-waiting try-lock (retention's probe, the slot take), so nobody ever waits behind them; the order rules skip them as held leases and as requests.
3. `cargo-custody` (outermost class, may wait for every inner class) gets the edge to `daemon-startup` like to every other non-inert class.
Idempotent: refuses to run twice."""
import json, sys

path = sys.argv[1]
policy = json.load(open(path, encoding="utf8"))
classes = {row["id"]: row for row in policy["classes"]}
if {"daemon-startup", "graph-pin", "daemon-slot"} & set(classes):
    sys.exit("already edited")


def drop(row, value):
    before = len(row["match"])
    row["match"] = [m for m in row["match"] if m["value"] != value]
    assert len(row["match"]) == before - 1, value


drop(classes["tool-singleton"], "nx:daemon-startup:")
drop(classes["tool-singleton"], "nx:daemon-slot:")
drop(classes["graph"], "build-graph:")


def row(id, rank, scope, title, prefix, inert=False):
    value = {"id": id, "rank": rank, "scope": scope, "owner": "graph"}
    if inert:
        value["inert"] = True
    value["title"] = title
    value["match"] = [{"kind": "prefix", "value": prefix}]
    return value


pin = row("graph-pin", 0, "path", {
    "en": "Pins of immutable graph snapshots: a shared read of a content-addressed file whose only exclusive request is the non-waiting probe of retention, so nothing ever waits behind a pin; inert, it orders against no other lease",
    "de": "Pins unveränderlicher Graph-Schnappschüsse: ein geteiltes Lesen einer inhaltsadressierten Datei, deren einzige exklusive Anfrage die nicht wartende Prüfung der Aufbewahrung ist, daher wartet nie jemand hinter einem Pin; träge, ordnet er sich gegen keine andere Lease ein",
}, "build-graph:", True)
slot = row("daemon-slot", 0, "checkout", {
    "en": "Resident slots of Nx daemons: held for a daemon's whole life and requested only by non-waiting try-locks, so nothing ever waits for a slot; inert, holding one orders against and blocks no other lease",
    "de": "Residente Plätze der Nx-Daemons: für das ganze Leben eines Daemons gehalten und nur durch nicht wartende Try-Locks angefragt, daher wartet nie jemand auf einen Platz; träge, ordnet das Halten eines Platzes sich gegen keine andere Lease ein und blockiert keine",
}, "nx:daemon-slot:", True)
startup = row("daemon-startup", 8, "checkout", {
    "en": "Exclusive start of one Nx daemon namespace: taken last, once every image, tool and graph lease of the start is open, and held only while the daemon starts; nothing may be awaited while it is held",
    "de": "Exklusiver Start eines Nx-Daemon-Namensraums: wird zuletzt genommen, sobald jede Image-, Werkzeug- und Graph-Lease des Starts offen ist, und nur gehalten, solange der Daemon startet; solange sie gehalten wird, darf nichts abgewartet werden",
}, "nx:daemon-startup:")
at = [value["id"] for value in policy["classes"]].index("graph") + 1
policy["classes"][at:at] = [pin, slot]
policy["classes"].append(startup)

custody = next(edge for edge in policy["allowed"] if edge["from"] == "cargo-custody" and edge["to"] == "image")
last = max(index for index, edge in enumerate(policy["allowed"]) if edge["from"] == "cargo-custody" and edge["to"] != "cargo-custody")
policy["allowed"].insert(last + 1, {"from": "cargo-custody", "to": "daemon-startup", "reason": custody["reason"]})

open(path, "w", encoding="utf8").write(json.dumps(policy, indent=2, ensure_ascii=False) + "\n")
print("edited", path)
