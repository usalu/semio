/** ♻️ Stages paid octet-page retirement without changing generic list element accounting. */
import assert from "node:assert/strict";
import { join } from "node:path";

const value="🧰️framework/🔨️modules/🌱️value";
const edit=(text:string,from:string,to:string)=>{
  assert.equal(text.split(from).length,2,from.slice(0,100));
  return text.replace(from,to);
};
const paths=[join(value,"📋️list/📤️discard/🦀️.rs"),join(value,"♻️retirement/🦀️.rs"),join(value,"🧬️retained-clone/📦️paged/🦀️.rs")];
const original=await Promise.all(paths.map(path=>Bun.file(path).text()));
let [discard, retire, bytes]=original;
const selector=` pub fn discard_plain_tail(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{
  let grant=effect.grant();let demand=match self.plain_tail_discard_demand(grant.maximum_items,grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};`;
const shared=` pub fn discard_plain_tail(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{
  self.discard_tail(effect, false)
 }
 /// ♻️ Retires one flat no-drop payload group within its originating byte and page limits.
 pub fn discard_plain_tail_page(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{
  self.discard_tail(effect, true)
 }
 fn discard_tail(&mut self,effect:&mut RetainedAllocationEffect<'_>, page:bool)->RetainedWorkOutcome<RetainedCloneProgress>{
  let grant=effect.grant();
  let maximum_items=if page && grant.maximum_items != 0 { usize::MAX } else { grant.maximum_items };
  let demand=match self.plain_tail_discard_demand(maximum_items,grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};`;
discard=edit(discard!,selector,shared);
discard=edit(discard,"copied_items:demand.items,copied_bytes:demand.physical.copy_bytes", "copied_items:if page {1} else {demand.items},copied_bytes:demand.physical.copy_bytes");
retire=edit(retire!,`pub struct PagedListRetirement<T: RetireOwned, const N: usize> {
    owner: ManuallyDrop<crate::list::PagedList<T, N>>,
    released: bool,`,`pub struct PagedListRetirement<T: RetireOwned, const N: usize> {
    owner: ManuallyDrop<crate::list::PagedList<T, N>>,
    released: bool,
    octet_pages: bool,`);
const attach = "impl<T: RetireOwned, const N: usize> RetirementCursor for PagedListRetirement<T, N> {";
retire=edit(retire,attach,`impl<const N: usize> PagedListRetirement<u8, N> {
    /// 🛡️ Preserves page-sized retirement for native octets without reclassifying any typed list.
    #[doc(hidden)]
    pub fn from_octets(owner: crate::list::PagedList<u8,N>) -> Self {
        Self { owner: ManuallyDrop::new(owner), released: false, octet_pages: true, pending: ManuallyDrop::new(None) }
    }
}

`+attach);
retire=edit(retire,"let turn=self.owner.discard_plain_tail(effect);return match turn.result{",[
  "let turn=if self.octet_pages { self.owner.discard_plain_tail_page(effect) } else { self.owner.discard_plain_tail(effect) };",
  "          return match turn.result{",
].join("\n"));
retire=edit(retire,"PagedListRetirement { owner: ManuallyDrop::new(self), released: false, pending:ManuallyDrop::new(None) }",
  "PagedListRetirement { owner: ManuallyDrop::new(self), released: false, octet_pages: false, pending:ManuallyDrop::new(None) }");
bytes=edit(bytes!,"        self.into_retained_bytes().retirement_seed()", 
  "        crate::retirement::PagedListRetirement::from_octets(self.into_retained_bytes())");
const rows=paths.map((path,i)=>[path,new Bun.CryptoHasher("sha256").update(original[i]!).digest("hex"),[discard,retire,bytes][i]]);
await Bun.write(join(import.meta.dir,"🗑️generated","paged-octet-retirement-overlay.json"),JSON.stringify(rows));
console.log("[DEBUG] Staged paid octet retirement by physical page");
