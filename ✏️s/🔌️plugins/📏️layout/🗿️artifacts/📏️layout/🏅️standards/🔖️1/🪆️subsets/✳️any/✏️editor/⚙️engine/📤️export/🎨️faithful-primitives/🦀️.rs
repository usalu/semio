//! 🎨️ Shared faithful vector paths and bounded nonzero scan conversion for layout exports.
use super::*;
#[path="📐️affine/🦀️.rs"]
pub(super) mod affine;
use affine::AffineEdge as Edge;
use semio_framework_2d_engine::{PathSegment, Vec2};
use semio_framework_2d_text_font::work::FontWorkLedger;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value_persistent::list::PagedList;
use semio_framework_pixels::image_decoding::{ImageDecodeInput, ImageDecodeJob};

const SEGMENT_LIMIT: usize = MAX_LAYOUT_EXPORT_DECODED_ITEMS * 8;
const SAMPLE_OFFSETS: [f64; 4] = [0.125, 0.375, 0.625, 0.875];
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct PaintPath {
    pub segments: PagedList<PathSegment, { usize::MAX }>,
    pub edges: PagedList<Edge, { usize::MAX }>,
    pub fill: Option<[f32; 4]>,
    pub stroke: Option<[f32; 4]>,
    pub stroke_width: f64,
    pub bounds: [f64; 4],
    pub affine: [f64;6],
    inverse: Option<[f64;4]>,
    world:Option<[f64;4]>,
}
impl PaintPath {
    pub(super) fn new(fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>, stroke_width: f64) -> Self { Self { segments: PagedList::new(), edges: PagedList::new(), fill, stroke:if stroke_width==0.0{None}else{stroke}, stroke_width, affine:affine::IDENTITY,inverse:Some([1.0,0.0,0.0,1.0]),world:None,bounds: [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY] } }
    fn transformed(mut self,matrix:[f64;6])->Result<Self,String>{if !matrix.into_iter().all(f64::is_finite){return Err("layout-export-affine-nonfinite".into())}self.affine=matrix;self.inverse=affine::inverse_linear(matrix);Ok(self)}
    pub(super) fn world_bounds(&self)->Option<[f64;4]>{self.world.or_else(||affine::bounds(self.affine,self.bounds,if self.stroke.is_some(){self.stroke_width*0.5}else{0.0}))}
    fn segment(&mut self, value: PathSegment, work:&mut FontWorkLedger) -> Result<bool, String> { append(&mut self.segments, value, work) }
    fn edge(&mut self, value: Edge, work:&mut FontWorkLedger) -> Result<bool, String> {
        if !append(&mut self.edges, value, work)? { return Ok(false); }
        for point in [value.a, value.b] { self.bounds[0] = self.bounds[0].min(point[0]); self.bounds[1] = self.bounds[1].min(point[1]); self.bounds[2] = self.bounds[2].max(point[0]); self.bounds[3] = self.bounds[3].max(point[1]); }
        Ok(true)
    }
}
#[derive(semio_framework_value::RetireOwned)]
pub(super) enum PaintShape { Path(PaintPath), Image { source: usize } }
#[derive(Clone, Copy)]
struct Curve { points: [Vec2; 4], degree: u8, depth: u8 }
semio_framework_value::artifact_retire_leaf!(Curve);
struct CurveStack{rows:[Curve;25],length:usize}
semio_framework_value::artifact_retire_leaf!(CurveStack);
impl CurveStack{
 fn new()->Self{Self{rows:[Curve{points:[[0.0;2];4],degree:0,depth:0};25],length:0}}
 fn last(&self)->Option<&Curve>{self.length.checked_sub(1).map(|at|&self.rows[at])}
 fn pop(&mut self){self.length=self.length.saturating_sub(1);}
 fn push(&mut self,value:Curve)->Result<(),String>{if self.length==self.rows.len(){return Err("layout-export-curve-stack-limit".into());}self.rows[self.length]=value;self.length+=1;Ok(())}
}
#[derive(semio_framework_value::RetireOwned)]
struct BuildPath { path: PaintPath, curves: CurveStack }
#[derive(semio_framework_value::RetireOwned)]
struct PaintOwners { decoders: PagedList<ImageDecodeJob, { usize::MAX }>, shapes: PagedList<PaintShape, { usize::MAX }>, build: Option<BuildPath> }
pub(super) struct FidelityState {
    owners: PaintOwners,
    item: usize,
    source_segment: usize,
    flatten: usize,
    current: Vec2,
    first: Vec2,
    phase: u8,
    segments: usize,
    edge_count: usize,
    image_bytes: usize,
    image_pixels: usize,
    image_count: usize,
    image_decoder: Option<usize>,
    pub prepared: bool,
    pub svg_phase: u8,
    pub svg_segment: usize,
    pub svg_byte: usize,
    sampling: affine::AffineCoverageCursor,
}
impl RetireOwned for FidelityState { fn retirement(self) -> Box<dyn RetirementCursor> { self.owners.retirement() } fn retirement_birth_bytes(&self) -> Option<usize> { self.owners.retirement_birth_bytes() } fn controlled_retirement_supported() -> bool { true } }
fn append<T>(list: &mut PagedList<T, { usize::MAX }>, value: T, work:&mut FontWorkLedger) -> Result<bool, String> { if list.len() >= SEGMENT_LIMIT { return Err("layout-export-path-segment-limit".into()); } if !work.slot(list)? { return Ok(false); } work.push(list,value).map_err(|(error,_)|error.to_owned())?; Ok(true) }
pub(super) fn rotate(p: Vec2, angle: f32, center: [f32; 2]) -> Vec2 { let (s, c) = f64::from(angle).sin_cos(); let dx = p[0] - f64::from(center[0]); let dy = p[1] - f64::from(center[1]); [f64::from(center[0]) + dx * c - dy * s, f64::from(center[1]) + dx * s + dy * c] }
pub(super) fn mapped(segment: &PathSegment, map: impl Fn(Vec2) -> Vec2) -> Result<PathSegment, String> { Ok(match segment { PathSegment::Move { to } => PathSegment::Move { to: map(*to) }, PathSegment::Line { to } => PathSegment::Line { to: map(*to) }, PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: map(*ctrl), to: map(*to) }, PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: map(*ctrl1), ctrl2: map(*ctrl2), to: map(*to) }, PathSegment::Close => PathSegment::Close, _ => return Err("layout-export-path-arc-not-resolved".into()) }) }
fn midpoint(a: Vec2, b: Vec2) -> Vec2 { [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5] }

impl FidelityState {
    pub fn new() -> Self { Self { owners: PaintOwners { decoders: PagedList::new(), shapes: PagedList::new(), build: None }, item: 0, source_segment: 0, flatten: 0, current: [0.0; 2], first: [0.0; 2], phase: 0, segments: 0, edge_count: 0, image_bytes: 0, image_pixels: 0, image_count: 0, image_decoder: None, prepared: false, svg_phase: 0, svg_segment: 0, svg_byte: 0, sampling:Default::default() } }
    pub fn shape(&self, index: usize) -> Option<&PaintShape> { self.owners.shapes.get(index) }
    
    pub fn image_count(&self) -> usize { self.image_count }
    fn next_item(&mut self) { self.item += 1; self.source_segment = 0; self.image_decoder = None; self.flatten = 0; self.current = [0.0; 2]; self.first = [0.0; 2]; self.phase = 0; }
    pub fn prepare_image_one(&mut self, items: &mut PlanItems, snapshot:&LayoutSnapshot,context: &mut StepContext<'_>) -> Result<bool, String> {
        let Some(PdfItem::Image { proxy, link,raster, placeholder, .. }) = items.get_mut(self.item) else { return Ok(true); };
        if *placeholder { return Ok(true); }
        if let Some(image) = raster.as_mut() {
            super::pdf_png_and_package_encoding::raster::raster_length(image.width, image.height, &image.rgba, super::pdf_png_and_package_encoding::raster::RasterPlane::Alpha)?;
            if image.resource.is_none() {
                self.image_pixels = self.image_pixels.checked_add(image.rgba.len() / 4).ok_or("layout-export-image-pixel-limit")?;
                if self.image_pixels > MAX_LAYOUT_EXPORT_PIXELS as usize { return Err("layout-export-image-pixel-limit".into()); }
                image.resource = Some(self.image_count); self.image_count += 1;
            }
            return Ok(true);
        }
        let proxy = image_proxy_fields(proxy.as_deref(),*link,snapshot)?;
        let mime = if proxy.starts_with("data:image/png;") { "image/png" } else if proxy.starts_with("data:image/jpeg;") { "image/jpeg" } else { return Err("layout-export-image-media-unsupported".into()); };
        let index = match self.image_decoder { Some(index) => index, None => {
            let mut work=FontWorkLedger::new(context.retained_grant());let reserved=work.slot(&mut self.owners.decoders);context.consume_retained(work.receipt()).map_err(|error|error.to_string())?;if !reserved? { return Ok(false); }
            self.image_bytes = self.image_bytes.checked_add(proxy.len()).ok_or("layout-export-image-source-limit")?;
            if self.image_bytes > MAX_LAYOUT_EXPORT_RAW_BYTES { return Err("layout-export-image-source-limit".into()); }
            let decoder = ImageDecodeJob::new(ImageDecodeInput { mime, data: proxy, max_source_bytes: MAX_LAYOUT_EXPORT_RAW_BYTES, max_bytes: MAX_LAYOUT_EXPORT_OUTPUT_BYTES.min(64 << 20), max_pixels: MAX_LAYOUT_EXPORT_PIXELS as usize, max_chunks: 512 }).map_err(|error| error.to_string())?;
            let index = self.owners.decoders.len(); let pushed=work.push(&mut self.owners.decoders,decoder);context.consume_retained(work.receipt()).map_err(|error|error.to_string())?;pushed.map_err(|(error,_)|error)?; self.image_decoder = Some(index); return Ok(false);
        } };
        let decoder = self.owners.decoders.get_mut(index).unwrap();
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..context.retained_grant() };
        if decoder.progress().done {
            if let Some((image, receipt)) = decoder.take_result(grant).map_err(|error| error.to_string())? {
                context.consume_retained(receipt).map_err(|error| error.to_string())?;
                *raster = Some(PdfRaster { width: image.width, height: image.height, resource: None, rgba: image.pixels });
                return Ok(false);
            }
        } else { let (_, receipt) = decoder.advance(proxy, grant).map_err(|error| error.to_string())?; context.consume_retained(receipt).map_err(|error| error.to_string())?; }
        Ok(false)
    }
    pub fn prepare_one(&mut self, items: &PlanItems, text: Option<&ExportTextState>, drawing:Option<&DrawingCompositionState>,work:&mut FontWorkLedger) -> Result<(), String> {
        let Some(item) = items.get(self.item) else { self.prepared = true; return Ok(()); };
        if self.phase == 0 {
            match item {
                PdfItem::Image { raster, placeholder, affine:outer, .. } => { if *placeholder { self.owners.build = Some(BuildPath { path: PaintPath::new(Some([0.92, 0.88, 0.84, 1.0]), Some([0.75, 0.35, 0.2, 1.0]), 1.0).transformed(outer.unwrap_or(affine::IDENTITY))?, curves: CurveStack::new() }); } else { if raster.is_none() { return Err("layout-export-image-unavailable".into()); } if append(&mut self.owners.shapes, PaintShape::Image { source: self.item },work)? { self.next_item(); } return Ok(()); } }
                PdfItem::DrawingPath{index}=>{let info=drawing.ok_or("layout-export-drawing-source-missing")?.path(*index)?.info;self.owners.build=Some(BuildPath{path:PaintPath::new(info.paint.fill,info.paint.stroke,info.paint.stroke_width).transformed(info.affine)?,curves:CurveStack::new()});}
                PdfItem::DrawingOutline{paint,affine,..}=>self.owners.build=Some(BuildPath{path:PaintPath::new(paint.fill,paint.stroke,paint.stroke_width).transformed(*affine)?,curves:CurveStack::new()}),
                PdfItem::Outline { color, .. } => self.owners.build = Some(BuildPath { path: PaintPath::new(Some(*color), None, 0.0), curves: CurveStack::new() }),
                PdfItem::Rect { fill, stroke, .. } => self.owners.build = Some(BuildPath { path: PaintPath::new(*fill, *stroke, 1.0), curves: CurveStack::new() }),
                PdfItem::Stroke { fill, color, width, .. } => self.owners.build = Some(BuildPath { path: PaintPath::new(*fill, Some(*color), f64::from(*width)), curves: CurveStack::new() }),
                
            }
            affine::validate_paint(self.owners.build.as_ref().ok_or("layout-export-paint-path-missing")?.path.stroke_width).map_err(str::to_string)?;self.phase = 1; return Ok(());
        }
        if self.phase == 1 {
            
            self.phase = 2; return Ok(());
        }
        if self.phase == 2 {
            let n = self.source_segment;
            let segment = match item {
                PdfItem::DrawingPath{index}=>drawing.ok_or("layout-export-drawing-source-missing")?.path(*index)?.segments.get(n).cloned(),
                PdfItem::DrawingOutline{label,glyph,..}=>drawing.ok_or("layout-export-drawing-source-missing")?.segment(*label,*glyph,n)?.cloned(),
                PdfItem::Outline { source, glyph, italic, rotation, center, .. } => text.ok_or("layout-export-paragraph-source-missing")?.mapped_segment(*source,*glyph,n,*italic,*rotation,*center)?, 
                PdfItem::Rect { x, y, width, height, rotation, .. } => { let points = [[*x, *y], [x + width, *y], [x + width, y + height], [*x, y + height]]; match n { 0..=3 => { let p = affine::point(affine::rotation(*rotation,[x + width * 0.5,y + height * 0.5]),points[n]); Some(if n == 0 { PathSegment::Move { to: p } } else { PathSegment::Line { to: p } }) }, 4 => Some(PathSegment::Close), _ => None } }
                PdfItem::Image { x, y, width, height, rotation, placeholder: true, mark, .. } => placeholder_segment(n, *x, *y, *width, *height, *rotation, mark),
                PdfItem::Stroke { points, closed, .. } => points.get(n).map(|p| if n == 0 { PathSegment::Move { to: [f64::from(p.0), f64::from(p.1)] } } else { PathSegment::Line { to: [f64::from(p.0), f64::from(p.1)] } }).or_else(|| (*closed && n == points.len()).then_some(PathSegment::Close)),
                
                _ => None,
            };
            if let Some(segment) = segment { if self.segments >= SEGMENT_LIMIT { return Err("layout-export-path-segment-limit".into()); } if self.owners.build.as_mut().unwrap().path.segment(segment,work)? { self.source_segment += 1; self.segments += 1; } } else { self.phase = 3; }
            return Ok(());
        }
        if self.phase == 3 { let before = self.owners.build.as_ref().unwrap().path.edges.len(); self.flatten_one(work)?; self.edge_count += self.owners.build.as_ref().unwrap().path.edges.len() - before; if self.edge_count > SEGMENT_LIMIT { return Err("layout-export-path-edge-limit".into()); } return Ok(()); }
        if !work.slot(&mut self.owners.shapes)? { return Ok(()); }
        let mut build = self.owners.build.take().unwrap();build.path.world=build.path.world_bounds();
        work.push(&mut self.owners.shapes,PaintShape::Path(build.path)).map_err(|(error,_)|error)?;
        self.next_item(); Ok(())
    }
    fn flatten_one(&mut self,work:&mut FontWorkLedger) -> Result<(), String> {
        let build = self.owners.build.as_mut().unwrap();
        if let Some(curve) = build.curves.last().copied() {
            let last = curve.points[curve.degree as usize];
            let flat = affine::flat_enough(build.path.affine,curve.points,curve.degree);
            if flat { if build.path.edge(Edge { a: curve.points[0], b: last, stroke: true },work)? { build.curves.pop(); } return Ok(()); }
            if curve.depth >= 24 { return Err("layout-export-curve-subdivision-limit".into()); }
            let p = curve.points; let a = midpoint(p[0], p[1]); let b = midpoint(p[1], p[2]);
            let (left, right) = if curve.degree == 2 { let m = midpoint(a, b); ([p[0], a, m, m], [m, b, p[2], p[2]]) } else { let c = midpoint(p[2], p[3]); let d = midpoint(a, b); let e = midpoint(b, c); let m = midpoint(d, e); ([p[0], a, d, m], [m, e, c, p[3]]) };
            build.curves.pop(); build.curves.push(Curve { points: right, degree: curve.degree, depth: curve.depth + 1 })?; build.curves.push(Curve { points: left, degree: curve.degree, depth: curve.depth + 1 })?; return Ok(());
        }
        let Some(segment) = build.path.segments.get(self.flatten) else { if build.path.fill.is_some() && self.current != self.first { if build.path.edge(Edge { a: self.current, b: self.first, stroke: false },work)? { self.current = self.first; } return Ok(()); } self.phase = 4; return Ok(()); };
        match segment {
            PathSegment::Move { to } => { let to = *to; if build.path.fill.is_some() && self.current != self.first && !build.path.edge(Edge { a: self.current, b: self.first, stroke: false },work)? { return Ok(()); } self.current = to; self.first = to; }
            PathSegment::Line { to } => { let to = *to; if !build.path.edge(Edge { a: self.current, b: to, stroke: true },work)? { return Ok(()); } self.current = to; }
            PathSegment::Close => { if !build.path.edge(Edge { a: self.current, b: self.first, stroke: true },work)? { return Ok(()); } self.current = self.first; }
            PathSegment::Quad { ctrl, to } => { let (ctrl, to) = (*ctrl, *to); build.curves.push(Curve { points: [self.current, ctrl, to, to], degree: 2, depth: 0 })?; self.current = to; }
            PathSegment::Cubic { ctrl1, ctrl2, to } => { let (ctrl1, ctrl2, to) = (*ctrl1, *ctrl2, *to); build.curves.push(Curve { points: [self.current, ctrl1, ctrl2, to], degree: 3, depth: 0 })?; self.current = to; }
            _ => return Err("layout-export-path-arc-not-resolved".into()),
        }
        self.flatten += 1; Ok(())
    }
    pub fn reset_sample(&mut self){self.sampling.reset();}
    pub fn sample_path_one(&mut self,shape:usize,x:u32,y:u32)->Option<([f32;4],[f32;4])>{let Some(PaintShape::Path(path))=self.owners.shapes.get(shape) else{return None};let coverage=self.sampling.advance(&path.edges,path.affine,path.inverse,path.stroke.map(|_|path.stroke_width),x,y,64).coverage?;let mut fill=path.fill.unwrap_or([0.0;4]);let mut stroke=path.stroke.unwrap_or([0.0;4]);fill[3]*=coverage[0];stroke[3]*=coverage[1];Some((fill,stroke))}

}
pub(super) fn source_over(pixel: &mut [u8], color: [f32; 4]) { let alpha = color[3].clamp(0.0, 1.0); let destination_alpha = f32::from(pixel[3]) / 255.0; let output_alpha = alpha + destination_alpha * (1.0 - alpha); for channel in 0..3 { let value = color[channel].clamp(0.0, 1.0) * alpha + f32::from(pixel[channel]) / 255.0 * destination_alpha * (1.0 - alpha); pixel[channel] = if output_alpha > 0.0 { (value / output_alpha * 255.0).round() as u8 } else { 0 }; } pixel[3] = (output_alpha * 255.0).round() as u8; }
pub(super) struct SvgSegment<'a>(&'a PathSegment);
pub(super) fn svg_segment(segment:&PathSegment)->Result<SvgSegment<'_>,&'static str>{if !matches!(segment,PathSegment::Move{..}|PathSegment::Line{..}|PathSegment::Quad{..}|PathSegment::Cubic{..}|PathSegment::Close){return Err("layout-export-path-arc-not-resolved");}Ok(SvgSegment(segment))}
impl std::fmt::Display for SvgSegment<'_>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self.0{PathSegment::Move{to}=>write!(f,"M{} {}",to[0],to[1]),PathSegment::Line{to}=>write!(f,"L{} {}",to[0],to[1]),PathSegment::Quad{ctrl,to}=>write!(f,"Q{} {} {} {}",ctrl[0],ctrl[1],to[0],to[1]),PathSegment::Cubic{ctrl1,ctrl2,to}=>write!(f,"C{} {} {} {} {} {}",ctrl1[0],ctrl1[1],ctrl2[0],ctrl2[1],to[0],to[1]),PathSegment::Close=>f.write_str("Z"),_=>Err(std::fmt::Error)}}}
#[path="🎨️svg-paint/🦀️.rs"]mod svg_paint;
pub(super) fn svg_color(color:Option<[f32;4]>,slot:&str)->Result<svg_paint::PaintAttributes,&'static str>{svg_paint::attributes(slot,color)}

fn image_proxy_fields<'a>(proxy:Option<&'a str>,link:Option<usize>,snapshot:&'a LayoutSnapshot)->Result<&'a str,String>{proxy.or_else(||link.and_then(|at|snapshot.links.get(at)).and_then(|link|link.proxy_data_url.as_deref())).ok_or_else(||"layout-export-image-unavailable".into())}
/// 🧷️ Borrows the original inline or snapshot-linked proxy under the immutable source lease.
pub(super) fn image_proxy<'a>(item:&'a PdfItem,snapshot:&'a LayoutSnapshot)->Result<&'a str,String>{let PdfItem::Image{proxy,link,..}=item else{return Err("layout-export-image-missing".into())};image_proxy_fields(proxy.as_deref(),*link,snapshot)}
pub(super) fn image_affine(item:&PdfItem)->Option<[f64;6]>{let PdfItem::Image{x,y,width,height,rotation,affine:outer,..}=item else{return None};let center=[f64::from(*x)+f64::from(*width)*0.5,f64::from(*y)+f64::from(*height)*0.5];Some(affine::compose(outer.unwrap_or(affine::IDENTITY),affine::rotation(f64::from(*rotation),center)))}
pub(super) fn image_bounds(item:&PdfItem)->Option<[f64;4]>{let PdfItem::Image{x,y,width,height,..}=item else{return None};affine::bounds(image_affine(item)?,[f64::from(*x),f64::from(*y),f64::from(*x)+f64::from(*width),f64::from(*y)+f64::from(*height)],0.0)}

pub(super) fn image_sample(item: &PdfItem, px: u32, py: u32) -> [f32; 4] {
    let PdfItem::Image { x, y, width, height, raster: Some(raster), .. } = item else { return [0.0; 4]; };
    if *width <= 0.0 || *height <= 0.0 || raster.width == 0 || raster.height == 0 { return [0.0; 4]; }
    let Some(matrix)=image_affine(item) else{return [0.0;4]};let Some(inverse)=affine::inverse_linear(matrix) else{return [0.0;4]};let mut sum = [0.0f64; 4];
    for sy in SAMPLE_OFFSETS { for sx in SAMPLE_OFFSETS {
        let p = affine::unmap(matrix,inverse,[f64::from(px)+sx,f64::from(py)+sy]);
        let u = (p[0] - f64::from(*x)) / f64::from(*width); let v = (p[1] - f64::from(*y)) / f64::from(*height);
        if !(0.0..1.0).contains(&u) || !(0.0..1.0).contains(&v) { continue; }
        let tx = u * f64::from(raster.width) - 0.5; let ty = v * f64::from(raster.height) - 0.5; let ix = tx.floor(); let iy = ty.floor(); let fx = tx - ix; let fy = ty - iy;
        for (ox, oy, weight) in [(0.0, 0.0, (1.0 - fx) * (1.0 - fy)), (1.0, 0.0, fx * (1.0 - fy)), (0.0, 1.0, (1.0 - fx) * fy), (1.0, 1.0, fx * fy)] {
            let rx = (ix + ox).clamp(0.0, f64::from(raster.width - 1)) as usize; let ry = (iy + oy).clamp(0.0, f64::from(raster.height - 1)) as usize; let at = (ry * raster.width as usize + rx) * 4;
            let a = f64::from(raster.rgba[at + 3]) / 255.0 * weight; for channel in 0..3 { sum[channel] += f64::from(raster.rgba[at + channel]) / 255.0 * a; } sum[3] += a;
        }
    } }
    if sum[3] == 0.0 { return [0.0; 4]; } [(sum[0] / sum[3]) as f32, (sum[1] / sum[3]) as f32, (sum[2] / sum[3]) as f32, (sum[3] / 16.0) as f32]
}



fn placeholder_segment(n: usize, x: f64, y: f64, width: f64, height: f64, rotation: f64, mark: &str) -> Option<PathSegment> {
    let point = |p: [f64;2], move_to: bool| { let to = affine::point(affine::rotation(rotation,[x+width*0.5,y+height*0.5]),p); if move_to { PathSegment::Move { to } } else { PathSegment::Line { to } } };
    if n < 4 { return Some(point([[x,y],[x+width,y],[x+width,y+height],[x,y+height]][n],n==0)); }
    if n == 4 { return Some(PathSegment::Close); }
    let inset = 4.0_f64.min(width*0.2).min(height*0.2); if inset < 0.5 { return None; }
    let (l,r,t,b)=(x+inset,x+width-inset,y+inset,y+height-inset);let (cx,cy)=((l+r)*0.5,(t+b)*0.5);let at=n-5;
    let value = match mark {
        "stroke" => [([l,t],true),([r,b],false)].get(at).copied(),
        "page" => { if at==4{return Some(PathSegment::Close);} [([l,t],true),([r,t],false),([r,b],false),([l,b],false)].get(at).copied() },
        "map" => [([cx,t],true),([cx,b],false),([l,cy],true),([r,cy],false)].get(at).copied(),
        "curve" => [([l,b],true),([cx,t],false),([cx,t],true),([r,b],false)].get(at).copied(),
        "grid" => {let dx=(r-l)/3.0;let dy=(b-t)/3.0;[([l+dx,t],true),([l+dx,b],false),([l+dx*2.0,t],true),([l+dx*2.0,b],false),([l,t+dy],true),([r,t+dy],false),([l,t+dy*2.0],true),([r,t+dy*2.0],false)].get(at).copied()},
        _ => None,
    }; value.map(|(p,m)|point(p,m))
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod funded_tests;
