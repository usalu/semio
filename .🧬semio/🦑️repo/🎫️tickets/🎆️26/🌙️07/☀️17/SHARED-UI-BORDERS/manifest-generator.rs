#[path = "../../../../../../../🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"]
mod manifest;
#[path = "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🟦️projection/🦀️.rs"]
mod ui_contract;

fn main() {
    manifest::validate().unwrap();
    ui_contract::validate().unwrap();
    assert_eq!(ui_contract::TYPES.len(), 92);
    for (name, content) in [("🪪️manifest", manifest::render_typescript()), ("📜️ui-contract", ui_contract::render_typescript())] {
        let path = std::path::Path::new("🧰️framework/🔨️modules/🛂️manifest/🤖️generated").join(name).join("🟦️.ts");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &content).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), content);
        println!("{} bytes {}", content.len(), path.display());
    }
}
