"""📦 Archive git-ignored leftovers of the old checkout that the live repo lacks: `python archive-ignored-leftovers.py <old> <zip>`."""
import os, sys, zipfile

old, target = sys.argv[1], sys.argv[2]
roots = [
    ".repo/🎫",
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/.🧬semio",
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/generated",
    "✏️s/🔌️plugins/🎞️animate/📦️packages",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🔷️.cs",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/🤖️generated.py",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/🎭️storybook-interaction",
    "🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs",
    "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/📦️packages/🦀️rust/Cargo.lock",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale",
    "🧰️framework/🛍️products/💻️os/🧫️fixtures",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.lock",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔮️oracles/📦️package-boundary-classification/📦️packages/🦀️rust/Cargo.lock",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity/📦️packages/🐹️go/🤖️generated.go",
]
skipped = {"node_modules", "__pycache__"}
count = 0
with zipfile.ZipFile(target, "w", zipfile.ZIP_DEFLATED, allowZip64=True) as archive:
    for root in roots:
        path = os.path.join(old, root)
        if os.path.isfile(path):
            archive.write(path, root)
            count += 1
            continue
        for folder, names, files in os.walk(path):
            names[:] = [name for name in names if name not in skipped]
            for file in files:
                if file.endswith((".exe", ".pdb", ".pyc")):
                    continue
                full = os.path.join(folder, file)
                archive.write(full, os.path.relpath(full, old).replace(os.sep, "/"))
                count += 1
print(count, os.path.getsize(target))
