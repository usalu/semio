#!/usr/bin/env python3
"""H73 schema edits, applied to a root (live tree or overlay): `schema_edits.py ROOT` where ROOT is the `unit-cache` directory."""
import sys
root=sys.argv[1];check=len(sys.argv)>2 and sys.argv[2]=="check"
def sub(path,old,new):
    text=open(path,encoding="utf8").read()
    if text.count(old)!=1:raise SystemExit(f"anchor count {text.count(old)} in {path}: {old[:60]}")
    if not check:open(path,"w",encoding="utf8").write(text.replace(old,new))
SEED='''        "uploader": {'''
policy=root+"/🛰️fleet/🧬️schema/🔣️.json"
sub(policy,'''        "linkedCrateTypes":''','''        "seed": { "namespace": "semio.cargo.unit-cache.seed-closure/v1", "directory": "seed", "slotsFile": "slots", "usedFile": "used.log", "acDirectory": "ac", "casDirectory": "cas", "concurrency": 8, "maximumBytes": 4294967296, "maximumBlobBytes": 134217728, "maximumSlots": 32768, "maximumRecordBytes": 4194304, "deadlineMilliseconds": 180000, "progressMilliseconds": 2000, "maximumKeyAgeMilliseconds": 2592000000, "eventTailBytes": 16777216 },
        "linkedCrateTypes":''')
msgs=root+"/🧬️schema/🔣️.json"
sub(msgs,'''        "retentionConflict": {''','''        "seedProgress": {
          "en": "[repo cargo] seeding the lane from the fleet cache: {done}/{total} units, {mebibytes} MiB, {seconds} s",
          "de": "[repo cargo] Lane aus dem Flotten-Cache vorbefüllt: {done}/{total} Units, {mebibytes} MiB, {seconds} s"
        },
        "seedDone": {
          "en": "[repo cargo] lane seeded before compiling: {fetched} of {total} units prefetched ({blobs} blobs, {mebibytes} MiB) in {seconds} s, {absent} not in the fleet cache{stopped}",
          "de": "[repo cargo] Lane vor dem Kompilieren vorbefüllt: {fetched} von {total} Units vorgeholt ({blobs} Blobs, {mebibytes} MiB) in {seconds} s, {absent} nicht im Flotten-Cache{stopped}"
        },
        "seedNone": {
          "en": "[repo cargo] no fleet closure record for this request yet ({reason}); units are looked up one by one and the record is published after this run",
          "de": "[repo cargo] Für diese Anfrage gibt es noch keinen Flotten-Hüllenbericht ({reason}); Units werden einzeln nachgeschlagen und der Bericht nach diesem Lauf veröffentlicht"
        },
        "seedPublished": {
          "en": "[repo cargo] fleet closure record published: {slots} unit slots ({added} new)",
          "de": "[repo cargo] Flotten-Hüllenbericht veröffentlicht: {slots} Unit-Slots ({added} neu)"
        },
        "runReport": {
          "en": "[repo cargo] units: {total} in total, {local} local hits, {fleet} fetched from the fleet ({prefetched} served from the lane seed), {compiled} compiled here, {offloaded} offloaded to other machines, {scripts} build scripts served",
          "de": "[repo cargo] Units: {total} insgesamt, {local} lokale Treffer, {fleet} aus dem Flotten-Cache geholt ({prefetched} aus der Lane-Vorbefüllung), {compiled} hier kompiliert, {offloaded} auf andere Maschinen ausgelagert, {scripts} Build-Skripte bedient"
        },
        "retentionConflict": {''')
