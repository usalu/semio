"""Registers the EN 1996 wave 2 kinds in the Python vocabulary adapter, the mutate feature and the oracle manifest. Idempotent."""
import os, re, copy
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))
T = f"{SUB}/🧪️tests/🪨️mutate-en1996-1"
ls = leaves()
p = f"{T}/🐍️.py"; s = rd(p)
if '"change-pier-position"' not in s:
    s = s.replace('    "change-hk-earth",\n]', '    "change-hk-earth",\n' + "".join(f'    "{l["kind"]}",\n' for l in ls) + "]", 1)
    s = s.replace('    "change-hk-earth": ("🪨change-hk-earth", "✅apply"),\n', '    "change-hk-earth": ("🪨change-hk-earth", "✅apply"),\n' + "".join(f'    "{l["kind"]}": ("{dirname(l)}", "✅apply"),\n' for l in ls), 1)
    wr(p, s)
p = f"{T}/🥒️.feature"; s = rd(p)
if "change-pier-position" not in s:
    row = lambda l: f"      | {l['kind']:<34} | {dirname(l):<34} | ✅apply  |\n"
    anchor = "      | change-hk-earth                    | 🪨change-hk-earth                    | ✅apply  |\n"
    assert s.count(anchor) == 2
    s = s.replace(anchor, anchor + "".join(row(l) for l in ls))
    n0 = 58
    s = s.replace("every one of the 58 kinds", f"every one of the {58 + len(ls)} kinds").replace("The 58 kinds address", f"The {58 + len(ls)} kinds address")
    wr(p, s)
p = f"{SUB}/🔮️oracles/🔣️.json"; d = jload(p)
if "change-pier-position" not in json.dumps(d):
    def walk(n):
        if isinstance(n, dict):
            for v in n.values():
                yield from walk(v)
        elif isinstance(n, list):
            yield n
            for v in n:
                yield from walk(v)
    for lst in list(walk(d)):
        if "change-hk-earth" in lst and all(isinstance(x, str) for x in lst):
            lst.extend(l["kind"] for l in ls)
        elif lst and isinstance(lst[0], dict):
            for x in list(lst):
                if x.get("mutationId") == "change-hk-earth":
                    for l in ls:
                        n = copy.deepcopy(x)
                        n.update(mutationId=l["kind"], sourceMutationDirectoryName=dirname(l), mutationDirectoryName=dirname(l))
                        lst.append(n)
                elif x.get("id") == "change-hk-earth":
                    for l in ls:
                        n = copy.deepcopy(x)
                        n["id"] = l["kind"]
                        n["productionDispatch"].update(operation=l["kind"], variant=pascal(l["kind"]))
                        lst.append(n)
    jdump(p, d)
print("vectors registered", len(ls))
