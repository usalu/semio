//! Integration: `semio-store` HTTP (GraphQL + `POST /install`).

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use semio::change_command::ChangeKitCommand;
use semio::id::Id;
use semio::kit::KitFullDto;
use serde_json::{json, Value};

fn spawn_server() -> Result<(std::process::Child, u16, String), Box<dyn std::error::Error + Send + Sync>> {
    let exe = std::path::Path::new(env!("CARGO_BIN_EXE_semio-store"));
    let mut child = Command::new(exe)
        .env("SEMIO_STORE_PORT", "0")
        .env("RUST_LOG", "off")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let stdout = child.stdout.take().ok_or("no stdout")?;
    let mut br = BufReader::new(stdout);
    let mut line = String::new();
    br.read_line(&mut line)?;
    let j: Value = serde_json::from_str(line.trim())?;
    let port: u16 = j
        .get("port")
        .and_then(|p| p.as_u64().and_then(|n| u16::try_from(n).ok()))
        .ok_or("port in ready line")?;
    let base = format!("http://127.0.0.1:{}", port);
    Ok((child, port, base))
}

async fn post_gql(
    client: &reqwest::Client,
    base: &str,
    query: &str,
    variables: Option<Value>,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let mut body = json!({ "query": query });
    if let Some(v) = variables {
        body["variables"] = v;
    }
    let r = client
        .post(format!("{base}/graphql"))
        .json(&body)
        .send()
        .await?;
    let t = r.text().await?;
    let v: Value = serde_json::from_str(&t).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{e}, body: {t}"),
        )
    })?;
    Ok(v)
}

async fn post_install(
    client: &reqwest::Client,
    base: &str,
    body: &Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let st = client.post(format!("{base}/install")).json(body).send().await?;
    if !st.status().is_success() {
        return Err(format!("install {}: {}", st.status(), st.text().await?).into());
    }
    Ok(())
}

#[tokio::test]
async fn sidecar_create_snapshot_name_change_undo() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut child, _port, base) = spawn_server()?;
    let client = reqwest::Client::new();
    let kid = Id::new_v7();
    let dto = KitFullDto {
        id: kid,
        name: "A".to_string(),
        ..Default::default()
    };
    post_install(
        &client,
        &base,
        &json!({ "create": { "dto": serde_json::to_value(&dto)? } }),
    )
    .await?;

    let cmds = vec![ChangeKitCommand::Name {
        name: "Renamed".to_string(),
    }];
    let cmd_j = serde_json::to_value(&cmds)?;
    let m1 = post_gql(
        &client,
        &base,
        "mutation($req: JSON!) { kitCommandShell(commandKind: \"changeKitWithInverse\", request: $req) }",
        Some(json!({
            "req": { "variables": { "commands": cmd_j } }
        })),
    )
    .await?;
    if m1.get("errors").is_some() {
        return Err(format!("graphql errors: {m1}").into());
    }
    let inv = m1
        .pointer("/data/kitCommandShell/data/changeKitWithInverse/inverse")
        .cloned()
        .ok_or("inverse in response")?;

    let q2 = post_gql(
        &client,
        &base,
        "query { kitStore { liveFullDto } }",
        None,
    )
    .await?;
    let name = q2
        .pointer("/data/kitStore/liveFullDto/name")
        .and_then(|n| n.as_str())
        .ok_or("liveFullDto.name")?;
    assert_eq!(name, "Renamed");

    let m3 = post_gql(
        &client,
        &base,
        "mutation($req: JSON!) { kitCommandShell(commandKind: \"changeKitCommands\", request: $req) }",
        Some(json!({
            "req": { "variables": { "commands": inv } }
        })),
    )
    .await?;
    if m3.get("errors").is_some() {
        return Err(format!("graphql m3: {m3}").into());
    }

    let q4 = post_gql(
        &client,
        &base,
        "query { kitStore { liveFullDto } }",
        None,
    )
    .await?;
    let name2 = q4
        .pointer("/data/kitStore/liveFullDto/name")
        .and_then(|n| n.as_str())
        .ok_or("name2")?;
    assert_eq!(name2, "A");

    let _ = child.kill();
    Ok(())
}

#[tokio::test]
async fn sidecar_planner_then_execute_field_patch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut child, _port, base) = spawn_server()?;
    let client = reqwest::Client::new();
    let kid = Id::new_v7();
    let dto = KitFullDto {
        id: kid.clone(),
        name: "P".to_string(),
        ..Default::default()
    };
    post_install(
        &client,
        &base,
        &json!({ "create": { "dto": serde_json::to_value(&dto)? } }),
    )
    .await?;

    let m2 = post_gql(
        &client,
        &base,
        "mutation($req: JSON!) { kitCommandShell(commandKind: \"changeKitCommandsForFieldPatch\", request: $req) }",
        Some(json!({
            "req": { "variables": {
                "kind": "Kit",
                "id": kid.as_str(),
                "field": "name",
                "value": "Q"
            } }
        })),
    )
    .await?;
    let cmds = m2
        .pointer("/data/kitCommandShell/data/changeKitCommandsForFieldPatch")
        .cloned()
        .ok_or("planner cmds")?;

    let m3 = post_gql(
        &client,
        &base,
        "mutation($req: JSON!) { kitCommandShell(commandKind: \"changeKitCommands\", request: $req) }",
        Some(json!({ "req": { "variables": { "commands": cmds } } })),
    )
    .await?;
    if m3.get("errors").is_some() {
        return Err(format!("execute: {m3}").into());
    }

    let q4 = post_gql(
        &client,
        &base,
        "query { kitStore { liveFullDto } }",
        None,
    )
    .await?;
    let name = q4
        .pointer("/data/kitStore/liveFullDto/name")
        .and_then(|n| n.as_str());
    assert_eq!(name, Some("Q"));

    let _ = child.kill();
    Ok(())
}

#[tokio::test]
async fn sidecar_backbone_attach_status_conflicts_sync_detach() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut child, _port, base) = spawn_server()?;
    let client = reqwest::Client::new();
    let kid = Id::new_v7();
    let dto = KitFullDto {
        id: kid,
        name: "Bb".to_string(),
        ..Default::default()
    };
    post_install(
        &client,
        &base,
        &json!({ "create": { "dto": serde_json::to_value(&dto)? } }),
    )
    .await?;

    let m2 = post_gql(
        &client,
        &base,
        "mutation { kitCommandShell(commandKind: \"listConflicts\", request: {}) }",
        None,
    )
    .await?;
    let items = m2
        .pointer("/data/kitCommandShell/data/listConflicts/listConflicts/items")
        .and_then(|x| x.as_array())
        .ok_or("listConflicts")?;
    assert!(items.is_empty(), "expected no conflicts");

    let mut bb_path = std::env::temp_dir();
    bb_path.push(format!("semio-store-dev-backbone-{}.json", Id::new_v7().as_str()));

    let m3 = post_gql(
        &client,
        &base,
        "mutation($req: JSON!) { kitCommandShell(commandKind: \"attachBackbone\", request: $req) }",
        Some(json!({
            "req": { "variables": { "config": { "dev": { "path": bb_path.to_string_lossy() } } } }
        })),
    )
    .await?;
    assert_eq!(
        m3.pointer("/data/kitCommandShell/data/attachBackbone/attachBackbone/ok"),
        Some(&json!(true))
    );

    let m4 = post_gql(
        &client,
        &base,
        "mutation { kitCommandShell(commandKind: \"backboneStatus\", request: {}) }",
        None,
    )
    .await?;
    assert_eq!(
        m4.pointer("/data/kitCommandShell/data/backboneStatus/backboneStatus/attached"),
        Some(&json!(true))
    );
    assert_eq!(
        m4.pointer("/data/kitCommandShell/data/backboneStatus/backboneStatus/kind"),
        Some(&json!("dev"))
    );

    let m5 = post_gql(
        &client,
        &base,
        "mutation { kitCommandShell(commandKind: \"syncNow\", request: {}) }",
        None,
    )
    .await?;
    assert_eq!(
        m5.pointer("/data/kitCommandShell/data/syncNow/syncNow/ok"),
        Some(&json!(true))
    );

    let m6 = post_gql(
        &client,
        &base,
        "mutation { kitCommandShell(commandKind: \"detachBackbone\", request: {}) }",
        None,
    )
    .await?;
    assert_eq!(
        m6.pointer("/data/kitCommandShell/data/detachBackbone/detachBackbone/ok"),
        Some(&json!(true))
    );

    let m7 = post_gql(
        &client,
        &base,
        "mutation { kitCommandShell(commandKind: \"backboneStatus\", request: {}) }",
        None,
    )
    .await?;
    assert_eq!(
        m7.pointer("/data/kitCommandShell/data/backboneStatus/backboneStatus/attached"),
        Some(&json!(false))
    );

    let _ = std::fs::remove_file(&bb_path);
    let _ = child.kill();
    Ok(())
}
