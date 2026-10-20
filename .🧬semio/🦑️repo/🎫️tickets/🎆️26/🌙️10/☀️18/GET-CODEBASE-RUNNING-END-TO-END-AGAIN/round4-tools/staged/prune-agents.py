#!/usr/bin/env python3
"""Removes from the merged agents-fix manifest the one file fnsplit4a-finish rewrites wholesale (`rust-non-repo-join-bases/🟦️.ts`: fnsplit moves its body into three new modules, so the agents-fix comment conversion of the old body
has no anchor; the inline `// 🔓️ ...` comments move with the code into `🔶️tokens`, `🔸️resolve-qualified-function`, `🟠️visit` and stay for a follow-up). Run after build.mjs."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
path = HERE + "/agents-fix/staged/manifest.json"
manifest = json.load(open(path, encoding="utf8"))
kept = [row for row in manifest["files"] if not row["path"].endswith("rust-non-repo-join-bases/🟦️.ts")]
assert len(kept) == len(manifest["files"]) - 1
manifest["files"] = kept
json.dump(manifest, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("pruned 1 file; %d left" % len(kept))
