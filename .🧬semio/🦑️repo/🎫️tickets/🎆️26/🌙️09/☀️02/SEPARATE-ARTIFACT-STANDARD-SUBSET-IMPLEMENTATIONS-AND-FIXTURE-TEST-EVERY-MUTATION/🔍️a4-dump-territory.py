import json

TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

d = json.load(open(".🧬semio/🦑️repo/⚡️cache/breaches/testing.json"))
mine = [b for b in d if b["id"] == "missing-fixture"]


def excl(s):
    if "🗄️stdio/🗿️artifacts/🧿️semio" in s:
        return True
    if "🏗️fem/" in s:
        return True
    if "🎬️sequence" in s:
        return True
    if "🗒️note" in s:
        return True
    if "➗️mathematical" in s:
        return True
    return False


territory = [b for b in mine if not excl(b["scope"])]
with open(f"{TICKET}/🗑️generated/a4-territory-before.json", "w") as f:
    json.dump(territory, f, indent=2, ensure_ascii=False)
print("saved", len(territory))
