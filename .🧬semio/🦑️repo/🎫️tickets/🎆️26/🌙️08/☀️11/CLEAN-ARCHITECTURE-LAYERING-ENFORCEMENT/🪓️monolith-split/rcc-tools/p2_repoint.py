"""🚢️ Re-points the five hub shipped-fleet owner-fixture reads to the stdio-owned crate constant."""
import re, sys
path = "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs"
text = open(path, encoding="utf-8").read()
pattern = re.compile(r'include_str!\("\.\./\.\./\.\./\.\./\.\./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️\.json"\)')
assert len(pattern.findall(text)) == 5, len(pattern.findall(text))
new = pattern.sub("semio_s_artifact_stdio_contract::testing::SHIPPED_FLEET_SQLITE_OWNERS", text)
open(path, "w", encoding="utf-8").write(new)
print("replaced 5")
