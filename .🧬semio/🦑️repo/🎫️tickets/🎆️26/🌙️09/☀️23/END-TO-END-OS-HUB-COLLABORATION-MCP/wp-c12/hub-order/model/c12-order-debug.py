"""🔎️ C12: temporary [DEBUG] trace of the hub-order seeded property (seed 1) in the OVERLAY only — never in the set.
usage: python3 c12-order-debug.py --apply | --revert   (C12_REPO = overlay root)"""
import os
import sys

REPO = os.environ["C12_REPO"]
LAWS = os.path.join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs")
EDITS = [
    ("""    /// ⚖️ Every replica equals the hub's log: its applied operations, its state (the log's fold), nothing left undecided.
    fn assert_hub_order(""", """    fn debug_state(&self) -> String {
        let short = |ids: &[String]| ids.iter().map(|id| id.chars().take(11).collect::<String>()).collect::<Vec<_>>().join(",");
        self.replicas.iter().enumerate().map(|(i, r)| format!("r{i}{}:[{}] L[{}] in{}", if r.cut { "(cut)" } else { "" }, short(&self.applied_operations(i)), short(r.store.backbone.as_ref().expect("attached").unconfirmed_operations()), r.inbound.len())).collect::<Vec<_>>().join(" | ")
    }

    /// ⚖️ Every replica equals the hub's log: its applied operations, its state (the log's fold), nothing left undecided.
    fn assert_hub_order("""),
    ("""        for _ in 0..property["actions"].as_u64().expect("actions") {""", """        for step in 0..property["actions"].as_u64().expect("actions") {"""),
    ("""                _ => {}
            }
        }
        hub.quiesce().await;""", """                _ => {}
            }
            if seed == 1 {
                eprintln!("[DEBUG] c12 step {step} {action} r{index} log=[{}] {}", hub.log.iter().map(|(id, _)| id.chars().take(11).collect::<String>()).collect::<Vec<_>>().join(","), hub.debug_state());
            }
        }
        hub.quiesce().await;"""),
]
text = open(LAWS, encoding="utf-8").read()
for old, new in EDITS:
    if "--apply" in sys.argv:
        if new not in text:
            assert text.count(old) == 1, old[:60]
            text = text.replace(old, new)
    else:
        if new in text:
            text = text.replace(new, old)
open(LAWS, "w", encoding="utf-8").write(text)
print("debug", "applied" if "--apply" in sys.argv else "reverted", "[DEBUG] lines:", text.count("[DEBUG] c12"))
