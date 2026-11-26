#!/usr/bin/env python3
"""⏱️ Verify the actual Rust quiet-slice fixture and an independent SQLite pacing model."""
import json
from pathlib import Path
import re
import sqlite3
import subprocess

root = Path.cwd()
ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/bridge-current"
plugin = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor"
slice_owner = plugin / "🔄️turn/⏱️quiet-slice-turn"
data = json.loads((slice_owner/"🔣️.json").read_text())
db = sqlite3.connect(":memory:")
for case in data["archiveMaintenance"]:
    measured = db.execute("""
      WITH boundary AS (
        SELECT ? AND NOT ? AND NOT ?
          AND ? IS NOT NULL AND ? IS NOT NULL
          AND ? >= ? AND ? - ? < ? AS may_quote
      )
      SELECT CASE WHEN may_quote THEN ? ELSE 0 END,
             CASE WHEN may_quote THEN 1 ELSE 0 END
      FROM boundary
    """, (case["idleIngress"],case["cancelled"],case["yielding"],
          case["currentUs"],case["startedUs"],case["currentUs"],case["startedUs"],
          case["currentUs"],case["startedUs"],data["sliceUs"],case["funded"])).fetchone()
    assert measured == (int(case["open"]),case["queries"]), (case["name"], measured)
pump = data["archivePump"]
cleanup = plugin / "🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs"
limit = int(re.search(r"LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN: usize = (\d+)", cleanup.read_text())[1])
assert limit == pump["limit"]
pacing = db.execute("""
    WITH RECURSIVE per_turn(n,elapsed) AS (
       VALUES(1,?)
       UNION ALL SELECT n+1,elapsed+? FROM per_turn WHERE n < ? AND elapsed < ?
    ), drain(remaining,crossings) AS (
       VALUES(?,0)
       UNION ALL SELECT max(0,remaining-(SELECT max(n) FROM per_turn)),crossings+1
                 FROM drain WHERE remaining > 0
    )
    SELECT max(crossings) FROM drain
""", (pump["unitUs"],pump["unitUs"],limit,data["sliceUs"],pump["calls"])).fetchone()[0]
assert pacing == pump["crossings"]
print(f"[DEBUG] SQLite archive pace: {pump['calls']} pump calls, {pacing} crossings in the fixture", flush=True)

source = output/"archive-slice-law.rs"
predicate = (slice_owner/"🦀️.rs").read_text().split("#[derive(Clone, Copy)]",1)[0]
rust_bool = lambda value: "true" if value else "false"
rust_option = lambda value: f"Some({value})" if value is not None else "None"
cases = [
    f"""    {{
        let mut queries = 0;
        let result = original::archive_maintenance_slice_open(
          {rust_bool(row["idleIngress"])}, {rust_bool(row["cancelled"])}, {rust_bool(row["yielding"])},
          {rust_option(row["startedUs"])}, {rust_option(row["currentUs"])},
          || {{ queries += 1; Ok::<_, ()>({rust_bool(row["funded"])}) }},
        ).unwrap();
        assert_eq!((result, queries), ({rust_bool(row["open"])}, {row["queries"]}), "{{}}", {json.dumps(row["name"])});
    }}"""
    for row in data["archiveMaintenance"]
]
source.write_text("mod original {\n"+predicate+"\n}\nfn main(){\n"+"\n".join(cases)+"""
    assert_eq!(original::archive_maintenance_slice_open(true, false, false, Some(0), Some(1), || Err("quote rejected")), Err("quote rejected"));
    println!("[DEBUG] Eight original Rust archive-slice rows and quote propagation pass");
}
""")
rustc = subprocess.check_output(["rustup","which","rustc"],text=True).strip()
result = subprocess.run([rustc,"--edition=2021","-Zthreads=1",str(source),"-o",str(output/"archive-slice-law")])
if result.returncode:
    raise SystemExit(result.returncode)
subprocess.run([output/"archive-slice-law"],check=True)
