"""🩺 Verifies whether the installed Rust artifacts can resolve their exact transitive peers."""
from pathlib import Path
import json
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
build = Path(sys.argv[1]).resolve()
out = ticket / "🗑️generated/native-wgpu-save/extern-proof"
out.mkdir(exist_ok=True)
rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
artifacts = {file.name[3:-6]: file for file in build.glob("*/*/out/lib*.rmeta")}
cases = {
    "dirs_sys_next-b73d0e214745bc28": 'fn main(){ assert!(dirs_sys_next::home_dir().is_some()); }',
    "gimli-fff57f7217e30a1d": 'fn main(){ let _ = gimli::LittleEndian; }',
}
for key, body in cases.items():
    direct = artifacts[key]
    name = key.rsplit("-",1)[0]
    source = out / (name + ".rs")
    source.write_text(body + "\n", encoding="utf8")
    cmd = [*rustc, "--edition=2021", "--emit=metadata", str(source), "-o", str(source.with_suffix(".rmeta")),
        "--extern", f"{name}={direct}"]
    omitted = subprocess.run(cmd, text=True, capture_output=True)
    assert omitted.returncode != 0 and re.search(r"error\[E046[03]\]", omitted.stderr), omitted.stderr
    deps = json.loads(direct.with_suffix(".rmeta.semio-deps").read_bytes())["dependencies"]
    closure = sorted({artifacts[key].parent for key,_ in deps if key in artifacts})
    fixed = subprocess.run([*cmd, *(f"-Ldependency={p}" for p in closure)], text=True, capture_output=True)
    (out / (name + ".log")).write_text("[DEBUG] omitted transitive metadata paths\n" + omitted.stderr +
        "\n[DEBUG] included exact installed providers\n" + fixed.stdout + fixed.stderr, encoding="utf8")
    assert fixed.returncode == 0, fixed.stderr
    print(f"[DEBUG] {name} reproduces the missing metadata and links with its {len(closure)} exact existing dependency paths")
