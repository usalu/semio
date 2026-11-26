use std::{env, fs, path::PathBuf};
use bim_energy::standards::v1::subsets::any::io::export::energy::model_to_energy;
use bim_examples::standards::v1::subsets::any::io::export::energy::document;
use pack_json::{from_json_str, JsonMemberPolicy};

fn main() {
    let root = PathBuf::from(env::args_os().nth(1).expect("energy fixture root"));
    for case in ["🏠️room", "🏘️pair", "🧱️stack"] {
        let path = root.join(case);
        let input = fs::read_to_string(path.join("📸️snapshot/🔣️.json")).unwrap();
        let snapshot = from_json_str(&input, JsonMemberPolicy::Reject).unwrap();
        let (model, _) = model_to_energy(&snapshot).unwrap();
        for (name, text) in [("🔋️model.json", document::json_text(&model).unwrap()), ("🗣️.dsl.semio", document::dsl_text(&model).unwrap())] {
            fs::write(path.join(name), text).unwrap();
        }
        println!("{case}: {} zones, {} surfaces", model.zones.len(), model.surfaces.len());
    }
}
