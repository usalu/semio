//! 🔤️ Text fonts of the layout scene: the layout sans font, the installed font set and its install errors, the byte and native font installers, the font asset names and declarations and the package font set. See [`super`].

use super::*;

//#region ⚙️Scene
pub static LAYOUT_SANS: &[u8] = include_bytes!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🖼️assets/🔤️MapLabelSans.ttf");

pub(super) static TEXT_FONTS: OnceLock<TextFontSet<'static>> = OnceLock::new();

/// 🚫️ Why a host's built-in text fonts were refused by [`install_text_fonts`] and its doors.
#[derive(Debug)]
pub enum TextFontsInstallError {
    Unregistrable(TextFontError),
    WrongFontCount { expected: usize, found: usize },
    AlreadyInstalled,
    PackageAssetMissing { name: String },
    #[cfg(not(target_arch = "wasm32"))]
    Unavailable,
    #[cfg(not(target_arch = "wasm32"))]
    Unreadable(TextFontReadError),
}

impl std::fmt::Display for TextFontsInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unregistrable(error) => std::fmt::Display::fmt(error, f),
            Self::WrongFontCount { expected, found } => write!(f, "expected {expected} built-in text font files in TextFontRole::ALL order, got {found}"),
            Self::AlreadyInstalled => f.write_str("built-in text fonts are already installed"),
            Self::PackageAssetMissing { name } => write!(f, "the host delivered no package asset {name}"),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Unavailable => f.write_str("no built-in text font catalog found (set SEMIO_TEXT_FONTS to the 🖼️assets/🔤️fonts directory)"),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Unreadable(error) => std::fmt::Display::fmt(error, f),
        }
    }
}

impl std::error::Error for TextFontsInstallError {}

/// 🔌️ The host's built-in text font port for the layout engine: every [`LayoutEngine::new`] after this call shapes
/// with the same Anta, Kelly Slab, Share Tech Mono and Noto Emoji fallback families the text system always had.
/// The set is registered once up front, so a refused set does not consume the slot.
pub fn install_text_fonts(fonts: TextFontSet<'static>) -> Result<(), TextFontsInstallError> {
    if TEXT_FONTS.get().is_some() {
        return Err(TextFontsInstallError::AlreadyInstalled);
    }
    TextSystem::new(&fonts).map_err(TextFontsInstallError::Unregistrable)?;
    TEXT_FONTS.set(fonts).map_err(|_| TextFontsInstallError::AlreadyInstalled)
}

/// 🔌️ [`install_text_fonts`] for a host that received the fifteen files by value in [`TextFontRole::ALL`] order (a
/// browser fetch handed across a wasm door); the buffers live for the process.
pub fn install_text_font_bytes(files: Vec<Vec<u8>>) -> Result<(), TextFontsInstallError> {
    if files.len() != TextFontRole::ALL.len() {
        return Err(TextFontsInstallError::WrongFontCount { expected: TextFontRole::ALL.len(), found: files.len() });
    }
    let leaked: Vec<&'static [u8]> = files.into_iter().map(|bytes| &*Box::leak(bytes.into_boxed_slice())).collect();
    let mut next = leaked.into_iter();
    let fonts = TextFontSet::try_from_loader::<std::convert::Infallible>(|_| Ok(next.next().unwrap_or(&[])));
    match fonts {
        Ok(fonts) => install_text_fonts(fonts),
    }
}

/// 💾️ The native door: reads the fifteen files from `directory`, or when `None` from `SEMIO_TEXT_FONTS`, else from
/// the `🖼️assets/🔤️fonts` catalog found above the working directory or the executable, then installs them.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_text_fonts(directory: Option<&std::path::Path>) -> Result<(), TextFontsInstallError> {
    let directory = match directory {
        Some(directory) => directory.to_path_buf(),
        None => native_text_fonts_directory().ok_or(TextFontsInstallError::Unavailable)?,
    };
    let files = TextFontFiles::read(&directory).map_err(TextFontsInstallError::Unreadable)?;
    let files: &'static TextFontFiles = Box::leak(Box::new(files));
    install_text_fonts(files.set())
}

#[cfg(not(target_arch = "wasm32"))]
fn native_text_fonts_directory() -> Option<std::path::PathBuf> {
    if let Some(directory) = std::env::var_os("SEMIO_TEXT_FONTS") {
        return Some(directory.into());
    }
    let catalog = std::path::Path::new("🧰️framework/🔨️modules/🖼️assets/🔤️fonts");
    let starts = [std::env::current_dir().ok(), std::env::current_exe().ok().and_then(|exe| exe.parent().map(std::path::Path::to_path_buf))];
    starts.into_iter().flatten().find_map(|start| start.ancestors().map(|ancestor| ancestor.join(catalog)).find(|candidate| candidate.join(TextFontRole::Sans.asset_path()).is_file()))
}

/// 📂️ The repository directory the layout package's built-in text font assets are named under: an asset's declared name is the
/// repository-relative path of its file, which is what lets a host resolve it without a registry.
pub const TEXT_FONT_ASSET_ROOT: &str = "🧰️framework/🔨️modules/🖼️assets/🔤️fonts";

const TEXT_FONT_ASSETS: [(TextFontRole, u64, &str); 15] = [
    (TextFontRole::Sans, 26772, "6e9112853217f39bc753c94ab0f16fd7dc57fa56003cfb981b0bb7b0c514487f"),
    (TextFontRole::Serif, 27608, "33c29e564b169dbbc1a90be9f050fcde7fb204232bc4f3c52bf4b844e31dbba6"),
    (TextFontRole::Mono, 19540, "31ff08942c8f4ff98276c050f039a2685d91d923867ea6c776c6a291a086fc13"),
    (TextFontRole::EmojiRegions, 23652, "afbacbae52dbd6a6250f9ffd1e690227564c744508d703d36df14c322816b657"),
    (TextFontRole::EmojiFlags, 8536, "ebbfea501df1f01063a103c99ac6c76a6a239ee45856a57c6730d8c4456803e4"),
    (TextFontRole::EmojiSymbols, 83404, "207ecd3bb593edda35f515399f8d2aad0021d72525ba0a5ac212001f318cc4e5"),
    (TextFontRole::EmojiObjects, 146732, "fa6c50b00fb39b2f984a53fbf03f2ae5c2624045d1738614e80c1e68c59267a7"),
    (TextFontRole::EmojiActivities, 79792, "4eaa8acc313c6fc34aa9a0b6a2ff24ebb8a910568d2c1b3f4fed2aeead58ba15"),
    (TextFontRole::EmojiTravel, 86008, "654c549ae1488f355924e53b9bfd150d672df6b0c053b21b8dd17a2f8a2c6718"),
    (TextFontRole::EmojiFood, 80404, "0bb1b805256ac06bfe079eed3624fd3ffe2117e854c8ab5cafa87baa3778b3ec"),
    (TextFontRole::EmojiNature, 178048, "b107e95dcd571bd40408cf5906f91780171abd9edf4328af54888e95a28549d1"),
    (TextFontRole::EmojiPeople, 150208, "ff5d9966357cff7cf59bae37208e2c7fb473bd538add5e11676be691eba87f42"),
    (TextFontRole::EmojiFaces, 136852, "b972f7d349cd89ac6c6fee0b0eaed39c1656aa51d74e932cac89abd21f00fff9"),
    (TextFontRole::EmojiJoinedForms, 30548, "dfe0fc53375e8bb7f6fbaa639087f905d7aebf07108816d92524e9b3d81b2a50"),
    (TextFontRole::EmojiSupplement, 5444, "5b6d1483840ee81afd7220816e6cca4713692f85626273d1a827f8b878f32a35"),
];

/// 📦️ The declared name of the package asset that carries `role`'s font file.
pub fn text_font_asset_name(role: TextFontRole) -> String {
    format!("{TEXT_FONT_ASSET_ROOT}/{}", role.asset_path())
}

/// 📦️ The fifteen built-in text fonts as package asset declarations (name, exact size, SHA-256), in [`TextFontRole::ALL`]
/// order, for `PluginBuilder::asset`. The host preloads them into `Event::InstanceOpen.assets`.
pub fn text_font_asset_declarations() -> Vec<semio_framework::AssetDeclaration> {
    TEXT_FONT_ASSETS
        .iter()
        .map(|(role, size_bytes, sha256)| semio_framework::AssetDeclaration {
            name: text_font_asset_name(*role),
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Text, form: semio_framework_plugin::MediaForm::Document },
            size_bytes: *size_bytes,
            sha256: (*sha256).to_string(),
        })
        .collect()
}

/// 📥️ The built-in text font set a host delivered as package assets: `read` answers a declared asset name with its bytes
/// (`semio_framework_plugin::reactor::package_asset` in the guest). Fails with [`TextFontsInstallError::PackageAssetMissing`]
/// when the host delivered no such asset, which a caller treats as "this host delivers none".
pub fn package_text_font_set(read: impl Fn(&str) -> Option<&'static [u8]>) -> Result<TextFontSet<'static>, TextFontsInstallError> {
    let mut missing = None;
    let fonts = TextFontSet::try_from_loader(|role| {
        let name = text_font_asset_name(role);
        read(&name).ok_or_else(|| {
            missing = Some(name);
        })
    });
    fonts.map_err(|()| TextFontsInstallError::PackageAssetMissing { name: missing.unwrap_or_default() })
}

/// 📥️ [`package_text_font_set`] followed by [`install_text_fonts`]: what the layout app does when an instance opens.
pub fn install_package_text_fonts(read: impl Fn(&str) -> Option<&'static [u8]>) -> Result<(), TextFontsInstallError> {
    install_text_fonts(package_text_font_set(read)?)
}
//#endregion ⚙️Scene

/// 💾️ Native headless jobs borrow the asset catalog once without registering a UI text system.
#[cfg(not(target_arch = "wasm32"))]
fn native_export_fonts() -> Option<TextFontSet<'static>> {
    static FILES: OnceLock<TextFontFiles> = OnceLock::new();
    cached_font_files(&FILES, || native_text_fonts_directory().and_then(|directory| TextFontFiles::read(&directory).ok())).map(TextFontFiles::set)
}

#[cfg(not(target_arch = "wasm32"))]
fn cached_font_files(files: &OnceLock<TextFontFiles>, load: impl FnOnce() -> Option<TextFontFiles>) -> Option<&TextFontFiles> {
    if files.get().is_none() { let _ = files.set(load()?); }
    files.get()
}

fn export_font_source_from_set<'a>(index: usize, fonts: &TextFontSet<'a>) -> Option<&'a [u8]> {
    if index == 0 { return Some(LAYOUT_SANS); }
    TextFontRole::ALL.get(index - 1).map(|role| fonts.get(*role))
}

fn cached_export_font_set(fonts: &OnceLock<TextFontSet<'static>>, load: impl FnOnce() -> Option<TextFontSet<'static>>) -> Option<&TextFontSet<'static>> {
    if fonts.get().is_none() { let _ = fonts.set(load()?); }
    fonts.get()
}

/// 📚️ Export identities freeze the first available installed or native catalog, with Layout Sans first.
/// Native resolution needs no UI registration; guests borrow their delivered package assets directly.
pub fn export_font_source(index: usize) -> Option<&'static [u8]> {
    static FONTS: OnceLock<TextFontSet<'static>> = OnceLock::new();
    if index == 0 { return Some(LAYOUT_SANS); }
    if index >= export_font_count() { return None; }
    let fonts = cached_export_font_set(&FONTS, || {
        if let Some(fonts) = TEXT_FONTS.get() { return Some(*fonts); }
        #[cfg(not(target_arch = "wasm32"))]
        { native_export_fonts() }
        #[cfg(target_arch = "wasm32")]
        { package_text_font_set(semio_framework_plugin::reactor::package_asset).ok() }
    })?;
    export_font_source_from_set(index, fonts)
}

/// 🔢️ Catalog identities remain fixed before any font source is requested.
pub fn export_font_count() -> usize { 1 + TextFontRole::ALL.len() }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod headless_tests;
