import json,glob
p=glob.glob("🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json")[0]
d=json.load(open(p,encoding="utf8")); defs=d["definitions"]
c=defs["Protocol"]["const"]
c["handoverEnvironment"]="SEMIO_HOST_CAPACITY_HANDOVER"
c["handoverMaximumAgeMilliseconds"]=120000
c["parkMaximumMilliseconds"]=120000
c["handoverRefusals"]=["gone","token","expired","identity","operation"]
defs["Handover"]={"type":"object","additionalProperties":False,"required":["token","fromPid","fromStarted","offeredAtMs"],"properties":{"token":{"type":"string","pattern":"^[A-Za-z0-9_-]{22}$"},"fromPid":{"type":"integer","minimum":1},"fromStarted":{"type":"string","minLength":1,"maxLength":64},"offeredAtMs":{"type":"integer","minimum":0}}}
defs["Grant"]["properties"]["handover"]={"$ref":"#/definitions/Handover"}
defs["HandoverFacts"]={"type":"object","additionalProperties":False,"required":["grant","grantCurrent","offered","operation","parent","nowMs"],"properties":{"grant":{"oneOf":[{"type":"null"},{"$ref":"#/definitions/Grant"}]},"grantCurrent":{"type":"boolean"},"offered":{"type":"string"},"operation":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},"parent":{"oneOf":[{"type":"null"},{"type":"object","additionalProperties":False,"required":["pid","started"],"properties":{"pid":{"type":"integer","minimum":1},"started":{"type":"string","minLength":1}}}]},"nowMs":{"type":"integer","minimum":0}}}
defs["HandoverCase"]={"type":"object","additionalProperties":False,"required":["id","input","expected"],"properties":{"id":{"type":"string","minLength":1},"input":{"type":"object"},"expected":{"type":"string"}}}
m=defs["Message"]
for k in ("handover","adopted","handoverRefused"):
    m["properties"][k]={"type":"string"}
    if k not in m["required"]: m["required"].append(k)
m["properties"]["handoverReasons"]={"type":"object","additionalProperties":False,"required":c["handoverRefusals"],"properties":{k:{"type":"string"} for k in c["handoverRefusals"]}}
if "handoverReasons" not in m["required"]: m["required"].append("handoverReasons")
en=defs["Messages"]["const"]["en"]; de=defs["Messages"]["const"]["de"]
en["handover"]="[capacity] keeping the {operation} grant ({jobs} job(s), {reserved} MB reserved) for the command that replaces this one: it does not queue again"
de["handover"]="[capacity] die Zusage für {operation} ({jobs} Job(s), {reserved} MB reserviert) bleibt für den Befehl, der diesen ersetzt: er reiht sich nicht erneut ein"
en["adopted"]="[capacity] took over the {operation} grant of the command this one replaces ({jobs} job(s), {reserved} MB reserved): no queue"
de["adopted"]="[capacity] Zusage für {operation} des ersetzten Befehls übernommen ({jobs} Job(s), {reserved} MB reserviert): keine Warteschlange"
en["handoverRefused"]="[capacity] the grant of the command this one replaces cannot be taken over ({reason}): queueing like any other request"
de["handoverRefused"]="[capacity] die Zusage des ersetzten Befehls kann nicht übernommen werden ({reason}): Anfrage reiht sich wie jede andere ein"
en["handoverReasons"]={"gone":"the grant is gone or no longer current","token":"the handover token does not match","expired":"the offer expired","identity":"this process is not the one the grant was offered to","operation":"it was granted for another operation"}
de["handoverReasons"]={"gone":"die Zusage ist erloschen oder nicht mehr aktuell","token":"die Übergabemarke passt nicht","expired":"das Angebot ist abgelaufen","identity":"dieser Prozess ist nicht der, dem die Zusage angeboten wurde","operation":"sie wurde für eine andere Operation erteilt"}
open(p,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
