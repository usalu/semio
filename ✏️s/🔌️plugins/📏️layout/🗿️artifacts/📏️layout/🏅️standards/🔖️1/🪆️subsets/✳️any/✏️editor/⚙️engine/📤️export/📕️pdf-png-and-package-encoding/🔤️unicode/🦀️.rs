//! 🔤️ Complete Unicode scalar CMaps preserve editable text alongside faithful outline paint.
pub(super) const PDF_UNICODE_PREFIX: &str = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n/CMapName /LayoutUnicode def\n/CMapType 2 def\n1 begincodespacerange\n<00000000> <0010FFFF>\nendcodespacerange\n";
pub(super) const PDF_UNICODE_SUFFIX: &str = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";
pub(super) const PDF_ENCODING_CMAP: &str = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> def\n/CMapName /LayoutInvisibleUnicode def\n/CMapType 1 def\n/WMode 0 def\n1 begincodespacerange\n<00000000> <0010FFFF>\nendcodespacerange\n1 beginnotdefrange\n<00000000> <0010FFFF> 0\nendnotdefrange\nendcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";
pub(super) struct UnicodeRange{first:usize,last:usize,target:usize,width:usize}
pub(super) fn unicode_range(at:usize)->Option<UnicodeRange>{if at>=1026{return None;}let first=match at{0=>0,1=>0xe000,_=>0x10000+(at-2)*1024};let last=match at{0=>0xd7ff,1=>0xffff,_=>first+1023};Some(UnicodeRange{first,last,target:match at{0=>0,1=>0xe000,_=>(0xd800+at-2)*65536+0xdc00},width:if at<2{4}else{8}})}
impl std::fmt::Display for UnicodeRange{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"1 beginbfrange\n<{:08X}> <{:08X}> <{:0width$X}>\nendbfrange\n",self.first,self.last,self.target,width=self.width)}}
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
