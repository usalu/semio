/// 🪑️ Transfers one complete owned locale cell into its original empty matrix slot or returns the same owner.
pub fn set_owned_cell(&mut self,terminology:Terminology,locale:Locale,text:String)->Result<(),String>{
    let cell=&mut self.cells[terminology.index()][locale.index()];
    if !matches!(cell,Cow::Borrowed("")){return Err(text)}
    *cell=Cow::Owned(text);Ok(())
}
