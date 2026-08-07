//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("norm")
        .label("Norm")
        .version("0.1.0")
        .setup(crate::register_norm_exports)
        .register_document_app::<crate::apps::din4108::Din4108PlayApp>(crate::apps::din4108::create_din4108_app())
        .register_document_app::<crate::apps::din16798::Din16798PlayApp>(crate::apps::din16798::create_din16798_app())
        .register_document_app::<crate::apps::din18599::Din18599PlayApp>(crate::apps::din18599::create_din18599_app())
        .register_document_app::<crate::apps::en1990::En1990PlayApp>(crate::apps::en1990::create_en1990_app())
        .register_document_app::<crate::apps::en1991::En1991PlayApp>(crate::apps::en1991::create_en1991_app())
        .register_document_app::<crate::apps::en1992::En1992PlayApp>(crate::apps::en1992::create_en1992_app())
        .register_document_app::<crate::apps::en1993::En1993PlayApp>(crate::apps::en1993::create_en1993_app())
        .register_document_app::<crate::apps::en1994::En1994PlayApp>(crate::apps::en1994::create_en1994_app())
        .register_document_app::<crate::apps::en1995::En1995PlayApp>(crate::apps::en1995::create_en1995_app())
        .register_document_app::<crate::apps::en1996::En1996PlayApp>(crate::apps::en1996::create_en1996_app())
        .register_document_app::<crate::apps::en1997::En1997PlayApp>(crate::apps::en1997::create_en1997_app())
        .register_document_app::<crate::apps::en1998::En1998PlayApp>(crate::apps::en1998::create_en1998_app())
        .register_document_app::<crate::apps::en1999::En1999PlayApp>(crate::apps::en1999::create_en1999_app())
        .register_document_app::<crate::apps::iso16757::Iso16757PlayApp>(crate::apps::iso16757::create_iso16757_app())
        .register_document_app::<crate::apps::vdi3805::Vdi3805PlayApp>(crate::apps::vdi3805::create_vdi3805_app())
        .build()
}
