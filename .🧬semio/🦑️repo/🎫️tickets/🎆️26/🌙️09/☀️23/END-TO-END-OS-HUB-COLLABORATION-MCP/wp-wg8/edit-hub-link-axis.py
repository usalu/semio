import pathlib

root = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements")
hub = root / "🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs"
text = hub.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""/// 🪪️ Whether a hub session exists at all. `None` — no sign-in surface mounted — is deliberately not
/// the same as `SignedOut`: only the latter is actionable, and only it offers an entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubSessionPresence {
    SignedIn,
    SignedOut,
    None,
}

/// 📶️ The aggregate the footer pill paints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubConnectionState {
    SignedOut,
    Live { peer_count: usize },
    Connecting,
    Reconnecting,
    Offline,
}
""", """/// 🪪️ Whether a hub session exists at all — the hub projection schema's `session` axis
/// (`🧬️schema/🔗️hub-projection`). `None` — no hub configured at all — is deliberately not the same as
/// `SignedOut`: only the latter is actionable, and only it offers an entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HubSessionPresence {
    SignedIn,
    SignedOut,
    None,
}

/// 🔗️ What the shell's own session revalidation last learned about the hub link, independent of any
/// document — the hub projection schema's `link` axis: a held session still `Verifying`, `Reachable`, or
/// `Unreachable`, a short shortage ridden out on a bounded backoff while the verified authority is kept.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HubLink {
    Verifying,
    Reachable,
    Unreachable,
}

/// 📶️ The aggregate the footer pill paints — the hub projection schema's `summary.state`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubConnectionState {
    Local,
    SignedOut,
    Live { peer_count: usize },
    Online,
    Connecting,
    Reconnecting,
}
""")
replace("""/// 📶️ Folds every attached document's remote state into one aggregate, best state first.
///
/// 🧮️ The three laws U1 §8 pinned, restated so a reader need not chase the TypeScript: one `Live`
/// document means the hub is reachable; a document still dialling outranks one already in backoff;
/// everything detached — or nothing attached at all — is `Offline`. `peer_count` is the **max** over
/// live documents, never a sum: a sum would double-count a peer who has two documents open, which no
/// reader could interpret. `SignedOut` outranks every transport state, because no transport state
/// means anything without a session.
pub fn hub_connection_summary(statuses: &[HubDocumentRemote], session: HubSessionPresence) -> HubConnectionSummary {
    if session == HubSessionPresence::SignedOut {
        return HubConnectionSummary { state: HubConnectionState::SignedOut, actionable: true };
    }
    let actionable = session == HubSessionPresence::SignedIn;
    let mut peer_count: Option<usize> = None;
    let mut connecting = false;
    let mut backoff = false;
    for status in statuses {
        match status {
            HubDocumentRemote::Live { peer_count: peers } => peer_count = Some(peer_count.unwrap_or(0).max(*peers)),
            HubDocumentRemote::Connecting => connecting = true,
            HubDocumentRemote::Backoff => backoff = true,
            HubDocumentRemote::Detached => continue,
        }
    }
    let state = match (peer_count, connecting, backoff) {
        (Some(peer_count), _, _) => HubConnectionState::Live { peer_count },
        (None, true, _) => HubConnectionState::Connecting,
        (None, false, true) => HubConnectionState::Reconnecting,
        (None, false, false) => HubConnectionState::Offline,
    };
    HubConnectionSummary { state, actionable }
}
""", """/// 📶️ Folds the shell's own hub link and every attached document's remote state into one aggregate,
/// best state first — the rules of the hub projection schema (`🧬️schema/🔗️hub-projection`), whose
/// fixture both this fold and React's `hubConnectionSummaryV1` pass.
///
/// 🧮️ No hub configured is `Local`; no session is `SignedOut`, because no transport state means anything
/// without one. One `Live` document means the hub answers right now, so it wins over everything else
/// and `peer_count` is the **max** over live documents, never a sum (a sum would double-count a peer who
/// has two documents open). An `Unreachable` link is a shortage in progress (`Reconnecting`), which
/// outranks a document still dialling; a `Verifying` link or a dialling document is `Connecting`, which
/// outranks a document in backoff (`Reconnecting`); a reachable link with no live document is `Online`.
pub fn hub_connection_summary(statuses: &[HubDocumentRemote], session: HubSessionPresence, link: HubLink) -> HubConnectionSummary {
    match session {
        HubSessionPresence::None => return HubConnectionSummary { state: HubConnectionState::Local, actionable: false },
        HubSessionPresence::SignedOut => return HubConnectionSummary { state: HubConnectionState::SignedOut, actionable: true },
        HubSessionPresence::SignedIn => {}
    }
    let mut peer_count: Option<usize> = None;
    let mut connecting = false;
    let mut backoff = false;
    for status in statuses {
        match status {
            HubDocumentRemote::Live { peer_count: peers } => peer_count = Some(peer_count.unwrap_or(0).max(*peers)),
            HubDocumentRemote::Connecting => connecting = true,
            HubDocumentRemote::Backoff => backoff = true,
            HubDocumentRemote::Detached => continue,
        }
    }
    let state = match (peer_count, link) {
        (Some(peer_count), _) => HubConnectionState::Live { peer_count },
        (None, HubLink::Unreachable) => HubConnectionState::Reconnecting,
        (None, HubLink::Verifying) => HubConnectionState::Connecting,
        (None, HubLink::Reachable) if connecting => HubConnectionState::Connecting,
        (None, HubLink::Reachable) if backoff => HubConnectionState::Reconnecting,
        (None, HubLink::Reachable) => HubConnectionState::Online,
    };
    HubConnectionSummary { state, actionable: true }
}
""")
replace("""    pub fn icon_id(self) -> &'static str {
        match self {
            Self::SignedOut => "user",
            Self::Live { .. } => "cloud",
            Self::Connecting => "loader-2",
            Self::Reconnecting => "rotate-ccw",
            Self::Offline => "link-2-off",
        }
    }

    /// 🏷️ The state's own wire spelling, so a probe reads the state without parsing a translation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignedOut => "signedOut",
            Self::Live { .. } => "live",
            Self::Connecting => "connecting",
            Self::Reconnecting => "reconnecting",
            Self::Offline => "offline",
        }
    }
""", """    pub fn icon_id(self) -> &'static str {
        match self {
            Self::Local => "link-2-off",
            Self::SignedOut => "user",
            Self::Live { .. } | Self::Online => "cloud",
            Self::Connecting => "loader-2",
            Self::Reconnecting => "rotate-ccw",
        }
    }

    /// 🏷️ The state's own wire spelling (the schema's `summary.state`), so a probe reads the state
    /// without parsing a translation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::SignedOut => "signedOut",
            Self::Live { .. } => "live",
            Self::Online => "online",
            Self::Connecting => "connecting",
            Self::Reconnecting => "reconnecting",
        }
    }
""")
hub.write_text(text)
print("hub ok")
