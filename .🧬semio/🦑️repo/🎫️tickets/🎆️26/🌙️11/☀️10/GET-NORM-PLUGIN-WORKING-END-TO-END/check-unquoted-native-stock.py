"""🧾️ Compare the native receiving work and release law with an independent SQLite calculation."""
from pathlib import Path
import json
import sqlite3
import subprocess
import sys

ticket = Path(__file__).resolve().parent
fixture = Path(sys.argv[1]) if len(sys.argv) > 1 else ticket / "receiving-work-release-cases.json"
cases = json.loads(fixture.read_text())
oracle = sqlite3.connect(":memory:")
for case in cases:
    actual = oracle.execute(
        "select min(?1 / 2 + ?1 % 2, 1048576), min(?2 / 2 + ?2 % 2, 67108864)",
        (case["issuedWork"], case["issuedRelease"]),
    ).fetchone()
    assert actual == (case["reservedWork"], case["reservedRelease"]), (case, actual)
print(f"[DEBUG] SQLite verified all {len(cases)} receiving work and release rows")

if len(sys.argv) > 2:
    source = Path(sys.argv[2]).read_text()
    generated = ticket / "🗑️generated"
    source_file = generated / "unquoted-native-stock.rs"
    executable = generated / "unquoted-native-stock"
    start = source.index("const UNQUOTED_RECEIVING_COPY_BYTES")
    policy = source[start:source.index("#[cfg(test)]", start)]
    if "--candidate" in sys.argv[3:]:
        legacy_release = "maximum_release_bytes: whole.maximum_release_bytes.div_ceil(2),"
        assert policy.count(legacy_release) == 1
        policy = policy.replace(
            legacy_release,
            "maximum_release_bytes: whole.maximum_release_bytes.div_ceil(2).min(UNQUOTED_RECEIVING_RELEASE_BYTES),",
        )
        policy += """
const UNQUOTED_RECEIVING_RELEASE_BYTES: usize = 64 << 20;
const UNQUOTED_RECEIVING_WORK: u64 = 1 << 20;
fn unquoted_receiving_work(fuel: u64) -> u64 {
    fuel.div_ceil(2).min(UNQUOTED_RECEIVING_WORK)
}
"""
    if "fn unquoted_receiving_work" not in policy:
        expression = source.split("None=>(unquoted_receiving_grant(whole),", 1)[1].split(")};", 1)[0]
        policy += "fn unquoted_receiving_work(fuel: u64) -> u64 { " + expression.replace("self.fuel_remaining", "fuel") + " }\n"
    rows = ",\n".join(
        f'({row["issuedWork"]}, {row["issuedRelease"]}, {row["reservedWork"]}, {row["reservedRelease"]})'
        for row in cases
    )
    source_file.write_text("""
#[allow(dead_code)]
struct G {
    maximum_items: usize,
    maximum_copy_bytes: usize,
    maximum_capacity_bytes: usize,
    maximum_release_bytes: usize,
    maximum_depth: usize,
}
""" + policy + """
fn main() {
    for (issued_work, issued_release, reserved_work, reserved_release) in [
""" + rows + """
    ] {
        let grant = G {
            maximum_items: 8_388_608, maximum_copy_bytes: 536_870_912,
            maximum_capacity_bytes: 536_870_912, maximum_release_bytes: issued_release,
            maximum_depth: 64,
        };
        assert_eq!(
            (unquoted_receiving_work(issued_work), unquoted_receiving_grant(grant).maximum_release_bytes),
            (reserved_work, reserved_release),
        );
    }
}
""")
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                    "--edition=2021", str(source_file), "-o", str(executable)], check=True)
    subprocess.run([str(executable)], check=True)
    print(f"[DEBUG] Compiled repository policy matched all {len(cases)} rows")
