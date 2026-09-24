"""R8 one-off: shifts the lifecycle fixture's rollback boundaries by the layout-update close turns and re-signs them.

The signature is the flow laws' own FNV-1a-64 over the canonical JSON (`flow_hostile_fixture_digest`)."""
import json, struct, sys
PATH = sys.argv[1]
SHIFT = int(sys.argv[2])

def canonical(value):
    if value is None: return "null"
    if value is True: return "true"
    if value is False: return "false"
    if isinstance(value, (int, float)): return "f64:%016x" % struct.unpack("<Q", struct.pack("<d", float(value)))[0]
    if isinstance(value, str): return json.dumps(value, ensure_ascii=False)
    if isinstance(value, list): return "[" + "".join(canonical(item) + "," for item in value) + "]"
    return "{" + "".join(json.dumps(key, ensure_ascii=False) + ":" + canonical(value[key]) + "," for key in sorted(value)) + "}"

def digest(value):
    result = 14695981039346656037
    for byte in canonical(value).encode("utf8"):
        result ^= byte
        result = (result * 1099511628211) % (1 << 64)
    return result

text = open(PATH, encoding="utf8").read()
fixture = json.loads(text)
ledger = fixture["transferControlLedger"]
signatures = fixture["hostileVectorDigests"]["transferControlLedger"]
check = [(entry["boundary"], digest(entry), int(signature)) for entry, signature in zip(ledger, signatures)]
stale = [row for row in check if row[1] != row[2]]
print("stale before:", stale)
if SHIFT == 0:
    raise SystemExit
for index, entry in enumerate(ledger):
    target = entry["protocol"]["target"]
    if "rollbackSteps" not in target:
        continue
    old_steps, old_digest = target["rollbackSteps"], str(digest(entry))
    new_steps = old_steps + SHIFT
    needle = f'"rollbackSteps": {old_steps},'
    start = text.index(f'"boundary": "{entry["boundary"]}"')
    at = text.index(needle, start)
    text = text[:at] + f'"rollbackSteps": {new_steps},' + text[at + len(needle):]
    target["rollbackSteps"] = new_steps
    new_digest = str(digest(entry))
    assert text.count(f'"{old_digest}"') == 1, old_digest
    text = text.replace(f'"{old_digest}"', f'"{new_digest}"')
    print(entry["boundary"], old_steps, "->", new_steps, old_digest, "->", new_digest)
json.loads(text)
open(PATH, "w", encoding="utf8").write(text)
