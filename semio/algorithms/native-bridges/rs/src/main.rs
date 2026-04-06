mod header { // 🔖Header
// 💻 semio/algorithms/native-bridges/rs/src/main.rs
// Specs: Read JSON op+payload from stdin; write JSON {ok,result,error} to stdout.
// Summary: Rust native bridge for algorithms Storybook proxy using semio/rs library only.
// 2026 Ueli Saluz <ueli@semio-tech.com>
} // 🔖Header

use semio::delete_pieces_and_connections_in_design;
use semio::flatten_design;
use semio::Design;
use semio::Kit;
use serde::Deserialize;
use serde::Serialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BridgeRequest {
    op: String,
    kit: Kit,
    #[serde(default)]
    design: Option<Design>,
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
    match req.op.as_str() {
        "flatten" => {
            let dc = flatten_design(&req.kit, &req.design_guid);
            match serde_json::to_value(&dc) {
                Ok(v) => write_ok(v),
                Err(e) => write_err(format!("marshal flatten: {e}")),
            }
        }
        "delete" => {
            let design = match req.design {
                Some(d) => d,
                None => {
                    write_err("missing design".to_string());
                    return;
                }
            };
            let diff = delete_pieces_and_connections_in_design(
                &req.kit,
                &design,
                &req.piece_guids,
                &req.connection_guids,
            );
            match serde_json::to_value(&diff) {
                Ok(v) => write_ok(v),
                Err(e) => write_err(format!("marshal delete: {e}")),
            }
        }
        _ => write_err(format!("unknown op: {}", req.op)),
    }
}

fn write_ok(result: serde_json::Value) {
    let resp = BridgeResponse {
        ok: true,
        result: Some(result),
        error: None,
    };
    println!("{}", serde_json::to_string(&resp).unwrap());
}

fn write_err(msg: String) {
    let resp = BridgeResponse {
        ok: false,
        result: None,
        error: Some(msg),
    };
    println!("{}", serde_json::to_string(&resp).unwrap());
}
