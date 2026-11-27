import json,os
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
path=L+"📒️allowlist/🔣️.json"
d=json.loads(open(path,encoding="utf8").read())
R="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def E(file,site,cls,en,de):return {"path":R+file,"site":site,"class":cls,"reason":{"en":en,"de":de}}
probe=("Reads only the metadata of a mount or backing file (existence, device) to report an image that is not attached; it reads no content and the answer is the same while a writer holds the image.","Liest nur Metadaten eines Mounts oder der Backing-Datei (Existenz, Gerät), um ein nicht angehängtes Image zu melden; es liest keinen Inhalt, und die Antwort ist dieselbe, während ein Schreiber das Image hält.")
new=[
 E("⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts","(","custody","The image's own reset and in-place regrow: it runs inside the exclusive non-waiting lease it takes itself or under the issued resource control of the store custody, so it is the writer the shared readers wait for.","Das Zurücksetzen und das Vergrößern des Images an Ort und Stelle: es läuft in der exklusiven, nicht wartenden Lease, die es selbst nimmt, oder unter der ausgestellten Ressourcenkontrolle der Store-Obhut, ist also der Schreiber, auf den die geteilten Leser warten."),
 E("⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🟦️.ts","(","custody","Writes the durable charge receipt of the store under its exclusive `repository-cache-charge` lease and the reservation of the opened image; it is part of the store custody itself.","Schreibt den dauerhaften Belastungsbeleg des Stores unter dessen exklusiver `repository-cache-charge`-Lease und der Reservierung des geöffneten Images; es ist Teil der Store-Obhut selbst."),
 E("⚡️caching/💾️storage/🧾️ledger/🟦️.ts","statSync(","metadata-probe",*probe),
 E("🏗️build/🏡️resident/🔐️consent/🔎️probe/🟨️.mjs","readdirSync(","metadata-probe","Lists the mount directories once so the operating system shows its consent dialog for the launcher; it reads no content and the answer does not depend on a writer.","Listet die Mount-Verzeichnisse einmal auf, damit das Betriebssystem seinen Zustimmungsdialog für den Starter zeigt; es liest keinen Inhalt, und die Antwort hängt nicht von einem Schreiber ab."),
 E("🏗️build/🧹️cache-gc/⌨️command/🟦️.ts","existsSync(join(graph","metadata-probe",*probe),
 E("🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts","statSync(","metadata-probe",*probe),
 E("🏗️build/🌐️remote-execution/👷️worker/🟦️.ts","(","unrouted","Finding F124: the collection of the worker home (blob cache, retirement of dead workers' private roots) reads and removes below the dependencies image without its shared lease; route the two callers in the cache command through the shared-read helper.","Befund F124: die Sammlung des Worker-Verzeichnisses (Blob-Cache, Entfernen der privaten Wurzeln toter Worker) liest und entfernt unterhalb des Dependencies-Images ohne dessen geteilte Lease; die zwei Aufrufer im Cache-Befehl über den Shared-Read-Helfer führen."),
 E("🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts","(","unrouted","Finding F124: the collection runner measures and removes units below mounted images under the exclusive area lease only, not the shared lease of the image; give the runner ports an image reader per partition and run every walk and removal inside it.","Befund F124: der Sammel-Läufer misst und entfernt Einheiten unterhalb gemounteter Images nur unter der exklusiven Bereichs-Lease, nicht unter der geteilten Lease des Images; den Läufer-Ports einen Image-Leser je Partition geben und jede Wanderung und Entfernung darin ausführen."),
]
d["entries"]=d["entries"]+new
d["classes"]=d["classes"]+["unrouted"]
txt=json.dumps(d,indent=1,ensure_ascii=False)+"\n"
tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(txt);os.replace(tmp,path);print("entries ok",len(d["entries"]))
