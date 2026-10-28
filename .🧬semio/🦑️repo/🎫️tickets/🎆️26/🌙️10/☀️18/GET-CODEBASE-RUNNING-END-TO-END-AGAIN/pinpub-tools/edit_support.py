"""🧰 [pinpub] replays the support edit of batch `pinned` onto <root> (anchored; exit 1 = anchor lost): `withGraphTestInvocationV1` takes an optional Original budget for laws that open native images."""
import sys
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts"
path = f"{sys.argv[1]}/{REL}"
s = open(path, encoding="utf8").read()
old = 'export const withGraphTestInvocationV1=(body:(invocation:any)=>Promise<void>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"graph-fixture-publisher",maximumElapsedMilliseconds:20000},'
new = '/** 🪪️ Runs a law inside one issued graph fixture Original; a law that opens native images passes a budget above their 10 s cleanup reserves (`reserveMilliseconds` of the native storage schema). */\nexport const withGraphTestInvocationV1=(body:(invocation:any)=>Promise<void>,maximumElapsedMilliseconds=20000)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"graph-fixture-publisher",maximumElapsedMilliseconds},'
if s.count(old) != 1: sys.exit("anchor lost: withGraphTestInvocationV1")
open(path, "w", encoding="utf8").write(s.replace(old, new))
print("edited support")
