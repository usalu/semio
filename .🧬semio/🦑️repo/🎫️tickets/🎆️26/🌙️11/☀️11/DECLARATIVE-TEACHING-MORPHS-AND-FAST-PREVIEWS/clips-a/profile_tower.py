"""cProfile of warm tower builds (orders already on disk): profile_tower.py <k> [<k> ...]"""
import cProfile
import importlib
import pstats
import sys
import time

visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
draw = visuals.NakaginDrawing()
pods = model.towers()[1].first_capsule_storey


def build(k):
    return draw.tower_axon(at=(0, -1), scale=0.05, towers=(1,), storeys=k + pods, where=lambda s, k=k: s.storey < k, podium=False, capital=True, types=True)


ks = [int(value) for value in sys.argv[1:]]
for k in ks:
    t0 = time.perf_counter()
    build(k)
    print("k", k, round(time.perf_counter() - t0, 2), "s (first sight)")
profiler = cProfile.Profile()
profiler.enable()
t0 = time.perf_counter()
build(ks[0] + 100 if False else ks[-1] + 1)
profiler.disable()
print("next k", round(time.perf_counter() - t0, 2))
pstats.Stats(profiler).sort_stats("cumulative").print_stats(28)
