# 🔬 Anchored text edit of taxonomy.json adding fixed filename contracts for workspace-root tool files that sit outside the repository root and outside a package root. Idempotent; aborts when an anchor is missing or not unique.
import sys
T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
s = open(T, encoding="utf8").read()
if '"workspace-package-manifest"' in s:
    sys.exit("already applied")
anchor = '''    "root-bun-config": {'''
if s.count(anchor) != 1:
    sys.exit(f"anchor count {s.count(anchor)}")
def contract(id_, pattern, authority, reason, verification):
    return f'''    "{id_}": {{
      "pathPattern": "{pattern}",
      "authority": "{authority}",
      "reason": "{reason}",
      "configurability": "unconfigurable",
      "scope": {{
        "kind": "path-pattern"
      }},
      "verification": "{verification}",
      "expires": null
    }},
'''
rows = [
    ("workspace-package-manifest", "*/package.json", "Node.js", "Bun workspace manifest of a top-level product workspace that is not the repository root and not a language package root", "bun pm"),
    ("workspace-bun-lock", "*/bun.lock", "Bun", "Dependency lock of a top-level product workspace that is not the repository root", "bun install --frozen-lockfile"),
    ("workspace-typescript-config", "*/tsconfig.json", "TypeScript", "Compiler configuration of a top-level product workspace", "tsc --showConfig"),
    ("workspace-cargo-manifest", "**/Cargo.toml", "cargo", "Cargo workspace manifest of a workspace root that is not the repository root and not a package root", "cargo metadata"),
    ("workspace-cargo-lock", "**/Cargo.lock", "Cargo", "Cargo dependency lock of a workspace root that is not the repository root and not a package root", "cargo metadata"),
    ("cargo-nextest-config", "**/.config/nextest.toml", "cargo-nextest", "cargo-nextest profile discovery below its workspace root", "cargo nextest show-config"),
]
s = s.replace(anchor, "".join(contract(*r) for r in rows) + anchor)
open(T, "w", encoding="utf8").write(s)
print("ok")
