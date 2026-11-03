#!/usr/bin/env python3
"""Self-test of the re-run safety of `convert.py`: a value that already is a variant is never rewritten (no default, no legacy remap), a legacy value converts once, a second pass changes nothing, a conflicting value is reported as `lossy`.
Usage: python3 -I convert_selftest.py"""
import importlib.util, os, types
spec = importlib.util.spec_from_file_location("convert", os.path.join(os.path.dirname(os.path.abspath(__file__)), "convert.py"))
cv = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cv)
variants = [cv.Variant("C24", ["C24", "c24-old"], "C24", "C24"), cv.Variant("Gl28h", ["GL28h"], "GL28h", "GL28h"), cv.Variant("Lvl32", ["LVL32", "gl28h"], "LVL32", "LVL32")]
conv = cv.Conversion("en1995", ["TimberMember.strength_class"], "StrengthClass", variants, "x", "C24")
for v in variants:
    for t in (v.name[:1].lower() + v.name[1:],):
        conv.legacy.setdefault(t, v.name)
site = types.SimpleNamespace(c="strengthClass", siblings={"id", "role", "serviceClass", "support"}, patches={}, patch_sets={}, patch_names=set())
plan = types.SimpleNamespace(sites=[site], conv=conv, tokens={v.name: v.name[:1].lower() + v.name[1:] for v in variants})
plan.map_value = lambda token, alt_of=None: cv.Plan.map_value(plan, token)
plan.current_variant = lambda token: cv.Plan.current_variant(plan, token)
ctx = types.SimpleNamespace(todo=[], log=[], note=lambda kind, path, text: ctx.todo.append(f"{kind}: {text}"), rel=lambda p: p)
src = '{"id":"a","role":"floor","serviceClass":1,"support":"x","strengthClass":"%s"}'
def run(value):
    return cv.rule_values(ctx, plan, "x.json", src % value)
assert run("GL28h") == src % "gl28h", "legacy value converts"
assert run("bogus") == src % "c24", "unknown legacy value takes the default"
assert run("lvl32") == src % "lvl32", "converted value is kept"
assert run("c24") == src % "c24"
ctx.todo.clear()
assert run("gl28h") == src % "gl28h", "a converted value that the legacy table maps elsewhere is kept"
assert any(x.startswith("lossy:") for x in ctx.todo), "and reported"
for value in ("GL28h", "bogus", "lvl32", "c24", "gl28h"):
    once = run(value)
    assert run(once.split('"strengthClass":"')[1].rstrip('"}')) == once, "second pass is a no-op"
print("convert_selftest ok")
