import pathlib
path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
def swap(old, new):
    global text
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
swap('''    /// ⏳️ How long [`KernelPoolState::run_turn`] keeps continuing one unsettled turn with empty-event
    /// grants before reporting it. The same order as `RUN_TURN_OUTCOME_TIMEOUT`'s per-grant wait:
    /// each continuation is itself bounded by the 100 ms `TURN_BUDGET` wall grant.
    const RUN_TURN_SETTLE_BUDGET: Duration = Duration::from_secs(30);
''', '''    /// ⏳️ How long one [`KernelPoolState::run_turn`] request may keep its actor's turn going —
    /// preemption resumes and settle continuations together. The same order as
    /// `RUN_TURN_OUTCOME_TIMEOUT`'s per-grant wait: every grant is itself bounded by the 100 ms
    /// `TURN_BUDGET` wall. A guest still preempted when it runs out is reported as wedged; a guest
    /// still answering `MoreWork` is handed back settled-as-far-as-it-got, never mid-call.
    const RUN_TURN_SETTLE_BUDGET: Duration = Duration::from_secs(30);

    /// 🤫️ Consecutive continuation turns that carried nothing after which [`KernelPoolState::run_turn`]
    /// stops settling a guest that still answers `MoreWork`: that guest is waiting on a host round trip
    /// no empty turn delivers. Derived exactly like the React host's `PLUGIN_UI_QUIESCENT_CONTINUATIONS`
    /// (`🔌️PluginRuntime/🟦️.tsx`) — the patch pages one surface may take
    /// (`max_patch_bytes / max_text_bytes`, `🖱️ui/🧬️contract/🛡️limits`) times its continuation batch of 8.
    const RUN_TURN_QUIESCENT_CONTINUATIONS: usize = (1_048_576usize).div_ceil(65_536) * 8;
''')
swap('''        fn absorb(&mut self, later: ExchangeOutcome) -> Result<(), String> {''', '''        /// 🤫️ Whether this turn carried nothing a caller could act on — no frame, effect or surface
        /// document and an idle command ingress. The native twin of the browser drive's
        /// `shardTurnCarriesNothingV1` (`🎭️actor/🖼️wire-turn/🟦️.ts`) at this host's level, where
        /// patches have already become surface documents and shell messages frames.
        fn carries_nothing(&self) -> bool {
            self.frames.is_empty() && self.effects.is_empty() && self.surfaces.is_empty() && matches!(self.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)
        }

        fn absorb(&mut self, later: ExchangeOutcome) -> Result<(), String> {''')
path.write_text(text)
print("consts + carries_nothing added")
