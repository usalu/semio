//! 🔌️ Temporary check of the ticket (not part of the repository build): loads the energy model files the BIM export committed with the energy artifact's own readers (its `s.stdio.json` deserializer and its DSL
//! parser), checks that both give the same snapshot and that `Model::validate` accepts it. Usage: `w2-wp20-energy-loader <case directory>...`.

use semio_framework_os_kernel::ArtifactDsl;
use semio_s_artifact_energy_model::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes;
use semio_s_artifact_energy_model::EnergyModelSnapshot;

fn check(dir: &str) -> Result<String, String> {
    let json = std::fs::read(format!("{dir}/🔋️model.json")).map_err(|error| error.to_string())?;
    let dsl = std::fs::read_to_string(format!("{dir}/🗣️.dsl.semio")).map_err(|error| error.to_string())?;
    let from_json = deserialize_bytes(&json).map_err(|error| format!("json: {error}"))?;
    let from_dsl = <EnergyModelSnapshot as ArtifactDsl>::parse_dsl(&dsl).map_err(|error| format!("dsl: {error}"))?;
    if from_json != from_dsl {
        return Err("the JSON and the DSL text load to different snapshots".to_string());
    }
    from_json.model.validate().map_err(|diagnostics| format!("validate: {diagnostics:?}"))?;
    let model = &from_json.model;
    Ok(format!("{} zones, {} surfaces, {} fenestrations, {} constructions, {} schedules, valid", model.zones.len(), model.surfaces.len(), model.fenestrations.len(), model.constructions.len(), model.schedules.constants.len() + model.schedules.daily.len() + model.schedules.weekly.len()))
}

fn main() {
    let mut failed = false;
    for dir in std::env::args().skip(1) {
        match check(&dir) {
            Ok(summary) => println!("{dir}: {summary}"),
            Err(problem) => {
                failed = true;
                println!("[FAIL] {dir}: {problem}");
            }
        }
    }
    std::process::exit(i32::from(failed));
}
