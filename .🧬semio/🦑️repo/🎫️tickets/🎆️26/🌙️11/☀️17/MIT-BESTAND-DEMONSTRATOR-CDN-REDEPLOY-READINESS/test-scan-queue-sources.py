"""🧪️ Compare reader results with independent fixture byte, file and Rust line counts."""
import json
from pathlib import Path
import subprocess
import tempfile

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
FIXTURES = json.loads((TICKET / "🔣️roster-source-cases.json").read_text())

with tempfile.TemporaryDirectory(prefix="source-law-", dir=TICKET / "🗑️generated") as directory:
    fixture = Path(directory)
    units = []
    for row in FIXTURES:
        case = fixture / row["name"]
        case.mkdir()
        manifest = f'[package]\nname = "{row["name"]}"\n'
        if "override" in row:
            manifest += "[package.metadata.semio.rust-source-boundary]\n" + "".join(f"{key} = {value}\n" for key, value in row["override"].items())
        (case / "Cargo.toml").write_text(manifest)
        imports = [f"mod leaf{i};\n" for i in range(row["leaves"])]
        source = ("".join(imports) + "".join(f"fn item_{index}() {{}}\n" for index in range(row["lines"] - len(imports)))).encode()
        content = source[:-1].ljust(row["bytes"], b" ")
        assert len(content) == row["bytes"]
        entry = case / "root.rs"
        entry.write_bytes(content)
        for index in range(row["leaves"]):
            (case / f"leaf{index}.rs").write_bytes(b"")
        assert sum(bool(line.strip()) for line in content.decode().splitlines()) == row["lines"]
        units.append({"pkg_id": f"path+{case.as_uri()}#{row['name']}@0.1.0", "target": {"name": row["name"], "src_path": str(entry), "kind": ["lib"], "edition": "2024"}, "mode": "build", "profile": {"name": "release", "opt_level": "3", "debug_assertions": False, "panic": "unwind"}, "features": [], "platform": None})
    graph = fixture / "fixture.graph.json"
    graph.write_text(json.dumps({"units":units}))
    result = subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", str(TICKET / "scan-queue-sources.ts"), str(graph)], cwd=ROOT, encoding="utf-8", capture_output=True)
    report = json.loads(result.stdout)
    assert report["checked"] == len(FIXTURES), (report, result.stderr)
    expected = {row["name"]: row["refuses"] for row in FIXTURES if row["refuses"]}
    actual = {row["crate"]: row["limitsExceeded"] for row in report["refused"]}
    assert actual == expected, (actual, expected, report)
    assert result.returncode == 1, (result.returncode, result.stderr)
print("Source preflight exercised files, bytes, active lines, exact boundaries and a crate override")
