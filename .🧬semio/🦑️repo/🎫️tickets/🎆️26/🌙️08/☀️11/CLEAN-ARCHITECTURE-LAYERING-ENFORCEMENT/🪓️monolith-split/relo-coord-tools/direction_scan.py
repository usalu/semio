"""🧭️ Coordinator direction scan for the wrong-direction relocation: upward Cargo path dependencies, upward `#[path]`/`include*` targets, and residual domain vocabulary in the relocated framework/os areas.

Usage: python3 direction_scan.py  (prints a report; exit 0 always)
"""
import os, re, subprocess, sys

REPO = "/Users/ueli/Documents/semio"
LAYERS = [("framework", "🧰️framework/🔨️modules"), ("framework", "🧰️framework/📦️packages"), ("os", "🧰️framework/🛍️products/💻️os"), ("plugin", "✏️s"), ("product", "🌎️hub"), ("product", "♻️mit-bestand"), ("product", "🏢️semio-tech")]
RANK = {"framework": 0, "os": 1, "plugin": 2, "product": 3}


def layer(rel):
    for name, prefix in LAYERS:
        if rel == prefix or rel.startswith(prefix + "/"):
            return name
    return None


def tracked(patterns):
    out = subprocess.run(["git", "--no-optional-locks", "ls-files", "-z", "--", *patterns], cwd=REPO, capture_output=True).stdout.decode()
    return [p for p in out.split("\0") if p]


def upward(src_rel, target_abs):
    tgt_rel = os.path.relpath(os.path.normpath(target_abs), REPO)
    a, b = layer(src_rel), layer(tgt_rel)
    return a is not None and b is not None and RANK[b] > RANK[a], tgt_rel


def cargo_edges():
    hits = []
    for rel in tracked(["*Cargo.toml"]) + [p for p in untracked() if p.endswith("Cargo.toml")]:
        try:
            text = open(os.path.join(REPO, rel), encoding="utf-8").read()
        except OSError:
            continue
        for m in re.finditer(r'path\s*=\s*"([^"]+)"', text):
            up, tgt = upward(rel, os.path.join(REPO, os.path.dirname(rel), m.group(1)))
            if up:
                hits.append(f"{rel} -> {tgt}")
    return hits


def untracked():
    out = subprocess.run(["git", "--no-optional-locks", "ls-files", "-z", "--others", "--exclude-standard", "--", "🧰️framework", "✏️s", "🌎️hub"], cwd=REPO, capture_output=True).stdout.decode()
    return [p for p in out.split("\0") if p]


def source_edges():
    hits = []
    files = [p for p in tracked(["🧰️framework/*.rs", "✏️s/*.rs"]) + untracked() if p.endswith(".rs")]
    pat = re.compile(r'(?:#\[path\s*=\s*|include(?:_str|_bytes)?!\s*\(\s*)"([^"]+)"')
    for rel in files:
        try:
            text = open(os.path.join(REPO, rel), encoding="utf-8").read()
        except OSError:
            continue
        for m in pat.finditer(text):
            up, tgt = upward(rel, os.path.join(REPO, os.path.dirname(rel), m.group(1)))
            if up:
                hits.append(f"{rel} -> {tgt}")
    return hits


VOCAB = [
    ("framework manifest names product vocabulary", "🧰️framework/🔨️modules/🛂️manifest", r"product_vocabulary|GUMBALL_CHROME|setTransformGumballFlag|HISTORY_EDIT_[A-Z_]+\s*:\s*&str|pub use semio_framework_tool_run"),
    ("framework tiled-map names GIS", "🧰️framework/🔨️modules/🗺️surface", r"(?i)\bgis\b|gis_map|gismap|openmaptiles|openfreemap|GIS_MAP"),
    ("hub inference runtime names GIS plugin crates", "🌎️hub/💡️inference", r"semio_s_artifact_gis|gis-map-binding|gis_map_binding"),
    ("os infinite names puzzle 2d", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite", r"(?i)puzzle.?2d"),
    ("plugin SDK instance names tool-run/time-travel domains", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance", r"tool_run_contract|tool_jobs::\{?(time_travel|tool_run)|is_time_travel_action_id|is_tool_run_action_id|ui_wgpu"),
    ("plugin host names os_vcs / ui_wgpu", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host", r"os_kernel::os_vcs|pub use semio_framework_os_kernel::os_vcs|ui_wgpu"),
]


def vocab():
    rows = []
    for title, root, rx in VOCAB:
        r = subprocess.run(["/usr/bin/grep", "-rnE", "--include=*.rs", "--include=*.ts", "--include=*.tsx", "--include=*.toml", rx.replace("(?i)", ""), root] + (["-i"] if rx.startswith("(?i)") else []), cwd=REPO, capture_output=True, text=True)
        lines = [l for l in r.stdout.splitlines() if "/node_modules/" not in l]
        rows.append((title, root, lines))
    return rows


def plugin_cargo_ui_wgpu():
    out = []
    for rel in ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml"]:
        p = os.path.join(REPO, rel)
        if os.path.exists(p):
            out += [f"{rel}: {l.strip()}" for l in open(p, encoding="utf-8") if re.match(r"\s*ui_wgpu\b|\s*semio-framework-ui-wgpu|.*ui-wgpu", l)]
    return out


def hub_cargo_gis():
    p = os.path.join(REPO, "🌎️hub/Cargo.toml")
    text = open(p, encoding="utf-8").read()
    deps = re.search(r"^\[dependencies\](.*?)(^\[|\Z)", text, re.S | re.M)
    return [l.strip() for l in (deps.group(1) if deps else "").splitlines() if re.search(r"gis|stdio-semio-(drawing|value)", l)]


if __name__ == "__main__":
    c, s = cargo_edges(), source_edges()
    print(f"## Upward Cargo path dependencies: {len(c)}")
    print("\n".join(c))
    print(f"## Upward #[path]/include* edges: {len(s)}")
    print("\n".join(s))
    for title, root, lines in vocab():
        print(f"## {title} ({root}): {len(lines)}")
        print("\n".join(l[:240] for l in lines[:40]))
    print("## ui_wgpu in plugin / plugin-host Cargo.toml:", plugin_cargo_ui_wgpu())
    print("## hub [dependencies] GIS plugin crates:", hub_cargo_gis())
