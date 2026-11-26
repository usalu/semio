//! 🧬️ Verifies the ModelMutation and typed ModelDiff paths for six repair fixtures.

use bim_mutations::standards::v1::subsets::any::schema::mutations::{ModelMutation, apply_model_mutation};
use bim_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use pack_json::{JsonMemberPolicy, from_json_str, to_json_string};
use protocol::ColdMutation;
use std::{env, fs, path::PathBuf};

const CASES: &[&str] = &[
    "♨️set-type-thermal-data/🧨️states-a-curtain-wall",
    "➖️create-beam/💪️adds-an-arc-beam",
    "🏛️create-column/💪️adds-a-leaning-column",
    "🥖️split-beam/💪️splits-an-arc-beam",
    "🦖️delete-curtain-wall/💪️cascades-its-overrides",
    "🔪️trim-extend-wall/🌀️extends-an-arc",
];

fn main() {
    let mut args = env::args_os().skip(1).map(PathBuf::from);
    let source = args.next().expect("fixture source directory");
    let output = args.next().expect("generated output directory");
    for case in CASES {
        let directory = source.join(case);
        let before = fs::read_to_string(directory.join("📸️snapshot/⬅️before/🔣️.json")).expect(case);
        let mutation = fs::read_to_string(directory.join("🦠️mutation/🔣️.json")).expect(case);
        let before: ModelSnapshot = from_json_str(&before, JsonMemberPolicy::Reject).expect(case);
        let mutation: ModelMutation = from_json_str(&mutation, JsonMemberPolicy::Reject).expect(case);
        let after = apply_model_mutation(&before, &mutation).expect(case);
        let delta = mutation.diff(&before);
        assert_eq!(protocol::apply_diff_cold(delta.diff(), &before).expect(case), after, "{case}");
        assert_ne!(after, before, "{case} must change the model");
        let generated = output.join(case);
        fs::create_dir_all(&generated).expect(case);
        fs::write(generated.join("after.json"), to_json_string(&after)).expect(case);
        fs::write(generated.join("diff.json"), to_json_string(delta.diff())).expect(case);
        println!("{case}");
    }
}
