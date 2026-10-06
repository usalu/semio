/// 🔎️ One borrowed icon source resolved by the owner-local shortcode tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IconShortcodeMatch<'a> {
    Emoji(&'a str),
    Themed(&'a str),
    Catalog(&'a str),
}

/// 🔎️ The defining generator supplies unique tables in UTF-8 key order.
pub(crate) fn resolve_icon_shortcode<'a>(code: &str, tables: [&'a [(&'a str, &'a str)]; 3]) -> Option<IconShortcodeMatch<'a>> {
    let key = code.trim();
    if key.is_empty() { return None; }
    for (index, table) in tables.into_iter().enumerate() {
        if let Ok(position) = table.binary_search_by(|(name, _)| name.cmp(&key)) {
            let value = table[position].1;
            return Some(match index { 0 => IconShortcodeMatch::Emoji(value), 1 => IconShortcodeMatch::Themed(value), _ => IconShortcodeMatch::Catalog(value) });
        }
    }
    None
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
