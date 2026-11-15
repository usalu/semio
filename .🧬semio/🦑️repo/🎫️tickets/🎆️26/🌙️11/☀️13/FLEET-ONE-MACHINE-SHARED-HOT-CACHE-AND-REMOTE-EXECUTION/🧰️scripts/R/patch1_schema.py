import json,sys
R="/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json"
d=json.load(open(R,encoding='utf8'))
D=d['definitions']
if 'minimumMegabytes' in D['Request']['properties']: sys.exit("already patched")
D['Request']['properties']['minimumMegabytes']={"type":"integer","minimum":0,"maximum":1048576}
for r in ("shrunk","never","starved"):
    D['Reason']['enum'].append(r)
M=D['Message']
M['required'].append("shrunk");M['properties']['shrunk']={"type":"string"}
for r in ("never","starved"):
    M['properties']['reasons']['required'].append(r);M['properties']['reasons']['properties'][r]={"type":"string"}
for e in ("memoryNeverFits","memoryStarved"):
    M['properties']['errors']['required'].append(e);M['properties']['errors']['properties'][e]={"type":"string"}
en=D['Messages']['const']['en'];de=D['Messages']['const']['de']
en['shrunk']="[capacity] admitted {operation} with {reserved} of {wanted} MB: only {spare} MB are free above the {floor} MB floor right now, and {minimum} MB are enough to start"
de['shrunk']="[capacity] {operation} mit {reserved} von {wanted} MB zugelassen: gerade sind nur {spare} MB über der Untergrenze von {floor} MB frei, und {minimum} MB genügen zum Start"
en['reasons']['never']="the request can never fit this machine"
de['reasons']['never']="die Anfrage passt nie auf diesen Rechner"
en['reasons']['starved']="not even the least memory it needs is free and nothing of ours is running"
de['reasons']['starved']="nicht einmal der kleinste benötigte Speicher ist frei, und nichts von uns läuft"
en['errors']['memoryNeverFits']="{operation} needs at least {minimum} MB of memory, but this machine has {total} MB of which {floor} MB stay reserved, so at most {capacity} MB can ever be given out. It cannot run here: use a machine with more memory (at least {needed} MB), or take the result from the fleet cache instead of computing it here."
de['errors']['memoryNeverFits']="{operation} braucht mindestens {minimum} MB Speicher, dieser Rechner hat aber {total} MB, von denen {floor} MB reserviert bleiben, also können höchstens {capacity} MB vergeben werden. Es kann hier nicht laufen: einen Rechner mit mehr Speicher verwenden (mindestens {needed} MB) oder das Ergebnis aus dem Flottencache holen, statt es hier zu berechnen."
en['errors']['memoryStarved']="{operation} needs at least {minimum} MB of free memory, but after {seconds} s only {spare} MB are free above the {floor} MB floor of this machine ({total} MB) and nothing of ours is running, so waiting longer cannot help. Close other applications (or stop the resident graph service with `repo service host stop`, which can hold more than 1 GB) until {minimum} MB are free, then run the command again."
de['errors']['memoryStarved']="{operation} braucht mindestens {minimum} MB freien Speicher, aber nach {seconds} s sind nur {spare} MB über der Untergrenze von {floor} MB dieses Rechners ({total} MB) frei, und nichts von uns läuft; längeres Warten hilft nicht. Andere Anwendungen schließen (oder den residenten Graphdienst mit `repo service host stop` beenden, er kann über 1 GB halten), bis {minimum} MB frei sind, und den Befehl dann erneut ausführen."
open(R,'w',encoding='utf8').write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
print("ok")
