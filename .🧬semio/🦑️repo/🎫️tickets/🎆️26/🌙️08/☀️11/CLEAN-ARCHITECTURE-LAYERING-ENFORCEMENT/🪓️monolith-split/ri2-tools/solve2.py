#!/usr/bin/env python3
# 🧮️ Re-homing solver, phase 2: recursive lowering of free dependencies (a unit moves down together with its higher dependencies), pinned component units never move.
import json, collections, sys
from units import *

ms, its, units, edges, defs, loose = build()
rev = collections.defaultdict(set)
for u, vs in edges.items():
    for v in vs:
        rev[v].add(u)
home = {u["id"]: u["crate"] for u in units}
size = {u["id"]: sum(its[m]["end"] - its[m]["start"] + 1 for m in u["members"]) for u in units}
COMPONENTS = {"ChromeState", "SurfaceState", "TutorialState", "SettleState", "PluginState", "HistoryState", "PrefsState", "DirectoryState", "SyncState", "DocumentState", "OpeningState", "WindowState", "ShellDockState", "ShellInputState", "CommandState", "ShellState"}
pinned = set()
for u in units:
    f = its[u["members"][0]]
    if f["name"] in COMPONENTS and f["kind"] == "struct":
        pinned.add(u["id"])
moves = []


def r(c): return rank_of(c)


def nm(uid):
    f = its[units[uid]["members"][0]]
    return f"{f['name'][:40]}@{(home[uid] or 'RES')[6:]}/{f['slug'][:12]}"


def crate_of_rank(rk):
    for c in SHELL_ORDER:
        if RANK[c] == rk:
            return c


def try_lower(v, target_rank, plan, seen):
    """Plan lowering unit v to <= target_rank (with its deps). plan: dict unit->rank. Returns True/False."""
    if v in plan and plan[v] <= target_rank:
        return True
    cur = r(plan.get(v, None) and crate_of_rank(plan[v]) or home[v])
    if home[v] is None and v not in plan:
        pass
    users_min = min([r(home[w]) for w in rev[v] if home[w] is not None and w not in plan] + [plan[w] for w in rev[v] if w in plan] + [TOP])
    tr = min(target_rank, users_min)
    if cur <= tr and v not in plan:
        return True
    if v in pinned:
        return False
    if v in seen:
        return True
    seen = seen | {v}
    plan[v] = tr
    for w in edges[v]:
        if home[w] is None and w not in plan:
            continue
        wr = plan[w] if w in plan else r(home[w])
        if wr > tr:
            if not try_lower(w, tr, plan, seen):
                return False
    return True


changed = True
rounds = 0
conflicts = []
while changed and rounds < 40:
    changed = False
    rounds += 1
    for uid in sorted(edges):
        cu = home[uid]
        if cu is None:
            continue
        for v in sorted(edges[uid]):
            cv = home[v]
            if cv is None or r(cv) <= r(cu):
                continue
            plan = {}
            if try_lower(v, r(cu), plan, frozenset()):
                for w, rk in plan.items():
                    new = crate_of_rank(rk)
                    if new != home[w]:
                        moves.append((w, home[w], new, f"lower with {nm(v)} below user {nm(uid)}"))
                        home[w] = new
                        changed = True
            else:
                conflicts.append((nm(uid), nm(v)))
print("rounds", rounds, "moves", len(moves), "conflicts", len(set(conflicts)))
for m in moves:
    print("  MOVE", nm(m[0]), (m[1] or "RES")[6:], "->", (m[2] or "RES")[6:], f"({size[m[0]]}L)", "|", m[3][:100])
for c in sorted(set(conflicts)):
    print("  CONFLICT", c)
json.dump([[m[0], m[1], m[2], m[3]] for m in moves], open(GEN + "/ri2-moves2.json", "w"), ensure_ascii=False)
