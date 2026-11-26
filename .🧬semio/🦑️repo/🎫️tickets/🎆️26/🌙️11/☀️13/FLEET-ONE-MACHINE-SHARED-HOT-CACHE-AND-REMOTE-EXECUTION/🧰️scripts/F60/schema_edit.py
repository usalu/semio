import json,os,sys
p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json"
t=open(p,encoding="utf8").read()
def once(old,new):
    global t
    assert t.count(old)==1,(old,t.count(old))
    t=t.replace(old,new)
once('''          "cancelledMemoryMilliseconds": 3600000
        },
        "unitCeiling": {''','''          "cancelledMemoryMilliseconds": 3600000
        },
        "growth": {
          "description": "Grant growth: a unit that needs more than the parent memory grant asks the host for a supplementary reservation of the difference (the fair queue of the host capacity gate decides against free memory, the warden thresholds and the interactive reserve) and waits for it at most `waitMilliseconds`, printing a progress line every `progressMilliseconds`. A unit that no memory this host can ever hold carries is queued for a larger machine; a unit that no machine of the fleet can hold is refused. `fleetDirectory` is the folder below the fleet store that holds one `memory.json` per origin.",
          "waitMilliseconds": 900000,
          "progressMilliseconds": 30000,
          "fleetDirectory": "origins",
          "fleetFile": "memory.json",
          "fleetStalenessMilliseconds": 86400000
        },
        "unitCeiling": {''')
en='''          "growth": {
            "asking": "unit {unit} needs about {predicted} MB but its memory grant is {grant} MB: asking the host to grow the grant by {extra} MB (at most {minutes} min)",
            "waiting": "unit {unit} still waits for {extra} MB of grant growth: it needs about {predicted} MB, grant {grant} MB, host {available} MB available, {reserve} MB reserved; waited {seconds} s",
            "grown": "unit {unit}: the memory grant grew to {granted} MB after {seconds} s",
            "timeout": "unit {unit} got no grant growth within {minutes} min: it needs about {predicted} MB, grant {grant} MB; the host may be busy, retry or use a fleet worker",
            "beyondHost": "unit {unit} needs about {predicted} MB but this host can hold at most {holdable} MB beside its reserves: it is queued for a larger machine (the largest holds {largest} MB) and never compiles here",
            "refused": "no machine of the fleet can hold unit {unit}: it needs about {predicted} MB and the largest host ({origin}) holds at most {largest} MB",
            "cancelled": "unit {unit}: waiting for grant growth was cancelled after {seconds} s"
          },
'''
de='''          "growth": {
            "asking": "Einheit {unit} braucht etwa {predicted} MB, ihre Speicherzuteilung beträgt aber {grant} MB: der Host wird gebeten, die Zuteilung um {extra} MB zu vergrößern (höchstens {minutes} min)",
            "waiting": "Einheit {unit} wartet weiter auf {extra} MB Zuteilungswachstum: sie braucht etwa {predicted} MB, Zuteilung {grant} MB, Host {available} MB verfügbar, {reserve} MB reserviert; {seconds} s gewartet",
            "grown": "Einheit {unit}: die Speicherzuteilung ist nach {seconds} s auf {granted} MB gewachsen",
            "timeout": "Einheit {unit} bekam innerhalb von {minutes} min kein Zuteilungswachstum: sie braucht etwa {predicted} MB, Zuteilung {grant} MB; der Host ist womöglich ausgelastet, erneut versuchen oder einen Flotten-Worker nutzen",
            "beyondHost": "Einheit {unit} braucht etwa {predicted} MB, dieser Host kann neben seinen Reserven höchstens {holdable} MB halten: sie wird für eine größere Maschine eingereiht (die größte hält {largest} MB) und wird hier nie kompiliert",
            "refused": "Keine Maschine der Flotte kann Einheit {unit} halten: sie braucht etwa {predicted} MB, der größte Host ({origin}) hält höchstens {largest} MB",
            "cancelled": "Einheit {unit}: das Warten auf Zuteilungswachstum wurde nach {seconds} s abgebrochen"
          },
'''
i=t.index('"Messages"')
a=t.index('          "unitCeiling": {',i); t=t[:a]+en+t[a:]
j=t.index('"de": {',a+len(en)+10)
b=t.index('          "unitCeiling": {',j); t=t[:b]+de+t[b:]
once('"reason": {"enum": ["parent-memory", "fleet-placement"]},','"reason": {"enum": ["parent-memory", "fleet-placement", "beyond-host", "no-host-fits"]},')
json.loads(t)
tmp=p+".f60.tmp"; open(tmp,"w",encoding="utf8").write(t); os.replace(tmp,p)
