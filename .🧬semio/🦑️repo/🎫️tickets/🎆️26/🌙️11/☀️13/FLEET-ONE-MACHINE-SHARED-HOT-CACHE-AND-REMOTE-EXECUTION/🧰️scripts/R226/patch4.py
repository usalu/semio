S="/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226"
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧬️schema/🔣️.json"
t=open(f"{S}/base/{P}",encoding="utf8").read()
def once(t,old,new):
    assert t.count(old)==1,(old[:70],t.count(old)); return t.replace(old,new)
t=once(t,'"holderWaitPatterns": ["holds the dependency store since", "hält den Abhängigkeitsspeicher seit",','"holderWaitPatterns": ["holds the dependency store since", "hält den Abhängigkeitsspeicher seit", "holds the lease .+? since", "hält die Lease .+? seit",')
t=once(t,') holds the dependency store since (?<since>',') holds (?:the dependency store|the lease .+?) since (?<since>')
t=once(t,') hält den Abhängigkeitsspeicher seit (?<since>',') hält (?:den Abhängigkeitsspeicher|die Lease .+?) seit (?<since>')
open(f"{S}/new/{P}","w",encoding="utf8").write(t)
import json;json.loads(t)
