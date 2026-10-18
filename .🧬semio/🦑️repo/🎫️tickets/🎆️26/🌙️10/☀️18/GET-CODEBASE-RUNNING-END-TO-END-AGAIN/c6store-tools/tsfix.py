"""📝 C6STORE source-text reader table: every TS law that reads a split Rust file as text is re-pointed to the module tree (tsrepoint.py). usage: tsfix.py [--apply]"""
import sys
sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from tsrepoint import repoint
S = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/"
OS = "🧰️framework/🛍️products/💻️os/"
STR = "🏪️store/🦀️.rs"
TABLE = {
    "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️retirement/🧪️tests/🟦️.ts": [(26, STR)],
    "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🟦️.test.ts": [(220, "🌈️bin-03/🔬️unit/🦀️.rs")],
    "🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🗄️durable-three-store-assembly/🟦️.ts": [(38, "🗄️durable-group/🦀️.rs")],
    "🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts": [(599, "🔄️sync/🦀️.rs")],
    "🌎️hub/🧪️tests/🔗️execution-target-lease-checks/🟦️.ts": [(411, "🔄️sync/🦀️.rs")],
    "🌎️hub/🧪️tests/🪪️check-in-process/🟦️.ts": [(382, STR)],
    "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🧪️tests/🟦️.ts": [(42, "storePath")],
    "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/📝️drafts/🧪️tests/🟦️.ts": [(14, "🧱️core/📥️lifted/🏪️store/🦀️.rs")],
    "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/🟦️.ts": [(15, STR), (26, STR)],
    "🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts": [(28, "🧬️owned-schema/🦀️.rs")],
    OS + "📦️packages/🟦️typescript/📜️script.ts": [(163, "🔄️sync/🦀️.rs")],
    OS + "📦️packages/🦀️rust/🎛️commands/🧬️retained-clone-check/🟦️.ts": [(196, "🧱️core/📥️lifted/🏪️store/🦀️.rs")],
    S + "♻️retirement/📦️backing/🧪️tests/🟦️.ts": [(12, "🎚️config/📥️retained/🦀️.rs")],
    S + "🔁️replay/🛑️cancel/🧪️tests/🟦️.ts": [(22, '"../../🦀️.rs"')],
    S + "🔄️sync/🪪️actor-identity/🧪️tests/🟦️.ts": [(12, '"../../🦀️.rs"'), (12, "🖥️native/🎭️actor/🦀️.rs")],
    S + "🔗️backbone/♻️retirement/🧪️tests/🟦️.ts": [(41, "⚙️engine/📥️lifted/🏪️store/🦀️.rs")],
    S + "🕰️history/📐️planning/🎟️admission/🧪️tests/🟦️.ts": [(15, "🧱️core/📥️lifted/🏪️store/🦀️.rs"), (23, "🧱️core/📥️lifted/🏪️store/🦀️.rs"), (30, "🧱️core/📥️lifted/🏪️store/🦀️.rs")],
    S + "🕰️history/📐️planning/🧪️tests/🟦️.ts": [(12, "🧱️core/📥️lifted/🏪️store/🦀️.rs"), (12, "🔗️read/🦀️.rs")],
    S + "🚪️io/🚫️refusal/📬️outcome/🧪️tests/🟦️.ts": [(2, '"../../../../🦀️.rs"')],
    S + "🧩️composition/📨️emission/📦️owned/🌱️birth/🧪️tests/🟦️.ts": [(13, "👥️space-member/📥️lifted/🏪️store/🦀️.rs")],
    S + "🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts": [(245, "🫧️ephemeral/🦀️.rs"), (266, "🔗️read/🦀️.rs"), (376, "🧬️owned-schema/🦀️.rs")],
    S + "🧩️composition/📬️publication/🤝️group/🧪️tests/🔬️unit/🟦️.ts": [(35, '"../../../../../🦀️.rs"'), (53, '"../../../../../🦀️.rs"'), (73, '"../../../../../🦀️.rs"'), (83, '"../../../../../🦀️.rs"')],
    S + "🧩️composition/🚪️open/🏭️operation/📏️birth/🧪️tests/🔬️unit/🟦️.ts": [(19, '"../../../../../../🦀️.rs"')],
    S + "🧩️composition/🗄️durable-group/🧪️tests/♻️journal/🟦️.ts": [(21, "🗄️durable-group/🦀️.rs"), (22, '"../🔬️unit/🦀️.rs"')],
    S + "🧪️tests/🧪️supersede-replay/🟦️.ts": [(92, '"./🦀️.rs"')],
    S + "🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts": [],
    OS + "🔨️modules/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts": [(150, STR)],
    OS + "🖥️host/📦️packages/🦀️rust/🎛️commands/🏭️member-factory-identity/🟦️.ts": [(143, STR)],
    OS + "🖥️host/📦️packages/🦀️rust/🎛️commands/🚪️member-open/🟦️.ts": [(154, "👥️space-member/📥️lifted/🏪️store/🦀️.rs")],
}
apply = "--apply" in sys.argv
bad = 0
for f, sites in TABLE.items():
    if not sites: continue
    for m in repoint(f, sites, apply):
        print(m)
        if m.startswith("MISS"): bad += 1
print("MISS total", bad, "applied" if apply and not bad else "dry run")
