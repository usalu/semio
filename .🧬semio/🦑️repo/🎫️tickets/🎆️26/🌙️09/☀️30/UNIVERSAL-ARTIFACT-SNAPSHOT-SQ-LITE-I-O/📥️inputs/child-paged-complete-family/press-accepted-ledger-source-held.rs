//! 🪪️ Registry reconciliation borrows only the actually accepted scrub ledger.
use super::{PressLeaf,press_child_registry::{PressChildRegistry,PressChildIdentity}};
use semio_framework_tool_machine::ScrubLedger;
use semio_framework_value::{NativeEncodeControl,ValueError};

/// 👓️ A fresh immutable accepted-ledger borrow excludes every abandoned preview and incoming owner.
pub struct AcceptedPressChildSources<'a,M,CM>{ledger:&'a ScrubLedger<PressLeaf<M,CM>>}
impl<'a,M:Clone+'static,CM:Clone+'static> AcceptedPressChildSources<'a,M,CM>{
    pub fn borrow(ledger:&'a ScrubLedger<PressLeaf<M,CM>>)->Self{Self{ledger}}
    fn referenced(&self,identity:PressChildIdentity,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(0)?;for leaf in self.ledger.provisional(){control.step()?;if matches!(leaf,PressLeaf::Child(current) if *current==identity){return Ok(true)}}Ok(false)})
    }
    /// ♻️ The caller takes committed owners first, then marks only identities absent from the accepted ledger.
    pub fn reconcile(&self,registry:&mut PressChildRegistry,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{registry.reconcile(|identity,control|self.referenced(identity,control),control)}
}
