#!/usr/bin/env python3
"""🐍️ Independent numpy 3D frame oracle for the fem3d `concrete-forest` example.

Reads the committed `🗣️.dsl.semio` asset with its own minimal parser (nodes, frames, materials,
sections, supports, load cases with `member-udl` + self weight, combinations), assembles standard
12×12 Euler–Bernoulli space-frame stiffness (Przemieniecki) with global-direction UDL fixed-end loads
and consistent self weight, solves with numpy, and prints per-node displacements for every case and
combination as JSON — to be diffed against the Rust engine's `[DEBUG] concrete-forest sls …` rows.
No Rust is imported; the only shared input is the DSL text.
"""
import json
import math
import re
import sys

import numpy as np

G = 9.81


def parse(text):
    doc = {"nodes": {}, "elements": [], "materials": {}, "sections": {}, "supports": {}, "cases": {}, "combinations": {}}
    lines = text.splitlines()
    i = 0
    section = None
    case = None
    while i < len(lines):
        line = lines[i].strip()
        i += 1
        if not line:
            continue
        if line.startswith("elements {"):
            section = "elements"; continue
        if re.match(r"^(nodes|materials|sections|supports|load-cases|combinations|solids|analysis) ", line) or line.startswith("analysis {"):
            section = line.split(" ")[0]; continue
        if line == "}":
            if case is not None and section == "load-cases":
                case = None
                continue
            section = None; continue
        if section == "elements" and line.startswith("frame "):
            kv = dict(part.split("=", 1) for part in line.split()[1:])
            doc["elements"].append({"id": kv["id"], "start": kv["start"], "end": kv["end"], "material": kv["material-id"], "section": kv["section-id"], "roll": float(kv["roll"])})
        elif section == "nodes":
            nid, x, y, z = line.split()
            doc["nodes"][nid] = np.array([float(x), float(y), float(z)])
        elif section == "materials":
            m = re.match(r'^(\S+) "([^"]*)" (\S+) (\S+) (\S+) (\S+)$', line)
            doc["materials"][m.group(1)] = {"e": float(m.group(3)), "g": float(m.group(4)), "rho": float(m.group(6))}
        elif section == "sections":
            m = re.match(r'^(\S+) "([^"]*)" (\S+) (\S+) (\S+) (\S+)$', line)
            doc["sections"][m.group(1)] = {"a": float(m.group(3)), "iy": float(m.group(4)), "iz": float(m.group(5)), "j": float(m.group(6))}
        elif section == "supports":
            m = re.match(r"^(\S+) (\S+) \[ (.*) \]$", line)
            doc["supports"][m.group(2)] = m.group(3).split()
        elif section == "load-cases":
            if case is None and line in ("true", "false"):
                doc["cases"][doc.pop("_pending")]["self_weight"] = line == "true"
            elif case is None:
                m = re.match(r'^(\S+) "([^"]*)" \{$', line)
                case = m.group(1)
                doc["cases"][case] = {"udl": [], "nodal": [], "self_weight": False}
                doc["_pending"] = case
            elif line.startswith("member-udl "):
                kv = dict(part.split("=", 1) for part in line.split()[1:])
                doc["cases"][case]["udl"].append((kv["element-id"], float(kv["wx"]), float(kv["wy"]), float(kv["wz"])))
            elif line.startswith("nodal "):
                kv = dict(part.split("=", 1) for part in line.split()[1:])
                doc["cases"][case]["nodal"].append((kv["node-id"], kv["dof"], float(kv["value"])))
        elif section == "combinations":
            m = re.match(r"^(\S+) (\S+) \{$", line)
            if m:
                terms = {}
                while True:
                    inner = lines[i].strip(); i += 1
                    if inner == "}":
                        break
                    for part in inner.split():
                        k, v = part.split("=")
                        terms[k] = float(v)
                doc["combinations"][m.group(1)] = terms
    return doc


DOFS = ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"]


def local_axes(p1, p2, roll):
    d = p2 - p1
    L = np.linalg.norm(d)
    ex = d / L
    ref = np.array([1.0, 0.0, 0.0]) if abs(ex[2]) > 0.99 else np.array([0.0, 0.0, 1.0])
    ey = np.cross(ref, ex); ey /= np.linalg.norm(ey)
    ez = np.cross(ex, ey)
    c, s = math.cos(roll), math.sin(roll)
    ey, ez = ey * c + ez * s, ez * c - ey * s
    return L, np.array([ex, ey, ez])


def local_stiffness(E, Gm, A, Iy, Iz, J, L):
    k = np.zeros((12, 12))
    a = E * A / L
    t = Gm * J / L
    k[0, 0] = k[6, 6] = a; k[0, 6] = k[6, 0] = -a
    k[3, 3] = k[9, 9] = t; k[3, 9] = k[9, 3] = -t
    for (I, idx, sgn) in ((Iz, [1, 5, 7, 11], 1.0), (Iy, [2, 4, 8, 10], -1.0)):
        b = E * I / L
        blk = np.array([[12 * b / L**2, 6 * b / L, -12 * b / L**2, 6 * b / L], [6 * b / L, 4 * b, -6 * b / L, 2 * b], [-12 * b / L**2, -6 * b / L, 12 * b / L**2, -6 * b / L], [6 * b / L, 2 * b, -6 * b / L, 4 * b]])
        S = np.diag([1, sgn, 1, sgn])
        blk = S @ blk @ S
        for r in range(4):
            for c in range(4):
                k[idx[r], idx[c]] = blk[r, c]
    return k


def fixed_end_udl(L, w_local):
    wx, wy, wz = w_local
    f = np.zeros(12)
    f[0] = f[6] = wx * L / 2
    f[1] = f[7] = wy * L / 2; f[5] = wy * L**2 / 12; f[11] = -wy * L**2 / 12
    f[2] = f[8] = wz * L / 2; f[4] = -wz * L**2 / 12; f[10] = wz * L**2 / 12
    return f


def solve(doc, case):
    ids = list(doc["nodes"])
    index = {nid: i for i, nid in enumerate(ids)}
    n = 6 * len(ids)
    K = np.zeros((n, n)); F = np.zeros(n)
    udl_by_element = {}
    for eid, wx, wy, wz in case["udl"]:
        udl_by_element[eid] = np.array([wx, wy, wz])
    for el in doc["elements"]:
        p1, p2 = doc["nodes"][el["start"]], doc["nodes"][el["end"]]
        L, R = local_axes(p1, p2, el["roll"])
        mat, sec = doc["materials"][el["material"]], doc["sections"][el["section"]]
        kl = local_stiffness(mat["e"], mat["g"], sec["a"], sec["iy"], sec["iz"], sec["j"], L)
        T = np.zeros((12, 12))
        for o in (0, 3, 6, 9):
            T[o:o + 3, o:o + 3] = R
        kg = T.T @ kl @ T
        w = udl_by_element.get(el["id"], np.zeros(3)).copy()
        if case["self_weight"]:
            w[2] += -mat["rho"] * sec["a"] * G
        fl = fixed_end_udl(L, R @ w)
        fg = T.T @ fl
        dof = [6 * index[el["start"]] + d for d in range(6)] + [6 * index[el["end"]] + d for d in range(6)]
        K[np.ix_(dof, dof)] += kg
        F[dof] += fg
    for nid, dof, value in case["nodal"]:
        F[6 * index[nid] + DOFS.index(dof)] += value
    fixed = [6 * index[nid] + DOFS.index(d) for nid, ds in doc["supports"].items() for d in ds]
    free = [i for i in range(n) if i not in fixed]
    u = np.zeros(n)
    u[free] = np.linalg.solve(K[np.ix_(free, free)], F[free])
    reactions = K[fixed] @ u - F[fixed]
    return {nid: u[6 * i:6 * i + 6].tolist() for nid, i in index.items()}, float(sum(reactions[j] for j, dof in enumerate(fixed) if dof % 6 == 2))


def main(path):
    doc = parse(open(path, encoding="utf-8").read())
    out = {}
    for cid, case in doc["cases"].items():
        u, rz = solve(doc, case)
        out[cid] = {"displacements": u, "reaction_z": rz}
    for cid, terms in doc["combinations"].items():
        u = {nid: [0.0] * 6 for nid in doc["nodes"]}
        for base, factor in terms.items():
            for nid, vals in out[base]["displacements"].items():
                u[nid] = [a + factor * b for a, b in zip(u[nid], vals)]
        out[cid] = {"displacements": u}
    json.dump(out, sys.stdout, indent=1)


if __name__ == "__main__":
    main(sys.argv[1])
