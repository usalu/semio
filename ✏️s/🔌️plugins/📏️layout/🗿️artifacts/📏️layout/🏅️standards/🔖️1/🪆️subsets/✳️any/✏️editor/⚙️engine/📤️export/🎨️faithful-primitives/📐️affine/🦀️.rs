//! 📐️ Exact local-to-world affine custody and bounded local-space coverage.
use semio_framework_2d_engine::Vec2;
use semio_framework_value_persistent::list::PagedList;
pub type Matrix=[f64;6];
pub const IDENTITY:Matrix=[1.0,0.0,0.0,1.0,0.0,0.0];
#[derive(Clone,Copy)]
pub struct AffineEdge{pub a:Vec2,pub b:Vec2,pub stroke:bool}
semio_framework_value::artifact_retire_leaf!(AffineEdge);
pub trait AffineEdges{fn edge(&self,at:usize)->Option<AffineEdge>;}
impl AffineEdges for PagedList<AffineEdge,{usize::MAX}>{fn edge(&self,at:usize)->Option<AffineEdge>{self.get(at).copied()}}
pub fn validate_paint(width:f64)->Result<(),&'static str>{if !width.is_finite()||width<0.0{Err("layout-export-stroke-width")}else{Ok(())}}
pub fn point(m:Matrix,p:Vec2)->Vec2{[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]]}
pub fn vector(m:Matrix,p:Vec2)->Vec2{[m[0]*p[0]+m[2]*p[1],m[1]*p[0]+m[3]*p[1]]}
pub fn compose(outer:Matrix,inner:Matrix)->Matrix{[outer[0]*inner[0]+outer[2]*inner[1],outer[1]*inner[0]+outer[3]*inner[1],outer[0]*inner[2]+outer[2]*inner[3],outer[1]*inner[2]+outer[3]*inner[3],outer[0]*inner[4]+outer[2]*inner[5]+outer[4],outer[1]*inner[4]+outer[3]*inner[5]+outer[5]]}
pub fn rotation(angle:f64,center:Vec2)->Matrix{let(s,c)=angle.sin_cos();[c,s,-s,c,center[0]*(1.0-c)+center[1]*s,center[1]*(1.0-c)-center[0]*s]}
pub fn inverse_linear(m:Matrix)->Option<[f64;4]>{let scale=m[..4].iter().fold(0.0_f64,|s,v|s.max(v.abs()));if scale==0.0||!m.into_iter().all(f64::is_finite){return None}let(a,b,c,d)=(m[0]/scale,m[1]/scale,m[2]/scale,m[3]/scale);let det=a*d-b*c;if det==0.0{return None}let result=[d/det/scale,-b/det/scale,-c/det/scale,a/det/scale];result.into_iter().all(f64::is_finite).then_some(result)}
pub fn unmap(m:Matrix,inverse:[f64;4],p:Vec2)->Vec2{let x=p[0]-m[4];let y=p[1]-m[5];[inverse[0]*x+inverse[2]*y,inverse[1]*x+inverse[3]*y]}
pub fn bounds(m:Matrix,b:[f64;4],pad:f64)->Option<[f64;4]>{if !b.into_iter().all(f64::is_finite)||!m.into_iter().all(f64::is_finite){return None}let mut result=[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY];for p in [[b[0],b[1]],[b[2],b[1]],[b[2],b[3]],[b[0],b[3]]]{let p=point(m,p);result[0]=result[0].min(p[0]);result[1]=result[1].min(p[1]);result[2]=result[2].max(p[0]);result[3]=result[3].max(p[1]);}let x=if pad==0.0{0.0}else{pad*m[0].hypot(m[2])};let y=if pad==0.0{0.0}else{pad*m[1].hypot(m[3])};if x.is_infinite(){result[0]=f64::NEG_INFINITY;result[2]=f64::INFINITY;}else{result[0]-=x;result[2]+=x;}if y.is_infinite(){result[1]=f64::NEG_INFINITY;result[3]=f64::INFINITY;}else{result[1]-=y;result[3]+=y;}Some(if result.into_iter().any(f64::is_nan){[f64::NEG_INFINITY,f64::NEG_INFINITY,f64::INFINITY,f64::INFINITY]}else{result})}
fn distance(p:Vec2,a:Vec2,b:Vec2)->f64{let d=[b[0]-a[0],b[1]-a[1]];let length=d[0]*d[0]+d[1]*d[1];let t=if length==0.0{0.0}else{((p[0]-a[0])*d[0]+(p[1]-a[1])*d[1])/length}.clamp(0.0,1.0);(p[0]-a[0]-d[0]*t).powi(2)+(p[1]-a[1]-d[1]*t).powi(2)}
pub fn stroke_hit(p:Vec2,a:Vec2,b:Vec2,radius:f64)->bool{let scale=p.into_iter().chain(a).chain(b).chain([radius]).fold(0.0_f64,|s,v|s.max(v.abs()));if scale==0.0{return true}distance(p.map(|v|v/scale),a.map(|v|v/scale),b.map(|v|v/scale))<=(radius/scale).powi(2)}
pub fn flat_enough(m:Matrix,points:[Vec2;4],degree:u8)->bool{let origin=points[0];let endpoint=vector(m,[points[degree as usize][0]-origin[0],points[degree as usize][1]-origin[1]]);(1..degree as usize).all(|at|distance(vector(m,[points[at][0]-origin[0],points[at][1]-origin[1]]),[0.0;2],endpoint)<=0.000625)}
pub struct SvgMatrix(Matrix);
pub fn svg(m:Matrix)->SvgMatrix{SvgMatrix(m)}
impl std::fmt::Display for SvgMatrix{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{let m=self.0;write!(f,"matrix({} {} {} {} {} {})",m[0],m[1],m[2],m[3],m[4],m[5])}}
#[derive(Clone,Copy,Default)]
pub struct AffineCoverageCursor{sample:u8,edge:usize,winding:i32,stroke:bool,fill_samples:u8,stroke_samples:u8}
pub struct AffineCoverageTurn{pub coverage:Option<[f32;2]>,pub visited:usize}
impl AffineCoverageCursor{
 pub fn reset(&mut self){*self=Self::default();}
 /// 👣️ Evaluates at most the incoming number of local-edge/sample events without retaining source geometry.
 pub fn advance(&mut self,edges:&impl AffineEdges,m:Matrix,inverse:Option<[f64;4]>,stroke_width:Option<f64>,x:u32,y:u32,maximum_items:usize)->AffineCoverageTurn{
  if maximum_items==0{return AffineCoverageTurn{coverage:None,visited:0}}let Some(inverse)=inverse else{self.reset();return AffineCoverageTurn{coverage:Some([0.0;2]),visited:1}};
  for visited in 1..=maximum_items{let p=unmap(m,inverse,[f64::from(x)+(f64::from(self.sample%4)+0.5)*0.25,f64::from(y)+(f64::from(self.sample/4)+0.5)*0.25]);if let Some(edge)=edges.edge(self.edge){if edge.a[1]<=p[1]&&edge.b[1]>p[1]||edge.a[1]>p[1]&&edge.b[1]<=p[1]{let t=(p[1]-edge.a[1])/(edge.b[1]-edge.a[1]);let intersection=edge.a[0]*(1.0-t)+edge.b[0]*t;if intersection>p[0]{self.winding+=if edge.b[1]>edge.a[1]{1}else{-1};}}if edge.stroke&&stroke_width.filter(|width|*width>0.0).is_some_and(|width|stroke_hit(p,edge.a,edge.b,width*0.5)){self.stroke=true}self.edge+=1;}else{self.fill_samples+=u8::from(self.winding!=0);self.stroke_samples+=u8::from(self.stroke);self.sample+=1;self.edge=0;self.winding=0;self.stroke=false;if self.sample==16{let coverage=[f32::from(self.fill_samples)/16.0,f32::from(self.stroke_samples)/16.0];self.reset();return AffineCoverageTurn{coverage:Some(coverage),visited};}}}AffineCoverageTurn{coverage:None,visited:maximum_items}
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
