"""🦀️ Runs `cargo test` of one crate in a MIRROR with a private target and build directory: the mirror's workspace keeps only that crate as member (its path dependencies join on their own), the plain rustc runs (no repo wrapper). usage: python3 -I cargotest.py <mirror-root> <crate-dir-rel> [cargo test args...]"""
import os, re, shutil, subprocess, sys
mirror, crate, rest = os.path.abspath(sys.argv[1]), sys.argv[2], sys.argv[3:]
assert "/sm/" in mirror or "/tmp/m/" in mirror, "a mirror root is required"
manifest = os.path.join(mirror, "Cargo.toml")
text = open(manifest, encoding="utf8").read()
start = text.index("members = [")
end = text.index("\n]", start)
rows = re.findall(r'"([^"]+)"', text[start:end])
kept = [crate]
open(manifest, "w", encoding="utf8").write(text[:start] + "members = [\n" + "".join(f'    "{row}",\n' for row in kept) + text[end + 1:].lstrip("\n").join(["", ""]) if False else text[:start] + "members = [\n" + "".join(f'    "{row}",\n' for row in kept) + "]" + text[end + 2:])
private = os.path.join(mirror, ".tmp", "lp-cargo")
env = dict(os.environ, CARGO_TARGET_DIR=os.path.join(private, "t"), CARGO_BUILD_BUILD_DIR=os.path.join(private, "b"), CARGO_BUILD_RUSTC="rustc", CARGO_BUILD_RUSTC_WRAPPER="", RUSTC_WRAPPER="")
print(f"[DEBUG] members kept {len(kept)} of {len(rows)}", flush=True)
done = subprocess.run(["cargo", "test", "--manifest-path", os.path.join(mirror, crate, "Cargo.toml"), *rest], cwd=mirror, env=env)
sys.exit(done.returncode)
