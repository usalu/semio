import sys
root = sys.argv[1]
base = root + "/🧰️framework"
files = [
 "/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/📦️packages/🦀️rust/Cargo.toml",
 "/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🦀️.rs",
 "/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/📦️packages/🦀️rust/📋️project.json",
 "/🔨️modules/📡️replication/🎮️mutation/🧳️retained/📦️packages/🦀️rust/📋️project.json",
 "/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json",
 "/🔨️modules/📡️replication/🎮️mutation/🧳️retained/📦️packages/🦀️rust/Cargo.toml",
 "/🔨️modules/📡️replication/🧪️tests/🧫️fixtures/🏛️crate-partition/🔣️.json",
 "/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml",
 "/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs",
]
pairs = [("replication-mutation-retained", "replication-retained"), ("replication_mutation_retained", "replication_retained")]
total = 0
for name in files:
    text = open(base + name).read()
    count = sum(text.count(old) for old, _ in pairs)
    assert count > 0, name
    for old, new in pairs:
        text = text.replace(old, new)
    open(base + name, "w").write(text)
    total += count
print("renamed", total, "occurrences in", len(files), "files")
