import json, sys
sys.path.insert(0, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lang-d-tools")
from conv import catalog
rows = json.load(open(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lang-d-tools/ckey2-messages.json", encoding="utf8"))["messages"]
catalog("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🔣️.json", {r["key"]: (r["en"], r["de"]) for r in rows})
print(len(rows), "messages added to the build catalog")
