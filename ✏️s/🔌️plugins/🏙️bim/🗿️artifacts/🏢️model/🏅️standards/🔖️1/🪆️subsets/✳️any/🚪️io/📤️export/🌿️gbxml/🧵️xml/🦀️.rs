//! 🧵️ Drives the canonical gbXML element grammar into borrowed byte pages.

use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

const DECLARATION: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
const SPACES: &[u8] = &[b' '; 128];
const DEPTH: usize = 16;
const ATTRIBUTES: usize = 16;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Phase {
    #[default]
    Open,
    AttributeLength,
    LeafLength,
    AttributeSpacing,
    AttributeName,
    Equals,
    AttributeValue,
    Quote,
    OpenEnd,
    Text,
    NextChild,
    ChildSpacing,
    CloseSpacing,
    CloseName,
    CloseEnd,
}

#[derive(Clone, Copy, Default)]
struct ElementCursor {
    phase: Phase,
    attribute: usize,
    child: usize,
    offset: usize,
    escaped_offset: usize,
    width: usize,
    closing_width: usize,
    newline: bool,
}

/// 🧾️ Fixed-depth encoder for the element-only children and single-text leaves of the gbXML tree.
#[derive(Clone)]
pub struct GbxmlTextCursor {
    stack: [ElementCursor; DEPTH],
    path: [usize; DEPTH],
    depth: usize,
    declaration: usize,
    column: usize,
    ended: bool,
    complete: bool,
}

impl Default for GbxmlTextCursor {
    fn default() -> Self {
        Self { stack: [ElementCursor::default(); DEPTH], path: [0; DEPTH], depth: 0, declaration: 0, column: 0, ended: false, complete: false }
    }
}

impl GbxmlTextCursor {
    /// 📏️ Whether the final line feed has been produced.
    pub fn is_complete(&self) -> bool { self.complete }

    /// ✍️ Writes at most one page and performs at most `work` lexical operations.
    pub fn write(&mut self, root: &XmlNode, output: &mut [u8], work: usize) -> Result<usize, &'static str> {
        self.write_source(root, output, work)
    }

    /// 📡️ Emits from the original borrowed tree view without needing an owned document.
    pub fn write_source(&mut self, source: &impl GbxmlTreeSource, output: &mut [u8], work: usize) -> Result<usize, &'static str> {
        let mut tail = Page { bytes: output, written: 0, column: &mut self.column };
        let mut steps = 0;
        while steps < work && !tail.is_full() && !self.complete {
            if self.declaration < DECLARATION.len() {
                steps += 1;
                tail.copy(DECLARATION, &mut self.declaration);
                continue;
            }
            if self.ended {
                steps += 1;
                let mut offset = 0;
                self.complete = tail.copy(b"\n", &mut offset);
                continue;
            }
            let depth = self.depth;
            let movement = source.with_element(&self.path[..depth], |node| -> Result<Movement, &'static str> {
                while steps < work && !tail.is_full() {
                    steps += 1;
                    let movement = step(&mut self.stack[depth], depth, node, &mut tail)?;
                    if !matches!(movement, Movement::Stay) { return Ok(movement); }
                }
                Ok(Movement::Stay)
            })?;
            match movement? {
                Movement::Stay => (),
                Movement::Probe(child) => {
                    self.path[self.depth] = child;
                    if source.has_element(&self.path[..=depth])? {
                        self.stack[depth].phase = Phase::ChildSpacing;
                    } else {
                        self.stack[depth].child += 1;
                    }
                }
                Movement::Child(child) => {
                    if self.depth + 1 == DEPTH { return Err("gbXML element nesting exceeds the encoder depth"); }
                    self.path[self.depth] = child;
                    self.depth += 1;
                    self.stack[self.depth] = ElementCursor::default();
                }
                Movement::Parent if self.depth == 0 => self.ended = true,
                Movement::Parent => self.depth -= 1,
            }
        }
        Ok(tail.written)
    }
}

enum Movement { Stay, Probe(usize), Child(usize), Parent }

/// 🔖️ One quoted gbXML attribute.
#[derive(Clone, Copy, Default)]
pub struct GbxmlAttribute<'a> { pub name: &'a str, pub value: GbxmlText<'a> }

/// 🧷️ One original text fragment projected through its gbXML normalization.
#[derive(Clone, Copy)]
pub enum GbxmlTextPart<'a> { Text(&'a str), Name(&'a str), Label(&'a str) }

/// 🔤️ A borrowed string or an ordered set of borrowed fragments.
#[derive(Clone, Copy)]
pub enum GbxmlText<'a> { Text(&'a str), Parts(&'a [GbxmlTextPart<'a>]) }

impl Default for GbxmlText<'_> { fn default() -> Self { Self::Text("") } }

impl GbxmlText<'_> {
    pub(crate) fn next(self, mut offset: usize) -> Option<(Option<char>, usize)> {
        let parts;
        let source = match self {
            Self::Text(source) => { parts = [GbxmlTextPart::Text(source)]; &parts[..] }
            Self::Parts(parts) => parts,
        };
        for part in source {
            let raw = match part { GbxmlTextPart::Text(raw) | GbxmlTextPart::Name(raw) | GbxmlTextPart::Label(raw) => raw };
            if offset >= raw.len() { offset -= raw.len(); continue; }
            let character = raw.get(offset..)?.chars().next()?;
            let filtered = match part {
                GbxmlTextPart::Name(_) if !character.is_ascii_alphanumeric() && !matches!(character, '_' | '.' | '-') => Some('_'),
                GbxmlTextPart::Label(_) if character.is_control() && !matches!(character, '\t' | '\n' | '\r') => None,
                _ => Some(character),
            };
            return Some((filtered, character.len_utf8()));
        }
        None
    }
}

/// 🧩️ One element in the gbXML element-only tree.
#[derive(Clone, Copy)]
pub struct GbxmlElement<'a> {
    pub name: &'a str,
    pub attributes: &'a [GbxmlAttribute<'a>],
    pub contents: GbxmlContent<'a>,
}

/// 📃️ The three element content forms allowed by the gbXML writer.
#[derive(Clone, Copy)]
pub enum GbxmlContent<'a> { Text(GbxmlText<'a>), Elements(usize), Slots(usize), Empty }

/// 🌳️ Lends a shallow element view at a path for one lexical operation.
pub trait GbxmlTreeSource {
    fn with_element<R>(&self, path: &[usize], read: impl FnOnce(GbxmlElement<'_>) -> R) -> Result<R, &'static str>;
    fn has_element(&self, path: &[usize]) -> Result<bool, &'static str> {
        self.with_element(path, |_| true)
    }
}

impl GbxmlTreeSource for XmlNode {
    fn with_element<R>(&self, path: &[usize], read: impl FnOnce(GbxmlElement<'_>) -> R) -> Result<R, &'static str> {
        let mut node = self;
        for &index in path {
            let XmlNode::Element { children, .. } = node else { return Err("gbXML path traverses a text node"); };
            node = children.get(index).ok_or("gbXML path refers to an absent child")?;
        }
        let XmlNode::Element { name, attrs, children } = node else { return Err("gbXML requires an element at this path"); };
        let contents = match children.as_slice() {
            [] => GbxmlContent::Empty,
            [XmlNode::Text { text }] => GbxmlContent::Text(GbxmlText::Text(text)),
            _ => GbxmlContent::Elements(children.len()),
        };
        let mut attributes = [GbxmlAttribute::default(); ATTRIBUTES];
        let active = attributes.get_mut(..attrs.len()).ok_or("gbXML element has too many attributes")?;
        for (target, XmlAttr { name, value }) in active.iter_mut().zip(attrs) {
            *target = GbxmlAttribute { name, value: GbxmlText::Text(value) };
        }
        Ok(read(GbxmlElement { name, attributes: active, contents }))
    }
}

fn step(cursor: &mut ElementCursor, depth: usize, node: GbxmlElement<'_>, page: &mut Page<'_>) -> Result<Movement, &'static str> {
    let GbxmlElement { name, attributes: attrs, contents } = node;
    let leaf = match contents { GbxmlContent::Text(text) => Some(text), _ => None };
    let mut next = cursor.phase;
    match cursor.phase {
        Phase::Open => {
            if cursor.offset == 0 && !page.copy(b"<", &mut cursor.offset) { return Ok(Movement::Stay); }
            if page.copy_shifted(name.as_bytes(), 1, &mut cursor.offset) {
                next = if attrs.is_empty() { Phase::OpenEnd } else { Phase::AttributeLength };
            }
        }
        Phase::AttributeLength => {
            let attribute = &attrs[cursor.attribute];
            if cursor.offset == 0 {
                cursor.width = attribute.name.len().saturating_add(3);
                cursor.closing_width = 0;
            }
            if let Some((character, bytes)) = attribute.value.next(cursor.offset) {
                if let Some(character) = character {
                    cursor.width = cursor.width.saturating_add(escaped(character, true).map_or(character.len_utf8(), |part| part.len()));
                }
                cursor.offset += bytes;
                return Ok(Movement::Stay);
            }
            if cursor.attribute + 1 == attrs.len() {
                if leaf.is_some() {
                    cursor.closing_width = name.len().saturating_add(4);
                    next = Phase::LeafLength;
                } else {
                    cursor.closing_width = if matches!(contents, GbxmlContent::Empty) { 2 } else { 1 };
                    next = Phase::AttributeSpacing;
                }
            } else {
                next = Phase::AttributeSpacing;
            }
        }
        Phase::LeafLength => {
            let text = leaf.expect("leaf width is measured only for text");
            if let Some((character, bytes)) = text.next(cursor.offset) {
                if let Some(character) = character {
                    cursor.closing_width = cursor.closing_width.saturating_add(escaped(character, false).map_or(1, |part| part.len()));
                }
                cursor.offset += bytes;
                return Ok(Movement::Stay);
            }
            next = Phase::AttributeSpacing;
        }
        Phase::AttributeSpacing => {
            if cursor.offset == 0 {
                cursor.newline = page.column.saturating_add(1).saturating_add(cursor.width).saturating_add(cursor.closing_width) > 120;
            }
            let done = if cursor.newline { page.indent((depth + 1) * 4, &mut cursor.offset) } else { page.copy(b" ", &mut cursor.offset) };
            if done { next = Phase::AttributeName; }
        }
        Phase::AttributeName => if page.copy(attrs[cursor.attribute].name.as_bytes(), &mut cursor.offset) { next = Phase::Equals; },
        Phase::Equals => if page.copy(b"=\"", &mut cursor.offset) { next = Phase::AttributeValue; },
        Phase::AttributeValue => if page.escape(attrs[cursor.attribute].value, true, cursor) { next = Phase::Quote; },
        Phase::Quote => if page.copy(b"\"", &mut cursor.offset) {
            cursor.attribute += 1;
            next = if cursor.attribute < attrs.len() { Phase::AttributeLength } else { Phase::OpenEnd };
        },
        Phase::OpenEnd => {
            if matches!(contents, GbxmlContent::Empty) {
                if page.copy(b"/>", &mut cursor.offset) { return Ok(Movement::Parent); }
            } else if page.copy(b">", &mut cursor.offset) {
                next = if leaf.is_some() { Phase::Text } else { Phase::NextChild };
            }
        }
        Phase::Text => if page.escape(leaf.expect("only a leaf has text"), false, cursor) { next = Phase::CloseName; },
        Phase::NextChild => next = match contents {
            GbxmlContent::Slots(count) if cursor.child < count => return Ok(Movement::Probe(cursor.child)),
            GbxmlContent::Elements(count) if cursor.child < count => Phase::ChildSpacing,
            _ => Phase::CloseSpacing,
        },
        Phase::ChildSpacing => if page.indent((depth + 1) * 2, &mut cursor.offset) {
            let child = cursor.child;
            cursor.child += 1;
            cursor.phase = Phase::NextChild;
            cursor.offset = 0;
            return Ok(Movement::Child(child));
        },
        Phase::CloseSpacing => if page.indent(depth * 2, &mut cursor.offset) { next = Phase::CloseName; },
        Phase::CloseName => {
            if cursor.offset < 2 && !page.copy(b"</", &mut cursor.offset) { return Ok(Movement::Stay); }
            if page.copy_shifted(name.as_bytes(), 2, &mut cursor.offset) { next = Phase::CloseEnd; }
        }
        Phase::CloseEnd => if page.copy(b">", &mut cursor.offset) { return Ok(Movement::Parent); },
    }
    if next != cursor.phase {
        cursor.offset = 0;
        cursor.escaped_offset = 0;
        cursor.phase = next;
    }
    Ok(Movement::Stay)
}

fn escaped(character: char, attribute: bool) -> Option<&'static [u8]> {
    Some(match (character, attribute) {
        ('&', _) => b"&amp;",
        ('<', _) => b"&lt;",
        ('\r', _) => b"&#13;",
        ('>', false) => b"&gt;",
        ('"', true) => b"&quot;",
        ('\t', true) => b"&#9;",
        ('\n', true) => b"&#10;",
        _ => return None,
    })
}

struct Page<'a> { bytes: &'a mut [u8], written: usize, column: &'a mut usize }

impl Page<'_> {
    fn is_full(&self) -> bool { self.written == self.bytes.len() }

    fn copy(&mut self, source: &[u8], offset: &mut usize) -> bool {
        let count = (source.len() - *offset).min(self.bytes.len() - self.written);
        let part = &source[*offset..*offset + count];
        self.bytes[self.written..self.written + count].copy_from_slice(part);
        if let Some(index) = part.iter().rposition(|byte| *byte == b'\n') {
            *self.column = count - index - 1;
        } else {
            *self.column = self.column.saturating_add(count);
        }
        self.written += count;
        *offset += count;
        *offset == source.len()
    }

    fn copy_shifted(&mut self, source: &[u8], shift: usize, position: &mut usize) -> bool {
        let mut offset = *position - shift;
        let done = self.copy(source, &mut offset);
        *position = shift + offset;
        done
    }

    fn indent(&mut self, spaces: usize, position: &mut usize) -> bool {
        if *position == 0 && !self.copy(b"\n", position) { return false; }
        self.copy_shifted(&SPACES[..spaces], 1, position)
    }

    fn escape(&mut self, source: GbxmlText<'_>, attribute: bool, cursor: &mut ElementCursor) -> bool {
        let Some((character, consumed)) = source.next(cursor.offset) else { return true; };
        let Some(character) = character else { cursor.offset += consumed; return false; };
        let mut utf8 = [0; 4];
        let bytes = escaped(character, attribute).unwrap_or_else(|| character.encode_utf8(&mut utf8).as_bytes());
        if self.copy(bytes, &mut cursor.escaped_offset) {
            cursor.offset += consumed;
            cursor.escaped_offset = 0;
        }
        false
    }
}
