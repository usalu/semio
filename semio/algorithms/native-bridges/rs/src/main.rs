mod header { // 🧲Header
// 💻 semio/algorithms/native-bridges/rs/src/main.rs
// Specs: Read JSON op+payload from stdin; write JSON {ok,result,error} to stdout.
// Summary: Rust native bridge for algorithms Storybook proxy using semio/rs library only.
// 2026 Ueli Saluz <ueli@semio-tech.com>
} // 🧲Header

use semio::{DesignStoreRef, Guid, KitFullDto, KitStore, KitStoreRef, SemioReport};
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
    match req.op.as_str() {
        "flatten" => {
            let report = match futures_lite::future::block_on(semio::KitStore::flatten_design_async(
                &kit_ref,
                &req.design_guid,
            )) {
                Ok(r) => r,
                Err(e) => {
                    write_err(e.to_string());
                    return;
                }
            };
            match serde_json::to_value(&report) {
                Ok(v) => write_ok(v),
                Err(e) => write_err(format!("marshal flatten: {e}")),
            }
        }
        "delete" => {
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
            let piece_guids: Vec<Guid> = req.piece_guids.into_iter().map(Guid::from).collect();
            let connection_guids: Vec<Guid> =
                req.connection_guids.into_iter().map(Guid::from).collect();
            let report = match design_ref.write() {
                Ok(mut d) => d.delete_change(&piece_guids, &connection_guids),
                Err(_) => SemioReport::err("design lock poisoned"),
            };
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
