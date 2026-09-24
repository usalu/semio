"""📦️ Prints the cargo package name owning each given Rust source (nearest ancestor `📦️packages/🦀️rust/Cargo.toml` whose lib path reaches it)."""
import os, re, sys
for path in sys.argv[1:]:
    directory = os.path.dirname(os.path.abspath(path))
    while directory != "/":
        manifest = os.path.join(directory, "📦️packages", "🦀️rust", "Cargo.toml")
        if os.path.exists(manifest):
            print(re.search(r'^name\s*=\s*"([^"]+)"', open(manifest, encoding="utf-8").read(), re.M).group(1), path)
            break
        directory = os.path.dirname(directory)
