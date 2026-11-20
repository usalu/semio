//! 🔺️ Publishes one normalized parent row and removes stale override rows by original position.

use super::*;
use semio_s_artifact_layout_layout_diff::{LayoutDiff, LayoutPagesDelta, LayoutPagesModification, LayoutParentPageInsertion, LayoutParentPageRemoval, LayoutParentPagesDelta, PageOverrideRemoval, PagePatch};
use semio_s_artifact_layout_layout_diff::retained::apply::page_cache::{PageCachePreparation, PageLayerCacheSource};
use semio_s_artifact_layout_layout_model::{Frame, Layer, Page, ParentPage, snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_mutations_decisions::{LayoutMutationSource, LayoutMutationView, set_parent_page_content::SetParentPageContent};

#[derive(Default, Clone, Copy)]
enum Phase {
    #[default]
    ParentRemoval,
    ParentInsert,
    PageHeaders,
    RemovedId,
    ParentId,
    ParentName,
    Layers,
    Frames,
    Cache,
    Install,
    Page,
    PageId,
    OverrideHeaders,
    OverrideId,
    Ready,
}

struct Source<'a>(&'a SetParentPageContent);

impl PageLayerCacheSource for Source<'_> {
    fn identity(&self) -> usize { self.0 as *const SetParentPageContent as usize }
    fn layer_count(&self) -> usize { self.0.layers.len() }
    fn layer_id(&self, row: usize) -> &str { &self.0.layers[row].id }
    fn frame_count(&self) -> usize { self.0.frames.len() }
    fn frame_id(&self, row: usize) -> &str { self.0.frames[row].id() }
    fn frame_layer_id(&self, row: usize) -> &str { self.0.frames[row].layer_id() }
}

fn proposal<M: LayoutMutationSource>(command: RetainedCloneRef<'_, M>) -> RetainedCloneRef<'_, SetParentPageContent> {
    command.project(901, |mutation| match mutation.view() {
        LayoutMutationView::SetParentPageContent(value) => value,
        _ => unreachable!("validated parent content command"),
    })
}

fn parent(base: RetainedCloneRef<'_, LayoutSnapshot>, index: usize) -> RetainedCloneRef<'_, ParentPage> {
    base.project(7, |base| &base.parent_pages).project(index, move |pages| &pages[index])
}

fn page(base: RetainedCloneRef<'_, LayoutSnapshot>, index: usize) -> RetainedCloneRef<'_, Page> {
    base.project(9, |base| &base.pages).project(index, move |pages| &pages[index])
}

pub struct ParentContentOutput {
    diff: Saved<LayoutDiff>,
    cache: Option<PageCachePreparation>,
    text: Field<String>,
    layers: Field<Vec<Layer>>,
    frames: Field<Vec<Frame>>,
    phase: Phase,
    page: usize,
    entry: usize,
    closing: bool,
}

impl Default for ParentContentOutput {
    fn default() -> Self {
        Self { diff: Saved::empty_shell(), cache: Default::default(), text: Default::default(), layers: Default::default(), frames: Default::default(), phase: Default::default(), page: 0, entry: 0, closing: false }
    }
}

impl ParentContentOutput {
    pub fn birth_work() -> Result<u64, ValueError> {
        Ok(OriginalField::<String>::embedded_birth_work()? + OriginalField::<Vec<Layer>>::embedded_birth_work()? + OriginalField::<Vec<Frame>>::embedded_birth_work()?)
    }

    fn text_source<'a>(&self, plan: &ParentContentPlan, base: RetainedCloneRef<'a, LayoutSnapshot>) -> RetainedCloneRef<'a, String> {
        match self.phase {
            Phase::RemovedId | Phase::ParentId => parent(base, plan.parent().expect("resolved parent")).project(0, |row| &row.id),
            Phase::ParentName => parent(base, plan.parent().expect("resolved parent")).project(1, |row| &row.name),
            Phase::PageId => page(base, plan.pages()[self.page].page).project(0, |row| &row.id),
            Phase::OverrideId => {
                let inherited = plan.pages()[self.page];
                let index = plan.removed_overrides(inherited)[self.entry];
                page(base, inherited.page).project(14, |row| &row.overrides).project(index, move |rows| &rows[index].object_id)
            }
            _ => unreachable!("parent content text phase"),
        }
    }

    pub fn demand<M: LayoutMutationSource>(&self, plan: &ParentContentPlan, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError> {
        plan.verify(command, base)?;
        if self.closing || !plan.ready() || plan.unchanged() || plan.parent().is_none() { return Err(invalid()); }
        let body = body.min(BODY);
        let limit = maximum.saturating_sub(1);
        if self.text.recycling { return self.text.recycle_demand(body).map(nested); }
        use Phase::*;
        Ok(match self.phase {
            ParentRemoval => capacity::<LayoutParentPageRemoval>(1)?,
            ParentInsert => capacity::<LayoutParentPageInsertion>(1)?,
            PageHeaders => capacity::<LayoutPagesModification>(plan.changed_pages())?,
            RemovedId | ParentId | ParentName | PageId | OverrideId => nested(self.text.demand(self.text_source(plan, base), body, limit)?),
            Layers => nested(self.layers.demand(proposal(command).project(1, |p| &p.layers), body, limit)?),
            Frames => nested(self.frames.demand(proposal(command).project(2, |p| &p.frames), body, limit)?),
            Cache => self.cache.as_ref().map_or(Ok(fixed()), |cache| cache.demand(&Source(proposal(command).get())).map(nested))?,
            Install => nested(self.cache.as_ref().ok_or_else(invalid)?.install_demand()?),
            OverrideHeaders => capacity::<PageOverrideRemoval>(plan.removed_overrides(plan.pages()[self.page]).len())?,
            Page | Ready => fixed(),
        })
    }

    pub fn work_demand<M: LayoutMutationSource>(&self, plan: &ParentContentPlan, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64) -> Result<u64, ValueError> {
        plan.verify(command, base)?;
        if self.closing || !plan.ready() || plan.unchanged() || plan.parent().is_none() { return Err(invalid()); }
        let body = body.min(BODY);
        let limit = maximum.saturating_sub(1);
        use Phase::*;
        let work = if self.text.recycling { self.text.recycle_work(body)? } else { match self.phase {
            RemovedId | ParentId | ParentName | PageId | OverrideId => self.text.work(self.text_source(plan, base), body, limit)?,
            Layers => self.layers.work(proposal(command).project(1, |p| &p.layers), body, limit)?,
            Frames => self.frames.work(proposal(command).project(2, |p| &p.frames), body, limit)?,
            Cache => self.cache.as_ref().map_or(Ok(0), |cache| cache.work_demand(&Source(proposal(command).get())))?,
            _ => 0,
        }};
        work.checked_add(1).ok_or_else(invalid)
    }

    pub fn advance<M: LayoutMutationSource>(&mut self, plan: &ParentContentPlan, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
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
        let limit = maximum.saturating_sub(1);
        if self.text.recycling {
            return Ok(RetainedCloneStep::Progress(paid(self.text.recycle(body, &mut effect.nested(child(grant)), recover)?)));
        }
        let index = plan.parent().ok_or_else(invalid)?;
        let mut progress = paid(Default::default());
        use Phase::*;
        match self.phase {
            ParentRemoval if !recover => {
                progress = reserve(&mut self.diff.start(LayoutDiff::default())?.parent_pages.insert(LayoutParentPagesDelta::default()).removed, 1, effect)?;
                self.phase = ParentInsert;
            }
            ParentInsert if !recover => {
                let rows = &mut self.diff.get_mut()?.parent_pages.as_mut().ok_or_else(invalid)?.inserted;
                progress = reserve(rows, 1, effect)?;
                let old = &base.get().parent_pages[index];
                let row = ParentPage { id: String::new(), name: String::new(), width: old.width, height: old.height, layer_ids: Vec::new(), layers: Vec::new(), frames: Vec::new() };
                rows.push(LayoutParentPageInsertion { index, row });
                self.phase = PageHeaders;
            }
            PageHeaders if !recover => {
                if plan.changed_pages() != 0 {
                    progress = reserve(&mut self.diff.get_mut()?.pages.insert(LayoutPagesDelta::default()).modified, plan.changed_pages(), effect)?;
                }
                self.phase = RemovedId;
            }
            RemovedId | ParentId | ParentName | PageId | OverrideId => {
                let source = self.text_source(plan, base);
                let (value, receipt) = self.text.step(source, body, limit, &mut effect.nested(child(grant)), recover)?;
                progress = paid(receipt);
                if let Some(value) = value {
                    let diff = self.diff.get_mut()?;
                    self.phase = match self.phase {
                        RemovedId => { diff.parent_pages.as_mut().ok_or_else(invalid)?.removed.push(LayoutParentPageRemoval { id: value, index }); ParentId }
                        ParentId => { diff.parent_pages.as_mut().ok_or_else(invalid)?.inserted[0].row.id = value; ParentName }
                        ParentName => { diff.parent_pages.as_mut().ok_or_else(invalid)?.inserted[0].row.name = value; Layers }
                        PageId => { diff.pages.as_mut().ok_or_else(invalid)?.modified.push(LayoutPagesModification { id: value, patch: PagePatch::default() }); OverrideHeaders }
                        OverrideId => {
                            let rows = plan.removed_overrides(plan.pages()[self.page]);
                            diff.pages.as_mut().ok_or_else(invalid)?.modified.last_mut().ok_or_else(invalid)?.patch.overrides.removed.push(PageOverrideRemoval { id: value, index: rows[self.entry] });
                            self.entry += 1;
                            if self.entry == rows.len() { self.page += 1; self.entry = 0; Page } else { OverrideId }
                        }
                        _ => unreachable!(),
                    };
                }
            }
            Layers => {
                let (value, receipt) = self.layers.step(proposal(command).project(1, |p| &p.layers), body, limit, &mut effect.nested(child(grant)), recover)?;
                progress = paid(receipt);
                if let Some(value) = value {
                    self.diff.get_mut()?.parent_pages.as_mut().ok_or_else(invalid)?.inserted[0].row.layers = value;
                    self.phase = Frames;
                }
            }
            Frames => {
                let (value, receipt) = self.frames.step(proposal(command).project(2, |p| &p.frames), body, limit, &mut effect.nested(child(grant)), recover)?;
                progress = paid(receipt);
                if let Some(value) = value {
                    self.diff.get_mut()?.parent_pages.as_mut().ok_or_else(invalid)?.inserted[0].row.frames = value;
                    self.phase = Cache;
                }
            }
            Cache if !recover => {
                let Some(cache) = self.cache.as_mut() else {
                    self.cache = Some(PageCachePreparation::new());
                    return Ok(RetainedCloneStep::Progress(progress));
                };
                let source = Source(proposal(command).get());
                let allocating = cache.demand(&source)?.capacity_bytes != 0;
                if allocating {
                    if effect.actual().is_some() { return Err(invalid().into()); }
                    *effect.actual() = Some(0);
                }
                let step = cache.advance(&source, &mut effect.nested(child(grant)));
                if allocating {
                    *effect.actual() = Some(step.as_ref().map_or_else(|error| error.retained_progress(), |step| step.progress()).retained_capacity_bytes);
                }
                progress = paid(step?.progress());
                if cache.is_ready() { self.phase = Install; }
            }
            Install if !recover => {
                let row = &mut self.diff.get_mut()?.parent_pages.as_mut().ok_or_else(invalid)?.inserted[0].row;
                let step = self.cache.as_mut().ok_or_else(invalid)?.install_prepared_parent(row, child(grant))?;
                progress = paid(step.progress());
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = Page; }
            }
            Page if !recover => {
                if let Some(&row) = plan.pages().get(self.page) {
                    if plan.removed_overrides(row).is_empty() { self.page += 1; } else { self.phase = PageId; }
                } else {
                    self.phase = Ready;
                }
            }
            OverrideHeaders if !recover => {
                let rows = &mut self.diff.get_mut()?.pages.as_mut().ok_or_else(invalid)?.modified.last_mut().ok_or_else(invalid)?.patch.overrides.removed;
                progress = reserve(rows, plan.removed_overrides(plan.pages()[self.page]).len(), effect)?;
                self.phase = OverrideId;
            }
            _ => return Err(invalid().into()),
        }
        Ok(if matches!(self.phase, Ready) { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn take_demand(&self) -> RetirementDemand { Saved::<LayoutDiff>::take_demand() }
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Result<(Option<LayoutDiff>, RetainedCloneProgress), ValueError> {
        if self.closing || !matches!(self.phase, Phase::Ready) { return Err(invalid()); }
        self.diff.take(grant)
    }

    retirement!(text, layers, frames, cache, diff);
}

impl Drop for ParentContentOutput {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "parent content output requires funded retirement");
    }
}
