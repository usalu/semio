use serde::{Deserialize, Serialize};

// Mimics ProgramSnapshot's shape: a "schema" string plus many unrelated domain fields.
#[derive(Serialize)]
struct ProgramLike {
    schema: String,
    meta: String,
    stakeholders: Vec<String>,
}

// Mimics XlsxSnapshot's shape: "schema" required (no default), "opc"/"workbook" both #[serde(default)].
#[derive(Deserialize, Debug, Default)]
struct OpcLike {
    #[serde(default)]
    parts: Vec<String>,
}
#[derive(Deserialize, Debug, Default)]
struct WorkbookLike {
    #[serde(default)]
    sheets: Vec<String>,
}
#[derive(Deserialize, Debug)]
struct XlsxLike {
    schema: String,
    #[serde(default)]
    opc: OpcLike,
    #[serde(default)]
    workbook: WorkbookLike,
}

fn main() {
    let program = ProgramLike {
        schema: "architect.program".to_string(),
        meta: "Sample Clinic".to_string(),
        stakeholders: vec!["Facilities Director".to_string(), "Reception".to_string()],
    };
    let value = serde_json::to_value(&program).unwrap();
    println!("program JSON = {}", value);
    let xlsx: Result<XlsxLike, _> = serde_json::from_value(value);
    match xlsx {
        Ok(x) => println!("xlsx-like result = {:?}  (sheets.len()={}, parts.len()={})", x, x.workbook.sheets.len(), x.opc.parts.len()),
        Err(e) => println!("deserialize FAILED: {e}"),
    }
}
