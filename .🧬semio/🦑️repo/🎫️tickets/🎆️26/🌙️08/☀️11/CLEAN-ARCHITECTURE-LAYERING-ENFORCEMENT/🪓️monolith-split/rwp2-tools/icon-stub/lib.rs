
#![allow(dead_code)]
pub mod semio_framework_canvas {
    use std::collections::HashMap;
    use std::sync::OnceLock;
    pub struct IconPack { plain: HashMap<String, String> }
    impl IconPack {
        pub fn plain(&self, id: &str) -> Option<&str> { self.plain.get(id).map(String::as_str) }
        pub fn plain_len(&self) -> usize { self.plain.len() }
        pub fn plain_ids(&self) -> Vec<&str> { let mut ids: Vec<&str> = self.plain.keys().map(String::as_str).collect(); ids.sort_unstable(); ids }
    }
    static INSTALLED: OnceLock<IconPack> = OnceLock::new();
    pub fn icon_pack() -> Option<&'static IconPack> { INSTALLED.get() }
    #[derive(Clone, Copy)] pub struct RuntimeAssets<'a> { pub pack: &'a [u8] }
    #[derive(Debug, PartialEq)] pub enum RuntimeAssetsInstallError { AlreadyInstalled } impl std::fmt::Display for RuntimeAssetsInstallError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("x") } }
    pub enum RuntimeAsset { IconPack }
    impl RuntimeAsset { pub fn repository_directory(&self) -> &'static str { "icons" } }
    impl RuntimeAssets<'static> {
        pub fn from_root(root: &std::path::Path) -> std::io::Result<Self> {
            let dir = root.join("icons");
            let mut plain = HashMap::new();
            for entry in std::fs::read_dir(dir)? { let p = entry?.path(); plain.insert(p.file_stem().unwrap().to_string_lossy().trim_start_matches(|c: char| !c.is_ascii_alphanumeric()).to_string(), std::fs::read_to_string(&p)?); }
            let _ = INSTALLED.set(IconPack { plain });
            Ok(RuntimeAssets { pack: b"" })
        }
    }
    pub fn install_runtime_assets(_a: RuntimeAssets<'static>) -> Result<(), RuntimeAssetsInstallError> { Ok(()) }
}
pub mod ui_wgpu { pub mod wgpu {
    pub struct IconAtlas { pub width: u32, pub height: u32, pub pixels: Vec<u8>, entries: Vec<(String, [f32; 4])> }
    impl IconAtlas {
        pub fn from_packed(width: u32, height: u32, pixels: Vec<u8>, entries: Vec<(String, [f32; 4])>) -> Self { Self { width, height, pixels, entries } }
        pub fn icon_uv(&self, id: &str) -> Option<[f32; 4]> { self.entries.iter().find(|e| e.0 == id).map(|e| e.1) }
    }
}}
pub mod usvg {
    pub struct Options; pub struct Db; impl Db { pub fn load_system_fonts(&mut self) {} }
    impl Default for Options { fn default() -> Self { Options } }
    impl Options { pub fn fontdb_mut(&mut self) -> Db { Db } }
    pub struct Tree(pub String); pub struct Size; impl Size { pub fn width(&self) -> f32 { 24.0 } pub fn height(&self) -> f32 { 24.0 } }
    impl Tree { pub fn from_str(s: &str, _o: &Options) -> Result<Tree, ()> { if s.contains("<svg") { Ok(Tree(s.into())) } else { Err(()) } } pub fn size(&self) -> Size { Size } }
}
pub mod tiny_skia {
    pub struct Pixmap(Vec<u8>, u32); pub struct PixmapMut<'a>(&'a mut Vec<u8>); pub struct Transform;
    impl Transform { pub fn from_scale(_a: f32, _b: f32) -> Self { Transform } }
    impl Pixmap { pub fn new(w: u32, _h: u32) -> Option<Pixmap> { Some(Pixmap(vec![0; (w * w * 4) as usize], w)) } pub fn as_mut(&mut self) -> PixmapMut<'_> { PixmapMut(&mut self.0) } pub fn take(self) -> Vec<u8> { self.0 } }
    impl<'a> PixmapMut<'a> { pub fn fill(&mut self) { for (i, b) in self.0.iter_mut().enumerate() { if i % 4 == 3 { *b = 255; } } } }
}
pub mod resvg { pub fn render(_t: &crate::usvg::Tree, _x: crate::tiny_skia::Transform, p: &mut crate::tiny_skia::PixmapMut) { p.fill(); } }
use semio_framework_canvas::{icon_pack, IconPack};
include!("icon_body.rs");
