import fs from "node:fs";
const p = "commands-conditions.rs";
let s = fs.readFileSync(p, "utf8");
s = s.replace(`pub fn handle_apply(payload: &ApplyConditions, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let spaces = spaces_of(snapshot, &payload.ids, &ctx.selected);`, `/// 🌡️ A command on the conditions of spaces: what it emits for a snapshot and the current selection.
pub trait Conditions {
    fn emit(&self, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault>;
}

impl Conditions for ApplyConditions {
    fn emit(&self, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        apply(self, snapshot, selected)
    }
}

impl Conditions for ClearConditions {
    fn emit(&self, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        clear(self, snapshot, selected)
    }
}

fn apply(payload: &ApplyConditions, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let spaces = spaces_of(snapshot, &payload.ids, selected);`);
s = s.replace(`pub fn handle_clear(payload: &ClearConditions, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let spaces = spaces_of(snapshot, &payload.ids, &ctx.selected);`, `fn clear(payload: &ClearConditions, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let spaces = spaces_of(snapshot, &payload.ids, selected);`);
s = s.replace(`#[cfg(test)]
#[path`, `pub fn handle<P: Conditions>(payload: &P, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    payload.emit(doc.snapshot, &ctx.selected)
}

#[cfg(test)]
#[path`);
fs.writeFileSync(p, s);
