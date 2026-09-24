from pathlib import Path

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))
mains = [p for p in space.rglob("🦀️.rs") if "explore" in str(p) and p.parent.name.endswith("main") and "tests" not in p.parts]
print("mains", mains)
main = mains[0]
mt = main.read_text()
# Check available IconName usage nearby
if "IconName::" in mt:
    icons = sorted(set([line.strip() for line in mt.splitlines() if "IconName::" in line]))
    print("icons", icons[:20])

marker = 'if row.origin == "hub" && row.role == Some(crate::DirectorySpaceRole::Author) {'
if "ephemeralLocalOnly" in mt:
    print("already patched")
else:
    insert = '''    if row.data_class == "ephemeralLocalOnly" {
        for action in [
            home_row_action(IconName::CloudUpload, labels.action_promote, "promoteToHubSpace", &row.id)?,
            home_row_action(IconName::HardDrive, labels.action_persist, "persistLocally", &row.id)?,
        ] {
            actions.try_push(action).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
        }
        return Ok(actions);
    }
    '''
    if marker not in mt:
        raise SystemExit("marker missing")
    mt = mt.replace(marker, insert + marker, 1)
    main.write_text(mt)
    print("patched", main)
