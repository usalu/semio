# Boot Schema Registration Qualification

Native620 reported an unnecessary qualification in the plugin framework's boot schema registration helper. The enclosing app module already imports the UI contract exports, so the compiler recommended `schema_metadata::register_scope_exports()`. Applied that exact shortening; the invoked registration function and its behavior are unchanged.

Changed file: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.

The fresh compiler recheck is pending. No lint allowances were added.
