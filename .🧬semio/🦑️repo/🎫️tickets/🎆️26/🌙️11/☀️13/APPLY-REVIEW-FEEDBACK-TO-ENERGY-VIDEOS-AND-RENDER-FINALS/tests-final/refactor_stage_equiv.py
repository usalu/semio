import importlib
import manim

stage_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")


def old_regroup(scene, group):
    family = set(group.get_family())
    scene.remove(*[m for m in scene.mobjects if m is not group and family.intersection(m.get_family())])
    scene.add(group)


def build():
    scene = manim.Scene()
    a, b, c = manim.Square(), manim.Circle(), manim.Dot()
    other, tail = manim.Triangle(), manim.Line()
    group = manim.VGroup(a, manim.VGroup(b, c))
    scene.add(other, a, tail, b, c)
    return scene, group, other, tail


for name in ("sub", "nested"):
    s1, g1, o1, t1 = build()
    old_regroup(s1, g1)
    s2, g2, o2, t2 = build()
    holder = type("B", (), {"mobject": g2})()
    stage_module.Stage._consume(type("S", (), {"scene": s2})(), holder)
    names = lambda s, g, o, t: ["g" if m is g else "o" if m is o else "t" if m is t else "?" for m in s.mobjects]
    assert names(s1, g1, o1, t1) == names(s2, g2, o2, t2) == ["o", "t", "g"], (names(s1, g1, o1, t1), names(s2, g2, o2, t2))
print("EQUAL scene order old _regroup vs Stage._consume: ['other', 'tail', 'group']")
