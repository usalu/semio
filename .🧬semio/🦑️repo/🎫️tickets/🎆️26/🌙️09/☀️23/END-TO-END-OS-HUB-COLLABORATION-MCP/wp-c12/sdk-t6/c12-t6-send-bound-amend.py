"""🧵️ C12: one-shot, idempotent amendment of `c12-sdk-composition-patch.py` after the overlay proof `t6-overlay-proof-1` (E0599 ×2):
`maintenance_stage_step` and the maintenance impl carry `M: Send`, the composition impl (follow pass, admissions, member lanes)
does not. The stage-4 body becomes `child_root_retirement_step` in the composition impl beside `child_member_retirement_step`
(stage 4 calls it), `reclaim_child_retirements` drives those two bodies directly, and `derivable_follow_awaits_retirement`
moves into the composition impl. Usage: python3 c12-t6-send-bound-amend.py"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PATCH = os.path.join(HERE, "c12-sdk-composition-patch.py")
SDK = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
text = open(PATCH, encoding="utf-8").read()
if "CHILD_ROOT_FN = " in text:
    print("already amended")
    raise SystemExit(0)
sdk = open(SDK, encoding="utf-8").read()
head = "                4 => {\n                    let Some((index, generation)) = self.child_content_retirements.next_id_from(self.maintenance_child_root_cursor) else {\n"
start = sdk.index(head)
end = sdk.index("                5 => {\n", start)
arm = sdk[start:end]
assert sdk.count(head) == 1 and arm.endswith("                }\n") and '"""' not in arm and "\\" not in arm
body = [line[8:] if line.strip() else line for line in arm.splitlines(keepends=True)[1:-1]]
assert all(line.startswith("            ") or not line.strip() for line in body)
child_root_fn = (
    "        /// 🍂️ One bounded unit of child-root retirement — the body of stage [`MAINTENANCE_CHILD_ROOT_STAGE`], also run out of\n"
    "        /// turn under child pressure and by [`Self::reclaim_child_retirements`] (which needs it without the maintenance impl's\n"
    "        /// `M: Send`).\n"
    "        fn child_root_retirement_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {\n"
    + "".join(body)
    + "        }\n\n"
)


def swap(old, new):
    global text
    assert text.count(old) == 1, (old[:80], text.count(old))
    text = text.replace(old, new)


swap('''ROOT_STAGE_OLD = """                4 => {
                    let Some((index, generation)) = self.child_content_retirements.next_id_from(self.maintenance_child_root_cursor) else {
"""
ROOT_STAGE_NEW = """                MAINTENANCE_CHILD_ROOT_STAGE => {
                    let Some((index, generation)) = self.child_content_retirements.next_id_from(self.maintenance_child_root_cursor) else {
"""
''', 'ROOT_STAGE_OLD = """' + arm + '"""\nROOT_STAGE_NEW = """                MAINTENANCE_CHILD_ROOT_STAGE => self.child_root_retirement_step(maximum_items, maximum_bytes),\n"""\n'
     + 'CHILD_ROOT_ANCHOR = """        fn child_member_retirement_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {\n"""\n'
     + 'CHILD_ROOT_FN = """' + child_root_fn + '"""\n')

swap('''        /// ⏳️ A follow pass waits on child retirements and the parent has not been followed since — pending, runnable
        /// work until maintenance returned them (see `child_follow_awaits_retirement`).
        fn derivable_follow_awaits_retirement(&self) -> bool {
            self.child_follow_awaits_retirement && self.store.generation_now() != self.followed_parent_generation
        }

"""''', '"""')

swap('''ADMIT_NEW = """        /// ♻️ Whether the next''', '''ADMIT_NEW = """        /// ⏳️ A follow pass waits on child retirements and the parent has not been followed since — pending, runnable
        /// work until maintenance returned them (see `child_follow_awaits_retirement`).
        fn derivable_follow_awaits_retirement(&self) -> bool {
            self.child_follow_awaits_retirement && self.store.generation_now() != self.followed_parent_generation
        }

        /// ♻️ Whether the next''')

swap('''                let mut released = false;
                for stage in [MAINTENANCE_CHILD_ROOT_STAGE, MAINTENANCE_CHILD_MEMBER_STAGE] {
                    let step = self.maintenance_stage_step(stage, 1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP)?;
                    released |= matches!(step, PluginCloseStep::Pending { released_items, released_bytes } if released_items > 0 || released_bytes > 0);
                }
                if !released {
                    break;
                }
''', '''                let released = |step: PluginCloseStep| matches!(step, PluginCloseStep::Pending { released_items, released_bytes } if released_items > 0 || released_bytes > 0);
                let root = released(self.child_root_retirement_step(1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP)?);
                let member = released(self.child_member_retirement_step(1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP)?);
                if !root && !member {
                    break;
                }
''')

swap('''        (ADMIT_OLD, ADMIT_NEW),
''', '''        (ADMIT_OLD, ADMIT_NEW),
        (CHILD_ROOT_ANCHOR, CHILD_ROOT_FN + CHILD_ROOT_ANCHOR),
''')

open(PATCH, "w", encoding="utf-8").write(text)
print("amended")
