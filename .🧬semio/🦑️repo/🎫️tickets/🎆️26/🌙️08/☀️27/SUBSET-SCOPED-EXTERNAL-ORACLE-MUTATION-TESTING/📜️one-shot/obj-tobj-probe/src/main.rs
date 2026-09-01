use std::io::Cursor;

fn load(text: &str) -> Result<Vec<tobj::Model>, tobj::LoadError> {
    let mut cursor = Cursor::new(text.as_bytes().to_vec());
    tobj::load_obj_buf(&mut cursor, &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default() }, |_| Ok(Default::default())).map(|(m, _)| m)
}

fn dump(label: &str, text: &str) {
    println!("=== {label} ===");
    match load(text) {
        Ok(models) => {
            println!("  OK models: {}", models.len());
            for m in &models {
                println!("  - name={:?} indices_len={}", m.name, m.mesh.indices.len());
            }
        }
        Err(e) => println!("  ERR: {e}"),
    }
}

fn main() {
    dump("multi-name-g-line", "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
g alpha\nf 1 2 3\n\
g alpha beta\nf 4 5 6\n");

    dump("bare-g-line-reset", "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
g alpha\nf 1 2 3\n\
g\nf 4 5 6\n");

    dump("bare-o-line-reset", "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
o alpha\nf 1 2 3\n\
o\nf 4 5 6\n");

    // continuous alpha covering both faces, no reset in between
    dump("alpha-covers-both-no-reset", "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
g alpha\nf 1 2 3\nf 4 5 6\n");
}
