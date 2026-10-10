//! 🦴️ Structural support and loading tools use current analytical members and emit authored semantic mutations.
use super::plane::P;
use super::session::{Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS, REJECTED};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::placement_of;
use crate::{ModelMutation, StructuralLocation, StructuralSupport, StructuralLoad, LoadCase, Point3, Restraints};

/// 🖱️ The four structural authoring gestures; line loads retain their first station until the second click.
pub struct Structure { kind: &'static str, anchor: Option<(String, f64, P)>, hover: Option<P> }
impl Structure {
    /// 🛠️ Arms one typed structural tool.
    pub fn new(kind: &'static str) -> Self { Self { kind, anchor: None, hover: None } }
    fn hit(ctx: &ToolContext<'_>, pointer: &Pointer) -> Option<(String, f64)> {
        let storey = ctx.storey()?;
        let placement = placement_of(ctx.snapshot, storey, ctx.inference.storey_levels.get(storey));
        let (sin,cos) = placement.rotation.sin_cos();
        let cursor = [placement.x+cos*pointer.at[0]-sin*pointer.at[1],placement.y+sin*pointer.at[0]+cos*pointer.at[1]];
        let reach = pointer.tolerance * PICK_PIXELS;
        ctx.inference.analytical_members.iter().filter(|(_,member)| member.storey==storey).flat_map(|(id,member)| {
            let count = member.path.len().saturating_sub(1).max(1);
            member.path.windows(2).enumerate().map(move |(index,pair)| {
                let dx=pair[1].x-pair[0].x;let dy=pair[1].y-pair[0].y;let squared=dx*dx+dy*dy;
                let station=if squared>1e-12 {((cursor[0]-pair[0].x)*dx+(cursor[1]-pair[0].y)*dy)/squared} else {0.0}.clamp(0.0,1.0);
                let distance=(cursor[0]-pair[0].x-station*dx).hypot(cursor[1]-pair[0].y-station*dy);
                (distance,id.clone(),(index as f64+station)/count as f64)
            })
        }).filter(|(distance,_,_)|*distance<=reach).min_by(|a,b|a.0.total_cmp(&b.0)).map(|(_,id,station)|(id,station))
    }
}
impl Tool for Structure {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => { self.hover=Some(pointer.at); Step::default() }
            ToolEvent::Down(pointer) => {
                self.hover=Some(pointer.at);
                let Some((member,station))=Self::hit(ctx,pointer) else {return Step::refuse(REJECTED)};
                let location=if self.kind=="load-line" { match self.anchor.take() {
                    None=>{self.anchor=Some((member,station,pointer.at));return Step::default();}
                    Some((first,start,_)) if first==member && (start-station).abs()>1e-6=>StructuralLocation::Line{start:start.min(station),end:start.max(station)},
                    _=>return Step::refuse(REJECTED),
                }} else if self.kind=="load-area" { StructuralLocation::Area } else { StructuralLocation::Point{station} };
                let zero=Point3{x:0.0,y:0.0,z:0.0};
                if self.kind=="support" {
                    let id=ctx.mint("support");
                    let fixed=!pointer.modifiers.shift;
                    return Step::write(ctx,ModelMutation::CreateSupport(crate::mutations::create_support::CreateSupport{id,support:StructuralSupport{name:member.clone(),member,location,offset:zero,restraints:Restraints{x:true,y:true,z:true,rx:fixed,ry:fixed,rz:fixed}}}));
                }
                let mut mutations=Vec::new();
                let load_case=match ctx.library_type(&ctx.snapshot.load_cases) {
                    Some(case)=>case,
                    None=>{let id=ctx.mint("load-case");mutations.push(ModelMutation::CreateLoadCase(crate::mutations::create_load_case::CreateLoadCase{id:id.clone(),load_case:LoadCase{name:id.clone(),category:"gravity".into(),factor:1.0}}));id}
                };
                if crate::location_problem(ctx.snapshot,&member,&location).is_some(){return Step::refuse(REJECTED);}
                let id=ctx.mint("load");
                mutations.push(ModelMutation::CreateLoad(crate::mutations::create_load::CreateLoad{id,load:StructuralLoad{name:member.clone(),load_case,member,location,force:Point3{x:0.0,y:0.0,z:-1000.0},moment:zero}}));
                Step::mutate(mutations)
            }
            ToolEvent::Escape | ToolEvent::Lost => { self.anchor=None;self.hover=None;Step::default() }
            _ => Step::default(),
        }
    }
    fn preview(&self, _: &ToolContext<'_>) -> Preview {
        match (&self.anchor,self.hover) { (Some((_,_,start)),Some(end))=>Preview::of(vec![Mark::dot(*start,Style::Handle),Mark::path(&[*start,end],false,Style::Ghost)]),(_,Some(at))=>Preview::of(vec![Mark::dot(at,Style::Handle)]),_=>Preview::default() }
    }
    fn anchor(&self) -> Option<P> {self.anchor.as_ref().map(|(_,_,p)|*p)}
}
