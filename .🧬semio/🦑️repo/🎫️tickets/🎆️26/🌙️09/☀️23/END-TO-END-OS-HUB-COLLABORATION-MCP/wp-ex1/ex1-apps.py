#!/usr/bin/env python3
"""🗺️ EX1: per artifact subset — published example ids (📚️examples leaves), ArtifactEditor/ArtifactApp impls with their examples()."""
import os, re, sys, json
R = "/Users/ueli/Documents/semio/✏️s/🔌️plugins"
out = []
for plugin in sorted(os.listdir(R)):
    if plugin.startswith(("🗄️", ".")) or not os.path.isdir(os.path.join(R, plugin)) or (len(sys.argv) > 1 and plugin not in sys.argv[1:]): continue
    for dp, dns, fns in os.walk(os.path.join(R, plugin)):
        dns[:] = [d for d in dns if d not in ("🧪️tests", "target", "node_modules")]
        if "🦀️.rs" not in fns: continue
        p = os.path.join(dp, "🦀️.rs"); t = open(p, encoding="utf-8").read()
        rel = os.path.relpath(p, R)
        short = re.sub(r"🏅️standards/🔖️[^/]+/🪆️subsets/", "", rel).replace("🗿️artifacts/", "").replace("/🦀️.rs", "")
        if "/📚️examples/" in rel:
            m = re.search(r'pub const ID: &str = "([^"]+)"', t)
            if m: out.append(f"EX  {short}  id={m.group(1)}")
        for m in re.finditer(r"impl(?:<[^>]*>)? (ArtifactEditor|ArtifactViewer|ArtifactApp|semio_framework_plugin::ArtifactEditor|semio_framework_plugin::ArtifactApp) for (\w+)", t):
            ex = re.search(r"fn examples\(\) -> Vec<[\w:]*ExampleSource> \{\s*(.*?)\s*\}\n", t[m.end():m.end()+20000], re.S)
            out.append(f"IMPL {short}  {m.group(1).split('::')[-1]} for {m.group(2)}  examples={' '.join(ex.group(1).split())[:160] if ex else '-'}")
print("\n".join(out))
