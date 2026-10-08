"""🔎 Lists the dashboard ready, tool, compound and axis declarations of every tracked or untracked project manifest (read-only audit helper of slice L-S5)."""
import json, subprocess

paths = subprocess.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*📋️project.json", "📋️project.json"], capture_output=True).stdout.decode("utf8").split("\0")
for path in sorted({path for path in paths if path and "🎫️tickets" not in path}):
    try:
        document = json.load(open(path, encoding="utf8"))
    except Exception as error:
        print("INVALID", path, error)
        continue
    name = document.get("name")
    project = (document.get("metadata") or {}).get("semio", {}).get("dashboard")
    if project:
        for tool in project.get("tools", []):
            print(f"TOOL {name}/{tool['id']} ready={tool.get('ready')} env={tool.get('env')} command={' '.join(tool['command'])[:90]}")
        for compound in project.get("compounds", []):
            print(f"COMPOUND {name}/{compound['id']} members={[member['run'] for member in compound['members']]}")
        for group in project.get("groups", []):
            print(f"GROUP {name}/{group['id']} target={group['target']} projects={len(group['projects'])}")
        if project.get("parameters"):
            print(f"AXES {name}: {[(axis['id'], axis.get('default')) for axis in project['parameters']]}")
    for target, value in (document.get("targets") or {}).items():
        declared = (value.get("metadata") or {}).get("semio", {}).get("dashboard")
        if declared and declared.get("ready"):
            print(f"READY {name}:{target} {declared['ready']} path={path}")
