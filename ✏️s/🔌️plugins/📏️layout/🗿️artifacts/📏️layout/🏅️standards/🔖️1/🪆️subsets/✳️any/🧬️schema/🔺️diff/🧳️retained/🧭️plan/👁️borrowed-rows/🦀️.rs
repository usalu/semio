//! 👁️ Original keys and borrowed sparse coordinates for retained collections.
use semio_s_artifact_layout_layout_model::{ParagraphStyle,CharacterStyle,TextStory,ImageLink,ParentPage,Spread,Page,Frame,Layer,PageOverride};
use semio_framework_value_dictionary::QuestionDictionaryEntry;
use super::keyed_plan::KeyedSource;
/// 🔑 Row keys stay with their original controlled owner.
pub trait BorrowedKey{fn borrowed_key(&self)->&str;}
macro_rules! key{($($row:ty=>$field:ident),+)=>{$(impl BorrowedKey for $row{fn borrowed_key(&self)->&str{self.$field.as_str()}})+};}
key!(ParagraphStyle=>id,CharacterStyle=>id,TextStory=>id,ImageLink=>id,ParentPage=>id,Spread=>id,Page=>id,Layer=>id,PageOverride=>object_id,QuestionDictionaryEntry=>question_id);
impl BorrowedKey for Frame{fn borrowed_key(&self)->&str{self.id()}}
/// 🩹 Every real sparse list exposes borrowed coordinates with no Parts conversion.
pub trait BorrowedDelta<R>{fn removed_len(&self)->usize;fn removed(&self,index:usize)->(&str,usize);fn inserted_len(&self)->usize;fn inserted(&self,index:usize)->(&str,usize);fn moved_len(&self)->usize;fn moved(&self,index:usize)->(&str,usize,usize);fn modified_len(&self)->usize;fn modified_key(&self,index:usize)->&str;}

/// 🪢 A source view cannot transfer, clone or mutate a row payload.
pub struct BorrowedRows<'a,R,D>{pub held:&'a [R],pub delta:&'a D}
impl<R:BorrowedKey,D:BorrowedDelta<R>> KeyedSource for BorrowedRows<'_,R,D>{
 fn held_len(&self)->usize{self.held.len()}fn held_key(&self,i:usize)->&str{self.held[i].borrowed_key()}
 fn removed_len(&self)->usize{self.delta.removed_len()}fn removed(&self,i:usize)->(&str,usize){self.delta.removed(i)}
 fn inserted_len(&self)->usize{self.delta.inserted_len()}fn inserted(&self,i:usize)->(&str,usize){self.delta.inserted(i)}
 fn moved_len(&self)->usize{self.delta.moved_len()}fn moved(&self,i:usize)->(&str,usize,usize){self.delta.moved(i)}
 fn modified_len(&self)->usize{self.delta.modified_len()}fn modified_key(&self,i:usize)->&str{self.delta.modified_key(i)}
}
