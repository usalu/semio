import json,os
M="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/"
SKIP=[("spool-full","upload spool full","Upload-Spool voll"),("upload-expired","upload expired","Upload abgelaufen"),("upload-expired-down","upload expired, cache down","Upload abgelaufen, Cache nicht erreichbar"),("upload-expired-denied","upload expired, credential denied","Upload abgelaufen, Berechtigung abgelehnt"),("upload-entry-gone","upload entry gone","Upload-Eintrag verschwunden"),("upload-failed","upload failed","Upload fehlgeschlagen"),("upload-dropped","upload dropped","Upload verworfen"),("key-incomplete","key incomplete","Schlüssel unvollständig"),("path-dependent","path dependent output","pfadabhängige Ausgabe"),("read-only","cache read-only","Cache nur lesbar"),("diverged","output diverged","Ausgabe abweichend"),("breaker-denied","circuit breaker denied","Unterbrecher: verweigert")]
DECLINE=[("local-slot","local slot free","lokaler Platz frei"),("cheap","too cheap to offload","zu billig zum Auslagern"),("unmapped-path","path not mapped","Pfad nicht abgebildet"),("infrastructure","infrastructure fault","Infrastrukturfehler"),("unmeasured","not measured","nicht gemessen"),("pin-failed","pin failed","Anheften fehlgeschlagen"),("not-portable","not portable","nicht portabel"),("worker-policy","worker policy","Worker-Richtlinie"),("no-idle-machine","no idle machine","keine freie Maschine"),("queue-timeout","queue timeout","Warteschlangen-Frist"),("incomplete-inputs","incomplete inputs","unvollständige Eingaben")]
def entries():
    for k,en,de in SKIP: yield ("cache.skip."+k,"cache-skip-reasons","Units not shared: "+en,"Nicht geteilte Einheiten: "+de)
    yield ("cache.skip.other","cache-skip-reasons","Units not shared: any other reason","Nicht geteilte Einheiten: jeder andere Grund")
    for k,en,de in DECLINE: yield ("exec.decline."+k,"offload-reasons","Offload declined: "+en,"Auslagerung abgelehnt: "+de)
    yield ("exec.decline.other","offload-reasons","Offload declined: any other reason","Auslagerung abgelehnt: jeder andere Grund")
    yield ("cache.remote.refused","remote-cache","Remote cache requests refused","Abgelehnte Remote-Cache-Anfragen")
rows=list(entries())
p=M+"📇️catalogue/🔣️.json";s=open(p,encoding="utf8").read()
assert "cache.skip.other" not in s
tail="\n    }\n  ]\n}"
assert s.rstrip("\n").endswith(tail.strip("\n")) or True
end=s.rindex("    }\n  ]\n}")
add="".join(',\n    {\n      "id": %s,\n      "kind": "counter",\n      "unit": "count",\n      "scope": "origin",\n      "source": "%s",\n      "label": {\n        "en": %s,\n        "de": %s\n      }\n    }'%(json.dumps(i),src,json.dumps(en,ensure_ascii=False),json.dumps(de,ensure_ascii=False)) for i,src,en,de in rows)
s=s[:end]+"    }"+add+s[end+len("    }"):]
t=p+".h5.tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,p)
json.loads(s)
p=M+"🛰️collector/🧬️schema/🔣️.json";raw=open(p,encoding="utf8").read();d=json.loads(raw)
cols=d["definitions"]["Collectors"]["const"]
for c in cols:
    if c["name"]=="remote-cache":
        c["ids"].append("cache.remote.refused");c["label"]={"en":"What the remote cache answered the clients of this machine (hits, misses, puts, conflicts, refusals by kind)","de":"Was der Remote-Cache den Clients dieser Maschine geantwortet hat (Treffer, Fehlzugriffe, Ablagen, Konflikte, Ablehnungen je Art)"}
at=[i for i,c in enumerate(cols) if c["name"]=="offload-counters"][0]
cols.insert(at+1,{"name":"cache-skip-reasons","cadence":"slow","source":"cache-skip-reasons","ids":[i for i,src,_,_ in rows if src=="cache-skip-reasons"],"label":{"en":"The unit cache skip reason counter files (units compiled and not shared, by reason)","de":"Die Zählerdateien der Überspringgründe des Einheiten-Caches (kompiliert und nicht geteilt, nach Grund)"},"scope":"origin"})
cols.insert(at+2,{"name":"offload-reasons","cadence":"slow","source":"offload-reasons","ids":[i for i,src,_,_ in rows if src=="offload-reasons"],"label":{"en":"The offload decline and fall-back reason counter files","de":"Die Zählerdateien der Ablehnungs- und Rückfallgründe der Auslagerung"},"scope":"origin"})
out=json.dumps(d,indent=2,ensure_ascii=False)+"\n"
t=p+".h5.tmp";open(t,"w",encoding="utf8").write(out);os.replace(t,p)
print(len(rows),"metrics added")
