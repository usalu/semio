import json, sys

T = sys.argv[1]
G = T + "/🗑️generated/1105b"
full = json.load(open(G + "/aud-census-out.json"))
rows = full["rows"]
LIM = {"maxSourceBytes": full["global"]["maxSourceBytes"]}


def dflt_view(r):
    d = r["byFs"].get("default")
    lim = r["limits"]
    if d is None:
        return {}
    ratio = max(d["bytes"] / lim["maxSourceBytes"], d["files"] / lim["maxSourceFiles"], d["lines"] / lim["maxActiveLines"])
    return {"status": "OVER" if ratio > 1 else "NEAR" if ratio >= 0.98 else "ok", "ratio": ratio, "maxBytes": d["bytes"], "maxFiles": d["files"], "maxLines": d["lines"]}


dflt = {r["manifest"]: dflt_view(r) for r in rows}

OWNER = [
    ("🌀️procedural", "proc"),
    ("🔋️energy", "energy"),
    ("🏗️fem", "statik"),
    ("🌊️flow", "flow"),
    ("🖥️host", "hostcheck"),
    ("🧵️job", "runtime"),
    ("🔌️plugin", "runtime"),
    ("🏪️store", "value"),
    ("🌱️value", "value"),
]


def owner(manifest):
    segments = "/" + manifest + "/"
    for needle, lane in OWNER:
        if "/" + needle + "/" in segments:
            return lane
    return "value (unowned)"


def pct(v, l):
    return f"{100.0 * v / l:.1f}%"


def merged(r):
    d = dflt.get(r["manifest"])
    return d


def line(r):
    d = merged(r)
    lim = r["limits"]
    dd = (d or {})
    dstat = dd.get("status", "-")
    dratio = dd.get("ratio", None)
    dtxt = "-" if dratio is None else f"{dd.get('status')} {dratio*100:.1f}% ({dd.get('maxBytes')} B / {dd.get('maxFiles')} f / {dd.get('maxLines')} L)"
    feat = "all-features only" if (dratio is not None and dratio < 1.0 and r["ratio"] >= 1.0) else ""
    return (
        f"| {r['package']} | `{r['manifest']}` | {r['maxBytes']} / {lim['maxSourceBytes']} ({pct(r['maxBytes'], lim['maxSourceBytes'])}) "
        f"| {r['maxFiles']} / {lim['maxSourceFiles']} | {r['maxLines']} / {lim['maxActiveLines']} ({pct(r['maxLines'], lim['maxActiveLines'])}) "
        f"| {r['ratio']*100:.1f}% | {dtxt} | {owner(r['manifest'])} |"
    )


hdr = ("| package | manifest | bytes (worst) | files | active lines | worst ratio | default features only | owner |\n"
       "| --- | --- | --- | --- | --- | --- | --- | --- |")
over = [r for r in rows if r["status"] == "OVER"]
near = [r for r in rows if r["status"] == "NEAR"]
err = [r for r in rows if r["status"] == "error"]
ok = [r for r in rows if r["status"] == "ok"]
band = {"<50%": 0, "50-90%": 0, "90-98%": 0}
for r in ok:
    if r["ratio"] < 0.5:
        band["<50%"] += 1
    elif r["ratio"] < 0.9:
        band["50-90%"] += 1
    else:
        band["90-98%"] += 1

out = []
out.append(f"Measured {len(rows)} crates (git tracked + untracked-not-ignored `Cargo.toml` with `[package]`, excluding `.🧬semio/`), {len(full['vanished'])} vanished during the run.")
out.append("")
out.append(f"- OVER any limit (worst case over feature sets and cfg): **{len(over)}**")
out.append(f"- OVER with default features only: **{sum(1 for r in over if dflt.get(r['manifest'], {}).get('ratio', 0) >= 1.0)}**")
out.append(f"- NEAR (98 % to 100 % of a limit, not over): **{len(near)}**")
out.append(f"- Reader error (gate would refuse or the reader could not decide; see errors): **{len(err)}**")
out.append(f"- ok under 98 %: {len(ok)} (below 50 %: {band['<50%']}, 50 to 90 %: {band['50-90%']}, 90 to 98 %: {band['90-98%']})")
out.append("")
out.append("### OVER (ranked by worst ratio)")
out.append("")
out.append(hdr)
for r in over:
    out.append(line(r))
out.append("")
out.append("### NEAR, within 2 % of a limit (ranked)")
out.append("")
out.append(hdr)
for r in near:
    out.append(line(r))
out.append("")
out.append("### Reader errors (not measured; the crate can still be refused by the gate)")
out.append("")
out.append("| package | manifest | reader message | owner |")
out.append("| --- | --- | --- | --- |")
for r in err:
    out.append(f"| {r['package']} | `{r['manifest']}` | {r['errors'][0] if r['errors'] else ''} | {owner(r['manifest'])} |")
open(G + "/aud-census-table.md", "w").write("\n".join(out) + "\n")

tsv = ["package\tmanifest\tstatus\tratio\tbytes\tbytesLimit\tfiles\tfilesLimit\tlines\tlinesLimit\tworstVariant\tdefaultStatus\tdefaultRatio\tdefaultBytes\tdefaultFiles\tdefaultLines\towner"]
for r in rows:
    d = dflt.get(r["manifest"], {})
    lim = r["limits"]
    tsv.append("\t".join(map(str, [r["package"], r["manifest"], r["status"], f"{r['ratio']:.4f}", r["maxBytes"], lim["maxSourceBytes"], r["maxFiles"], lim["maxSourceFiles"], r["maxLines"], lim["maxActiveLines"], r["maxBytesV"], d.get("status", "-"), f"{d.get('ratio', 0):.4f}" if d else "-", d.get("maxBytes", ""), d.get("maxFiles", ""), d.get("maxLines", ""), owner(r["manifest"])])))
open(G + "/aud-census-all.tsv", "w").write("\n".join(tsv) + "\n")
print(len(over), len(near), len(err), len(ok))
print("\n".join(out[:8]))
