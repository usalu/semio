mod header { // 🧲Header
// 💻 semio/algorithms/native-bridges/rs/src/main.rs
// Specs: Read JSON op+payload from stdin; write JSON {ok,result,error} to stdout.
// Summary: Rust native bridge for algorithms Storybook proxy using semio/rs library only.
// 2026 Ueli Saluz <ueli@semio-tech.com>
} // 🧲Header

use semio::{DesignStore, DesignStoreRef, Guid, KitFullDto, KitStore, KitStoreRef};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BridgeRequest {
    op: String,
    kit: KitFullDto,
    design_guid: String,
    #[serde(default)]
    piece_guids: Vec<String>,
    #[serde(default)]
    connection_guids: Vec<String>,
}

#[derive(Debug, Serialize)]
struct BridgeResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn main() {
    let mut buf = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
        write_err(format!("read stdin: {e}"));
        return;
    }
    let req: BridgeRequest = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(e) => {
            write_err(format!("parse request: {e}"));
            return;
        }
    };
    let kit_ref: KitStoreRef = KitStore::from_full_dto(req.kit);
    let design_ref: DesignStoreRef = {
        let guard = match kit_ref.read() {
            Ok(g) => g,
            Err(_) => {
                write_err("kit lock poisoned".into());
                return;
            }
        };
        match guard.design(&req.design_guid) {
            Some(d) => d,
            None => {
                write_err(format!("design {} not found", req.design_guid));
                return;
            }
        }
    };
    match req.op.as_str() {
        "flatten" => {
            let report = match design_ref.read() {
                Ok(d) => d.flatten_change(),
                Err(_) => {
                    write_err("design lock poisoned".into());
                    return;
                }
            };
            match serde_json::to_value(&report) {
                Ok(v) => write_ok(v),
                Err(e) => write_err(format!("marshal flatten: {e}")),
            }
        }
        "delete" => {
            let piece_guids: Vec<Guid> = req.piece_guids.into_iter().map(Guid::from).collect();
            let connection_guids: Vec<Guid> =
                req.connection_guids.into_iter().map(Guid::from).collect();
            let report = DesignStore::delete_pieces_and_connections_ref(
                &design_ref,
                &piece_guids,
                &connection_guids,
            );
            match serde_json::to_value(&report) {
                Ok(v) => write_ok(v),
                Err(e) => write_err(format!("marshal delete: {e}")),
            }
        }
        _ => write_err(format!("unknown op: {}", req.op)),
    }
}

fn write_ok(result: serde_json::Value) {
    let resp = BridgeResponse { ok: true, result: Some(result), error: None };
    println!("{}", serde_json::to_string(&resp).unwrap());
}

fn write_err(msg: String) {
    let resp = BridgeResponse { ok: false, result: None, error: Some(msg) };
    println!("{}", serde_json::to_string(&resp).unwrap());
}
