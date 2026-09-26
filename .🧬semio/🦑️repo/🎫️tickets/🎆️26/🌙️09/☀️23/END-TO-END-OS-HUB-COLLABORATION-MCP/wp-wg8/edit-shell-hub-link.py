import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""HubArtifactCreationState, HubArtifactOpening, HubDocumentRemote, HubSessionPresence, hub_artifact_creation_intent,""",
        """HubArtifactCreationState, HubArtifactOpening, HubConnectionState, HubDocumentRemote, HubLink, HubSessionPresence, hub_artifact_creation_intent,""")

replace("""    pub(crate) fn hub_projection(&self) -> ShellHubProjectionV1 {
        let authority = self.verified_session_authority.as_ref().map(|authority| ShellHubAuthorityV1::VerifiedSession { authorization_generation: authority.authorization_generation }).unwrap_or(ShellHubAuthorityV1::NoVerifiedSession);
        let documents = self.hub_documents.iter().map(|(document_key, remote)| ShellHubDocumentV1 { document_key: document_key.clone(), remote: remote.clone() }).collect();
        ShellHubProjectionV1 { authority, documents }
    }
""", """    /// 📶️ This shell's hub projection over the schema's two shell axes and its attached documents. The
    /// session is held exactly while a verified `/auth/sessions/me` authority is: a hub is always
    /// configured here (the connection book keeps the local bootstrap hub), so this shell is never
    /// `none`. The link is what the identity lane last learned: a hub it could not reach while it kept
    /// the cached identity is `unreachable`, a verified authority `reachable`, anything else `verifying`.
    pub(crate) fn hub_projection(&self) -> ShellHubProjectionV1 {
        let session = if self.verified_session_authority.is_some() { HubSessionPresence::SignedIn } else { HubSessionPresence::SignedOut };
        let link = match (self.identity_offline, self.verified_session_authority.is_some()) {
            (true, _) => HubLink::Unreachable,
            (false, true) => HubLink::Reachable,
            (false, false) => HubLink::Verifying,
        };
        let documents = self.hub_documents.iter().map(|(document_key, remote)| ShellHubDocumentV1 { document_key: document_key.clone(), remote: remote.clone() }).collect();
        ShellHubProjectionV1 { session, link, documents }
    }
""")
replace("""    fn hub_connection_state(&self) -> ShellHubConnectionState {
        shell_hub_connection_summary_v1(&self.hub_projection()).state
    }
""", """    fn hub_connection_state(&self) -> HubConnectionState {
        shell_hub_connection_summary_v1(&self.hub_projection()).state
    }
""")
replace("""        let hub = self.hub_connection_state().text(is_de);""", """        let hub = shell_hub_connection_text(self.hub_connection_state(), is_de);""")
replace("""                let actionable = hub == ShellHubConnectionState::SignedOut;""", """                let actionable = hub == HubConnectionState::SignedOut;""")
replace("""/// 📶️ The native shell's honest projection of React's aggregate hub badge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellHubConnectionState {
    SignedOut,
    Live(usize),
    Connecting,
    Reconnecting,
    Offline,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ShellHubAuthorityV1 {
    NoVerifiedSession,
    VerifiedSession {
        #[serde(rename = "authorizationGeneration")]
        authorization_generation: u64,
    },
}

""", "")
replace("""#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellHubProjectionV1 {
    pub authority: ShellHubAuthorityV1,
    pub documents: Vec<ShellHubDocumentV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellHubSummaryV1 {
    pub state: ShellHubConnectionState,
    pub peer_count: usize,
    pub document_count: usize,
}
""", """/// 📶️ The target-neutral hub projection (`🧬️schema/🔗️hub-projection`'s `projection`): the shell's
/// session and link axes and every attached document's remote state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellHubProjectionV1 {
    pub session: HubSessionPresence,
    pub link: HubLink,
    pub documents: Vec<ShellHubDocumentV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellHubSummaryV1 {
    pub state: HubConnectionState,
    pub peer_count: usize,
    pub document_count: usize,
}
""")
replace("""    let session = if matches!(projection.authority, ShellHubAuthorityV1::VerifiedSession { .. }) { HubSessionPresence::SignedIn } else { HubSessionPresence::SignedOut };
    let summary = crate::hub_connection::hub_connection_summary(&statuses, session);
    let (state, peer_count) = match summary.state {
        crate::hub_connection::HubConnectionState::SignedOut => (ShellHubConnectionState::SignedOut, 0),
        crate::hub_connection::HubConnectionState::Live { peer_count } => (ShellHubConnectionState::Live(peer_count), peer_count),
        crate::hub_connection::HubConnectionState::Connecting => (ShellHubConnectionState::Connecting, 0),
        crate::hub_connection::HubConnectionState::Reconnecting => (ShellHubConnectionState::Reconnecting, 0),
        crate::hub_connection::HubConnectionState::Offline => (ShellHubConnectionState::Offline, 0),
    };
    ShellHubSummaryV1 { state, peer_count, document_count }
}
""", """    let state = crate::hub_connection::hub_connection_summary(&statuses, projection.session, projection.link).state;
    let peer_count = match state {
        HubConnectionState::Live { peer_count } => peer_count,
        _ => 0,
    };
    ShellHubSummaryV1 { state, peer_count, document_count }
}
""")
replace("""impl ShellHubConnectionState {
    fn icon_id(self) -> &'static str {
        match self {
            Self::SignedOut => "user",
            Self::Live(_) => "cloud",
            Self::Connecting => "loader-2",
            Self::Reconnecting => "rotate-ccw",
            Self::Offline => "link-2-off",
        }
    }

    fn text(self, is_de: bool) -> String {
        match self {
            Self::SignedOut => shell_chrome_string("hub.signedOut", is_de).to_string(),
            Self::Live(peers) => format!("{} · {peers} {}", shell_chrome_string("hub.live", is_de), if peers == 1 { shell_chrome_string("hub.peerOne", is_de) } else { shell_chrome_string("hub.peerMany", is_de) }),
            Self::Connecting => shell_chrome_string("hub.connecting", is_de).to_string(),
            Self::Reconnecting => shell_chrome_string("hub.reconnecting", is_de).to_string(),
            Self::Offline => shell_chrome_string("hub.offline", is_de).to_string(),
        }
    }
}""", """/// 📶️ The footer hub badge's text for one fold state, en + de, term for term with React's
/// `HubConnectionIndicator` (`ui.sync.*`).
fn shell_hub_connection_text(state: HubConnectionState, is_de: bool) -> String {
    match state {
        HubConnectionState::Local => shell_chrome_string("hub.local", is_de).to_string(),
        HubConnectionState::SignedOut => shell_chrome_string("hub.signedOut", is_de).to_string(),
        HubConnectionState::Live { peer_count } => format!("{} · {peer_count} {}", shell_chrome_string("hub.live", is_de), if peer_count == 1 { shell_chrome_string("hub.peerOne", is_de) } else { shell_chrome_string("hub.peerMany", is_de) }),
        HubConnectionState::Online => shell_chrome_string("hub.online", is_de).to_string(),
        HubConnectionState::Connecting => shell_chrome_string("hub.connecting", is_de).to_string(),
        HubConnectionState::Reconnecting => shell_chrome_string("hub.reconnecting", is_de).to_string(),
    }
}""")
replace("""        ("hub.offline", false) => "offline",
        ("hub.offline", true) => "offline",
""", """        ("hub.online", false) => "online",
        ("hub.online", true) => "online",
        ("hub.local", false) => "local only",
        ("hub.local", true) => "nur lokal",
""")
path.write_text(text)
print("shell ok")

tests = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs")
text = tests.read_text()
replace("""fn state_name(state: ShellHubConnectionState) -> &'static str {
    match state {
        ShellHubConnectionState::SignedOut => "signedOut",
        ShellHubConnectionState::Live(_) => "live",
        ShellHubConnectionState::Connecting => "connecting",
        ShellHubConnectionState::Reconnecting => "reconnecting",
        ShellHubConnectionState::Offline => "offline",
    }
}

""", "")
replace("""fn neutral_authority_and_multi_document_vectors_fold_to_the_shared_summary() {
    let fixture: HubProjectionFixture = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔗️hub-projection/🔣️.json")).expect("hub projection fixture");
    for case in fixture.cases {
        let summary = shell_hub_connection_summary_v1(&case.projection);
        assert_eq!(state_name(summary.state), case.expected.state, "{} state", case.id);
        assert_eq!(summary.peer_count, case.expected.peer_count, "{} peers", case.id);
        assert_eq!(summary.document_count, case.expected.document_count, "{} documents", case.id);
        if let ShellHubConnectionState::Live(count) = summary.state {
            assert_eq!(count, summary.peer_count, "{} live state and summary disagree", case.id);
        }
    }
}""", """fn neutral_session_link_and_multi_document_vectors_fold_to_the_shared_summary() {
    let fixture: HubProjectionFixture = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔗️hub-projection/🔣️.json")).expect("hub projection fixture");
    for case in fixture.cases {
        let summary = shell_hub_connection_summary_v1(&case.projection);
        assert_eq!(summary.state.as_str(), case.expected.state, "{} state", case.id);
        assert_eq!(summary.peer_count, case.expected.peer_count, "{} peers", case.id);
        assert_eq!(summary.document_count, case.expected.document_count, "{} documents", case.id);
    }
}""")
replace("""    let projection = ShellHubProjectionV1 {
        authority: ShellHubAuthorityV1::VerifiedSession { authorization_generation: 1 },
        documents:""", """    let projection = ShellHubProjectionV1 {
        session: HubSessionPresence::SignedIn,
        link: HubLink::Reachable,
        documents:""")
replace("""    assert_eq!(summary.state, ShellHubConnectionState::Live(4));""", """    assert_eq!(summary.state, HubConnectionState::Live { peer_count: 4 });""")
replace("""    assert_eq!(shell.hub_connection_state(), ShellHubConnectionState::SignedOut);""", """    assert_eq!(shell.hub_connection_state(), HubConnectionState::SignedOut);""")
replace("""    assert_ne!(shell.hub_connection_state(), ShellHubConnectionState::SignedOut);""", """    assert_eq!(shell.hub_connection_state(), HubConnectionState::Online, "a verified session on a reachable hub with nothing attached");""")
tests.write_text(text)
print("tests ok")
