#!/usr/bin/env python3
# 🧮️ Re-homing solver, phase 1: repairs the upward edges of the staged units (lower a free dependency, else raise the user) and homes the residue units by their lowest user.
import json, collections, sys
from units import *

ms, its, units, edges, defs, loose = build()
rev = collections.defaultdict(set)
for u, vs in edges.items():
    for v in vs:
        rev[v].add(u)
home = {u["id"]: u["crate"] for u in units}                  # None for residue (monolith)
size = {u["id"]: sum(its[m]["end"] - its[m]["start"] + 1 for m in u["members"]) for u in units}
moves = []                                                   # (unit, from, to, reason)
ORDER = SHELL_ORDER


def r(c): return rank_of(c)


def name_of(uid):
    f = its[units[uid]["members"][0]]
    return f"{f['name'][:36]}@{(home[uid] or 'RES')[6:]}/{f['slug'][:14]}"


def lowest_ok(uid, cap):
    """Lowest feasible rank for unit uid = max rank of its deps; returns that crate or None when above cap."""
    lo = max([r(home[v]) for v in edges[uid] if home[v] is not None] + [-1])
    return lo


def crate_at_rank(rk):
    for c in ORDER:
        if RANK[c] == rk:
            return c
    return None


def crate_floor(rk):
    """Highest Shell crate of rank <= rk."""
    best = None
    for c in ORDER:
        if RANK[c] <= rk:
            best = c
    return best


changed = True
rounds = 0
conflicts = []
while changed and rounds < 50:
    changed = False
    rounds += 1
    for uid, vs in list(edges.items()):
        cu = home[uid]
        if cu is None:
            continue
        for v in vs:
            cv = home[v]
            if cv is None:
                continue
            if r(cv) > r(cu):
                # try lowering v to cu: feasible if all deps of v are at rank <= r(cu)
                depmax = max([r(home[w]) for w in edges[v] if home[w] is not None] + [-1])
                users_min = min([r(home[w]) for w in rev[v] if home[w] is not None] + [TOP])
                if depmax <= r(cu) and users_min >= r(cu):
                    moves.append((v, cv, cu, f"lower below user {name_of(uid)}"))
                    home[v] = cu
                    changed = True
                else:
                    # raise the user to cv if it is not pinned by its own users
                    users_min_u = min([r(home[w]) for w in rev[uid] if home[w] is not None] + [TOP])
                    if users_min_u >= r(cv):
                        moves.append((uid, cu, cv, f"raise to dependency {name_of(v)}"))
                        home[uid] = cv
                        changed = True
                        break
                    else:
                        conflicts.append((name_of(uid), name_of(v), depmax, users_min, users_min_u))
print("rounds", rounds, "moves", len(moves), "conflicts", len(set(conflicts)))
for m in moves:
    print("  MOVE", name_of(m[0]), (m[1] or "RES")[6:], "->", (m[2] or "RES")[6:], "|", m[3], f"({size[m[0]]}L)")
for c in sorted(set(conflicts)):
    print("  CONFLICT", c)
json.dump([[m[0], m[1], m[2], m[3]] for m in moves], open(GEN + "/ri2-moves1.json", "w"), ensure_ascii=False)
