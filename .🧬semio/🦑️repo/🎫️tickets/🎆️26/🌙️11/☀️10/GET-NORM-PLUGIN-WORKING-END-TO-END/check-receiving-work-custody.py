"""🧮 Compare the Energy Source Work witness against an independent SQLite calculation."""
import json
from pathlib import Path
import sqlite3
import subprocess

root = Path.cwd()
ticket = Path(__file__).resolve().parent
case = json.loads((ticket / "receiving-work-custody.json").read_text())
legacy_held = case["activeRowWork"] - case["redeemedRowWork"] + case["otherHeldWork"]
quote_input = 2 * (case["activeRowWork"] - case["rowControlWork"])
expected = (
    case["expectedLegacyFreeWork"],
    case["expectedLegacyOverrun"],
    case["expectedBoundedFreeWork"],
)
with sqlite3.connect(":memory:") as oracle:
    actual = oracle.execute(
        """with work as (
            select ?1 - ?2 - ?3 as legacy_free,
                   ?1 - ?2 - (min(?4 / 2 + ?4 % 2, ?5) + ?6 - ?7 + ?8) as bounded_free
        ) select legacy_free, ?9 - legacy_free, bounded_free from work""",
        (case["issuedWork"], case["spentBeforeMember"], legacy_held, quote_input,
         case["expectedBoundedQuote"], case["rowControlWork"], case["redeemedRowWork"],
         case["otherHeldWork"], case["freshMemberWork"]),
    ).fetchone()
assert actual == expected, actual
print(f"[DEBUG] SQLite Work custody {actual}", flush=True)

framework = root / "🧰️framework/🔨️modules/🧵️job"
policy = (framework / "⏱️context/🛬️owned/🦀️.rs").read_text()
physical = (root / "🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs").read_text()
ledger = (framework / "📬️outcome/📦️builder/🪙️return-escrow/🦀️.rs").read_text()
plugin = (root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/🏗️steps/🦀️.rs").read_text()
policy = policy[policy.index("const UNQUOTED_RECEIVING_WORK"):].split("fn unquoted_receiving_grant", 1)[0] + policy[policy.index("fn unquoted_receiving_work"):].split("#[cfg(test)]", 1)[0]
ledger = ledger[ledger.index("#[derive(Clone,Copy,Debug,PartialEq)]"):].split("/// 📋️ Data-only claim arithmetic", 1)[0]
physical = physical[physical.index("#[derive(Clone, Copy, Debug, Default"):].split("pub const RETAINED_CLONE_GRANT_SCHEMA", 1)[0] + physical[physical.index("/// ⚠️ Refusal identity"):].split("pub struct RetirementDemand", 1)[0].rsplit("#[derive", 1)[0]
physical = "\n".join(line.replace(", serde::Serialize, serde::Deserialize", "") for line in physical.splitlines() if not line.startswith("#[serde(")) + "\n"
source_wrapper = plugin[plugin.index("pub fn receiving_units("):].split("\n", 1)[0]
patch = (ticket / "receiving-work-custody.patch").read_text().splitlines()
legacy_wrapper = next(line[1:] for line in patch if line.startswith("-pub fn receiving_units("))
patched_wrapper = next(line[1:] for line in patch if line.startswith("+pub fn receiving_units("))
assert source_wrapper in (legacy_wrapper, patched_wrapper)
implementation = "production" if source_wrapper == patched_wrapper else "candidate"
candidate_wrapper = patched_wrapper.replace("semio_framework_job::", "")
generated = ticket / "🗑️generated"
rust = generated / "receiving-work-custody.rs"
exe = generated / "receiving-work-custody"
for mode, wrapper in (("legacy", legacy_wrapper), (implementation, candidate_wrapper)):
    rust.write_text("""
#![allow(dead_code)]
""" + physical + policy + ledger + wrapper + f"""
fn main() {{
    let mut snapshot = OriginalReturnBudgetSnapshot {{
        granted: RetainedCloneGrant::default(), fuel: {case['issuedWork']},
        spent: RetainedCloneProgress::default(), work: {case['spentBeforeMember']},
        held_items: 0, held_copy_bytes: 0, held_release_bytes: 0, held_capacity_bytes: 0,
        held_fuel: receiving_units({quote_input}) + {case['rowControlWork']} - {case['redeemedRowWork']} + {case['otherHeldWork']}, claims: 2, serial: 1,
    }};
    let (_, free) = snapshot.remaining().unwrap();
    let result = snapshot.advance_work_exact({case['freshMemberWork']});
    let followup = snapshot.advance_retained(RetainedCloneProgress::default());
    println!("{{free}} {{result:?}} {{followup:?}}");
}}
""")
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024",
                    str(rust), "-o", str(exe)], check=True)
    actual = subprocess.check_output([str(exe)], text=True).strip()
    expected_output = (f"{expected[0]} Some(WorkExceeded) Some(OwnershipExceeded)" if mode == "legacy"
                       else f"{expected[2]} None None")
    assert actual == expected_output, (mode, actual, expected_output)
    print(f"[DEBUG] Rust {mode} {actual}", flush=True)
