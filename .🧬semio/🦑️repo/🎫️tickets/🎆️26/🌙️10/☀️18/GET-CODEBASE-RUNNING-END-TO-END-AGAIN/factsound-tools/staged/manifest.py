#!/usr/bin/env python3
"""🧮 Writes manifest.json of this batch from base/ and files/ (base null for new files)."""
import hashlib, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/"
ROWS = {B + "🧠️memo/📤️relevance/🟦️.ts": "00.ts", B + "🟦️.ts": "01.ts", B + "🧠️memo/🧬️schema/🔣️.json": "02.json", B + "🧠️memo/🧪️tests/🎯️slice/🟦️.ts": "03.ts"}
sha = lambda path: hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
json.dump({rel: {"file": f, "base": sha(os.path.join(HERE, "base", f)), "new": sha(os.path.join(HERE, "files", f))} for rel, f in ROWS.items()}, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
