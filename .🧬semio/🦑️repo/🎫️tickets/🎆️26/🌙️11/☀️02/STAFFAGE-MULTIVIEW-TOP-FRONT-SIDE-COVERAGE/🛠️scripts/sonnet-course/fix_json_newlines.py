import json
import re
from pathlib import Path

path = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️staffage_only.json")
text = path.read_bytes().decode("utf-8")
start = text.index('"cast_rules"')
end = text.index('"cast_calls"')
body = re.sub(r'"def [^"]*"', lambda match: match.group(0).replace("\r\n", "\\n").replace("\n", "\\n"), text[start:end])
text = text[:start] + body + text[end:]
json.loads(text)
path.write_bytes(text.encode("utf-8"))
print("ok")
