// 🧪 Standalone scratch validation for Part-21 string escape/unescape (ISO 10303-21).
// Run: rustc -O escape_scratch.rs -o /tmp/escape_scratch && /tmp/escape_scratch
// Purpose: validate the exact algorithm before porting into the real crate module,
// per the ticket's mandated "standalone-scratch-crate" technique for escape-handling logic.

fn unescape_part21_string(body: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut chars = body.chars().peekable();
    loop {
        match chars.next() {
            None => break,
            Some('\'') => {
                if chars.peek() == Some(&'\'') {
                    chars.next();
                    out.push('\'');
                } else {
                    return Err(format!("unexpected bare quote in body: {body:?}"));
                }
            }
            Some('\\') => {
                match chars.next() {
                    Some('X') => match chars.peek() {
                        Some('2') => {
                            chars.next();
                            if chars.next() != Some('\\') {
                                return Err("expected \\ after \\X2".into());
                            }
                            loop {
                                let mut hex = String::new();
                                for _ in 0..4 {
                                    match chars.next() {
                                        Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                                        _ => return Err("bad \\X2\\ hex group".into()),
                                    }
                                }
                                let code = u32::from_str_radix(&hex, 16).map_err(|e| e.to_string())?;
                                let ch = char::from_u32(code).ok_or_else(|| format!("bad codepoint {code}"))?;
                                out.push(ch);
                                // terminator check: \X0\
                                let save: Vec<char> = chars.clone().take(4).collect();
                                if save.len() >= 3 && save[0] == '\\' && save[1] == 'X' && save[2] == '0' {
                                    chars.next(); // backslash
                                    chars.next(); // X
                                    chars.next(); // 0
                                    if chars.next() != Some('\\') {
                                        return Err("expected trailing \\ after \\X0".into());
                                    }
                                    break;
                                }
                            }
                        }
                        _ => {
                            if chars.next() != Some('\\') {
                                return Err("expected \\ after \\X".into());
                            }
                            let mut hex = String::new();
                            for _ in 0..2 {
                                match chars.next() {
                                    Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                                    _ => return Err("bad \\X\\ hex".into()),
                                }
                            }
                            if chars.next() != Some('\\') {
                                return Err("expected trailing \\ after \\X..".into());
                            }
                            let code = u32::from_str_radix(&hex, 16).map_err(|e| e.to_string())?;
                            out.push(char::from_u32(code).ok_or("bad byte")?);
                        }
                    },
                    other => return Err(format!("unsupported escape start: {other:?}")),
                }
            }
            Some(c) => out.push(c),
        }
    }
    Ok(out)
}

fn escape_part21_string(s: &str) -> String {
    let mut out = String::new();
    let mut run: Vec<char> = Vec::new();
    let flush = |run: &mut Vec<char>, out: &mut String| {
        if run.is_empty() {
            return;
        }
        out.push_str("\\X2\\");
        for c in run.drain(..) {
            out.push_str(&format!("{:04X}", c as u32));
        }
        out.push_str("\\X0\\");
    };
    for c in s.chars() {
        if c == '\'' {
            flush(&mut run, &mut out);
            out.push_str("''");
        } else if c == '\\' {
            flush(&mut run, &mut out);
            // escape the backslash itself to avoid ambiguity with \X.. sequences on reparse
            run.push(c);
            flush(&mut run, &mut out);
        } else if (0x20..=0x7E).contains(&(c as u32)) {
            flush(&mut run, &mut out);
            out.push(c);
        } else {
            run.push(c);
        }
    }
    flush(&mut run, &mut out);
    out
}

fn check(label: &str, raw: &str) {
    let escaped = escape_part21_string(raw);
    let back = unescape_part21_string(&escaped).expect("unescape must succeed");
    assert_eq!(back, raw, "round trip mismatch for {label}: escaped={escaped:?}");
    println!("OK  {label:<28} raw={raw:?} escaped={escaped:?}");
}

fn main() {
    check("plain ascii", "hello world");
    check("single quote", "it's a test");
    check("double single quote", "''already doubled''");
    check("backslash literal", "a\\b");
    check("backslash then X", "\\Xnotreal\\");
    check("unicode euro", "price: \u{20AC}5");
    check("unicode cjk", "\u{4E2D}\u{6587}");
    check("empty string", "");
    check("mixed", "Fenêtre \u{00E9}t\u{00E9} 'quoted' back\\slash");
    check("control char", "line1\nline2\ttab");

    // direct escape-form parse checks (values as would appear literally in a real file)
    assert_eq!(unescape_part21_string("it''s").unwrap(), "it's");
    assert_eq!(unescape_part21_string("\\X2\\00E9\\X0\\").unwrap(), "\u{00E9}");
    assert_eq!(unescape_part21_string("\\X2\\4E2D6587\\X0\\").unwrap(), "\u{4E2D}\u{6587}");
    assert_eq!(unescape_part21_string("\\X\\41\\").unwrap(), "A");

    println!("ALL PART21 ESCAPE SCRATCH CHECKS PASSED");
}
