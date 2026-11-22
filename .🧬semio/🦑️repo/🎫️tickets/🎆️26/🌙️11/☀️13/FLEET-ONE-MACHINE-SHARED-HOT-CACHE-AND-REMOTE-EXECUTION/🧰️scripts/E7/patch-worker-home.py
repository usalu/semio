ROOT="/Users/ueli/Documents/semio/🧰️framework/"
LIB=ROOT+"🛍️products/🦑️repo/🔨️modules/📚️library/"
LEDGER=LIB+"⚡️caching/💾️storage/🧾️ledger/"
AG=LIB+"🏗️build/🌐️remote-execution/🛰️sidecar/👷️agent/🟦️.ts"
def sub(p,o,n,c=1):
    t=open(p,encoding="utf8").read()
    if t.count(o)==0 and n in t:print("already",p[-30:]);return
    assert t.count(o)==c,(p[-50:],o[:70],t.count(o));open(p,"w",encoding="utf8").write(t.replace(o,n))
area='''    },
    {
      "id": "remote-execution-worker",
      "root": "native",
      "kind": "nested",
      "directory": "remote-execution",
      "partition": "dependencies",
      "budget": {
        "basisPoints": 1000
      },
      "managed": {
        "owner": "remote-execution"
      },
      "writers": [
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/👷️agent/🟦️.ts"
      ],
      "purpose": {
        "en": "Blob cache and private action roots of the remote execution worker of this machine, a tenth of the dependencies partition; its image is not logged by the file event service, so the thousands of small files per offloaded unit do not grow it.",
        "de": "Blob-Cache und private Aktionsverzeichnisse des Remote-Execution-Workers dieser Maschine, ein Zehntel der Abhängigkeits-Partition; ihr Image wird vom Dateiereignisdienst nicht protokolliert, so wachsen die tausenden kleinen Dateien je ausgelagerter Einheit ihn nicht."
      }
    }
  ],
  "needs": ['''
sub(LEDGER+"🔣️policy.json",'''    }
  ],
  "needs": [''',area)
sub(AG,'const directory=machinePathV1(context.root,".cache","f","remote-execution"),','const directory=cacheAreaHomeV1(context.root,"remote-execution-worker").directory,')
t=open(AG,encoding="utf8").read()
if "cacheAreaHomeV1}" not in t:
    i=t.index("\nimport")
    t=t[:i]+'\nimport {cacheAreaHomeV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";'+t[i:]
    open(AG,"w",encoding="utf8").write(t)
print("ok")
