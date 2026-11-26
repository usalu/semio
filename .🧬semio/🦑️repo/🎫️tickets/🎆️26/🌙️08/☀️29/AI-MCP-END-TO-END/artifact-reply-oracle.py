"""📨 Independently check typed artifact refusals over both original reply channels."""
import json
from pathlib import Path
import sqlite3

fixture = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/artifact-reply.json")
with sqlite3.connect(":memory:") as db:
    rows = db.execute("""
        with modes(framed) as (values (false), (true)),
        codes(fault, code, retryable) as (values
            ('budget.exceeded', 'BUDGET_EXCEEDED', true),
            ('job.cancelled', 'CANCELLED', false),
            ('plugin.unavailable', 'PLUGIN_UNAVAILABLE', true),
            ('app.command.rejected', 'SIDE_EFFECT_REJECTED', false),
            ('input.invalid', 'INPUT_INVALID', false))
        select framed, c.code, c.retryable, j.value
        from json_each(?) j cross join modes
        left join codes c on c.fault = json_extract(j.value, '$.fault')
    """, (fixture.read_text(),)).fetchall()
assert len(rows) == 10
for framed, code, retryable, source in rows:
    row = json.loads(source)
    assert (code, bool(retryable)) == (row["code"], row["retryable"]), (framed, row)
print(f"{len(rows)} artifact refusal routes matched")
