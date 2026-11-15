LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path.split('/')[-1],s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)
edit(LIB+'🏃️execution/🧬️schema/🌐️remote-execution.json',[
('''        "defaultActionSeconds",
        "maximumQueued",
        "maximumMessageBytes",''','''        "defaultActionSeconds",
        "maximumActionSeconds",
        "maximumQueued",
        "maximumWorkers",
        "maximumRequestBytes",
        "maximumMessageBytes",'''),
('''        "defaultActionSeconds": {
          "type": "integer",
          "minimum": 1,
          "maximum": 604800
        },
        "maximumQueued": {
          "type": "integer",
          "minimum": 1,
          "maximum": 10000000
        },''','''        "defaultActionSeconds": {
          "type": "integer",
          "minimum": 1,
          "maximum": 604800
        },
        "maximumActionSeconds": {
          "type": "integer",
          "minimum": 1,
          "maximum": 86400,
          "description": "The longest an action may run; a longer timeout in a request is cut to it."
        },
        "maximumQueued": {
          "type": "integer",
          "minimum": 1,
          "maximum": 10000000
        },
        "maximumWorkers": {
          "type": "integer",
          "minimum": 1,
          "maximum": 100000,
          "description": "How many workers may be registered at once."
        },
        "maximumRequestBytes": {
          "type": "integer",
          "minimum": 65536,
          "maximum": 1073741824,
          "description": "The largest body the worker API accepts."
        },'''),
('''        "network",
        "confinement"
      ],
      "properties": {
        "concurrency": {''','''        "network",
        "confinement",
        "actions"
      ],
      "properties": {
        "concurrency": {'''),
('''          "description": "`required`: a worker whose host has no confining sandbox refuses to start; `best-effort`: it starts with the private input root alone and reports `confinement: none`."
        }''','''          "description": "`required`: a worker whose host has no confining sandbox refuses to start; `best-effort`: it starts with the private input root alone and reports `confinement: none`."
        },
        "actions": {
          "enum": [
            "rustc",
            "any",
            "none"
          ],
          "description": "Which actions the worker runs. `rustc`: only the declared rustc unit of the offload, with arguments and environment from a closed allow-list (the default, and the only choice an unconfined worker accepts besides `none`); `any`: every command, refused at start on a worker without a confining sandbox; `none`: no action."
        }'''),
])
edit(LIB+'🔣️policy.json',[
('''      "defaultActionSeconds": 3600,
      "maximumQueued": 100000,''','''      "defaultActionSeconds": 1800,
      "maximumActionSeconds": 1800,
      "maximumQueued": 4096,
      "maximumWorkers": 256,
      "maximumRequestBytes": 16777216,'''),
('''      "confinement": "best-effort"''','''      "confinement": "best-effort",
      "actions": "rustc"'''),
])
print("policy ok")
