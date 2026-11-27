import json,os
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
path=L+"📒️allowlist/🔣️.json"
d=json.loads(open(path,encoding="utf8").read())
R="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def E(file,site,cls,en,de):return {"path":R+file,"site":site,"class":cls,"reason":{"en":en,"de":de}}
new=[
 E("⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts","(","custody","The nx graph cache store is the store custody of the nx partition itself: it opens, funds and retires the child images of its namespaces inside the consumer frame of the partition owner, so these checks run inside the custody the shared readers wait for.","Der Nx-Graph-Cache-Store ist selbst die Store-Obhut der Nx-Partition: er öffnet, finanziert und entfernt die Kind-Images seiner Namensräume im Verbraucher-Rahmen des Partitionseigners, diese Prüfungen laufen also innerhalb der Obhut, auf die die geteilten Leser warten."),
 E("🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🟦️.ts","lstatSync(","metadata-probe","Reads only the type and device of the image directory, its backing file and its mount to measure the slack of attached images; it reads no content and the answer is the same while a writer holds the image.","Liest nur Typ und Gerät des Image-Verzeichnisses, seiner Backing-Datei und seines Mounts, um den Leerraum angehängter Images zu messen; es liest keinen Inhalt, und die Antwort ist dieselbe, während ein Schreiber das Image hält."),
]
d["entries"]=d["entries"]+new
tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(json.dumps(d,indent=1,ensure_ascii=False)+"\n");os.replace(tmp,path);print("entries2 ok",len(d["entries"]))
