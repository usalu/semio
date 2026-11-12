import importlib, sys
sys.path.insert(0, "C:/git/semio")
t = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🧪️tests.🎞️full.🎬️execute.🐍️")
case = t.load("recipes.json")["cases"][0]
source, target, tr, scene = t.build(case)
anim = t.full.execute(source, target, transition=tr, fps=10)
t.start(anim, scene)
for a in (6/8, 7/8, 1.0, 6/8):
    anim.interpolate(a)
    print(a, anim.anims_finished, anim.anim_group_time, anim.max_end_time, [round(f.mobject.get_fill_opacity(), 3) for f in anim.flights])
