/// 👓️ Formatting borrows the same paged octets without materializing a second source owner.
impl std::fmt::Debug for OwnedOperationBytes{
    fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_list().entries(self.iter()).finish()}
}
impl PartialEq for OwnedOperationBytes{
    fn eq(&self,other:&Self)->bool{self.len()==other.len()&&self.iter().eq(other.iter())}
}
impl Eq for OwnedOperationBytes{}
