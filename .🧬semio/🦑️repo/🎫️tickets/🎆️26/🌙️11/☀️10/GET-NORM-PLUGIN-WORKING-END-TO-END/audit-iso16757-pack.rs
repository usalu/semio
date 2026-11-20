use std::fmt::Write;
use std::path::Path;
use std::process::ExitCode;

use pack::snapshot::{ArtifactDsl, ArtifactPack};
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

fn audit(dsl_path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(dsl_path).map_err(|error| error.to_string())?;
    let from_dsl = Iso16757Snapshot::parse_dsl(&text).map_err(|error| error.to_string())?;
    let pack_path = dsl_path.with_file_name("🎒️.pack.semio");
    let sibling = std::fs::read(&pack_path).map_err(|error| error.to_string())?;
    let problem = match Iso16757Snapshot::decode_pack(&sibling) {
        Ok(from_pack) if from_pack == from_dsl => return Ok(()),
        Ok(_) => "the decoded Pack and DSL snapshots differ".to_string(),
        Err(error) => format!("the checked-in Pack failed to decode: {error}"),
    };

    let candidate = from_dsl.encode_pack();
    let decoded = Iso16757Snapshot::decode_pack(&candidate).map_err(|error| error.to_string())?;
    if decoded != from_dsl || candidate != decoded.encode_pack() {
        return Err("the production Pack encoder lacks a stable typed round trip".to_string());
    }
    let name = dsl_path
        .parent()
        .and_then(Path::file_name)
        .and_then(|part| part.to_str())
        .ok_or("expected an example directory name")?;
    let mut hex = String::with_capacity(candidate.len() * 2);
    for byte in candidate {
        write!(&mut hex, "{byte:02x}").expect("writing to a String is infallible");
    }
    println!("[DEBUG] PACK_CANDIDATE {name} {hex}");
    Err(problem)
}

fn main() -> ExitCode {
    let args = std::env::args_os().skip(1);
    let mut failures = 0;
    for path in args {
        let path = Path::new(&path);
        match audit(path) {
            Ok(()) => println!("[DEBUG] PASS {}", path.display()),
            Err(error) => {
                failures += 1;
                println!("[DEBUG] FAIL {}: {error}", path.display());
            }
        }
    }
    if failures == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
