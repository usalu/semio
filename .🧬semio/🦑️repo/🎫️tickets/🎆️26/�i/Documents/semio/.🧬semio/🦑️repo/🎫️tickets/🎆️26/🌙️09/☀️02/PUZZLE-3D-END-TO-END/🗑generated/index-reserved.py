from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
plugin = None
for p in root.rglob("*.rs"):
    s = str(p)
    if s.endswith("plugin/\U0001f9b4.rs") and "modules" in s and "/os/" in s and "reactor" not in s and "tests" not in s:
        plugin = p
        break
if plugin is None:
    # fallback: walk known parent
    for p in root.iterdir():
        if "framework" in p.name:
            for q in p.rglob("*.rs"):
                s = str(q)
                if "plugin" in s and s.endswith(".rs") and "reactor" not in s and "tests" not in s and q.stat().st_size > 1_000_000:
                    plugin = q
                    break
        if plugin:
            break

print("PLUGIN", plugin)
print("SIZE", plugin.stat().st_size if plugin else 0)
needles = [
    "dispatch_framework_reserved_action",
    "run_framework_reserved_job",
    "commit_framework_history_route",
    "framework_reserved_job!",
    "fn handle_action",
    "fn handle_action_invocation",
    "fn plugin_handle_action",
    "drive_self_waking_ready",
    "admit_addressed_action_view",
    "allocate_operation_id",
    "take_typed_operation_effect",
    "Effect::SpawnJob",
    "ArtifactReservedToolJob",
    "FrameworkUndoJob",
    "FrameworkInteractionSelectJob",
    "fn dispatch_action",
    "async fn handle_action",
]
lines = plugin.read_text().splitlines()
for i, line in enumerate(lines, 1):
    for n in needles:
        if n in line:
            print(f"{i}: {line[:200]}")
            break
print("TOTAL_LINES", len(lines))
