//! 🔬️ Dumps `project_ifc_4_any` / `project_ifc_2x3_any` — the ruststep-backed projections the
//! SUBJECT half of both differential cases uses — as JSON, so the IfcOpenShell oracle's own
//! from-scratch projection of the same bytes can be diffed against them today, while no generated
//! Rust subject host links.
use semio_repo_test_host::Json;
use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v2x3::subsets::any::project_ifc_2x3_any;
use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v4::subsets::any::project_ifc_4_any;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: crosscheck <v4|v2x3> <input> <output.json>");
        return std::process::ExitCode::from(2);
    }
    let bytes = match std::fs::read(&args[2]) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("cannot read {}: {error}", args[2]);
            return std::process::ExitCode::from(2);
        }
    };
    let projected: Result<Json, String> = match args[1].as_str() {
        "v4" => project_ifc_4_any(&bytes),
        "v2x3" => project_ifc_2x3_any(&bytes),
        other => Err(format!("unknown subset {other}")),
    };
    match projected {
        Ok(json) => {
            if let Err(error) = std::fs::write(&args[3], json.to_string()) {
                eprintln!("cannot write {}: {error}", args[3]);
                return std::process::ExitCode::from(2);
            }
            println!("projected {} -> {}", args[2], args[3]);
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("projection failed: {error}");
            std::process::ExitCode::from(1)
        }
    }
}
