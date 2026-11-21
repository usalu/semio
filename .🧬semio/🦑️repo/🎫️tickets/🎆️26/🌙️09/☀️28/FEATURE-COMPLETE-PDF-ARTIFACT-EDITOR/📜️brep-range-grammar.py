"""🧪️ Checks the BREP match-arm token through the pinned repository Rust compiler."""

from pathlib import Path
import subprocess

out = Path(__file__).resolve().parent / "🗑️generated" / "brep-range-grammar"
out.mkdir(parents=True, exist_ok=True)
toolchain = subprocess.check_output(["repo", "tools", "exec", "rustup", "--light", "--", "show", "active-toolchain"], text=True).split()[0]
for name, token, expected in [("rejected", "7..=>", False), ("accepted", "7.. =>", True)]:
    src = out / f"{name}.rs"
    src.write_text(f"fn main() {{ let _ = match 7_u8 {{ {token} 1, _ => 0 }}; }}\n")
    result = subprocess.run(["repo", "tools", "exec", "rustup", "--light", "--", "run", toolchain, "rustc",
                             "--edition=2024", "--emit=metadata", "-o", str(out / f"{name}.rmeta"), str(src)],
                            text=True, capture_output=True)
    assert (result.returncode == 0) is expected, result.stderr
    if not expected:
        assert "unexpected `>` after inclusive range" in result.stderr, result.stderr
    print(f"[DEBUG] {token} {name} by the pinned repository Rust parser")
