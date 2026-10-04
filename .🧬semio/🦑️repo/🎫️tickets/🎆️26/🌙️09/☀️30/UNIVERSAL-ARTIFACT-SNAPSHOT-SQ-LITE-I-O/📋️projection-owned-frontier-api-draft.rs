impl<'c,'p> Projection<'c,'p>{
 /// 🗂️ Admits an owner's concrete borrowed traversal vector on the existing transfer ledger.
 pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{
  self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,count)?;super::transfer::reserve(count,self.control)
 }
 /// ➕️ Pays complete replacement backing before growing an owner's traversal frontier.
 pub fn push_frontier<T>(&mut self,frontier:&mut Vec<T>,value:T)->Result<(),ValueError>{
  super::transfer::grow(frontier,SqliteSnapshotPhase::ProjectSnapshot,self.control)?;frontier.push(value);Ok(())
 }
}
