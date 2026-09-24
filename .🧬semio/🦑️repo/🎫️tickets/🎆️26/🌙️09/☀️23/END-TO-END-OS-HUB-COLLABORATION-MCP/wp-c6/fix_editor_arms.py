from pathlib import Path

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))
editors = [p for p in space.rglob("🦀️.rs") if str(p).endswith("✳️any/✏️editor/🦀️.rs") and "🏠️home" in str(p)]
et = editors[0].read_text()

# Fix home_retained_extent - add promote/persist with CreateSpace-like sizing, and add to catch-all if any
if "HomeCommand::PromoteToHubSpace" not in et:
    et = et.replace(
        "HomeCommand::CopyInviteLink(payload) => (payload.space_id.len().saturating_add(payload.role.len()), HOME_RETAINED_SCALAR_BYTES),",
        "HomeCommand::CopyInviteLink(payload) => (payload.space_id.len().saturating_add(payload.role.len()), HOME_RETAINED_SCALAR_BYTES),\n"
        "        HomeCommand::PromoteToHubSpace(payload) => (payload.space_id.len().saturating_add(payload.name.len()), HOME_RETAINED_SCALAR_BYTES),\n"
        "        HomeCommand::PersistLocally(payload) => (payload.space_id.len().saturating_add(payload.folder_path.as_ref().map_or(0, String::len)), HOME_RETAINED_SCALAR_BYTES),",
        1,
    )
    print("extent arms added")

# Find other exhaustive matches on HomeCommand
import re
for m in re.finditer(r"match command \{", et):
    start = m.start()
    snip = et[start:start+800]
    if "PromoteToHubSpace" not in snip and "BindSpaceFile" in snip:
        print("--- other match ---")
        print(snip[:600])

# Also check handle dispatch - app_commands may auto-generate
if "PromoteToHubSpace" in et and "promote_to_hub_space::handle" not in et:
    # look for create_studio::handle pattern
    i = et.find("create_studio::handle")
    print("create_studio handle at", i)
    print(et[i-200:i+400] if i>=0 else "no direct handle")

# Add to HOME_RETAINED_TOOL_IDS / contracts as HostOnly if needed - promote/persist are effect relays like createSpace
if '"promoteToHubSpace"' not in et:
    et = et.replace(
        '"createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "presenceHeartbeat"',
        '"createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "promoteToHubSpace", "persistLocally", "presenceHeartbeat"',
    )
    et = et.replace(
        'ArtifactToolPublicationContract { tool_id: "createSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },',
        'ArtifactToolPublicationContract { tool_id: "createSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n'
        '    ArtifactToolPublicationContract { tool_id: "promoteToHubSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n'
        '    ArtifactToolPublicationContract { tool_id: "persistLocally", lanes: &[ArtifactToolPublicationLane::HostOnly] },',
    )
    print("retained contracts updated")

editors[0].write_text(et)
print("editor saved")

# Fix Event box clone in wgpu - use (*event).as_ref() or just event.as_ref().clone()
osroot = next(Path("/Users/ueli/Documents/semio").glob("*/🛍️products/💻️os"))
wgpu = next((osroot / "🔨️modules" / "📺️renderer").rglob("*/🐚️Shell/🎯️targets/*/🦀️.rs"))
wt = wgpu.read_text()
# Box<DirectoryEvent> - push((*event).clone()) is wrong syntax; should be live_events.push((*event).clone()) if Event{event: Box} then *event is DirectoryEvent
# Actually `event` is Box<DirectoryEvent>, so (*event).clone() works if DirectoryEvent: Clone
# Or event.as_ref().clone()
if "live_events.push((*event).clone())" in wt:
    wt = wt.replace("live_events.push((*event).clone())", "live_events.push(event.as_ref().clone())", 1)
    wgpu.write_text(wt)
    print("fixed event clone")

# Fix TS test import path - from 🧪tests/persistence-data-class to 🟦️.ts is ../../🟦️.ts not ../../../
tests_root = next(c for c in osroot.iterdir() if "tests" in c.name and c.is_dir())
ts = tests_root / "persistence-data-class" / "🟦️.ts"
if ts.exists():
    t = ts.read_text()
    t = t.replace('from "../../../🟦️.ts"', 'from "../../🟦️.ts"')
    t = t.replace('join(import.meta.dir, "../../../🔨️modules/', 'join(import.meta.dir, "../../🔨️modules/')
    ts.write_text(t)
    print("ts paths fixed", ts)

print("done")
