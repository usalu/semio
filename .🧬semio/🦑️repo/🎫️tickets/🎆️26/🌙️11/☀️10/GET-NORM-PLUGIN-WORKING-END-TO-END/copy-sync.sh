#!/usr/bin/env bash
# 🪞️ Clones the Rust workspace into a private copy outside the replicated tree and neutralises, only there, debug modules mounted from ticket folders.
set -u
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
COPY="${NORM_COPY:?absolute copy directory outside the repository}"
mkdir -p "$COPY/src"
for e in Cargo.toml Cargo.lock .cargo rust-toolchain.toml rustfmt.toml package.json ✏️s 🧰️framework 🌎️hub 🧪️tests 🧫️fixtures native 🎓️teaching 🏢️semio-tech ♻️mit-bestand 👴️leutwiler artifact-editors-steamdeck-launch-kit layout-plugin-launch-kit; do
  [ -e "$ROOT/$e" ] || continue
  rm -rf "${COPY:?}/src/$e"; cp -Rc "$ROOT/$e" "$COPY/src/$e"
done
find "$ROOT" -maxdepth 1 -type f -size -8M ! -name ".DS_Store" -exec cp -c {} "$COPY/src/" \;
cat > "$COPY/dbgstub.rs" <<'R'
pub fn on() -> bool { false }
pub fn add<A, B, C>(_: String, _: A, _: B, _: C) {}
pub fn strip(name: &str) -> String { name.into() }
pub fn short<N: ?Sized>() -> String { String::new() }
R
grep -rlE --include='*.rs' 'debug-ledger.rs"' "$COPY/src/🧰️framework" "$COPY/src/✏️s" | while read -r f; do
  sed -i '' -E "s#\[path=\"/[^\"]*debug-ledger.rs\"\]#[path=\"$COPY/dbgstub.rs\"]#" "$f"
done
python3 - "$COPY" <<'P'
import re, sys
copy = sys.argv[1]; path = copy + "/src/.cargo/config.toml"
text = open(path, encoding="utf-8").read()
for key in ("rustc", "rustc-wrapper", "build-dir"):
    text = re.sub(rf"(?m)^{key} = .*\n", "", text)
text = re.sub(r"(?m)^target-dir = .*$", f'target-dir = "{copy}/target"', text)
open(path, "w", encoding="utf-8").write(text)
P
date > "$COPY/synced"
