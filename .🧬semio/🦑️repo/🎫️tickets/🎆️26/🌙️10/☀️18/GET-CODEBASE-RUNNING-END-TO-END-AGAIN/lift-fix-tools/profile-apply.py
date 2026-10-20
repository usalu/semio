# Staged, not applied: removes the duplicated [profile.*] tables from .cargo/config.toml and aligns the stale A067 assertion with A087.
# usage: python3 profile-apply.py [--write]   (idempotent, anchored; refuses while the anchors drifted)
import sys
R = "/Users/ueli/Documents/semio/"
write = "--write" in sys.argv
L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
edits = [
    (R + ".cargo/config.toml",
     '\n[profile.prototype]\ninherits = "dev"\nopt-level = 0\ndebug = 1\nlto = false\ncodegen-units = 256\nincremental = true\n\n[profile.release]\nopt-level = 3\nlto = "thin"\ncodegen-units = 1\nstrip = "debuginfo"\nincremental = false\n',
     ""),
    (L + "🏃️process/🦀️cargo/🏭️inventory/🧪️tests/🟦️.ts",
     'as {profile:{prototype:{inherits:string}};features?:unknown}',
     'as {profile?:unknown;features?:unknown}'),
    (L + "🏃️process/🦀️cargo/🏭️inventory/🧪️tests/🟦️.ts",
     'expect(manifest.profile.prototype.inherits).toBe("dev");',
     'expect(manifest.profile).toBeUndefined();'),
]
for path, old, new in edits:
    text = open(path).read()
    if old in text:
        print("  occurrences", text.count(old))
        print("WOULD" if not write else "APPLY", path[-60:])
        if write: open(path, "w").write(text.replace(old, new))
    else:
        print("applied-or-drifted", path[-60:])
