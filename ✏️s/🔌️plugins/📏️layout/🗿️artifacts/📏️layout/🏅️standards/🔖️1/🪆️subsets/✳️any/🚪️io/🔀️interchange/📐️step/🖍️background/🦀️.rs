//! 🖍️ STEP background traversal borrows one qualified Drawing graph and retains numeric cursors.
use super::*;
use geometry::{IDENTITY,apply,multiply};
use semio_framework_io_ports::admission::vector::{VectorContentRef,VectorFillHead,VectorGraphSourceV1,VectorNodeSourceV1,VectorShapeRef};
use semio_framework_value::paged::Utf8Text;
use semio_s_artifact_stdio_contract::part21::Part21Instance;
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::draughting::DraughtingWriter;
#[path="📐️geometry/🦀️.rs"]mod geometry;

#[derive(Clone,Copy,Debug)]
struct Paint { fill:Option<[f64;3]>, stroke:Option<[f64;3]>, width:f64 }
impl Default for Paint {
    fn default()->Self { Self { fill:None, stroke:None, width:1.0 } }
}
#[derive(Debug)]
struct Route { index:usize, child:usize, entered:bool, content_done:bool, transform:[f64;16] }
#[derive(Debug)]
pub(super) struct StepBackground {
    root:usize, route:Vec<Route>, canvas:bool, paint:Paint, transform:[f64;16],
    points:[[f64;3];65], point_count:usize, current:[f64;2], start:[f64;2],
    node:usize, part:usize, segment:usize, text_chunk:usize, text_byte:usize,
    text_offset:usize, text_x:f64, text_y:f64,
}
fn rgba(color:[f64;4])->[f64;3] { [color[0],color[1],color[2]] }
fn matrix([a,b,c,d,e,f]:[f64;6])->[f64;16] {
    [a,b,0.0,0.0,c,d,0.0,0.0,0.0,0.0,1.0,0.0,e,f,0.0,1.0]
}
fn shape_segment(shape:VectorShapeRef<'_>, index:usize)->Option<VectorSegment> {
    let vertex = |[x,y]:[f64;2]| {
        let to=VectorPoint2{x,y};
        if index==0 { VectorSegment::MoveTo{to} } else { VectorSegment::LineTo{to} }
    };
    match shape {
        VectorShapeRef::Rect{x,y,width,height} => {
            if index==4 { return Some(VectorSegment::Close); }
            [[x,y],[x+width,y],[x+width,y+height],[x,y+height]].get(index).copied().map(vertex)
        }
        VectorShapeRef::Line{x1,y1,x2,y2} => [[x1,y1],[x2,y2]].get(index).copied().map(vertex),
        VectorShapeRef::Polygon(source) => {
            if index==source.point_count() { Some(VectorSegment::Close) } else { source.point(index).map(vertex) }
        }
        VectorShapeRef::Circle{cx,cy,r} => shape_segment(VectorShapeRef::Ellipse{cx,cy,rx:r,ry:r},index),
        VectorShapeRef::Ellipse{cx,cy,rx,ry} => {
            if index==64 { return Some(VectorSegment::Close); }
            if index>64 { return None; }
            let (sin,cos)=(index as f64*std::f64::consts::TAU/64.0).sin_cos();
            Some(vertex([cx+rx*cos,cy+ry*sin]))
        }
    }
}
impl StepBackground {
    pub(super) fn new()->Self {
        Self {
            root:0, route:Vec::new(), canvas:false, paint:Paint::default(), transform:IDENTITY,
            points:[[0.0;3];65], point_count:0, current:[0.0;2], start:[0.0;2], node:0,
            part:0, segment:0, text_chunk:0, text_byte:0, text_offset:0, text_x:0.0, text_y:0.0,
        }
    }
    pub(super) fn next_source(&mut self) {
        self.root=0;
        self.canvas=false;
    }
 fn point(&self,x:f64,y:f64,height:f64)->[f64;3]{let point=apply(self.transform,x,y);[point[0]*super::geometry::MM,(height-point[1])*super::geometry::MM,0.0]}
 fn flush(&mut self,writer:&mut DraughtingWriter,fill:bool)->Result<Vec<Part21Instance>,String>{if self.point_count<2{self.point_count=0;return Ok(Vec::new())}let name=format!("background-{}-{}",self.node,self.part);self.part+=1;if self.paint.stroke.is_some(){let scale=(self.transform[0]*self.transform[5]-self.transform[1]*self.transform[4]).abs().sqrt();writer.set_curve_width(self.paint.width*scale*super::geometry::MM)?;}let output=writer.path(&name,&self.points[..self.point_count],self.paint.stroke,if fill{self.paint.fill}else{None})?;self.point_count=0;Ok(output)}
 fn push(&mut self,point:[f64;3],writer:&mut DraughtingWriter,out:&mut Vec<Part21Instance>)->Result<(),String>{if self.point_count==65{if self.paint.fill.is_some(){return Err("layout.step.background-fill-point-limit".into());}let last=self.points[64];out.extend(self.flush(writer,false)?);self.points[0]=last;self.point_count=1;}self.points[self.point_count]=point;self.point_count+=1;Ok(())}
 fn finish_path(&mut self,writer:&mut DraughtingWriter)->Result<Vec<Part21Instance>,String>{if self.point_count>1&&self.paint.fill.is_some()&&self.points[0]!=self.points[self.point_count-1]{if self.point_count>=65{return Err("layout.step.background-fill-point-limit".into());}self.points[self.point_count]=self.points[0];self.point_count+=1;}self.flush(writer,true)}

    fn node<'a>(&self, graph:&'a dyn VectorGraphSourceV1)->Option<&'a dyn VectorNodeSourceV1> {
        let mut route=self.route.iter();
        let mut node=graph.root(route.next()?.index)?;
        for row in route { node=node.child(row.index)?; }
        Some(node)
    }
    fn enter(&mut self, transform:[f64;6], visible:bool, paint:&dyn semio_framework_io_ports::admission::vector::VectorPaintSourceV1)->Result<(),String> {
        if !visible {
            self.route.pop();
            return Ok(());
        }
        let parent=self.route.iter().rev().nth(1).map_or(IDENTITY,|row|row.transform);
        let row=self.route.last_mut().ok_or("layout.step.background-route")?;
        row.transform=multiply(parent,matrix(transform));
        row.entered=true;
        self.transform=row.transform;
        self.node+=1;
        self.part=0;
        self.segment=0;
        self.text_chunk=0;
        self.text_byte=0;
        self.text_offset=0;
        self.text_x=0.0;
        self.text_y=0.0;
        let fill=match paint.fill() {
            Some(VectorFillHead::Solid{color})=>Some(rgba(color)),
            Some(_)=>return Err("layout.step.background-gradient".into()),
            None=>None,
        };
        let stroke=paint.stroke().filter(|stroke| stroke.width!=0.0);
        self.paint=Paint{fill,stroke:stroke.map(|stroke|rgba(stroke.color)),width:stroke.map_or(1.0,|stroke|stroke.width)};
        Ok(())
    }
    fn emit_segment(&mut self,segment:VectorSegment,page:&Page,writer:&mut DraughtingWriter)->Result<Vec<Part21Instance>,String> {
        let mut out=Vec::new();
        match segment {
    VectorSegment::MoveTo{to}=>{if self.point_count>0{if self.paint.fill.is_some(){return Err("layout.step.background-compound-fill".into());}out.extend(self.flush(writer,false)?);}self.current=[to.x,to.y];self.start=self.current;let point=self.point(to.x,to.y,page.height);self.push(point,writer,&mut out)?;},
    VectorSegment::LineTo{to}=>{let point=self.point(to.x,to.y,page.height);self.push(point,writer,&mut out)?;self.current=[to.x,to.y];},
    VectorSegment::CubicTo{c1,c2,to}=>{for[x,y]in geometry::cubic_points(self.current,[c1.x,c1.y],[c2.x,c2.y],[to.x,to.y]){let point=self.point(x,y,page.height);self.push(point,writer,&mut out)?;}self.current=[to.x,to.y];},
    VectorSegment::QuadTo{c,to}=>{for[x,y]in geometry::quad_points(self.current,[c.x,c.y],[to.x,to.y]){let point=self.point(x,y,page.height);self.push(point,writer,&mut out)?;}self.current=[to.x,to.y];},
    VectorSegment::ArcTo{rx,ry,x_rotation,large_arc,sweep,to}=>{for[x,y]in geometry::arc_points(self.current,[to.x,to.y],[rx,ry],x_rotation,large_arc,sweep)?{let point=self.point(x,y,page.height);self.push(point,writer,&mut out)?;}self.current=[to.x,to.y];},VectorSegment::Close=>{let point=self.point(self.start[0],self.start[1],page.height);if self.point_count==0||self.points[self.point_count-1]!=point{self.push(point,writer,&mut out)?;}self.current=self.start;},

        }
        Ok(out)
    }
    fn text(&mut self, content:&dyn Utf8Text, [x,y,size]:[f64;3], page:&Page, writer:&mut DraughtingWriter)->Result<(bool,Vec<Part21Instance>),String> {
        let Some(chunk)=content.text_chunk(self.text_chunk) else { return Ok((true,Vec::new())); };
        if self.text_byte==chunk.len() {
            self.text_chunk+=1;
            self.text_byte=0;
            return Ok((false,Vec::new()));
        }
        let mut end=(self.text_byte+512).min(chunk.len());
        while !chunk.is_char_boundary(end) { end-=1; }
        if let Some(line)=chunk[self.text_byte..end].find('\n') { end=self.text_byte+line+1; }
        let literal=&chunk[self.text_byte..end];
        let point=self.point(x+self.text_x,y+self.text_y,page.height);
        let advance=literal.chars().count().max(1) as f64*size*0.6;
        let width=advance*self.transform[0].hypot(self.transform[1]);
        let height=size*self.transform[4].hypot(self.transform[5]);
        let output=writer.text(&format!("background-text-{}@{}",self.node,self.text_offset),literal,point,
            -self.transform[1].atan2(self.transform[0]),width*super::geometry::MM,height*super::geometry::MM,
            height*super::geometry::MM,self.paint.fill.or(self.paint.stroke))?;
        self.text_offset+=literal.len();
        self.text_byte=end;
        if literal.ends_with('\n') { self.text_x=0.0; self.text_y+=size*1.2; } else { self.text_x+=advance; }
        Ok((false,output))
    }
    fn image(&self, key:&dyn Utf8Text, width:f64, height:f64, page:&Page, writer:&mut DraughtingWriter)->Result<Vec<Part21Instance>,String> {
        let points=[[0.0,0.0],[width,0.0],[width,height],[0.0,height],[0.0,0.0]].map(|[x,y]|self.point(x,y,page.height));
        writer.set_curve_width(super::geometry::MM)?;
        let mut out=writer.path(&format!("background-image-{}",self.node),&points,Some([0.0,0.0,0.0]),None)?;
        let mark=key.text_chunk(0).filter(|value| value.len()==key.text_bytes() && value.len()<=512).ok_or("layout.step.background-image-identity")?;
        out.extend(writer.text(&format!("background-image-{} source",self.node),mark,self.point(0.0,9.0,page.height),
            -self.transform[1].atan2(self.transform[0]),width.abs().max(1.0)*super::geometry::MM,9.0*super::geometry::MM,9.0*super::geometry::MM,None)?);
        Ok(out)
    }
    pub(super) fn advance(&mut self, graph:&dyn VectorGraphSourceV1, page:&Page, writer:&mut DraughtingWriter)->Result<(bool,Vec<Part21Instance>),String> {
        if !self.canvas {
            self.canvas=true;
            if let Some(canvas)=graph.canvas() {
                if let Some(color)=canvas.background {
                    self.transform=IDENTITY;
                    let points=[[0.0,0.0],[canvas.width,0.0],[canvas.width,canvas.height],[0.0,canvas.height],[0.0,0.0]].map(|[x,y]|self.point(x,y,page.height));
                    let fill=Some([f64::from(color.r),f64::from(color.g),f64::from(color.b)]);
                    return Ok((false,writer.path("background-canvas",&points,None,fill)?));
                }
            }
            return Ok((false,Vec::new()));
        }
        if self.route.is_empty() {
            if self.root==graph.root_count() { return Ok((true,Vec::new())); }
            self.route.push(Route{index:self.root,child:0,entered:false,content_done:false,transform:IDENTITY});
            self.root+=1;
            return Ok((false,Vec::new()));
        }
        let source=self.node(graph).ok_or("layout.step.background-route")?;
        let node=source.descriptor();
        let row=self.route.last_mut().unwrap();
        if !row.entered {
            self.enter(node.transform,node.visible,node.paint)?;
            return Ok((false,Vec::new()));
        }
        self.transform=row.transform;
        if row.content_done {
            if row.child>=source.child_count() {
                self.route.pop();
            } else {
                let index=row.child;
                row.child+=1;
                if self.route.len()>=256 { return Err("layout.step.background-transform-depth".into()); }
                self.route.push(Route{index,child:0,entered:false,content_done:false,transform:IDENTITY});
            }
            return Ok((false,Vec::new()));
        }
        let segment=match node.content {
            VectorContentRef::Path(path)=>path.segment(self.segment),
            VectorContentRef::Shape(shape)=>shape.primitive().and_then(|shape|shape_segment(shape,self.segment)),
            VectorContentRef::Text{content,x,y,size,..}=>{
                let(done,out)=self.text(content,[x,y,size],page,writer)?;
                self.route.last_mut().unwrap().content_done=done;
                return Ok((false,out));
            }
            VectorContentRef::Image{key,width,height}=>{
                let out=self.image(key,width,height,page,writer)?;
                self.route.last_mut().unwrap().content_done=true;
                return Ok((false,out));
            }
            VectorContentRef::Group{..}=>None,
            VectorContentRef::Boolean{..}|VectorContentRef::Trace{..}=>return Err("layout.step.background-unresolved-shape".into()),
        };
        if let Some(segment)=segment {
            self.segment+=1;
            return Ok((false,self.emit_segment(segment,page,writer)?));
        }
        self.route.last_mut().unwrap().content_done=true;
        Ok((false,self.finish_path(writer)?))
    }
    pub(super) fn next_release_bytes(&self)->usize { self.route.capacity()*std::mem::size_of::<Route>() }
    pub(super) fn close_one(&mut self,bytes:usize)->bool {
        let demand=self.next_release_bytes();
        if demand==0||demand>bytes { return false; }
        self.route=Vec::new();
        true
    }
    pub(super) fn terminal_is_empty(&self)->bool { self.route.capacity()==0 }
}
