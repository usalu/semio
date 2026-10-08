use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn json_string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c if matches!(c as u32, 0x200b..=0x200f | 0x2028..=0x202e | 0x2060..=0x206f | 0xfe00..=0xfe0f | 0xfeff | 0xad | 0x20e3 | 0x0300..=0x036f) => out.push_str(&format!("\\u{:04x}", c as u32)),
            c if (c as u32) >= 0xe0000 && (c as u32) <= 0xe01ef => {
                let mut units = [0u16; 2];
                for unit in c.encode_utf16(&mut units) {
                    out.push_str(&format!("\\u{:04x}", unit));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main() {
    let segments: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap().lines().map(str::to_string).collect();
    let mut cases: Vec<(String, String)> = Vec::new();
    let mut add = |name: &str, text: &str| cases.push((name.to_string(), text.to_string()));
    add("ascii word", "semio");
    add("ascii sentence", "dashboard: 12 tasks");
    add("empty", "");
    add("cjk ideographs", "界面");
    add("fullwidth latin", "ｈｅｌｌｏ");
    add("hiragana", "ひらがな");
    add("combining acute", "e\u{301}");
    add("combining tilde word", "n\u{303}a");
    add("stacked combining marks", "Z\u{361}\u{361}A");
    add("hangul syllables", "한국어");
    add("hangul conjoining jamo", "\u{1100}\u{1161}\u{11a8}");
    add("devanagari conjunct", "क्षत्रिय");
    add("devanagari spacing mark", "हिन्दी");
    add("thai sara am", "กำลัง");
    add("arabic", "مرحبا");
    add("emoji presentation", "🙂");
    add("toolbox", "🧰");
    add("toolbox with vs16", "🧰\u{fe0f}");
    add("desktop computer text default", "🖥");
    add("desktop computer vs16", "🖥\u{fe0f}");
    add("keyboard text default", "⌨");
    add("keyboard vs16", "⌨\u{fe0f}");
    add("atom symbol vs16", "⚛\u{fe0f}");
    add("ballot box vs16", "☑\u{fe0f}");
    add("pencil vs16", "✏\u{fe0f}");
    add("warning vs16", "⚠\u{fe0f}");
    add("heart text default", "❤");
    add("heart vs16", "❤\u{fe0f}");
    add("sun vs16", "☀\u{fe0f}");
    add("crescent moon vs16", "🌙\u{fe0f}");
    add("watch vs15 text presentation", "⌚\u{fe0e}");
    add("keycap one", "1\u{fe0f}\u{20e3}");
    add("keycap number sign", "#\u{fe0f}\u{20e3}");
    add("keycap asterisk", "*\u{fe0f}\u{20e3}");
    add("enclosing keycap without vs16", "1\u{20e3}");
    add("flag switzerland", "🇨🇭");
    add("lone regional indicator", "🇨");
    add("two flags", "🇨🇭🇩🇪");
    add("three regional indicators", "🇨🇭🇩");
    add("thumbs up skin tone", "👍🏽");
    add("raised hand text default skin tone", "☝🏽");
    add("family zwj", "👨\u{200d}👩\u{200d}👧\u{200d}👦");
    add("technologist zwj", "🧑\u{200d}💻");
    add("artist zwj", "🧑\u{200d}🎨");
    add("england flag tag sequence", "🏴\u{e0067}\u{e0062}\u{e0065}\u{e006e}\u{e0067}\u{e007f}");
    add("rainbow flag zwj with vs16", "🏳\u{fe0f}\u{200d}🌈");
    add("heart on fire", "❤\u{fe0f}\u{200d}🔥");
    add("broken chain", "⛓\u{fe0f}\u{200d}💥");
    add("zero width space", "a\u{200b}b");
    add("soft hyphen", "a\u{ad}b");
    add("lone zero width joiner", "\u{200d}");
    add("lone variation selector 16", "\u{fe0f}");
    add("leading variation selector 16 word", "\u{fe0f}w2-log.json");
    add("repo path toolbox framework", "🧰\u{fe0f}framework");
    add("repo path modules", "🔨\u{fe0f}modules");
    add("repo path ui", "🖱\u{fe0f}ui");
    add("repo path tui", "⌨\u{fe0f}tui");
    add("repo path dashboard", "🎛\u{fe0f}dashboard");
    add("repo path products", "🛍\u{fe0f}products");
    add("repo path repo", "🦑\u{fe0f}repo");
    add("repo path command tree", "🌳\u{fe0f}command-tree");
    add("repo path windows", "🪟\u{fe0f}windows");
    add("repo path host", "🏃\u{fe0f}host");
    add("repo path fixtures", "🧫\u{fe0f}fixtures");
    add("repo path feature", "🥒\u{fe0f}.feature");
    add("repo path json", "🔣\u{fe0f}.json");
    add("repo path rust", "🦀\u{fe0f}.rs");
    add("repo path year", "🎆\u{fe0f}26");
    add("repo path month", "🌙\u{fe0f}09");
    add("repo path day", "☀\u{fe0f}23");
    add("repo path react", "⚛\u{fe0f}react");
    add("repo path renderer", "📺\u{fe0f}renderer");
    add("repo path engine artist zwj", "🧑\u{200d}🎨engine");
    add("repo path full", "🧰\u{fe0f}framework/🔨\u{fe0f}modules/🖱\u{fe0f}ui/⌨\u{fe0f}tui");
    add("status line", "✓ ui-tui · 12 tasks");
    for segment in &segments {
        cases.push((format!("repo segment {segment}"), segment.clone()));
    }
    println!("{{\n  \"schema\": \"text-width\",\n  \"oracle\": \"unicode-width 0.2.2 and unicode-segmentation 1.13.2 at Unicode 17.0.0\",\n  \"cases\": [");
    for (index, (name, text)) in cases.iter().enumerate() {
        let clusters: Vec<String> = text.graphemes(true).map(json_string).collect();
        let comma = if index + 1 == cases.len() { "" } else { "," };
        println!("    {{ \"name\": {}, \"text\": {}, \"clusters\": [{}], \"cells\": {} }}{comma}", json_string(name), json_string(text), clusters.join(", "), text.width());
    }
    println!("  ]\n}}");
}
