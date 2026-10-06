// 🪪️ Scrub leaves clone scalar identities while the registry retains each actual child source once.
use super::{ChildEmit,PluginCloseStep};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,list::PagedList};

/// 🪪️ A monotone runtime identity names one retained child, without cloning any source allocation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PressChildIdentity(u64);
struct PressChildCell{identity:PressChildIdentity,child:Option<ChildEmit>,retired:bool}

/// 🗂️ Every original child stays in admitted registry pages through commit, refusal, displacement and host abort.
pub struct PressChildRegistry{cells:PagedList<PressChildCell,{isize::MAX as usize}>,next:u64,live_owners:usize,retirement_index:usize,closing:bool}
impl PressChildRegistry{
    pub fn new()->Self{Self{cells:PagedList::empty(),next:1,live_owners:0,retirement_index:0,closing:false}}
    fn fault(detail:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,detail)}
    /// ➕️ Takes the caller's original owner only after the concrete registry slot is admitted.
    pub fn insert(&mut self,child:&mut Option<ChildEmit>,control:&mut NativeEncodeControl<'_>)->Result<PressChildIdentity,ValueError>{
        if self.closing||child.is_none(){return Err(Self::fault("press child registry cannot accept this retained owner"))}
        let next=self.next.checked_add(1).ok_or_else(||Self::fault("press child identities exhausted their actual scalar authority"))?;
        let owners=self.live_owners.checked_add(1).ok_or_else(||Self::fault("press child owner count exceeds addressable registry"))?;
        while !self.cells.has_reserved_slot(){let bytes=self.cells.next_allocation_bytes().map_err(ValueError::from)?;if bytes>4096{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"press child registry page exceeds its production grant"))}control.checkpoint()?;control.charge(bytes)?;let step=self.cells.reserve_one(4096).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed{return Err(Self::fault("press child registry admitted no backing"))}}
        let identity=PressChildIdentity(self.next);let cell=PressChildCell{identity,child:child.take(),retired:false};
        if let Err(mut cell)=self.cells.push_reserved(cell){*child=cell.child.take();return Err(Self::fault("press child registry lost its admitted slot"))}
        self.next=next;self.live_owners=owners;Ok(identity)
    }
    /// 🔎️ Marks a displaced owner only after the actual ledger's complete identity scan succeeds.
    pub fn reconcile(&mut self,mut referenced:impl FnMut(PressChildIdentity,&mut NativeEncodeControl<'_>)->Result<bool,ValueError>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        if self.closing{return Err(Self::fault("closing press registry cannot reconcile new ledger authority"))}
        for cell in &mut self.cells{control.checkpoint()?;if cell.child.is_some()&&!cell.retired{let live=referenced(cell.identity,control)?;cell.retired=!live;}control.step()?;}
        Ok(())
    }
    /// 📤️ Hands the unique live source back to publication, retaining its empty registry slot for physical close.
    pub fn take(&mut self,identity:PressChildIdentity,control:&mut NativeEncodeControl<'_>)->Result<ChildEmit,ValueError>{
        if self.closing{return Err(Self::fault("closing press registry cannot publish a child"))}
        for cell in &mut self.cells{control.checkpoint()?;if cell.identity==identity{if cell.retired{return Err(Self::fault("displaced press child cannot publish"))}let child=cell.child.take().ok_or_else(||Self::fault("press child identity has already transferred its original source"))?;self.live_owners-=1;return Ok(child)}control.step()?;}
        Err(Self::fault("press ledger names no actual retained child owner"))
    }
    /// 🚪️ Runtime close first clears scalar ledger authority, then retires every retained child explicitly.
    pub fn begin_close(&mut self){self.closing=true;self.retirement_index=0;}
    /// ♻️ Visits one registry cell or child ownership step under the unchanged physical grant.
    pub fn close_one(&mut self,maximum_items:usize,maximum_bytes:usize)->PluginCloseStep{
        let pending=|items,bytes|PluginCloseStep::Pending{released_items:items,released_bytes:bytes};
        if maximum_items==0||maximum_bytes==0{return pending(0,0)}
        if self.retirement_index<self.cells.len(){
            let cell=self.cells.get_mut(self.retirement_index).expect("retained registry ordinal");
            if (!cell.retired&&!self.closing)||cell.child.is_none(){self.retirement_index+=1;return pending(1,0)}
            let step=cell.child.as_mut().unwrap().close_one(1,maximum_bytes);
            return match step{PluginCloseStep::Complete=>{cell.child.take();self.live_owners-=1;self.retirement_index+=1;pending(1,0)},other=>other};
        }
        if self.live_owners!=0{self.retirement_index=0;return pending(0,0)}
        if self.cells.pop().is_some(){self.retirement_index=self.cells.len();return pending(1,0)}
        if !self.cells.terminal_is_empty(){return match self.cells.release_empty_page(maximum_bytes){Ok(step)=>pending(usize::from(step.progressed),step.released_allocation_bytes),Err(_)=>pending(0,0)}}
        PluginCloseStep::Complete
    }
    pub fn terminal_is_empty(&self)->bool{self.cells.terminal_is_empty()}
    pub fn next_close_byte_demand(&self)->usize{if let Some(cell)=self.cells.get(self.retirement_index){return cell.child.as_ref().filter(|_|cell.retired||self.closing).map(ChildEmit::next_close_byte_demand).unwrap_or(1).max(1)}if self.cells.len()!=0{return 1}if !self.cells.terminal_is_empty(){return self.cells.next_release_allocation_bytes().unwrap_or(1)}0}
}
impl Drop for PressChildRegistry{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"press child registry requires explicit retirement of original sources and real pages");}}
