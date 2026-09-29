#!/usr/bin/env python3
"""🔗️ G12 session 15 host fix (os-mcp 🏠️workspace, freeze-open host crate): an agent session holds at most
HUB_SESSION_LIVE_LINK_LIMIT live hub document links (one hub WAL writer slot each), one per app — opening another
document of an app supersedes the older one's link and makes the new one that app's session document; the least
recently used link closes first past the bound; an expired link is relinked by the next artifact_open.
Idempotent; --dry-run reports what would change. usage: python3 g12-live-links.py [--dry-run] [--root <tree>]"""
import pathlib
import sys

DRY = "--dry-run" in sys.argv
ROOT = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else pathlib.Path("/Users/ueli/Documents/semio")
WS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace"
LIB = WS / "🦀️.rs"
QUICK = WS / "🧪️tests/🔬️quick/🦀️.rs"
FIXTURE = WS / "🧫️fixtures/🔗️hub-live-links.json"

EDITS = {
    LIB: [
        (
            """    /// 🗿️ The artifact `route`'s live session document IS, or `None` when this workspace has bound
    /// none to that app — or more than one, which no single stamp could name truthfully.
    fn session_artifact_for(&self, route: &AppRoute) -> Option<String> {
        let bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut matches = bound.iter().filter(|(_, binding)| binding.plugin_id == route.plugin_id && route.app_id.as_deref().is_none_or(|app_id| binding.app_id == app_id)).map(|(artifact_id, _)| artifact_id.clone());
        let first = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        Some(first)
    }
""",
            """    /// 🗿️ The artifact `route`'s live session document IS ([`route_session_artifact`]): the hub document of that app
    /// the agent opened or edited last, or the one artifact a folder session bound to it — `None` when there is none, or
    /// two folder artifacts, which no single stamp could name truthfully.
    fn session_artifact_for(&self, route: &AppRoute) -> Option<String> {
        let bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        route_session_artifact(
            bound
                .iter()
                .filter(|(_, binding)| binding.plugin_id == route.plugin_id && route.app_id.as_deref().is_none_or(|app_id| binding.app_id == app_id))
                .map(|(artifact_id, binding)| (artifact_id.as_str(), binding.document.is_some(), binding.used)),
        )
        .map(str::to_string)
    }
""",
        ),
        (
            """        let artifact_id = self.session_artifact_for(route);
        let bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        hub_edit_refusal(role, &route.plugin_id, artifact_id.as_deref().and_then(|artifact_id| bound.get(artifact_id).map(|binding| (artifact_id, binding))))
    }
""",
            """        let artifact_id = self.session_artifact_for(route);
        let mut bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let refusal = hub_edit_refusal(role, &route.plugin_id, artifact_id.as_deref().and_then(|artifact_id| bound.get(artifact_id).map(|binding| (artifact_id, binding))));
        if let Some(binding) = artifact_id.as_deref().and_then(|artifact_id| bound.get_mut(artifact_id)).filter(|_| refusal.is_none()) {
            binding.used = hub_link_tick();
        }
        refusal
    }
""",
        ),
        (
            """    pub fn load_session_document(&mut self, instance: u32, artifact_id: &str, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {
        if self.session_documents.get(&instance).is_some_and(|loaded| loaded == artifact_id) {
            return Ok(());
        }
""",
            """    pub fn load_session_document(&mut self, instance: u32, artifact_id: &str, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {
        match self.session_documents.get(&instance) {
            Some(loaded) if loaded == artifact_id => return Ok(()),
            Some(_) => self.discard_instance(instance),
            None => {}
        }
""",
        ),
        (
            """    /// [`Self::discard_instance`] forgets it, because a thrown-away guest holds nothing.
    ///
    /// 🏁️ `LoadDocument` publishes""",
            """    /// [`Self::discard_instance`] forgets it, because a thrown-away guest holds nothing.
    ///
    /// 🔀️ A guest that holds ANOTHER artifact (its app's session document moved to a document the agent opened since,
    /// [`route_session_artifact`]) is thrown away first: a guest is only ever seeded fresh, never while its backbone is
    /// bound to a different document.
    ///
    /// 🏁️ `LoadDocument` publishes""",
        ),
        (
            """    /// 🔁️ Re-opening a document this session already bound must not open a SECOND actor: the
    /// first one holds the live socket, the presence lease and the outbox that still owes the hub
    /// this agent's envelopes. Only the canonical pair is refreshed.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bind_hub_session_document(""",
            """    /// 🔁️ Re-opening a document this session already bound must not open a SECOND actor: the
    /// first one holds the live socket, the presence lease and the outbox that still owes the hub
    /// this agent's envelopes. Only the canonical pair is refreshed — unless its link EXPIRED (a shortage outlived its
    /// bound), which a new open relinks with a fresh actor.
    ///
    /// 🔗️ Opening a new actor first closes the live links [`hub_links_to_close`] names — the app's older document
    /// (the one opened now becomes the app's session document) and, past [`HUB_SESSION_LIVE_LINK_LIMIT`], the least
    /// recently used — so one agent never holds more of the hub's document sockets than that (each is one of the hub's
    /// bounded WAL writer slots; an agent that walked 63 documents took them all, measured on 7800/p33).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bind_hub_session_document(""",
        ),
        (
            """        let established = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(artifact_id).filter(|binding| binding.backbone.is_some()).cloned();
        let (backbone, backbone_blocked_by, relayed, relay) = match established {
            Some(binding) => (binding.backbone, binding.backbone_blocked_by, binding.relayed, binding.relay),
            None => {
                let relay = Arc::new(HubRelay::default());""",
            """        let established = self
            .plugin_artifacts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(artifact_id)
            .filter(|binding| binding.backbone.is_some() && binding.relay.terminal().as_deref() != Some(store::sync::DocumentLinkStatus::LinkExpired.code()))
            .cloned();
        let (backbone, backbone_blocked_by, relayed, relay) = match established {
            Some(binding) => (binding.backbone, binding.backbone_blocked_by, binding.relayed, binding.relay),
            None => {
                self.close_hub_links(artifact_id, (lease.package.plugin_id.as_str(), lease.surface.app_id.as_str()));
                let relay = Arc::new(HubRelay::default());""",
        ),
        (
            """            backbone_blocked_by,
            relayed,
            relay,
        };
        self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), binding);
        Ok(true)
    }
""",
            """            backbone_blocked_by,
            relayed,
            relay,
            used: hub_link_tick(),
        };
        self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), binding);
        Ok(true)
    }

    /// 🔗️ Closes the live hub links [`hub_links_to_close`] names before `opening` (a document of `route` = plugin, app)
    /// links: each one's actor is closed (its socket, presence row and WAL writer slot released) and its binding keeps
    /// the reason, which a later edit of it reports until `artifact_open` relinks it. Every edit the agent was told
    /// succeeded is already acknowledged by the hub; an unacknowledged one was reverted and reported, so nothing an
    /// agent believes the hub holds rides on a closed actor.
    #[cfg(not(target_arch = "wasm32"))]
    fn close_hub_links(&self, opening: &str, route: (&str, &str)) {
        let closes = {
            let mut bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let live = bound.iter().filter(|(_, binding)| binding.backbone.is_some()).map(|(artifact_id, binding)| (artifact_id.clone(), binding.plugin_id.clone(), binding.app_id.clone(), binding.used)).collect::<Vec<_>>();
            let closes = hub_links_to_close(&live, opening, route, HUB_SESSION_LIVE_LINK_LIMIT);
            for (artifact_id, close) in &closes {
                if let Some(binding) = bound.get_mut(artifact_id) {
                    binding.backbone = None;
                    binding.backbone_blocked_by = Some(match close {
                        HubLinkClose::Relinked => "its expired link is being replaced by a fresh one".to_string(),
                        HubLinkClose::Superseded => format!("its link was closed when `{opening}` became its app's open hub document; artifact_open `{artifact_id}` again to edit it"),
                        HubLinkClose::Bounded => format!("its link was closed to keep this session within {HUB_SESSION_LIVE_LINK_LIMIT} live hub documents (least recently used first); artifact_open `{artifact_id}` again to edit it"),
                    });
                }
            }
            closes
        };
        for (artifact_id, _) in closes {
            self.artifact_host.close_key(&self.origin.artifact_document_key(&artifact_id));
        }
    }
""",
        ),
        (
            """relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default() });
        Ok((pack.len(), spr.len()))""",
            """relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });
        Ok((pack.len(), spr.len()))""",
        ),
        (
            """    /// 🚦️ The document actor's own sync reports, for a binding that opened one.
    pub relay: Arc<HubRelay>,
}
""",
            """    /// 🚦️ The document actor's own sync reports, for a binding that opened one.
    pub relay: Arc<HubRelay>,
    /// 🕰️ When the agent last opened or edited this hub document ([`hub_link_tick`]; 0 for a folder artifact): its app's
    /// session document is the one with the latest, and the least recent live link closes first past the bound.
    pub used: u64,
}
""",
        ),
        (
            """            .field("relayed", &self.relayed.load(std::sync::atomic::Ordering::Relaxed))
            .finish()""",
            """            .field("relayed", &self.relayed.load(std::sync::atomic::Ordering::Relaxed))
            .field("used", &self.used)
            .finish()""",
        ),
        (
            """/// 🚫️ Whether a coded message the document actor raised ends its link for good""",
            """/// 🔗️ At most this many hub documents hold a live link — a document socket, one of the hub's bounded WAL writer slots
/// — for one agent session at once ([`hub_links_to_close`]); fixture `🧫️fixtures/🔗️hub-live-links.json`.
pub const HUB_SESSION_LIVE_LINK_LIMIT: usize = 8;

/// 🔗️ Why a session closes one of its live hub links before it links another document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubLinkClose {
    /// ♻️ The document being opened again: its link expired and a fresh one replaces it.
    Relinked,
    /// 🔀️ Another document of the same app: the one being opened becomes the app's session document.
    Superseded,
    /// 📏️ The least recently used link past [`HUB_SESSION_LIVE_LINK_LIMIT`].
    Bounded,
}

/// 🔗️ The live hub links a session closes before `opening`, a document of `route` = (plugin, app), links — `live` =
/// (artifact, plugin, app, used) of every binding with a document actor: the opened document's own (expired) link, every
/// other live link of the same app, then the least recently used until the new link leaves at most `limit` live.
pub fn hub_links_to_close(live: &[(String, String, String, u64)], opening: &str, route: (&str, &str), limit: usize) -> Vec<(String, HubLinkClose)> {
    let mut closes = Vec::new();
    let mut kept = Vec::new();
    for (artifact_id, plugin_id, app_id, used) in live {
        if artifact_id == opening {
            closes.push((artifact_id.clone(), HubLinkClose::Relinked));
        } else if (plugin_id.as_str(), app_id.as_str()) == route {
            closes.push((artifact_id.clone(), HubLinkClose::Superseded));
        } else {
            kept.push((artifact_id, *used));
        }
    }
    kept.sort_by_key(|(_, used)| *used);
    let excess = (kept.len() + 1).saturating_sub(limit);
    closes.extend(kept.into_iter().take(excess).map(|(artifact_id, _)| (artifact_id.clone(), HubLinkClose::Bounded)));
    closes
}

/// 🧭️ A route's session document among the artifacts bound to it — (artifact, hub document?, used): the hub document the
/// agent opened or edited last, else the single folder artifact; two folder artifacts name none.
pub fn route_session_artifact<'a>(bound: impl IntoIterator<Item = (&'a str, bool, u64)>) -> Option<&'a str> {
    let (mut hub, mut folder, mut folders) = (None::<(&'a str, u64)>, None, 0_usize);
    for (artifact_id, hub_document, used) in bound {
        if !hub_document {
            folder = Some(artifact_id);
            folders += 1;
        } else if hub.is_none_or(|(_, latest)| used > latest) {
            hub = Some((artifact_id, used));
        }
    }
    hub.map(|(artifact_id, _)| artifact_id).or(folder.filter(|_| folders == 1))
}

/// 🕰️ The next tick of this process's hub-link clock — monotonic, so "opened or edited last" is a total order.
pub fn hub_link_tick() -> u64 {
    static CLOCK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    CLOCK.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
}

/// 🚫️ Whether a coded message the document actor raised ends its link for good""",
        ),
    ],
    QUICK: [
        (
            """        relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        relay: Arc::default(),
    };
    let document = binding("no `store::ArtifactCodec` is registered for artifact schema `s.note.note`");""",
            """        relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        relay: Arc::default(),
        used: 1,
    };
    let document = binding("no `store::ArtifactCodec` is registered for artifact schema `s.note.note`");""",
        ),
        (
            """.insert("journey-note-typed".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, document: None, backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default() });""",
            """.insert("journey-note-typed".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, document: None, backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });""",
        ),
        (
            """.insert("journey-note-second".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, document: None, backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default() });""",
            """.insert("journey-note-second".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, document: None, backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });""",
        ),
        (
            """            backbone_blocked_by: Some("no `store::ArtifactCodec` is registered for artifact schema `s.note.note`".to_string()),
            relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            relay: Arc::default(),
        },
    );""",
            """            backbone_blocked_by: Some("no `store::ArtifactCodec` is registered for artifact schema `s.note.note`".to_string()),
            relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            relay: Arc::default(),
            used: 0,
        },
    );""",
        ),
        (
            """/// 🪟️ LAW: a headless agent's transaction carries document operations and owned children only""",
            """/// 🔗️ LAW (fixture `🧫️fixtures/🔗️hub-live-links.json`, Python oracle `wp-g12/g12-live-links-oracle.py` of ticket
/// 26/09/23): an agent session keeps one live hub link per app and at most [`HUB_SESSION_LIVE_LINK_LIMIT`] in all —
/// opening a document supersedes its app's older link, relinks its own expired one, and closes the least recently used
/// past the bound; an app's session document is the hub document opened or edited last, a folder's the single one.
#[test]
fn a_hub_session_keeps_one_live_link_per_app_within_its_bound_and_edits_the_document_opened_last() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️hub-live-links.json")).expect("hub-live-links fixture parses");
    assert_eq!(fixture["limit"].as_u64(), Some(HUB_SESSION_LIVE_LINK_LIMIT as u64), "the fixture pins the session bound");
    let text = |value: &serde_json::Value| value.as_str().expect("string").to_string();
    for case in fixture["close"].as_array().expect("close cases") {
        let live = case["live"].as_array().expect("live").iter().map(|row| (text(&row[0]), text(&row[1]), text(&row[2]), row[3].as_u64().expect("used"))).collect::<Vec<_>>();
        let limit = case["limit"].as_u64().map_or(HUB_SESSION_LIVE_LINK_LIMIT, |limit| limit as usize);
        let closes = hub_links_to_close(&live, case["opening"].as_str().expect("opening"), (case["route"][0].as_str().expect("plugin"), case["route"][1].as_str().expect("app")), limit)
            .into_iter()
            .map(|(artifact_id, close)| serde_json::json!([artifact_id, match close { HubLinkClose::Relinked => "relinked", HubLinkClose::Superseded => "superseded", HubLinkClose::Bounded => "bounded" }]))
            .collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(closes), case["expect"], "{}", case["name"]);
    }
    for case in fixture["session"].as_array().expect("session cases") {
        let bound = case["bound"].as_array().expect("bound").iter().map(|row| (row[0].as_str().expect("artifact"), row[1].as_bool().expect("hub"), row[2].as_u64().expect("used"))).collect::<Vec<_>>();
        assert_eq!(route_session_artifact(bound), case["expect"].as_str(), "{}", case["name"]);
    }
    let (first, second) = (hub_link_tick(), hub_link_tick());
    assert!(second > first, "the hub-link clock is monotonic");
}

/// 🪟️ LAW: a headless agent's transaction carries document operations and owned children only""",
        ),
    ],
}

FIXTURE_TEXT = """{
  "limit": 8,
  "close": [
    { "name": "another app under the bound closes nothing", "live": [["n1", "note", "note.editor", 1]], "opening": "c1", "route": ["cad", "cad.editor"], "expect": [] },
    { "name": "another document of the same app supersedes its link", "live": [["n1", "note", "note.editor", 1], ["c1", "cad", "cad.editor", 2]], "opening": "n2", "route": ["note", "note.editor"], "expect": [["n1", "superseded"]] },
    { "name": "the same plugin's other app keeps its link", "live": [["b2", "block", "block.2d", 1]], "opening": "b3", "route": ["block", "block.3d"], "expect": [] },
    { "name": "an expired link of the document itself is relinked", "live": [["n1", "note", "note.editor", 4]], "opening": "n1", "route": ["note", "note.editor"], "expect": [["n1", "relinked"]] },
    { "name": "at the bound the least recently used closes", "live": [["a", "p1", "x", 5], ["b", "p2", "x", 2], ["c", "p3", "x", 9]], "opening": "d", "route": ["p4", "x"], "limit": 3, "expect": [["b", "bounded"]] },
    { "name": "past the bound every excess link closes, oldest first", "live": [["a", "p1", "x", 5], ["b", "p2", "x", 2], ["c", "p3", "x", 9]], "opening": "d", "route": ["p4", "x"], "limit": 2, "expect": [["b", "bounded"], ["a", "bounded"]] },
    { "name": "a superseded link already makes room", "live": [["a", "p1", "x", 5], ["b", "p2", "x", 2], ["c", "p3", "x", 9]], "opening": "d", "route": ["p1", "x"], "limit": 3, "expect": [["a", "superseded"]] },
    { "name": "the session bound holds eight", "live": [["d1", "p1", "x", 1], ["d2", "p2", "x", 2], ["d3", "p3", "x", 3], ["d4", "p4", "x", 4], ["d5", "p5", "x", 5], ["d6", "p6", "x", 6], ["d7", "p7", "x", 7], ["d8", "p8", "x", 8]], "opening": "d9", "route": ["p9", "x"], "expect": [["d1", "bounded"]] }
  ],
  "session": [
    { "name": "the hub document opened last", "bound": [["n1", true, 3], ["n2", true, 7]], "expect": "n2" },
    { "name": "the hub document edited last", "bound": [["n1", true, 9], ["n2", true, 7]], "expect": "n1" },
    { "name": "a hub document beside a folder artifact", "bound": [["f1", false, 0], ["n1", true, 2]], "expect": "n1" },
    { "name": "the single folder artifact", "bound": [["f1", false, 0]], "expect": "f1" },
    { "name": "two folder artifacts name none", "bound": [["f1", false, 0], ["f2", false, 0]], "expect": null },
    { "name": "nothing bound", "bound": [], "expect": null }
  ]
}
"""


def main() -> int:
    pending = []
    for path, edits in EDITS.items():
        text = path.read_text(encoding="utf-8")
        for old, new in edits:
            if text.count(new) == 1 and (new not in old):
                continue
            if text.count(old) != 1:
                print(f"PROBLEM {path.name}: anchor count {text.count(old)}: {old[:90]!r}")
                return 1
            pending.append((path, old, new))
    fixture_needed = not FIXTURE.exists() or FIXTURE.read_text(encoding="utf-8") != FIXTURE_TEXT
    print(f"{len(pending)} hunk(s) to apply, fixture {'to write' if fixture_needed else 'current'}")
    if DRY or (not pending and not fixture_needed):
        print("dry run" if DRY else "nothing to do")
        return 0
    texts = {path: path.read_text(encoding="utf-8") for path in EDITS}
    for path, old, new in pending:
        texts[path] = texts[path].replace(old, new, 1)
    if fixture_needed:
        FIXTURE.write_text(FIXTURE_TEXT, encoding="utf-8")
    for path in EDITS:
        path.write_text(texts[path], encoding="utf-8")
    print("applied")
    return 0


if __name__ == "__main__":
    sys.exit(main())
