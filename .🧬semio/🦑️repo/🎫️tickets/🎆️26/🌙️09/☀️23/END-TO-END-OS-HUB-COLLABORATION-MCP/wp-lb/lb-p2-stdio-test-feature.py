#!/usr/bin/env python3
"""🧪️ LB-P2 (prepared, lands after W3 announces 7800 on B3 — rule 30): the Codex stdio rollout's integration test
`🧪️tests/✏️editor-catalog` uses `semio_framework_plugin::artifact_app_laws`, which exists only under the SDK's
`artifact-app-testing` feature; the stdio plugin's `[dev-dependencies]` never enables it (measured:
`cargo test -p semio-s-plugin-stdio --test editor_catalog` → E0432, `wp-lb/generated/test-stdio-integration-1.txt`).
Fix: the dev-dependency every other plugin declares (`note`: `features = ["component-guest", "artifact-app-testing"]`).
Usage: lb-p2-stdio-test-feature.py --dry-run | --write"""
import sys
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml")
write = "--write" in sys.argv
if not write and "--dry-run" not in sys.argv:
    sys.exit(__doc__)
source = PATH.read_text(encoding="utf-8")
old = "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\n"
new = "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\nsemio-framework-plugin = { workspace = true, features = [\"artifact-app-testing\"] }\n"
problems = [] if source.count(old) == 1 and "artifact-app-testing" not in source else [f"anchor found {source.count(old)}×, feature present {'artifact-app-testing' in source}"]
print(f"files={0 if problems else 1} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    PATH.write_text(source.replace(old, new), encoding="utf-8")
