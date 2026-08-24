#!/usr/bin/env python3
"""📐 Complete ISO 10303-21 §6.4.2's string escapes in the shared Part-21 lexer.

`read_escape` implemented `\\X\\HH\\` and `\\X2\\…\\X0\\` and nothing else, so the doubled reverse
solidus the STRING production itself defines — `'\\\\'`, one literal backslash — was a hard parse
error.  The real committed IfcOpenShell export
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc` contains exactly
that at byte 138718 (`#966=IFCBUILDINGELEMENTPROXYTYPE(…,'\\\\',…)`), and it cost ALL 22 executable
`mutate-ifc-4` subject scenarios; `ruststep`, the registered independent reader, parses it fine.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14.
"""
PATH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📐️part21/🦀️component.rs"

with open(PATH, encoding="utf-8") as handle:
    text = handle.read()


def sub(old: str, new: str, count: int = 1) -> None:
    global text
    found = text.count(old)
    assert found == count, f"expected {count}, found {found}:\n{old[:300]}"
    text = text.replace(old, new)


# 1️⃣ the string reader carries the alphabet state `\P?\` selects and `\S\` shifts into
sub(
    """        self.pos += 1;
        let mut out = String::new();
        loop {
            match self.bump() {
                None => return Err(Part21Error::UnexpectedEof { at: self.pos, expected: "closing '" }),
                Some('\\'') => {
                    if self.peek() == Some('\\'') {
                        self.pos += 1;
                        out.push('\\'');
                    } else {
                        break;
                    }
                }
                Some('\\\\') => self.read_escape(&mut out)?,
                Some(c) => out.push(c),
            }
        }
        Ok(out)
    }""",
    """        self.pos += 1;
        let mut out = String::new();
        // 📖 §6.4.2's `alphabet` directive holds until the end of THIS string literal, so the
        // selected ISO 8859 part is per-literal state rather than per-document.
        let mut alphabet = 'A';
        loop {
            match self.bump() {
                None => return Err(Part21Error::UnexpectedEof { at: self.pos, expected: "closing '" }),
                Some('\\'') => {
                    if self.peek() == Some('\\'') {
                        self.pos += 1;
                        out.push('\\'');
                    } else {
                        break;
                    }
                }
                Some('\\\\') => self.read_escape(&mut out, &mut alphabet)?,
                Some(c) => out.push(c),
            }
        }
        Ok(out)
    }""",
)

# 2️⃣ the escape reader itself
old_escape = text[text.index("    /// 🔤️ `\\X\\HH\\` single byte") : text.index("    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9\n    fn read_number")]
new_escape = r'''    /// 🔤️ ISO 10303-21 §6.4.2's COMPLETE `control_directive` set, plus the STRING production's own
    /// doubled reverse solidus — called with the opening `\` already consumed.
    ///
    /// * `\\` — one literal REVERSE SOLIDUS. The string production escapes it by doubling exactly
    ///   as it escapes the apostrophe by doubling, and real exporters emit it: IfcOpenShell writes
    ///   `'\\'` for a one-character backslash name at byte 138718 of the committed
    ///   `🏗️nakagin-capsule-tower.ifc`. Rejecting it failed EVERY `mutate-ifc-4` subject scenario
    ///   (22 of 22 executable rows) until this wave, while `ruststep` — the registered independent
    ///   reader for that same case — read the file without complaint.
    /// * `\X\HH` — `arbitrary`: EXACTLY two hex digits and no terminator (`arbitrary = "\X\"
    ///   hex_one`). This lexer used to demand a trailing `\`, which would mis-parse a conformant
    ///   `\X\41\S\A` by eating the next directive's own opener.
    /// * `\X2\HHHH…\X0\` / `\X4\HHHHHHHH…\X0\` — `extended2`/`extended4` UCS-2 / UCS-4 runs.
    /// * `\S\c` — `page`: the character at `code(c) + 128` in the selected alphabet.
    /// * `\PA\`…`\PI\` — `alphabet`: which ISO 8859 part `\S\` shifts into. Only `A` (ISO 8859-1,
    ///   the one part where `code + 128` IS the Unicode codepoint) is decoded; `B`..`I` need
    ///   per-part mapping tables this codec does not carry, and a typed error naming the page is
    ///   the honest answer rather than a silently wrong character.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_escape(&mut self, out: &mut String, alphabet: &mut char) -> Result<(), Part21Error> {
        let start = self.pos - 1;
        match self.bump() {
            Some('\\') => {
                out.push('\\');
                Ok(())
            }
            Some('P') => {
                let selected = match self.bump() {
                    Some(c @ 'A'..='I') => c,
                    other => return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\P alphabet {other:?}") }),
                };
                if self.bump() != Some('\\') {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\P".into() });
                }
                *alphabet = selected;
                Ok(())
            }
            Some('S') => {
                if self.bump() != Some('\\') {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\S".into() });
                }
                if *alphabet != 'A' {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("\\S\\ on ISO 8859 page {alphabet} needs a mapping table this codec does not carry") });
                }
                match self.bump() {
                    Some(c) if (c as u32) < 0x80 => {
                        out.push(char::from_u32(c as u32 + 128).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: "bad \\S\\ character".into() })?);
                        Ok(())
                    }
                    other => Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\S\\ character {other:?}") }),
                }
            }
            Some('X') => match self.peek() {
                Some(width) if width == '2' || width == '4' => {
                    let group = if width == '2' { 4 } else { 8 };
                    self.pos += 1;
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("expected \\ after \\X{width}") });
                    }
                    loop {
                        let mut hex = String::new();
                        for _ in 0..group {
                            match self.bump() {
                                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                                _ => return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\X{width}\\ hex group") }),
                            }
                        }
                        let code = u32::from_str_radix(&hex, 16).map_err(|_| Part21Error::UnsupportedEscape { at: start, detail: "bad hex".into() })?;
                        let ch = char::from_u32(code).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: format!("bad codepoint {code}") })?;
                        out.push(ch);
                        if self.peek() == Some('\\') && self.peek_at(1) == Some('X') && self.peek_at(2) == Some('0') {
                            self.pos += 3;
                            if self.bump() != Some('\\') {
                                return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected trailing \\ after \\X0".into() });
                            }
                            break;
                        }
                    }
                    Ok(())
                }
                _ => {
                    if self.bump() != Some('\\') {
                        return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\X".into() });
                    }
                    let mut hex = String::new();
                    for _ in 0..2 {
                        match self.bump() {
                            Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                            _ => return Err(Part21Error::UnsupportedEscape { at: start, detail: "bad \\X\\ hex".into() }),
                        }
                    }
                    let code = u32::from_str_radix(&hex, 16).map_err(|_| Part21Error::UnsupportedEscape { at: start, detail: "bad hex".into() })?;
                    let ch = char::from_u32(code).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: format!("bad byte {code}") })?;
                    out.push(ch);
                    Ok(())
                }
            },
            other => Err(Part21Error::UnsupportedEscape { at: start, detail: format!("unsupported escape start {other:?}") }),
        }
    }

'''
text = text.replace(old_escape, new_escape)
assert new_escape in text

with open(PATH, "w", encoding="utf-8") as handle:
    handle.write(text)
print(f"patched {PATH}")
