use std::fmt::Write as _;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

fn ranges(pred: impl Fn(char) -> bool) -> Vec<(u32, u32)> {
    let mut out: Vec<(u32, u32)> = Vec::new();
    for cp in 0..=0x10FFFFu32 {
        let Some(c) = char::from_u32(cp) else { continue };
        if pred(c) {
            match out.last_mut() {
                Some(last) if last.1 + 1 == cp => last.1 = cp,
                _ => out.push((cp, cp)),
            }
        }
    }
    out
}

fn emit(name: &str, doc: &str, table: &[(u32, u32)]) -> String {
    let mut s = String::new();
    writeln!(s, "/// {doc}").unwrap();
    writeln!(s, "pub(super) const {name}: &[(u32, u32)] = &[").unwrap();
    let mut line = String::from("   ");
    for (lo, hi) in table {
        let item = format!(" (0x{lo:04x}, 0x{hi:04x}),");
        if line.len() + item.len() > 110 {
            writeln!(s, "{line}").unwrap();
            line = String::from("   ");
        }
        line.push_str(&item);
    }
    if line.trim().len() > 0 {
        writeln!(s, "{line}").unwrap();
    }
    writeln!(s, "];").unwrap();
    s
}

fn main() {
    let wide = ranges(|c| c.width() == Some(2) || c.width() == Some(3));
    let zero = ranges(|c| c.width() == Some(0));
    let vs16 = ranges(|c| c.width() == Some(1) && format!("{c}\u{fe0f}").as_str().width() == 2);
    let emoji_presentation = ranges(|c| c.width() == Some(2) && format!("{c}\u{fe0e}").as_str().width() == 1);
    let emoji_zwj = ranges(|c| c.width() == Some(2) && format!("{c}\u{200d}🔥").as_str().width() == 2);
    let modifier_base = ranges(|c| c.width().is_some_and(|w| w > 0) && !('\u{1f3fb}'..='\u{1f3ff}').contains(&c) && format!("{c}\u{1f3fd}").as_str().width() == 2);
    let mut out = String::new();
    out.push_str(&emit("WIDE", "Scalars of terminal width 2 (East_Asian_Width W/F and wide emoji).", &wide));
    out.push_str(&emit("ZERO", "Scalars of terminal width 0 (Default_Ignorable, Grapheme_Extend, Hangul V/T jamo, prepended marks).", &zero));
    out.push_str(&emit("VS16_BASE", "Width-1 scalars that become width 2 before U+FE0F (emoji presentation sequence bases).", &vs16));
    out.push_str(&emit("VS15_NARROW", "Width-2 emoji scalars that become width 1 before U+FE0E (text presentation sequence bases).", &emoji_presentation));
    out.push_str(&emit("EMOJI_ZWJ", "Width-2 emoji scalars that may start or continue an emoji ZWJ sequence.", &emoji_zwj));
    out.push_str(&emit("EMOJI_MODIFIER_BASE", "Scalars that combine with a skin tone modifier (U+1F3FB..U+1F3FF) into one two-cell glyph.", &modifier_base));
    println!("{out}");
    eprintln!("wide={} zero={} vs16={} vs15={}", wide.len(), zero.len(), vs16.len(), emoji_presentation.len());
    for s in ["🇦", "🇦🇧", "1\u{fe0f}\u{20e3}", "1\u{20e3}", "👨\u{200d}👩\u{200d}👧", "🖥\u{fe0f}", "🖥", "e\u{301}", "क\u{94d}ष", "\r\n", "\u{ad}", "ᄀ\u{1161}\u{11a8}", "👍🏽", "☝🏽", "🏴\u{e0067}\u{e0062}\u{e0065}\u{e006e}\u{e0067}\u{e007f}", "ก\u{e33}", "क\u{93e}", "a\u{200d}b", "\u{200b}"] {
        eprintln!("{:?} -> {}", s, s.width());
    }
}
