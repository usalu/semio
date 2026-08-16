//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::{Plugin, PluginAssemblyError};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the deleted `register_norm_exports`
/// `.setup()` fan-out with fifteen data declarations, one per norm family. `.setup()` is gone
/// entirely (W1d): the one remaining reason, the shared `NormConfig` config/presence schema every
/// one of the fifteen `PlayApp`s uses, is now an `ArtifactApp::app_schema()` override on each of the
/// fifteen — all fifteen return the identical `crate::config::schema::app_schema_descriptor()`
/// literal, and `register_document_app` (called once per `.document_app::<…>()` below)
/// registers it, mirroring the `🗒️note` exemplar exactly.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    let din4108 = crate::artifacts::din4108::declaration(crate::artifacts::din4108::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let din16798 = crate::artifacts::din16798::declaration(crate::artifacts::din16798::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let din18599 = crate::artifacts::din18599::declaration(crate::artifacts::din18599::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1990 = crate::artifacts::en1990::declaration(crate::artifacts::en1990::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1991 = crate::artifacts::en1991::declaration(crate::artifacts::en1991::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1992 = crate::artifacts::en1992::declaration(crate::artifacts::en1992::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1993 = crate::artifacts::en1993::declaration(crate::artifacts::en1993::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1994 = crate::artifacts::en1994::declaration(crate::artifacts::en1994::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1995 = crate::artifacts::en1995::declaration(crate::artifacts::en1995::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1996 = crate::artifacts::en1996::declaration(crate::artifacts::en1996::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1997 = crate::artifacts::en1997::declaration(crate::artifacts::en1997::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1998 = crate::artifacts::en1998::declaration(crate::artifacts::en1998::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1999 = crate::artifacts::en1999::declaration(crate::artifacts::en1999::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let iso16757 = crate::artifacts::iso16757::declaration(crate::artifacts::iso16757::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let vdi3805 = crate::artifacts::vdi3805::declaration(crate::artifacts::vdi3805::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    Plugin::builder("norm")
        .label("Norm")
        .version("0.1.0")
        .artifact(din4108)
        .artifact(din16798)
        .artifact(din18599)
        .artifact(en1990)
        .artifact(en1991)
        .artifact(en1992)
        .artifact(en1993)
        .artifact(en1994)
        .artifact(en1995)
        .artifact(en1996)
        .artifact(en1997)
        .artifact(en1998)
        .artifact(en1999)
        .artifact(iso16757)
        .artifact(vdi3805)
        .document_app::<crate::apps::din4108::Din4108PlayApp>(crate::apps::din4108::create_din4108_app())
        .document_app::<crate::apps::din16798::Din16798PlayApp>(crate::apps::din16798::create_din16798_app())
        .document_app::<crate::apps::din18599::Din18599PlayApp>(crate::apps::din18599::create_din18599_app())
        .document_app::<crate::apps::en1990::En1990PlayApp>(crate::apps::en1990::create_en1990_app())
        .document_app::<crate::apps::en1991::En1991PlayApp>(crate::apps::en1991::create_en1991_app())
        .document_app::<crate::apps::en1992::En1992PlayApp>(crate::apps::en1992::create_en1992_app())
        .document_app::<crate::apps::en1993::En1993PlayApp>(crate::apps::en1993::create_en1993_app())
        .document_app::<crate::apps::en1994::En1994PlayApp>(crate::apps::en1994::create_en1994_app())
        .document_app::<crate::apps::en1995::En1995PlayApp>(crate::apps::en1995::create_en1995_app())
        .document_app::<crate::apps::en1996::En1996PlayApp>(crate::apps::en1996::create_en1996_app())
        .document_app::<crate::apps::en1997::En1997PlayApp>(crate::apps::en1997::create_en1997_app())
        .document_app::<crate::apps::en1998::En1998PlayApp>(crate::apps::en1998::create_en1998_app())
        .document_app::<crate::apps::en1999::En1999PlayApp>(crate::apps::en1999::create_en1999_app())
        .document_app::<crate::apps::iso16757::Iso16757PlayApp>(crate::apps::iso16757::create_iso16757_app())
        .document_app::<crate::apps::vdi3805::Vdi3805PlayApp>(crate::apps::vdi3805::create_vdi3805_app())
        .try_build()
}
