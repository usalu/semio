#!/usr/bin/env python3
# 🧮️ Re-homing solver, phase 3: the residue units (still in the old chunk files) get the crate of their lowest user; conflicts are reported.
import json, collections, sys, runpy
sys.argv = [sys.argv[0]]
import io, contextlib
buf = io.StringIO()
with contextlib.redirect_stdout(buf):
    S = runpy.run_path(__import__("os").path.dirname(__import__("os").path.abspath(__file__)) + "/solve2.py")
its, units, edges, rev, home, size, pinned, r, nm, crate_of_rank, try_lower, TOP, RANK = [S[k] for k in "its units edges rev home size pinned r nm crate_of_rank try_lower TOP RANK".split()]
moves2 = S["moves"]
GEN = S["GEN"]
res = [u["id"] for u in units if home[u["id"]] is None and u["mk"] == "chunk"]
print("residue units", len(res))
# fixed point: a residue unit with a homed user is homed at the lowest user's rank
moves3 = []
for rnd in range(30):
    ch = False
    for uid in res:
        if home[uid] is not None:
            continue
        users = [r(home[w]) for w in rev[uid] if home[w] is not None]
        if not users:
            continue
        hi = min(users)
        home[uid] = crate_of_rank(hi)
        moves3.append((uid, None, home[uid], "lowest user"))
        ch = True
    if not ch:
        break
# conflicts: homed unit with dep higher than itself, or residue dep not homed -> homed above
conf = []
for rnd in range(30):
    ch = False
    for uid in res:
        cu = home[uid]
        if cu is None:
            continue
        for v in sorted(edges[uid]):
            cv = home[v]
            if cv is None:
                home[v] = cu
                moves3.append((v, None, cu, f"dep of {nm(uid)}"))
                ch = True
                continue
            if r(cv) > r(cu):
                plan = {}
                if try_lower(v, r(cu), plan, frozenset()):
                    for w, rk in plan.items():
                        new = crate_of_rank(rk)
                        if new != home[w]:
                            moves3.append((w, home[w], new, f"lower with {nm(v)} below {nm(uid)}"))
                            home[w] = new
                            ch = True
                else:
                    conf.append((nm(uid), nm(v)))
    if not ch:
        break
by = collections.defaultdict(list)
for m in moves3:
    by[m[2]].append(m)
for c in SHELL_ORDER if False else sorted(by, key=lambda c: RANK[c]):
    print("==", c, len(by[c]))
    for m in by[c]:
        print("   ", nm(m[0]), f"({size[m[0]]}L)", "|", m[3][:70])
print("CONFLICTS", len(set(conf)))
for c in sorted(set(conf)):
    print("  ", c)
stay = [u["id"] for u in units if home[u["id"]] is None and u["mk"] == "chunk"]
print("stay in monolith:", len(stay))
for uid in stay:
    f = its[units[uid]["members"][0]]
    print("   stay", f["slug"][:22], f["kind"], f["name"][:40], f"{size[uid]}L")
json.dump({"moves2": [[m[0], m[1], m[2]] for m in moves2], "moves3": [[m[0], m[1], m[2]] for m in moves3]}, open(GEN + "/ri2-moves3.json", "w"), ensure_ascii=False)
