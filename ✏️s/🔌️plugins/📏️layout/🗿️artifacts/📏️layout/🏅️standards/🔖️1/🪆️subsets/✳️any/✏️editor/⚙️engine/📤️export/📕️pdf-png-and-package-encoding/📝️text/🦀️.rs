//! 📝️ Marked text retains authored font details through allocation-free formatting views.
pub const PDF_LAYOUT_TEXT_PREFIX:&str="/SemioLayoutText << /SemioFontFamily <FEFF";
pub const PDF_LAYOUT_TEXT_SUFFIX:&str=" EMC\n";
pub struct FontFamilyScalar(char);
pub fn font_family_scalar(character:char)->FontFamilyScalar{FontFamilyScalar(character)}
impl std::fmt::Display for FontFamilyScalar{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{let mut units=[0u16;2];for unit in self.0.encode_utf16(&mut units){write!(f,"{unit:04X}")?;}Ok(())}}
pub struct TextProperties{weight:u32,italic:bool}
pub fn text_properties(weight:u32,italic:bool)->TextProperties{TextProperties{weight,italic}}
impl std::fmt::Display for TextProperties{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"> /SemioFontWeight {} /SemioItalic {} >> BDC\n",self.weight,self.italic)}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
