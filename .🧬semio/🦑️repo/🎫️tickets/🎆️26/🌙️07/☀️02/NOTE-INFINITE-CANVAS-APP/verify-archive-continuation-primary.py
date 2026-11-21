"""📬 Checks the production archive continuation projection against a portable SQLite witness."""
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import tempfile

ticket = Path(__file__).parent
base = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document")
fixture = json.loads((ticket / "note-archive-continuation-primary.json").read_text(encoding="utf-8"))
with sqlite3.connect(":memory:") as db:
    for row in fixture:
        got, = db.execute("SELECT CAST(substr(CAST(COALESCE(?, 'initializer-continuation-without-detail') AS BLOB), 1, 96) AS TEXT)", (row["message"],)).fetchone()
        assert got == row["expected"], row

owner = (base / "🧰️store-replacement-initializer-and-retirement/🦀️.rs").read_text(encoding="utf-8")
start = owner.index("let detail = cause.map_or")
end = owner.index("StoreInitializerOutcome::Continuation(", start)
projection = owner[start:end]
owner = (base / "🧾️store-replacement-vocabulary/🦀️.rs").read_text(encoding="utf-8")
start = owner.index("struct ArtifactStoreRefusalCode")
end = owner.index("pub(super) enum StoreInitializerOutcome", start)
bounded_refusal = owner[start:end].replace("pub(super) fn", "fn")
native = f"""
struct OriginalTurnFailure {{ primary: semio_framework_job::OriginalTurnPrimary }}
mod semio_framework_job {{
    pub struct Body {{ pub message: &'static str }}
    pub enum OriginalTurnPrimary {{ Value(Body), Fault(Body), Retirement(Body), NativeReceiving(Body), SourceClose(Body) }}
    impl OriginalTurnPrimary {{
        pub fn message(&self) -> &str {{
            match self {{
                Self::Value(body) | Self::Fault(body) | Self::Retirement(body) | Self::NativeReceiving(body) | Self::SourceClose(body) => body.message,
            }}
        }}
    }}
}}
{bounded_refusal}
fn project(cause: Option<&OriginalTurnFailure>) -> ArtifactStoreRefusalCode {{
    {projection}
    ArtifactStoreRefusalCode::from_page(detail)
}}
fn main() {{
"""
for row in fixture:
    if row["variant"] is None:
        cause = "None"
    else:
        variant, message = row["variant"], json.dumps(row["message"], ensure_ascii=False)
        cause = f"Some(&OriginalTurnFailure {{ primary: semio_framework_job::OriginalTurnPrimary::{variant}(semio_framework_job::Body {{ message: {message} }}) }})"
    native += f"assert_eq!(project({cause}).text(), {json.dumps(row['expected'], ensure_ascii=False)});\n"
native += f'println!("[DEBUG] archive continuation projected {len(fixture)} original cases");\n}}'
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as directory:
    binary = Path(directory) / ("continuation.exe" if os.name == "nt" else "continuation")
    subprocess.run(["repo", "tools", "exec", "--light", "rustc", "--", "--edition=2021", "-Awarnings", "-o", str(binary), "-"], input=native, text=True, check=True)
    subprocess.run(["repo", "capacity", "run", "--memory=64", "--", str(binary)], timeout=45, check=True)
