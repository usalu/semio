#!/usr/bin/env python3
"""🧾️ Config for a second-level cut: rehosts one family crate (its owner `🦀️.rs` is the "monolith" root) into child crates
rooted at the given sub-owners (each a folder below the crate owner), keeping the crate itself as the aggregator of what is
left. Consumers are the whole plugin, the hub and the fixture sweep.

usage: 🧾️sub-config.py <crate owner dir> <crate package> <out.json> <suffix>=<sub-owner dir relative to the owner>[:<module path>]...
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
owner, package, out = sys.argv[1], sys.argv[2], sys.argv[3]
plugin = "/".join(owner.split("/")[:3])
header = ["#![allow(async_fn_in_trait)]", "", "extern crate semio_framework_os_kernel as dsl;", "extern crate semio_framework_os_kernel as protocol;",
          "extern crate semio_framework_os_kernel as store;", "extern crate semio_framework_schema as framework_schema;", "extern crate semio_framework_value_derive as value_derive;"]
crates = {"agg": {"package": package, "owner": owner, "prefix": "", "description": None}}
rules, create = [], []
for spec in sys.argv[4:]:
    suffix, rest = spec.split("=", 1)
    sub, _, modpath = rest.partition(":")
    crates[suffix] = {"package": f"{package}-{suffix}", "owner": f"{owner}/{sub}", "prefix": modpath or re.sub(r"^[^a-z0-9]*", "", sub.split("/")[0]).replace("-", "_"), "description": None}
    rules.append([re.escape(f"{owner}/{sub}/"), suffix])
    if not os.path.exists(os.path.join(REPO, owner, sub, "🦀️.rs")):
        create.append({"path": f"{owner}/{sub}/🦀️.rs", "text": f"//! {sub.split('/')[-1][:2]} {package.replace('semio-s-artifact-', '')} {suffix.replace('-', ' ')}.\n"})
rules.append([re.escape(owner) + "/", "agg"])
cfg = {"root": f"{owner}/🦀️.rs", "crate": package.replace("-", "_"), "header": header, "crates": crates, "rules": rules, "create": create,
       "consumers": [plugin, "🌎️hub", "✏️s/🧑‍💻dev/🧹️fixture-sweep"], "workspace_root": "/".join(owner.split("/")[:5]) if owner.split("/")[3] == "🗿️artifacts" else None}
json.dump(cfg, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(out)
