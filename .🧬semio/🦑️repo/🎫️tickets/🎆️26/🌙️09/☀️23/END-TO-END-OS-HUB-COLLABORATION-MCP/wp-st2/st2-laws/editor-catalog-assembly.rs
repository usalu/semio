/// 🧩️ Assembles every stdio package once: assembly publishes the artifact document schemas an editor's snapshot edits
/// resolve (`snapshot-edit.schema-unregistered` otherwise), exactly as a host that loaded the stdio packages has them.
fn stdio_packages_assembled() {
    static ASSEMBLED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    ASSEMBLED.get_or_init(|| {
        semio_s_plugin_stdio::plugin().expect("stdio assembles");
        semio_s_plugin_stdio_image::plugin().expect("stdio-image assembles");
        semio_s_plugin_stdio_media::plugin().expect("stdio-media assembles");
        semio_s_plugin_stdio_cad::plugin().expect("stdio-cad assembles");
        semio_s_plugin_stdio_bim::plugin().expect("stdio-bim assembles");
        semio_s_plugin_stdio_mesh::plugin().expect("stdio-mesh assembles");
        semio_s_plugin_stdio_pdf::plugin().expect("stdio-pdf assembles");
        semio_s_plugin_stdio_office::plugin().expect("stdio-office assembles");
        semio_s_plugin_stdio_semio::plugin().expect("stdio-semio assembles");
        semio_s_plugin_stdio_binary::plugin().expect("stdio-binary assembles");
    });
}

