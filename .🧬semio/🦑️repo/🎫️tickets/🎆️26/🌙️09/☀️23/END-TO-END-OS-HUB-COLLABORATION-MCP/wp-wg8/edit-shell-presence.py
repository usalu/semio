import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""    pub presence_surface: Option<String>,
""", """    pub presence_surface: Option<String>,
    /// 🪪️ This shell's own hub-admitted presence identity on the attached document — the actor and
    /// palette index the hub's `Session` frame named. Board overlays never paint it, and the local
    /// colour is the hub's, never a guess (React's `publishLocalPresenceActorV1`).
    pub presence_self: Option<(String, u8)>,
    /// 🖱️ The last pointer position the shell saw, in logical screen space — what a board window's
    /// presence view turns into the world point under the pointer on the next heartbeat.
    pub presence_pointer: Option<(f32, f32)>,
""")
replace("""            presence_peers: Vec::new(),
            presence_surface: None,
""", """            presence_peers: Vec::new(),
            presence_surface: None,
            presence_self: None,
            presence_pointer: None,
""")
replace("""        self.presence_peers.clear();
        self.presence_surface = None;
        retirement.map(|_| ())
""", """        self.presence_peers.clear();
        self.presence_surface = None;
        self.presence_self = None;
        retirement.map(|_| ())
""")
replace("""                // 👥️ Peer session identity (actor + colour), sent once per connection. The sync
                // actor already stamps it onto outbound heartbeats itself, so the shell has
                // nothing further to fold in here — matched explicitly so a future variant
                // cannot be silently ignored by a catch-all.
                ArtifactEvent::Session { .. } => {}
""", """                // 👥️ This connection's hub-admitted identity (actor + colour), sent once per connection.
                // The sync actor stamps it onto outbound heartbeats itself; the shell keeps it so its
                // board overlays leave the local actor out and paint in the hub's own colour.
                ArtifactEvent::Session { actor, color } => {
                    self.presence_self = Some((actor, color));
                    changed = true;
                }
""")
replace("""        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.update_hover(x, y);
""", """        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        self.presence_pointer = Some((x, y));
        input.update_hover(x, y);
""")
replace("""        let label = Some(shell_presence_label(self.identity.as_ref(), &actor)).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let user_id = self.identity.as_ref().map(|identity| identity.user_id.clone()).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let peer = PresencePeer { actor, label, presence_pack: None, connected_at_ms, user_id, role: None, drag_ghost_json: None, interaction: None, color: None, surface: None, views: Vec::new(), ui: None, tool_run: None, principal_kind: None, active_tool: None };
        self.document_host.presence_heartbeat_key(&channel.document_key, chrome_now_ms() as u64, peer);
    }
""", """        let label = Some(shell_presence_label(self.identity.as_ref(), &actor)).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let user_id = self.identity.as_ref().map(|identity| identity.user_id.clone()).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let (views, active_tool) = self.board_presence_views();
        #[cfg(not(target_arch = "wasm32"))]
        let ephemeral = self.plugins.iter().find(|entry| entry.plugin_id == channel.plugin_id).and_then(|plugin| plugin.ephemeral_snapshot(channel.instance_id)).unwrap_or_default();
        #[cfg(not(target_arch = "wasm32"))]
        let (presence_pack, interaction) = (ephemeral.presence, ephemeral.interaction);
        #[cfg(target_arch = "wasm32")]
        let (presence_pack, interaction) = (None, None);
        let peer = PresencePeer { actor, label, presence_pack, connected_at_ms, user_id, role: None, drag_ghost_json: None, interaction, color: None, surface: None, views, ui: None, tool_run: None, principal_kind: None, active_tool };
        self.document_host.presence_heartbeat_key(&channel.document_key, chrome_now_ms() as u64, peer);
    }

    /// 👕️ Every board window of the attached session as the presence wire names it — camera, size and
    /// the world point under the pointer (`crate::canvas_presence::board_presence_view`) — plus the
    /// active utility of the board under the pointer, React's `publishLocalActiveToolV1`.
    fn board_presence_views(&self) -> (Vec<PresenceWindowView>, Option<String>) {
        let mut active_tool = None;
        let views = self
            .board2d_states
            .iter()
            .filter_map(|(host_id, surface)| {
                let (camera, utility) = crate::engine_canvas::board2d_presence_state(host_id)?;
                let view = crate::canvas_presence::board_presence_view(&surface.window_id, surface.bounds, camera, self.presence_pointer);
                if view.pointer.is_some() {
                    active_tool = utility;
                }
                Some(view)
            })
            .collect();
        (views, active_tool)
    }

    /// 👥️ Every board window's peer overlays from the verified roster: the other actors' cursors,
    /// viewports and marks this shell paints over its boards (`crate::canvas_presence::board_peer_overlays`,
    /// React's `CanvasPresenceOverlayV1`). Nothing before the hub named this connection's own actor.
    pub(crate) fn board_peer_overlays(&self) -> Vec<(Rect, crate::canvas_presence::BoardPeerOverlays)> {
        let Some((actor, color)) = self.presence_self.as_ref() else { return Vec::new() };
        self.board2d_states
            .iter()
            .filter_map(|(host_id, surface)| {
                let (camera, _) = crate::engine_canvas::board2d_presence_state(host_id)?;
                let overlays = crate::canvas_presence::board_peer_overlays(&self.presence_peers, &surface.window_id, surface.bounds, camera, actor, *color, &format!("board/{}", surface.window_id));
                (!overlays.cursors.is_empty() || !overlays.marks.is_empty()).then_some((surface.bounds, overlays))
            })
            .collect()
    }
""")
path.write_text(text)
print("ok")
