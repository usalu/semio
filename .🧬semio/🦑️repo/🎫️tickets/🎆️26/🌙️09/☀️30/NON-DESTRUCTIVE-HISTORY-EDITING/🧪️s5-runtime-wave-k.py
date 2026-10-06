"""🧹️ Wave K (goal-audit residue, coordinator 10:2x item 4): the last `.expect(` on the history verbs' path is an answered
refusal.

`step_supersede_authoring` (the undo or redo of a finalize, stepped per reactor turn) took its finished authoring with
`.expect("a finished authoring replay was held")`; a turn that finds none now answers `SupersedeAuthored::Refused` like the
guard at the head of the same function.

Loaded by `🧪️s5-runtime-land.py`.
"""

TT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"

TT_RS = [
    (
        """        let SupersedeAuthoring { replay, finalization } = self.time_travel.authoring.take().expect("a finished authoring replay was held");
""",
        """        let Some(SupersedeAuthoring { replay, finalization }) = self.time_travel.authoring.take() else { return Ok(SupersedeAuthored::Refused) };
""",
    ),
]


def files(_root):
    return {TT: TT_RS}
