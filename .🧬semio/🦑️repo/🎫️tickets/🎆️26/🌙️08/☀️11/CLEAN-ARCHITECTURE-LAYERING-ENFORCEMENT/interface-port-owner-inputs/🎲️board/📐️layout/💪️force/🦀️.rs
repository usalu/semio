//! 💪️ Caller-owned force buffers advance a bounded number of mathematical work units.
use super::{Vec2,ForceLayoutOptions,pairwise_repulsion_on_i_from_j};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};

/// 🧭️ No backing allocation or callback is retained between caller scheduling hops.
#[derive(Clone,Copy,Debug,Default)]
pub struct ForceLayoutState {iteration:u32,phase:u8,index:usize,other:usize}
impl ForceLayoutState {
 /// 📍️ Advances the original force equations with persistent velocity and cooling state.
 pub fn advance(&mut self,maximum_units:usize,positions:&mut[Vec2],radii:&[f64],edges:&[(usize,usize)],pins:&[Option<Vec2>],velocities:&mut[Vec2],forces:&mut[Vec2],options:&ForceLayoutOptions,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
  let n=positions.len();
  if radii.len()!=n||pins.len()!=n||velocities.len()!=n||forces.len()!=n{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"force buffer lengths differ"));}
  if maximum_units==0{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"force advance requires positive work credits"));}
  if n==0{return Ok(true);}
  let iterations=options.iterations.max(1);
  for _ in 0..maximum_units {
   if self.iteration>=iterations{return Ok(true);}
   control.step()?;
   let cool=(1.0-self.iteration as f64/iterations as f64).max(0.08);
   match self.phase {
    0=>{forces[self.index]=Vec2::ZERO;self.index+=1;if self.index==n{self.phase=1;self.index=0;self.other=1;}}
    1=>{if self.index+1>=n{self.phase=2;self.index=0;}else{let f=pairwise_repulsion_on_i_from_j(self.index,self.other,positions,radii,cool,options.repulsion_strength);forces[self.index]+=f;forces[self.other]-=f;self.other+=1;if self.other==n{self.index+=1;self.other=self.index+1;}}}
    2=>{if self.index==edges.len(){self.phase=3;self.index=0;}else{let(i,j)=edges[self.index];if i>=n||j>=n{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"force edge index exceeds positions"));}let delta=positions[j]-positions[i];let distance=delta.hypot().max(1e-4);let force=(delta/distance)*(options.spring_strength*cool*(distance-options.ideal_edge_length.max(1e-6)));forces[i]+=force;forces[j]-=force;self.index+=1;}}
    3=>{if options.gravity>0.0{forces[self.index]+=Vec2::new(options.center_x-positions[self.index].x,options.center_y-positions[self.index].y)*(options.gravity*cool);}self.index+=1;if self.index==n{self.phase=4;self.index=0;}}
    4=>{if pins[self.index].is_some(){forces[self.index]=Vec2::ZERO;}self.index+=1;if self.index==n{self.phase=5;self.index=0;}}
    5=>{let dt=options.time_step*cool.sqrt();let mut velocity=(velocities[self.index]+forces[self.index]*dt)*options.velocity_damping;let speed=velocity.hypot();if speed>options.max_speed{velocity*=options.max_speed/speed;}velocities[self.index]=velocity;if let Some(pin)=pins[self.index]{positions[self.index]=pin;velocities[self.index]=Vec2::ZERO;}else{positions[self.index]+=velocity*dt;}self.index+=1;if self.index==n{self.iteration+=1;self.phase=0;self.index=0;}}
    _=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"invalid force phase")),
   }
  }
  control.checkpoint()?;Ok(self.iteration>=iterations)
 }
}

/// 🎲️ One deterministic seed consumes no collection backing or hidden traversal.
pub fn seed_position(index:usize,position:&mut Vec2,pin:Option<Vec2>,anchor:Vec2,seed:&mut u64){
 if pin.is_some()||position.hypot()>=1e-9{return;}
 let t=index as f64;let angle=t*2.399_963_229_728_653_5;let radius=10.0+t.sqrt()*22.0;
 let jx=(super::rand_unit_interval(seed)-0.5)*6.0;let jy=(super::rand_unit_interval(seed)-0.5)*6.0;
 *position=anchor+Vec2::new(radius*angle.cos()+jx,radius*angle.sin()+jy);
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
