import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""    async fn exchange(client: &KernelClient, instance_id: u32, commands: Vec<AppCommand>) -> Result<crate::kernel_runtime::ExchangeOutcome, String> {
        client.exchange_commands(instance_id, commands).await
    }
""", """    async fn exchange(client: &KernelClient, instance_id: u32, commands: Vec<AppCommand>) -> Result<crate::kernel_runtime::ExchangeOutcome, String> {
        let outcome = client.exchange_commands(instance_id, commands).await?;
        observe_ephemeral(instance_id, &outcome.frames).await;
        Ok(outcome)
    }

    /// 👥️ The latest `AppFrame::Ephemeral` every native guest instance published, by instance.
    fn ephemeral_snapshots() -> &'static std::sync::Mutex<std::collections::BTreeMap<u32, ProgramEphemeralSnapshot>> {
        static SNAPSHOTS: std::sync::OnceLock<std::sync::Mutex<std::collections::BTreeMap<u32, ProgramEphemeralSnapshot>>> = std::sync::OnceLock::new();
        SNAPSHOTS.get_or_init(Default::default)
    }

    /// 👥️ Keeps the last `AppFrame::Ephemeral` of one command turn (contract-freeze §C7.6: the guest appends
    /// one to every command batch it answers), decoded: its interaction and tool run cross the presence wire
    /// typed, its presence pack verbatim.
    async fn observe_ephemeral(instance_id: u32, frames: &[AppFrame]) {
        let Some(AppFrame::Ephemeral { presence, presence_generation, interaction, tool_run, .. }) = frames.iter().rev().find(|frame| matches!(frame, AppFrame::Ephemeral { .. })) else { return };
        let interaction = if interaction.is_empty() { None } else { protocol::decode_presence_interaction(interaction, &mut 0).await.ok() };
        let tool_run = if tool_run.is_empty() { None } else { protocol::decode_presence_tool_run(tool_run).ok() };
        let snapshot = ProgramEphemeralSnapshot { presence: (*presence_generation > 0).then(|| presence.clone()), interaction, tool_run };
        ephemeral_snapshots().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(instance_id, snapshot);
    }

    /// 👥️ The last ephemeral state one instance's guest published, if it published any.
    pub fn ephemeral_snapshot(instance_id: u32) -> Option<ProgramEphemeralSnapshot> {
        ephemeral_snapshots().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&instance_id).cloned()
    }

    /// 🪦 Forgets a destroyed instance's ephemeral state.
    pub fn forget_ephemeral_snapshot(instance_id: u32) {
        ephemeral_snapshots().lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&instance_id);
    }
""")
replace("""    /// 🚧️ The old implementation was the literal `exchange(id, [])` drain design-abi.md §4 names as
    /// retired outright ("The `exchange(id, [])` drain disappears — guests are woken by events/
    /// timers/`next-wake`"). There is no synchronous poll-for-ephemeral-state left in the ABI;
    /// presence/ephemeral state will need to arrive as a pushed `Event::Message`/similar the kernel
    /// thread caches, which is real design work outside this packet's scope. Honest stub.
    pub fn ephemeral_snapshot(_instance_id: u32) -> Result<(Vec<u8>, u64, u64), String> {
        Err("ephemeral_snapshot: the empty-command poll it relied on is retired in channel v12 (design-abi.md §4) — guests must push ephemeral state via events now, not implemented in this packet".to_string())
    }

""", "")
replace("""    #[cfg(not(target_arch = "wasm32"))]
    pub fn ephemeral_snapshot(&self, instance_id: u32) -> Result<(Vec<u8>, u64, u64), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::ephemeral_snapshot(instance_id),
            #[cfg(target_arch = "wasm32")]
            _ => Err("ephemeral_snapshot unavailable".into()),
        }
    }
""", """    /// 👥️ The ephemeral state this instance's guest last published on a command turn — what the presence
    /// heartbeat carries as the peer's app presence pack, interaction and tool run (React's
    /// `ephemeralSnapshot`). `None` until the guest answered its first command.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn ephemeral_snapshot(&self, instance_id: u32) -> Option<ProgramEphemeralSnapshot> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::ephemeral_snapshot(instance_id),
        }
    }
""")
replace("""            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => client.destroy_app(instance_id),
        }
    }
""", """            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => {
                wasm_program_exchange::forget_ephemeral_snapshot(instance_id);
                client.destroy_app(instance_id);
            }
        }
    }
""")
replace("""enum ProgramBridgeBackend {""", """/// 👥️ One guest instance's last published ephemeral state (`AppFrame::Ephemeral`), decoded for the presence
/// wire: the app-owned presence pack (absent while the guest never published one), its declared-broadcast
/// selection and hover, and its tool run summary.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProgramEphemeralSnapshot {
    pub presence: Option<Vec<u8>>,
    pub interaction: Option<protocol::PresenceInteraction>,
    pub tool_run: Option<protocol::PresenceToolRun>,
}

enum ProgramBridgeBackend {""")
path.write_text(text)
print("ok")
