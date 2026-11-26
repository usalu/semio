use std::io::{self, Read, Write};

mod board_projection {
    include!(env!("SEMIO_UI_BOARD_PROJECTION"));
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let snapshot = semio_framework_pack_json::from_json_str(&input, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let painted = board_projection::puzzle2d_board_snapshot_json(&snapshot);
    io::stdout().write_all(painted.as_bytes()).unwrap();
}
