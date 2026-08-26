fn main() {
    let definition = semio_s_plugin_cad::editor::cad::create_cad_app();
    println!("[DEBUG] CAD manifest admitted with {} window kinds", definition.window_kinds.len());
}
