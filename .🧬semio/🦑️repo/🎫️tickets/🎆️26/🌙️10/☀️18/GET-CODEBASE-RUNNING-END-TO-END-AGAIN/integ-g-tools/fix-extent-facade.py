"""One-line fix of store-g's extent facade: six `../` like every sibling; refuses when the file changed meanwhile."""
import sys
path = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts"
text = open(path, encoding="utf8").read()
old = 'from "../../../../../📦️artifacts/'
new = 'from "../../../../../../📦️artifacts/'
if new in text: print("already fixed"); sys.exit(0)
if text.count(old) != 1: print("unexpected content", file=sys.stderr); sys.exit(1)
open(path, "w", encoding="utf8").write(text.replace(old, new))
print("fixed")
