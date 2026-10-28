//! 🧵️ Constant-space column and thread flow driven by actual paragraph source consumption.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowRect { pub x: f64, pub y: f64, pub width: f64, pub height: f64 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowBounds { pub x: f64, pub y: f64, pub width: f64, pub height: f64, pub rotation: f64 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowFrame { pub page_index: usize, pub frame_index: usize, pub parent: bool, pub story_index: usize, pub next: Option<usize>, pub columns: u32, pub gutter: f64, pub bounds: FlowBounds, pub inset: FlowRect }
/// 📚️ Immutable random access over admitted scalar metadata; implementations retain their own paged owners.
pub trait FlowFrameSource {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<FlowFrame>;
    fn is_empty(&self) -> bool { self.len() == 0 }
}
impl FlowFrameSource for [FlowFrame] {
    fn len(&self) -> usize { <[FlowFrame]>::len(self) }
    fn get(&self, index: usize) -> Option<FlowFrame> { <[FlowFrame]>::get(self,index).copied() }
}
#[derive(Clone, Copy)]
pub struct FlowFrameContext { pub page_index: usize, pub frame_index: usize, pub parent: bool, pub story_index: usize, pub next: Option<usize>, pub gutter: f64 }
/// 🧭️ Projects one resolved canonical text frame after stable story and thread addresses are admitted.
pub fn flow_frame_from_model(frame: &semio_s_artifact_layout_layout_model::Frame, context: FlowFrameContext) -> Result<FlowFrame, &'static str> {
    let semio_s_artifact_layout_layout_model::Frame::Text { bounds:b,inset:i,columns,.. } = frame else { return Err("layout.flow.text-frame"); };
    let value = FlowFrame { page_index:context.page_index,frame_index:context.frame_index,parent:context.parent,story_index:context.story_index,next:context.next,gutter:context.gutter,columns:*columns,bounds:FlowBounds{x:b.x,y:b.y,width:b.width,height:b.height,rotation:b.rotation},inset:FlowRect{x:i.x,y:i.y,width:i.width,height:i.height} };
    value.validate()?; Ok(value)
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowRegion { pub frame: usize, pub page_index: usize, pub frame_index: usize, pub parent: bool, pub story_index: usize, pub column: u32, pub start: usize, pub end: usize, pub rect: FlowRect, pub rotation: f64, pub center: [f64; 2] }
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowStep { Progress, Region(FlowRegion), Done, Overset, Cancelled }
#[derive(Clone, Copy)]
enum Terminal { Done, Overset, Cancelled }
impl Terminal { fn step(self) -> FlowStep { match self { Self::Done => FlowStep::Done, Self::Overset => FlowStep::Overset, Self::Cancelled => FlowStep::Cancelled } } }
#[derive(Clone, Copy)]
pub struct TextFlowCursor {
    frame: usize, column: u32, columns: u32, next_frame: Option<usize>, start: usize, entered: bool, visits: usize, story: Option<usize>,
    source: Option<(usize, usize)>, pending: Option<FlowRegion>, terminal: Option<Terminal>,
}
semio_framework_value::artifact_retire_leaf!(TextFlowCursor);
impl FlowFrame {
    fn validate(self) -> Result<(), &'static str> {
        let b = self.bounds; let i = self.inset;
        if [self.page_index, self.frame_index, self.story_index].into_iter().any(|at| at > u32::MAX as usize) || self.next.is_some_and(|at| at > u32::MAX as usize) || self.columns == 0 || ![self.gutter,b.x,b.y,b.width,b.height,b.rotation,i.x,i.y,i.width,i.height].into_iter().all(f64::is_finite) || [self.gutter,b.width,b.height,i.width,i.height].into_iter().any(|value| value < 0.0) { return Err("layout.flow.frame"); }
        Ok(())
    }
}
impl TextFlowCursor {
    /// 🧭️ Starts one independent thread head; source/frame owners remain with the original immutable caller.
    pub fn new(head: usize, start: usize) -> Result<Self, &'static str> {
        if head > u32::MAX as usize || start > u32::MAX as usize { return Err("layout.flow.address"); }
        Ok(Self { frame: head, column: 0, columns: 0, next_frame: None, start, entered: false, visits: 0, story: None, source: None, pending: None, terminal: None })
    }
    fn finish(&mut self, terminal: Terminal) -> FlowStep { self.terminal = Some(terminal); self.source = None; self.pending = None; terminal.step() }
    fn next(&mut self, next: Option<usize>) -> FlowStep { self.column = 0; self.entered = false; match next { Some(frame) => { self.frame = frame; FlowStep::Progress }, None => self.finish(Terminal::Overset) } }
    /// 👣️ Visits at most one frame or column; a pending region waits for its paragraph feedback.
    pub fn advance(&mut self, frames: &[FlowFrame], text: &str, grant: u32) -> Result<FlowStep, &'static str> {
        self.advance_source(frames,text,grant)
    }
    /// 📇️ Resolves one scalar row without flattening its retained paged metadata owner.
    pub fn advance_source<S: FlowFrameSource + ?Sized>(&mut self, frames: &S, text: &str, grant: u32) -> Result<FlowStep, &'static str> {
        if let Some(terminal) = self.terminal { return Ok(terminal.step()); }
        if grant == 0 { return Ok(FlowStep::Progress); }
        let source = (text.as_ptr() as usize, text.len());
        if self.source.is_some_and(|admitted| admitted != source) { return Err("layout.flow.stale-source"); }
        if text.len() > u32::MAX as usize || !text.is_char_boundary(self.start) { return Err("layout.flow.source-boundary"); }
        if self.start == text.len() { return Ok(self.finish(Terminal::Done)); }
        self.source = Some(source);
        if let Some(region) = self.pending { return Ok(FlowStep::Region(region)); }
        if frames.len() as u128 > u128::from(u32::MAX)+1 { return Err("layout.flow.source-length"); }
        let frame = frames.get(self.frame).ok_or("layout.flow.frame-missing")?; frame.validate()?;
        self.columns = frame.columns; self.next_frame = frame.next;
        if !self.entered {
            if self.visits >= frames.len() { return Err("layout.flow.cycle"); }
            if self.story.is_some_and(|story| story != frame.story_index) { return Err("layout.flow.story"); }
            self.story = Some(frame.story_index); self.visits += 1; self.entered = true;
        }
        let b = frame.bounds; let i = frame.inset;
        let left = i.x.max(0.0); let top = i.y.max(0.0); let right = b.width.min(i.x + i.width); let bottom = b.height.min(i.y + i.height);
        let width = (right-left-frame.gutter*f64::from(frame.columns-1))/f64::from(frame.columns); let height = bottom-top;
        if !width.is_finite() || !height.is_finite() { return Err("layout.flow.geometry"); }
        if width <= 0.0 || height <= 0.0 { return Ok(self.next(frame.next)); }
        let rect = FlowRect { x: b.x+left+f64::from(self.column)*(width+frame.gutter), y: b.y+top, width, height }; let center = [b.x+b.width*0.5,b.y+b.height*0.5];
        if ![rect.x,rect.y,center[0],center[1]].into_iter().all(f64::is_finite) { return Err("layout.flow.geometry"); }
        let region = FlowRegion { frame: self.frame, page_index: frame.page_index, frame_index: frame.frame_index, parent: frame.parent, story_index: frame.story_index, column: self.column, start: self.start, end: text.len(), rect, rotation: b.rotation, center };
        self.pending = Some(region); Ok(FlowStep::Region(region))
    }
    /// ✂️ Advances only to the actual consumed UTF-8 boundary, retaining rejected feedback atomically.
    pub fn accept(&mut self, text: &str, end: usize) -> Result<(), &'static str> {
        let region = self.pending.ok_or("layout.flow.feedback")?;
        if self.source != Some((text.as_ptr() as usize,text.len())) || !text.is_char_boundary(end) || end < region.start || end > region.end { return Err("layout.flow.feedback"); }
        self.pending = None; self.start = end;
        if end == text.len() { self.finish(Terminal::Done); return Ok(()); }
        self.column = region.column+1;
        if self.column >= self.columns { self.next(self.next_frame); }
        Ok(())
    }
    pub fn cancel(&mut self) { if self.terminal.is_none() { self.finish(Terminal::Cancelled); } }
    pub fn terminal_is_empty(&self) -> bool { self.terminal.is_some() && self.source.is_none() && self.pending.is_none() }
}
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path="📇️projection/🦀️.rs"]
pub mod projection;
