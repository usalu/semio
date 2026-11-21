//! 🔌️ Directional graph addresses over authored Flow channel names.
use std::borrow::Cow;

/// 🎚️ The graph granularity encoded into a framework interaction target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphTargetKind { Node, Edge, Handle }

impl GraphTargetKind {
    pub const fn granularity(self) -> &'static str {
        match self { Self::Node => "node", Self::Edge => "edge", Self::Handle => "handle" }
    }

    pub fn for_granularity(granularity: &str) -> Option<Self> {
        match granularity {
            "node" => Some(Self::Node),
            "edge" => Some(Self::Edge),
            "handle" => Some(Self::Handle),
            _ => None,
        }
    }

    pub const fn prefix(self) -> &'static str {
        match self { Self::Node => "n:", Self::Edge => "e:", Self::Handle => "h:" }
    }

    pub fn target(self, id: &str) -> String {
        format!("{}{id}", self.prefix())
    }

    pub fn raw(self, target: &str) -> Option<&str> {
        target.strip_prefix(self.prefix())
    }

    pub fn parse(target: &str) -> Option<(Self, &str)> {
        [Self::Node, Self::Edge, Self::Handle].into_iter().find_map(|kind| kind.raw(target).map(|id| (kind, id)))
    }
}

const CHANNEL_ESCAPES: [(char, &str); 4] = [('%', "25"), ('@', "40"), ('#', "23"), ('~', "7E")];

/// 🧭️ Encodes a canvas-local port address with a delimiter-safe channel.
pub fn graph_port_code(channel: &str, input: bool) -> Cow<'_, str> {
    let prefix = if input { "!i" } else if channel.is_empty() || channel.starts_with('!') { "!o" } else { "" };
    if prefix.is_empty() && !channel.contains(['%', '@', '#', '~']) {
        return Cow::Borrowed(channel);
    }
    let mut code = String::with_capacity(prefix.len() + channel.len());
    code.push_str(prefix);
    for character in channel.chars() {
        if let Some((_, escape)) = CHANNEL_ESCAPES.iter().find(|(value, _)| *value == character) {
            code.push('%');
            code.push_str(escape);
        } else {
            code.push(character);
        }
    }
    Cow::Owned(code)
}

/// 🔠️ Returns the text after the canonical output marker if its leading byte is admissible.
pub fn graph_output_channel_body(code: &str) -> Option<&str> {
    if let Some(channel) = code.strip_prefix("!o") {
        (channel.is_empty() || channel.starts_with('!')).then_some(channel)
    } else {
        (!code.is_empty() && !code.starts_with('!')).then_some(code)
    }
}

/// 📏️ Measures the first encoded channel unit, with zero denoting an invalid or missing unit.
pub fn graph_channel_code_unit_width(bytes: &[u8]) -> usize {
    match bytes.first() {
        Some(b'%') if CHANNEL_ESCAPES.iter().any(|(_, escape)| bytes[1..].starts_with(escape.as_bytes())) => 3,
        None | Some(b'%' | b'@' | b'#' | b'~') => 0,
        _ => 1,
    }
}

fn canonical_channel_code(mut bytes: &[u8]) -> bool {
    while !bytes.is_empty() {
        let width = graph_channel_code_unit_width(bytes);
        if width == 0 { return false; }
        bytes = &bytes[width..];
    }
    true
}

/// 🔁️ Recovers the authored channel at the connection and catalogue boundaries.
pub fn graph_port_channel(code: &str) -> Cow<'_, str> {
    let channel = code.strip_prefix("!i").or_else(|| code.strip_prefix("!o")).unwrap_or(code);
    if !channel.contains('%') { return Cow::Borrowed(channel); }
    let mut decoded = String::with_capacity(channel.len());
    let mut remaining = channel;
    while let Some((plain, encoded)) = remaining.split_once('%') {
        decoded.push_str(plain);
        if let Some((character, tail)) = CHANNEL_ESCAPES.iter().find_map(|(character, escape)| {
            encoded.strip_prefix(*escape).map(|tail| (*character, tail))
        }) {
            decoded.push(character);
            remaining = tail;
        } else {
            decoded.push('%');
            remaining = encoded;
        }
    }
    decoded.push_str(remaining);
    Cow::Owned(decoded)
}

/// 🎯️ Qualifies a directional canvas port for the graph interaction domain.
pub fn graph_port_id(widget: &str, channel: &str, input: bool) -> String {
    format!("{widget}@{}", graph_port_code(channel, input))
}

/// 🧷️ An authored widget and directional channel from a canonical graph handle.
#[derive(Debug, PartialEq, Eq)]
pub struct GraphPortAddress<'a> {
    pub widget: &'a str,
    pub channel: Cow<'a, str>,
    pub input: bool,
}

/// 🪪️ Identifies one viewport instance within its authored directional port.
#[derive(Debug, PartialEq, Eq)]
pub struct GraphInstanceAddress<'a> {
    pub port: GraphPortAddress<'a>,
    pub index: usize,
}

/// 🧩️ Resolves a canonical handle to its original widget and channel.
pub fn parse_graph_port_id(id: &str) -> Option<GraphPortAddress<'_>> {
    let (widget, code) = id.rsplit_once('@')?;
    if widget.is_empty() { return None; }
    let input = code.starts_with("!i");
    let body = if input { &code[2..] } else { graph_output_channel_body(code)? };
    if !canonical_channel_code(body.as_bytes()) { return None; }
    let channel = graph_port_channel(code);
    Some(GraphPortAddress { widget, channel, input })
}

/// 🔢️ Qualifies a viewport instance within the port that publishes it.
pub fn graph_instance_id(widget: &str, channel: &str, input: bool, index: usize) -> String {
    let port = graph_port_id(widget, channel, input);
    let direction = if input { "in-" } else { "" };
    format!("{port}#{direction}{index}")
}

/// 📍️ Resolves a viewport instance to its original directional port and index.
pub fn parse_graph_instance_id(id: &str) -> Option<GraphInstanceAddress<'_>> {
    let (port, index) = id.rsplit_once('#')?;
    let port = parse_graph_port_id(port)?;
    let index = if port.input { index.strip_prefix("in-")? } else { index };
    if index.is_empty() || index.len() > 1 && index.starts_with('0') || !index.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
    Some(GraphInstanceAddress { port, index: index.parse().ok()? })
}
