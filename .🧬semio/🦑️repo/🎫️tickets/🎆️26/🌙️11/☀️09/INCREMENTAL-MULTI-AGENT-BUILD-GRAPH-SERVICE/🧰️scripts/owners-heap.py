"""Lean heap-snapshot analysis for 7M-node V8 snapshots: breadth-first spanning tree (numpy), subtree sizes by path, self size by class, and a class diff of two snapshots.
usage: owners-heap.py tree SNAP [DEPTH] [TOP] | classes SNAP [TOP] | diff A B [TOP]"""
import json, re, sys
import numpy as np

def section(data, key, start=0):
    marker = ('"%s":[' % key).encode()
    at = data.find(marker, start)
    if at < 0: raise SystemExit("section missing: " + key)
    begin = at + len(marker)
    end = data.find(b"]", begin)
    return np.fromstring(data[begin:end].decode("ascii"), dtype=np.int64, sep=",") if end - begin < 3 * 10**8 else np.array(data[begin:end].decode("ascii").split(","), dtype=np.int64)

def load(path):
    data = open(path, "rb").read()
    head_at = data.find(b'"snapshot":')
    meta = json.JSONDecoder().raw_decode(data[head_at + len(b'"snapshot":'):].decode("utf8", "replace")[:2000000])[0]["meta"]
    nodes = section(data, "nodes"); edges = section(data, "edges")
    s_at = data.find(b'"strings":[')
    strings = json.JSONDecoder().raw_decode(data[s_at + len(b'"strings":'):].decode("utf8", "replace"))[0]
    del data
    nf, ef = meta["node_fields"], meta["edge_fields"]
    NF, EF = len(nf), len(ef)
    return meta, nodes.reshape(-1, NF), edges.reshape(-1, EF), strings, nf, ef

def prepare(path):
    meta, nodes, edges, strings, nf, ef = load(path)
    ni = {n: i for i, n in enumerate(nf)}; ei = {n: i for i, n in enumerate(ef)}
    N = nodes.shape[0]; NF = nodes.shape[1]
    count = nodes[:, ni["edge_count"]]
    first = np.concatenate([[0], np.cumsum(count)])
    etypes = meta["edge_types"][0]
    weak = etypes.index("weak") if "weak" in etypes else -1
    return dict(meta=meta, nodes=nodes, edges=edges, strings=strings, ni=ni, ei=ei, N=N, NF=NF, count=count, first=first, weak=weak,
                self=nodes[:, ni["self_size"]], typ=nodes[:, ni["type"]], name=nodes[:, ni["name"]], to=edges[:, ei["to_node"]] // NF)

def bfs(h):
    N, to, first, count = h["N"], h["to"], h["first"], h["count"]
    etype = h["edges"][:, h["ei"]["type"]]
    parent = np.full(N, -1, dtype=np.int64); pedge = np.full(N, -1, dtype=np.int64); depth = np.full(N, -1, dtype=np.int32)
    parent[0] = 0; depth[0] = 0
    frontier = np.array([0], dtype=np.int64); order = [frontier]; level = 0
    while frontier.size:
        counts = count[frontier]; total = int(counts.sum())
        if total == 0: break
        starts = first[frontier]
        offs = np.repeat(starts - np.concatenate([[0], np.cumsum(counts)[:-1]]), counts)
        idx = offs + np.arange(total)
        src = np.repeat(frontier, counts)
        keep = (etype[idx] != h["weak"]) & (depth[to[idx]] < 0)
        idx, src = idx[keep], src[keep]
        dst = to[idx]
        dst_unique, at = np.unique(dst, return_index=True)
        idx, src, dst = idx[at], src[at], dst_unique
        level += 1
        parent[dst] = src; pedge[dst] = idx; depth[dst] = level
        frontier = dst; order.append(frontier)
    return parent, pedge, depth, order

def edge_label(h, e):
    row = h["edges"][e]; t = h["meta"]["edge_types"][0][row[h["ei"]["type"]]]; v = row[h["ei"]["name_or_index"]]
    return str(h["strings"][v]) if t in ("property", "internal", "context", "shortcut", "variable", "hidden") and v < len(h["strings"]) else "[" + str(v) + "]"

def node_label(h, n):
    t = h["meta"]["node_types"][0][h["typ"][n]]; nm = h["strings"][h["name"][n]]
    return t + ":" + nm[:48].replace("\n", " ")

def subtree(h, parent, order):
    size = h["self"].astype(np.int64).copy()
    for level in reversed(order[1:]):
        np.add.at(size, parent[level], size[level])
    return size

def cmd_tree(path, depth_limit, top):
    h = prepare(path); parent, pedge, depth, order = bfs(h); size = subtree(h, parent, order)
    print("nodes", h["N"], "total MB", round(int(h["self"].sum()) / 1048576), "reachable", int((depth >= 0).sum()))
    cand = np.where((depth >= 1) & (depth <= depth_limit))[0]
    cand = cand[np.argsort(-size[cand])][:top]
    for n in cand:
        chain, v = [], int(n)
        while v != 0 and len(chain) < 12:
            chain.append(edge_label(h, pedge[v])); v = int(parent[v])
        print(round(int(size[n]) / 1048576, 1), "MB", node_label(h, int(n)), "<-", " < ".join(chain))

def classes(h):
    key = h["typ"].astype(np.int64) * (len(h["strings"]) + 1) + h["name"]
    u, inv = np.unique(key, return_inverse=True)
    sizes = np.bincount(inv, weights=h["self"]).astype(np.int64); counts = np.bincount(inv)
    out = {}
    for k, s, c in zip(u, sizes, counts):
        t, nm = divmod(int(k), len(h["strings"]) + 1)
        out[(h["meta"]["node_types"][0][t], h["strings"][nm][:60] if h["meta"]["node_types"][0][t] != "string" else "(string)")] = (out.get((h["meta"]["node_types"][0][t], "(string)" if h["meta"]["node_types"][0][t] == "string" else h["strings"][nm][:60]), (0, 0))[0] + int(s), out.get((h["meta"]["node_types"][0][t], "(string)" if h["meta"]["node_types"][0][t] == "string" else h["strings"][nm][:60]), (0, 0))[1] + int(c))
    return out

def cmd_classes(path, top):
    out = classes(prepare(path))
    for (t, n), (s, c) in sorted(out.items(), key=lambda kv: -kv[1][0])[:top]: print(round(s / 1048576, 1), "MB", c, t, n)

def cmd_diff(a, b, top):
    A, B = classes(prepare(a)), classes(prepare(b))
    rows = [(B.get(k, (0, 0))[0] - A.get(k, (0, 0))[0], B.get(k, (0, 0))[1] - A.get(k, (0, 0))[1], k) for k in set(A) | set(B)]
    for ds, dc, (t, n) in sorted(rows, key=lambda r: -abs(r[0]))[:top]: print(round(ds / 1048576, 1), "MB", dc, t, n)

if __name__ == "__main__":
    c = sys.argv[1]
    if c == "tree": cmd_tree(sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 6, int(sys.argv[4]) if len(sys.argv) > 4 else 40)
    elif c == "classes": cmd_classes(sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 40)
    elif c == "diff": cmd_diff(sys.argv[2], sys.argv[3], int(sys.argv[4]) if len(sys.argv) > 4 else 40)
