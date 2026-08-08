//! 🔧️ Setup facet for `📕️norm` — codec/language registration hooked via `.setup(...)`.

/// 🔌️ Registers every norm artifact's handcrafted languages and shared config codec.
pub fn register_norm_exports() {
    crate::artifacts::din4108::engine::register_pilot_languages();
    crate::artifacts::din16798::engine::register_pilot_languages();
    crate::artifacts::din18599::engine::register_pilot_languages();
    crate::artifacts::en1990::engine::register_pilot_languages();
    crate::artifacts::en1991::engine::register_pilot_languages();
    crate::artifacts::en1992::engine::register_pilot_languages();
    crate::artifacts::en1993::engine::register_pilot_languages();
    crate::artifacts::en1994::engine::register_pilot_languages();
    crate::artifacts::en1995::engine::register_pilot_languages();
    crate::artifacts::en1996::engine::register_pilot_languages();
    crate::artifacts::en1997::engine::register_pilot_languages();
    crate::artifacts::en1998::engine::register_pilot_languages();
    crate::artifacts::en1999::engine::register_pilot_languages();
    crate::artifacts::iso16757::engine::register_pilot_languages();
    crate::artifacts::vdi3805::engine::register_pilot_languages();
}
