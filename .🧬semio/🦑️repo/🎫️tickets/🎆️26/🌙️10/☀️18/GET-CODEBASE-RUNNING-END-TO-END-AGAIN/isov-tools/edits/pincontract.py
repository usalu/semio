"""📌 [isov] batch `pincontract`: the runtime-pin contract law pins the `[test] preload` list of bunfig.toml; the isolate `fence` batch added the live-store fence as its third entry, so the list the law expects follows. Re-run = rebase onto the current live bytes (exit 2 names a lost anchor)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mkbatch import build, live_bytes, replace_once
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/🟦️.ts"
base = live_bytes(REL); text = base.decode("utf-8")
text = replace_once(text, 'expect(parsed.test.preload).toEqual([guard,"./🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/📜️script.ts"]);', 'expect(parsed.test.preload).toEqual([guard,"./🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/📜️script.ts","./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🚧️fence/📜️script.ts"]);', "preload list")
build("pincontract", "📌 Staged batch `pincontract` (lane isov): the runtime-pin contract law expects the live-store fence as third test preload.", {REL: (base, text.encode("utf-8"))}, {REL: {"mode": "plain", "pass": 15, "fails": []}})
