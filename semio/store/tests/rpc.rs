//! Integration: spawn `semio-store`, speak NDJSON JSON-RPC 2.0.
//! Uses `CARGO_BIN_EXE_semio-store` (stable since Rust 1.64).

use std::io::Write as _;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use semio::change_command::ChangeKitCommand;
use semio::id::Id;
use semio::kit::KitFullDto;
use serde_json::{json, Value};

fn read_json_line(
    r: &mut impl BufRead,
    is_event: &mut bool,
) -> std::io::Result<Value> {
    let mut line = String::new();
    r.read_line(&mut line)?;
    if line.is_empty() {
        return Ok(Value::Null);
    }
    let v: Value = serde_json::from_str(line.trim())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    *is_event = v.get("method").and_then(|m| m.as_str()) == Some("event");
    Ok(v)
}

fn until_response(
    r: &mut impl BufRead,
    id: i64,
) -> Result<Value, Box<dyn std::error::Error>> {
    loop {
        let mut is_event = false;
        let v = read_json_line(r, &mut is_event).map_err(|e| e.to_string())?;
        if v.is_null() {
            return Err("eof".into());
        }
        if is_event {
            continue;
        }
        if v.get("id") == Some(&json!(id)) {
            if let Some(e) = v.get("error") {
                return Err(format!("jsonrpc error: {e}").into());
            }
            return v
                .get("result")
                .cloned()
                .ok_or_else(|| "missing result".to_string().into());
        }
    }
}

#[test]
fn sidecar_create_snapshot_name_change_undo() -> Result<(), Box<dyn std::error::Error>> {
    let exe = std::path::Path::new(env!("CARGO_BIN_EXE_semio-store"));
    let mut child = Command::new(exe)
        .env("SEMIO_STORE_NO_EVENTS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or("no stdin on child")?;
    let stdout = child.stdout.take().ok_or("no stdout on child")?;
    let mut reader = BufReader::new(stdout);

    let kid = Id::new_v7();
    let dto = KitFullDto {
        id: kid,
        name: "A".to_string(),
        ..Default::default()
    };
    let req1 = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "kit.create",
        "params": { "dto": serde_json::to_value(&dto)? }
    });
    writeln!(stdin, "{}", serde_json::to_string(&req1)?)?;
    let r1 = until_response(&mut reader, 1)?;
    if !r1.is_null() {
        return Err("expected null result for kit.create".into());
    }

    let cmds = vec![ChangeKitCommand::Name {
        name: "Renamed".to_string(),
    }];
    let cmds_v = serde_json::to_value(&cmds)?;
    let req2 = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "kit.executeChangeKitCommands",
        "params": { "cmds": cmds_v }
    });
    writeln!(stdin, "{}", serde_json::to_string(&req2)?)?;
    let r2 = until_response(&mut reader, 2)?;
    assert_eq!(
        r2.get("kind").and_then(|v| v.as_str()),
        Some("setKitMetadata")
    );
    let inv = r2
        .get("inverse")
        .cloned()
        .ok_or("missing inverse")?;

    writeln!(stdin, r#"{{"jsonrpc":"2.0","id":3,"method":"kit.snapshot","params":{{}}}}"#)?;
    let snap = until_response(&mut reader, 3)?;
    let name = snap
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or("snapshot.name")?;
    assert_eq!(name, "Renamed");

    // Apply the returned inverse commands (in forward order) to restore the original name.
    let inv_v = inv;
    let req4 = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "kit.executeChangeKitCommands",
        "params": { "cmds": inv_v }
    });
    writeln!(stdin, "{}", serde_json::to_string(&req4)?)?;
    let _r4 = until_response(&mut reader, 4)?;

    writeln!(stdin, r#"{{"jsonrpc":"2.0","id":5,"method":"kit.snapshot","params":{{}}}}"#)?;
    let snap5 = until_response(&mut reader, 5)?;
    let name2 = snap5
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or("snapshot2.name")?;
    assert_eq!(name2, "A");

    drop(stdin);
    let st = child.wait()?;
    assert!(st.success(), "sidecar should exit 0 on stdin EOF, got {st:?}");
    Ok(())
}

#[test]
fn sidecar_planner_then_execute_field_patch() -> Result<(), Box<dyn std::error::Error>> {
    let exe = std::path::Path::new(env!("CARGO_BIN_EXE_semio-store"));
    let mut child = Command::new(exe)
        .env("SEMIO_STORE_NO_EVENTS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdin = child.stdin.take().ok_or("no stdin on child")?;
    let stdout = child.stdout.take().ok_or("no stdout on child")?;
    let mut reader = BufReader::new(stdout);

    let kid = Id::new_v7();
    let dto = KitFullDto {
        id: kid.clone(),
        name: "P".to_string(),
        ..Default::default()
    };
    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "kit.create",
            "params": { "dto": serde_json::to_value(&dto)? }
        }))?
    )?;
    let _r1 = until_response(&mut reader, 1)?;

    let req2 = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "kit.changeKitCommandsForFieldPatch",
        "params": {
            "kind": "Kit",
            "id": kid.as_str(),
            "field": "name",
            "value": "Q"
        }
    });
    writeln!(stdin, "{}", serde_json::to_string(&req2)?)?;
    let cmds = until_response(&mut reader, 2)?;
    let req3 = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "kit.executeChangeKitCommands",
        "params": { "cmds": cmds }
    });
    writeln!(stdin, "{}", serde_json::to_string(&req3)?)?;
    let _r3 = until_response(&mut reader, 3)?;

    writeln!(stdin, r#"{{"jsonrpc":"2.0","id":4,"method":"kit.snapshot","params":{{}}}}"#)?;
    let snap = until_response(&mut reader, 4)?;
    assert_eq!(snap.get("name").and_then(|n| n.as_str()), Some("Q"));

    drop(stdin);
    let st = child.wait()?;
    assert!(st.success(), "sidecar exit {st:?}");
    Ok(())
}

#[test]
fn sidecar_backbone_attach_status_conflicts_sync_detach() -> Result<(), Box<dyn std::error::Error>> {
    let exe = std::path::Path::new(env!("CARGO_BIN_EXE_semio-store"));
    let mut child = Command::new(exe)
        .env("SEMIO_STORE_NO_EVENTS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdin = child.stdin.take().ok_or("no stdin on child")?;
    let stdout = child.stdout.take().ok_or("no stdout on child")?;
    let mut reader = BufReader::new(stdout);

    let kid = Id::new_v7();
    let dto = KitFullDto {
        id: kid,
        name: "Bb".to_string(),
        ..Default::default()
    };
    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "kit.create",
            "params": { "dto": serde_json::to_value(&dto)? }
        }))?
    )?;
    let _r1 = until_response(&mut reader, 1)?;

    let mut bb_path = std::env::temp_dir();
    bb_path.push(format!("semio-store-dev-backbone-{}.json", Id::new_v7().as_str()));

    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "conflicts.list",
            "params": {}
        }))?
    )?;
    let r2 = until_response(&mut reader, 2)?;
    let items = r2
        .get("listConflicts")
        .and_then(|x| x.get("items"))
        .and_then(|x| x.as_array())
        .ok_or("listConflicts.items")?;
    assert!(items.is_empty(), "expected no conflicts");

    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "backbone.attach",
            "params": {
                "config": { "dev": { "path": bb_path.to_string_lossy() } }
            }
        }))?
    )?;
    let r3 = until_response(&mut reader, 3)?;
    assert_eq!(
        r3.get("attachBackbone").and_then(|x| x.get("ok")),
        Some(&json!(true))
    );

    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "backbone.status",
            "params": {}
        }))?
    )?;
    let r4 = until_response(&mut reader, 4)?;
    assert_eq!(
        r4.get("backboneStatus").and_then(|x| x.get("attached")),
        Some(&json!(true))
    );
    assert_eq!(
        r4.get("backboneStatus").and_then(|x| x.get("kind")),
        Some(&json!("dev"))
    );

    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 5,
            "method": "coordinator.syncNow",
            "params": {}
        }))?
    )?;
    let r5 = until_response(&mut reader, 5)?;
    assert_eq!(
        r5.get("syncNow").and_then(|x| x.get("ok")),
        Some(&json!(true))
    );

    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 6,
            "method": "backbone.detach",
            "params": {}
        }))?
    )?;
    let r6 = until_response(&mut reader, 6)?;
    assert_eq!(
        r6.get("detachBackbone").and_then(|x| x.get("ok")),
        Some(&json!(true))
    );

    writeln!(
        stdin,
        "{}",
        serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "id": 7,
            "method": "backbone.status",
            "params": {}
        }))?
    )?;
    let r7 = until_response(&mut reader, 7)?;
    assert_eq!(
        r7.get("backboneStatus").and_then(|x| x.get("attached")),
        Some(&json!(false))
    );

    let _ = std::fs::remove_file(&bb_path);

    drop(stdin);
    let st = child.wait()?;
    assert!(st.success(), "sidecar exit {st:?}");
    Ok(())
}
