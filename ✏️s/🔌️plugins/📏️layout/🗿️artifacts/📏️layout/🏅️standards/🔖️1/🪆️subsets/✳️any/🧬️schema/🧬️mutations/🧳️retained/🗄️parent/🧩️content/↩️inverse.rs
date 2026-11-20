//! ↩️ Captures every inheriting page's overrides and the previous parent content in undo order.

use super::*;
use crate::MutationPayload;
use semio_s_artifact_layout_layout_model::{Frame, Layer, Page, PageOverride, ParentPage, snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_mutations_decisions::{set_page_overrides::SetPageOverrides, set_parent_page_content::SetParentPageContent};

#[derive(Clone, Copy, Default)]
enum Phase {
    #[default]
    Reserve,
    Page,
    PageId,
    PageOverrides,
    ParentId,
    ParentLayers,
    ParentFrames,
    Ready,
}

pub struct ParentContentInverse<M: RetireOwned> {
    rows: Saved<Vec<M>>,
    text: Field<String>,
    overrides: Field<Vec<PageOverride>>,
    layers: Field<Vec<Layer>>,
    frames: Field<Vec<Frame>>,
    phase: Phase,
    index: usize,
    closing: bool,
}

impl<M: RetireOwned> Default for ParentContentInverse<M> {
    fn default() -> Self {
        Self { rows: Saved::empty_shell(), text: Default::default(), overrides: Default::default(), layers: Default::default(), frames: Default::default(), phase: Default::default(), index: 0, closing: false }
    }
}

fn parent(base: RetainedCloneRef<'_, LayoutSnapshot>, index: usize) -> RetainedCloneRef<'_, ParentPage> {
    base.project(7, |base| &base.parent_pages).project(index, move |rows| &rows[index])
}

fn page(base: RetainedCloneRef<'_, LayoutSnapshot>, index: usize) -> RetainedCloneRef<'_, Page> {
    base.project(9, |base| &base.pages).project(index, move |rows| &rows[index])
}

impl<M> ParentContentInverse<M>
where
    M: RetireOwned + MutationPayload<SetPageOverrides> + MutationPayload<SetParentPageContent>,
{
    pub fn birth_work() -> Result<u64, ValueError> {
        Ok(OriginalField::<String>::embedded_birth_work()? + OriginalField::<Vec<PageOverride>>::embedded_birth_work()? + OriginalField::<Vec<Layer>>::embedded_birth_work()? + OriginalField::<Vec<Frame>>::embedded_birth_work()?)
    }

    pub fn demand(&self, plan: &ParentContentPlan, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError> {
        plan.verify(command, base)?;
        if self.closing || !plan.ready() || plan.unchanged() { return Err(invalid()); }
        let parent_index = plan.parent().ok_or_else(invalid)?;
        let body = body.min(BODY);
        let limit = maximum.saturating_sub(1);
        if self.text.recycling { return self.text.recycle_demand(body).map(nested); }
        if self.overrides.recycling { return self.overrides.recycle_demand(body).map(nested); }
        use Phase::*;
        Ok(match self.phase {
            Reserve => capacity::<M>(plan.pages().len().checked_add(1).ok_or_else(invalid)?)?,
            PageId => nested(self.text.demand(page(base, plan.pages()[self.index].page).project(0, |p| &p.id), body, limit)?),
            PageOverrides => nested(self.overrides.demand(page(base, plan.pages()[self.index].page).project(14, |p| &p.overrides), body, limit)?),
            ParentId => nested(self.text.demand(parent(base, parent_index).project(0, |p| &p.id), body, limit)?),
            ParentLayers => nested(self.layers.demand(parent(base, parent_index).project(5, |p| &p.layers), body, limit)?),
            ParentFrames => nested(self.frames.demand(parent(base, parent_index).project(6, |p| &p.frames), body, limit)?),
            Page | Ready => fixed(),
        })
    }

    pub fn work_demand(&self, plan: &ParentContentPlan, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64) -> Result<u64, ValueError> {
        plan.verify(command, base)?;
        let parent_index = plan.parent().ok_or_else(invalid)?;
        let body = body.min(BODY);
        let limit = maximum.saturating_sub(1);
        use Phase::*;
        let work = if self.text.recycling { self.text.recycle_work(body)? } else if self.overrides.recycling { self.overrides.recycle_work(body)? } else { match self.phase {
            PageId => self.text.work(page(base, plan.pages()[self.index].page).project(0, |p| &p.id), body, limit)?,
            PageOverrides => self.overrides.work(page(base, plan.pages()[self.index].page).project(14, |p| &p.overrides), body, limit)?,
            ParentId => self.text.work(parent(base, parent_index).project(0, |p| &p.id), body, limit)?,
            ParentLayers => self.layers.work(parent(base, parent_index).project(5, |p| &p.layers), body, limit)?,
            ParentFrames => self.frames.work(parent(base, parent_index).project(6, |p| &p.frames), body, limit)?,
            _ => 0,
        }};
        work.checked_add(1).ok_or_else(invalid)
    }

    pub fn advance(&mut self, plan: &ParentContentPlan, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        plan.verify(command, base)?;
        let grant = effect.grant();
        if !recover {
            if !retained_clone_grant_admits(grant, self.demand(plan, command, base, body, maximum)?) ||
                self.work_demand(plan, command, base, body, maximum)? > maximum.min(effect.remaining_work_units()?) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if !effect.performed(1)? { return Err(invalid().into()); }
        }
        let body = body.min(BODY);
        let ceiling = maximum.saturating_sub(1);
        let effect = &mut effect.nested(child(grant));
        if self.text.recycling {
            return Ok(RetainedCloneStep::Progress(paid(self.text.recycle(body, effect, recover)?)));
        }
        if self.overrides.recycling {
            return Ok(RetainedCloneStep::Progress(paid(self.overrides.recycle(body, effect, recover)?)));
        }
        let previous = parent(base, plan.parent().ok_or_else(invalid)?);
        let mut progress = paid(Default::default());
        use Phase::*;
        match self.phase {
            Reserve if !recover => {
                progress = reserve(self.rows.start(Vec::new())?, plan.pages().len().checked_add(1).ok_or_else(invalid)?, effect)?;
                self.phase = Page;
            }
            Page if !recover => {
                self.phase = if self.index < plan.pages().len() { PageId } else { ParentId };
            }
            PageId | ParentId => {
                let source = if matches!(self.phase, PageId) {
                    page(base, plan.pages()[self.index].page).project(0, |p| &p.id)
                } else {
                    previous.project(0, |p| &p.id)
                };
                let (value, receipt) = self.text.step(source, body, ceiling, effect, recover)?;
                progress = paid(receipt);
                if let Some(id) = value {
                    let rows = self.rows.get_mut()?;
                    self.phase = if matches!(self.phase, PageId) {
                        rows.push(<M as MutationPayload<SetPageOverrides>>::from_payload(SetPageOverrides { id, overrides: Vec::new() }));
                        PageOverrides
                    } else {
                        rows.push(<M as MutationPayload<SetParentPageContent>>::from_payload(SetParentPageContent { id, layers: Vec::new(), frames: Vec::new() }));
                        ParentLayers
                    };
                }
            }
            PageOverrides => {
                let source = page(base, plan.pages()[self.index].page).project(14, |p| &p.overrides);
                let (value, receipt) = self.overrides.step(source, body, ceiling, effect, recover)?;
                progress = paid(receipt);
                if let Some(overrides) = value {
                    let command = self.rows.get_mut()?.last_mut().and_then(<M as MutationPayload<SetPageOverrides>>::payload_mut).ok_or_else(invalid)?;
                    command.overrides = overrides;
                    self.index += 1;
                    self.phase = Page;
                }
            }
            ParentLayers => {
                let (value, receipt) = self.layers.step(previous.project(5, |p| &p.layers), body, ceiling, effect, recover)?;
                progress = paid(receipt);
                if let Some(layers) = value {
                    let command = self.rows.get_mut()?.last_mut().and_then(<M as MutationPayload<SetParentPageContent>>::payload_mut).ok_or_else(invalid)?;
                    command.layers = layers;
                    self.phase = ParentFrames;
                }
            }
            ParentFrames => {
                let (value, receipt) = self.frames.step(previous.project(6, |p| &p.frames), body, ceiling, effect, recover)?;
                progress = paid(receipt);
                if let Some(frames) = value {
                    let command = self.rows.get_mut()?.last_mut().and_then(<M as MutationPayload<SetParentPageContent>>::payload_mut).ok_or_else(invalid)?;
                    command.frames = frames;
                    self.phase = Ready;
                }
            }
            _ => return Err(invalid().into()),
        }
        Ok(if matches!(self.phase, Ready) { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn take_demand(&self) -> RetirementDemand { Saved::<Vec<M>>::take_demand() }
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Vec<M>>, RetainedCloneProgress), ValueError> {
        if self.closing || !matches!(self.phase, Phase::Ready) { return Err(invalid()); }
        self.rows.take(grant)
    }

    retirement!(text, overrides, layers, frames, rows);
}

impl<M: RetireOwned> Drop for ParentContentInverse<M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.closing && self.text.empty() && self.overrides.empty() && self.layers.empty() && self.frames.empty() && self.rows.empty()), "parent content inverse requires funded retirement");
    }
}
