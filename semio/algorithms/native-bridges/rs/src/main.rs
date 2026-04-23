mod header { // 🧲Header
// 💻 semio/algorithms/native-bridges/rs/src/main.rs
// Specs: Read JSON op+payload from stdin; write JSON {ok,result,error} to stdout.
// Summary: Rust native bridge for algorithms Storybook proxy using semio/rs library only.
// 2026 Ueli Saluz <ueli@semio-tech.com>
} // 🧲Header

use semio::{DesignStoreRef, Id, KitFullDto, KitGraph, KitGraphRef, SemioReport};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BridgeRequest {
    op: String,
    kit: KitFullDto,
    design_id: String,
    #[serde(default)]
    piece_ids: Vec<String>,
    #[serde(default)]
    connection_ids: Vec<String>,
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
    let kit_ref: KitGraphRef = KitGraph::from_full_dto(req.kit);
    match req.op.as_str() {
        "flatten" => {
            let report = match futures_lite::future::block_on(semio::KitGraph::flatten_design_async(
                &kit_ref,
                &req.design_id,
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
            let piece_ids: Vec<Id> = req.piece_ids.into_iter().map(Id::from).collect();
            let connection_ids: Vec<String> = req.connection_ids.into_iter().collect();
            for cid in &connection_ids {
                if let Err(e) =
                    KitGraph::delete_connection_in_design(&kit_ref, &req.design_id, cid.as_str())
                {
                    write_err(format!("delete connection {cid}: {e:?}"));
                    return;
                }
            }
            let design_ref: DesignStoreRef = {
                let guard = match kit_ref.read() {
                    Ok(g) => g,
                    Err(_) => {
                        write_err("kit lock poisoned".into());
                        return;
                    }
                };
                match guard.design(&req.design_id) {
                    Some(d) => d,
                    None => {
                        write_err(format!("design {} not found", req.design_id));
                        return;
                    }
                }
            };
            let report = match design_ref.write() {
                Ok(mut d) => {
                    let removed = d.delete_pieces(&piece_ids);
                    SemioReport::ok(serde_json::json!({ "removedPieces": removed }))
                }
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
