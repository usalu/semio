from pathlib import Path

root = Path("✏️s/🔌️plugins/🔱️trinity")
op = root / "🗿️artifacts" / "🔌️jack" / "🔧️op" / "🦀️component.rs"
print("OP EXISTS", op.exists())
if op.exists():
    t = op.read_text()
    print(t[:3500])
    print("====LEN", len(t))

patterns = [
    "projection",
    "GraphFixture",
    "RewriteRuleDocument",
    "RewriteRuleModel",
    "TrinityGraphDocument",
    "jack::op::",
    "RewriteRuleDiff",
    "TrinityGraphDiff",
    "Self::initial_snapshot",
    "self.initial_snapshot",
    "SetState",
    "CollectionDiff",
    "apply_trinity_graph",
    "TrinityGraphStore",
]
for pat in patterns:
    hits = []
    for p in root.rglob("*.rs"):
        try:
            t = p.read_text()
        except Exception:
            continue
        if pat not in t:
            continue
        for i, line in enumerate(t.splitlines(), 1):
            if pat in line:
                hits.append(f"{p}:{i}:{line.strip()[:140]}")
    print(f"\n### {pat} ({len(hits)})")
    for h in hits[:60]:
        print(h)
