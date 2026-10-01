# 📓️ API: Scrub Machine (Continuous Controls)

Owner: W3-T2-CONTROLS. Design §13.1. Every continuous control (slider, held spinner, number field without a
`commit` mode) is ONE framework tool. Plugins never write their own scrub machine, per-tick amend or static
coalesce key; they supply only the ABSOLUTE leaf of the value.

## 1. Plugin contract (what a plugin author writes)

1. Bind the control to your verb as today (`Trigger::Change`). The hosts send
   - live ticks `{value, gesture: "<control key>:<ms>", commit: false}`,
   - the release `{value, gesture, commit: true}`,
   - a host cancel `{gesture, abort: "blur" | "captureLost" | "frozen" | "baseMoved" | "retired"}` (no value).
2. Your handler reads `value` only and returns `Emit::mutations(leaves)` where `leaves` are the ABSOLUTE set leaves of
   that value computed against the committed document (`set-x{target, value}`; one per target of a multi-selection).
   Keep your `ui_scope`. No `Emit::amend`, no `coalesce_key`, no `description` (the history row label is the leaf's
   `SemanticMutation::label`), no gesture parsing. The same handler serves one-shot dispatches (MCP, keyboard) unchanged.
3. Delete the per-tick amend / static key path you replace.

## 2. Runtime (plugin runtime: `🔌️plugin/🛠️tool-machine/🦀️.rs`, glue in `🔌️plugin/🦀️.rs`; landed)

Since W3-T2-TEXT's typing runs the instance runtime is `ToolMachineRuntime<P, M>` (`VcsArtifactApp.tool_machines`:
`scrubs: ScrubLedger<M>` + `typing: TypingLedger<M>`, one shared overlay); the scrub path is unchanged:

- `dispatch_action` → `admit_tool_dispatch` reads the scrub arguments with `ScrubPhase::parse(gesture, commit, abort)`:
  `Abort` never reaches the app; `Tick`/`Commit` run your verb normally and tag its operation (`ToolTag::Scrub`).
- At the one point the operation's completion becomes its publication (`settle_tool_operation`), a tagged emit's
  `artifact_mutations` are moved into the window's `ScrubLedger<A::Mutation>` (tool `<appId>#<verb>`, actor = dispatch actor, base = the operation's
  canonical document revision): a tick publishes nothing (render seams overlay committed ⊕ provisional leaves, so the
  control and every derived view follow the value); the release publishes ONE edit via the `Emit::commit_transaction`
  shape (every op stamped with the minted `TransactionRef`, `coalesce_key`/`description` cleared). Other emit lanes
  (config, effects, events, UI scope) pass through per tick.
- Host aborts leave zero trace: `abort` argument (React/wgpu blur, unmount), a time-travel freeze (`frozen`), a window
  leaving the roster (`retired`), another press/tool in the window (`captureLost`), a moved document (`baseMoved`: the
  press reopens on the new revision; the leaves are absolute).
- `child_emits` of a scrub dispatch are NOT captured (they pass through as today). A composed-child scrub (flow F6)
  extends the same seam with a child leaf variant — coordinate with W3-T2-CONTROLS instead of inventing a parallel
  machine.

## 3. Framework API (`🧰️framework/🔨️modules/🛠️tool-machine`, crate `semio-framework-tool-machine`)

Rust (`🦀️.rs`, region `🔖️Scrub`) and TS twin (`🟦️.ts`), schema `🧬️schema/🔣️.json` (`ScrubPhase`, `ScrubInput`,
`ScrubOpen`, `ScrubStep`, `ScrubChart`, `ScrubLawFixture`), fixture `🧫️fixtures/🧫️scrub-law/🔣️.json`.

```rust
pub const SCRUB_GESTURE_ARG: &str = "gesture";
pub const SCRUB_COMMIT_ARG: &str = "commit";
pub const SCRUB_ABORT_ARG: &str = "abort";

pub enum ScrubPhase { Tick { gesture: String }, Commit { gesture: String }, Abort { gesture: String, reason: ToolAbortReason } }
impl ScrubPhase {
    pub fn parse(gesture: Option<&str>, commit: Option<bool>, abort: Option<&str>) -> Option<Self>; // None = one-shot dispatch
    pub fn gesture(&self) -> &str;
    pub fn input<M>(self, leaves: Vec<M>) -> ScrubInput<M>;   // leaves = your leaf constructor applied to the value
}
pub enum ScrubInput<M> { Tick { gesture: String, leaves: Vec<M> }, Commit { gesture: String, leaves: Vec<M> }, Abort { reason: ToolAbortReason } }

pub struct ScrubMachine<M>;                 // statechart idle → scrubbing; Effect = ToolYield<M> (a ToolMachine)
pub struct ScrubContext { pub gesture: Option<String>, pub keys: usize }
pub enum ScrubEvent<M> { Tick { gesture, leaves }, Commit { gesture, leaves } }
pub const SCRUB_FINGERPRINT: u64;           // pinned to the statechart! compilation of the chart
pub struct ScrubHost;

pub struct ScrubState<M> { pub states, pub tool, pub actor, pub gesture, pub base_revision, pub transaction: TransactionRef, pub entries: Vec<(String, M)> }

pub struct Scrub<M> ;                       // one press on one revision
impl<M: Clone + 'static> Scrub<M> {
    pub fn start(tool: impl Into<String>, actor: ActorId, base_revision: impl Into<String>) -> Self;
    pub fn resume(state: ScrubState<M>) -> Result<Self, ToolRefusal>;
    pub fn gesture(&self) -> Option<&str>;
    pub fn transaction(&self) -> Option<&ToolTransaction<M>>;   // the preview overlay
    pub fn send(&mut self, input: ScrubInput<M>, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal>;
    pub fn persist(self) -> Option<ScrubState<M>>;              // Some only while a transaction is open
}

pub struct ScrubLedger<M>;                  // every window's open scrub + the press it last closed
impl<M: Clone + 'static> ScrubLedger<M> {
    pub fn send(&mut self, window: &str, tool: &str, actor: &ActorId, base_revision: &str, input: ScrubInput<M>, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal>;
    pub fn abort(&mut self, window: &str, gesture: Option<&str>, reason: ToolAbortReason) -> ToolStep<M>;
    pub fn abort_all(&mut self, reason: ToolAbortReason) -> Vec<ToolStep<M>>;
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<ToolStep<M>>;
    pub fn open(&self, window: &str) -> Option<&ScrubState<M>>;
    pub fn windows(&self) -> impl Iterator<Item = &str>;
    pub fn provisional(&self) -> impl Iterator<Item = &M>;   // render overlay, window id order
    pub fn is_empty(&self) -> bool;
}
```

TS: `SCRUB_*_ARG`, `parseScrubPhase`, `ScrubInput`, `scrubMachine<M>()`, `scrubEvent`, `ScrubHost`, `ScrubState`,
`Scrub.start/resume/send/persist`, `ScrubLedger.send/abort/abortAll/retainWindows/open/windows/provisional`.

Laws (fixture, Rust + TS, xstate + fast-check oracle): each tick replaces the entries with its absolute leaves (upsert
by position `"0"`…, retract the rest); the release commits ONE edit (`Committed`, or `Empty` when the release has no
leaves); a release from idle is a one-shot press; the id is minted at the press's first upsert; another press or tool in
the window, or a moved document revision, drops the open press with zero trace and reopens; a late tick of a closed
(released or cancelled) press is a silent no-op; windows are independent; `abort_all`/`retain_windows` drop presses.
