//! 👁️ Real Layout lists lend keys and sparse coordinates without copying domain payloads.
use crate::*;
use semio_s_artifact_layout_layout_model::{ParagraphStyle,CharacterStyle,TextStory,ImageLink,ParentPage,Spread,Page,Frame,Layer,PageOverride};
use semio_framework_value_dictionary::QuestionDictionaryEntry;
use super::{keyed_plan::KeyedSource,positional_plan::PositionalSource,chained_plan::ChainedSource};
/// 🔑 Row keys stay with their original controlled owner.
pub trait BorrowedKey{fn borrowed_key(&self)->&str;}
macro_rules! key{($($row:ty=>$field:ident),+)=>{$(impl BorrowedKey for $row{fn borrowed_key(&self)->&str{self.$field.as_str()}})+};}
key!(ParagraphStyle=>id,CharacterStyle=>id,TextStory=>id,ImageLink=>id,ParentPage=>id,Spread=>id,Page=>id,Layer=>id,PageOverride=>object_id,QuestionDictionaryEntry=>question_id);
impl BorrowedKey for Frame{fn borrowed_key(&self)->&str{self.id()}}
/// 🩹 Every real sparse list exposes borrowed coordinates with no Parts conversion.
pub trait BorrowedDelta<R>{fn removed_len(&self)->usize;fn removed(&self,index:usize)->(&str,usize);fn inserted_len(&self)->usize;fn inserted(&self,index:usize)->(&str,usize);fn moved_len(&self)->usize;fn moved(&self,index:usize)->(&str,usize,usize);fn modified_len(&self)->usize;fn modified_key(&self,index:usize)->&str;}
macro_rules! parts{($row:ty,$delta:ty,|$len_owner:ident|$modified:expr,|$key_owner:ident,$key_index:ident|$modkey:expr)=>{impl BorrowedDelta<$row> for $delta{
 fn removed_len(&self)->usize{self.removed.len()}fn removed(&self,i:usize)->(&str,usize){let row=&self.removed[i];(&row.id,row.index)}
 fn inserted_len(&self)->usize{self.inserted.len()}fn inserted(&self,i:usize)->(&str,usize){let row=&self.inserted[i];(row.row.borrowed_key(),row.index)}
 fn moved_len(&self)->usize{self.moved.len()}fn moved(&self,i:usize)->(&str,usize,usize){let row=&self.moved[i];(&row.id,row.from,row.to)}
 fn modified_len(&self)->usize{let $len_owner=self;$modified}fn modified_key(&self,i:usize)->&str{let($key_owner,$key_index)=(self,i);$modkey}
 }};}
macro_rules! lists{($($row:ty=>$delta:ty),+)=>{$(parts!($row,$delta,|delta|delta.modified.len(),|delta,index|delta.modified[index].id.as_str());)+};}
lists!(ParagraphStyle=>LayoutParagraphStylesDelta,CharacterStyle=>LayoutCharacterStylesDelta,TextStory=>LayoutStoriesDelta,ImageLink=>LayoutLinksDelta,ParentPage=>LayoutParentPagesDelta,Spread=>LayoutSpreadsDelta,Page=>LayoutPagesDelta,Frame=>PageFramesDelta,Layer=>PageLayersDelta);
macro_rules! plain{($($row:ty=>$delta:ty),+)=>{$(parts!($row,$delta,|_delta|0,|_delta,_index|unreachable!("plain lists have no modification source"));)+};}
plain!(PageOverride=>PageOverridesDelta,QuestionDictionaryEntry=>LayoutDataEntriesDelta);
/// 🪢 A source view cannot transfer, clone or mutate a row payload.
pub struct BorrowedRows<'a,R,D>{pub held:&'a [R],pub delta:&'a D}
impl<R:BorrowedKey,D:BorrowedDelta<R>> KeyedSource for BorrowedRows<'_,R,D>{
 fn held_len(&self)->usize{self.held.len()}fn held_key(&self,i:usize)->&str{self.held[i].borrowed_key()}
 fn removed_len(&self)->usize{self.delta.removed_len()}fn removed(&self,i:usize)->(&str,usize){self.delta.removed(i)}
 fn inserted_len(&self)->usize{self.delta.inserted_len()}fn inserted(&self,i:usize)->(&str,usize){self.delta.inserted(i)}
 fn moved_len(&self)->usize{self.delta.moved_len()}fn moved(&self,i:usize)->(&str,usize,usize){self.delta.moved(i)}
 fn modified_len(&self)->usize{self.delta.modified_len()}fn modified_key(&self,i:usize)->&str{self.delta.modified_key(i)}
}
/// 🔢 Paid numeric indices select actual nested patches from their original owner in constant time.
pub struct BorrowedChain<'a,R,D,O>{pub held:&'a [R],pub indices:&'a [usize],pub owner:&'a O,pub delta:for<'b>fn(&'b O,usize)->&'b D}
impl<R:BorrowedKey,D:BorrowedDelta<R>,O> ChainedSource for BorrowedChain<'_,R,D,O>{
 fn held_len(&self)->usize{self.held.len()}fn held_key(&self,i:usize)->&str{self.held[i].borrowed_key()}fn steps_len(&self)->usize{self.indices.len()}
 fn removed_len(&self,s:usize)->usize{(self.delta)(self.owner,self.indices[s]).removed_len()}fn removed(&self,s:usize,i:usize)->(&str,usize){(self.delta)(self.owner,self.indices[s]).removed(i)}
 fn inserted_len(&self,s:usize)->usize{(self.delta)(self.owner,self.indices[s]).inserted_len()}fn inserted(&self,s:usize,i:usize)->(&str,usize){(self.delta)(self.owner,self.indices[s]).inserted(i)}
 fn moved_len(&self,s:usize)->usize{(self.delta)(self.owner,self.indices[s]).moved_len()}fn moved(&self,s:usize,i:usize)->(&str,usize,usize){(self.delta)(self.owner,self.indices[s]).moved(i)}
 fn modified_len(&self,s:usize)->usize{(self.delta)(self.owner,self.indices[s]).modified_len()}fn modified_key(&self,s:usize,i:usize)->&str{(self.delta)(self.owner,self.indices[s]).modified_key(i)}
}
/// 📍 Positional lists lend only their final extent and authored row coordinates.
pub struct BorrowedPositions<'a,D>{pub held_len:usize,pub delta:&'a D}
macro_rules! positions{($($delta:ty),+)=>{$(impl PositionalSource for BorrowedPositions<'_,$delta>{fn held_len(&self)->usize{self.held_len}fn target_len(&self)->Option<usize>{self.delta.len}fn rows_len(&self)->usize{self.delta.rows.len()}fn row_index(&self,i:usize)->usize{self.delta.rows[i].index}})+};}
positions!(TextStyleRunsDelta,PageGuidesDelta);

use super::list_move::OwnedInsertion;
macro_rules! inserted{($($insertion:ty=>$row:ty),+)=>{$(impl OwnedInsertion<$row> for $insertion{fn into_row(self)->$row{self.row}})+};}
inserted!(LayoutParagraphStyleInsertion=>ParagraphStyle,LayoutCharacterStyleInsertion=>CharacterStyle,LayoutStoryInsertion=>TextStory,LayoutLinkInsertion=>ImageLink,LayoutParentPageInsertion=>ParentPage,LayoutSpreadInsertion=>Spread,LayoutPageInsertion=>Page,PageFrameInsertion=>Frame,PageLayerInsertion=>Layer,PageOverrideInsertion=>PageOverride,LayoutDataEntryInsertion=>QuestionDictionaryEntry);
impl OwnedInsertion<semio_s_artifact_layout_layout_model::TextStyleRun> for TextStyleRunRow{fn into_row(self)->semio_s_artifact_layout_layout_model::TextStyleRun{self.run}}
impl OwnedInsertion<semio_s_artifact_layout_layout_model::LayoutRect> for PageGuideRow{fn into_row(self)->semio_s_artifact_layout_layout_model::LayoutRect{self.guide}}

use super::collection::OwnedDelta;
macro_rules! owned{($($row:ty=>$delta:ty=>$insertion:ty),+)=>{$(impl OwnedDelta<$row> for $delta{type Insertion=$insertion;fn insertion_rows(&mut self)->&mut Vec<$insertion>{&mut self.inserted}})+};}
owned!(ParagraphStyle=>LayoutParagraphStylesDelta=>LayoutParagraphStyleInsertion,CharacterStyle=>LayoutCharacterStylesDelta=>LayoutCharacterStyleInsertion,TextStory=>LayoutStoriesDelta=>LayoutStoryInsertion,ImageLink=>LayoutLinksDelta=>LayoutLinkInsertion,ParentPage=>LayoutParentPagesDelta=>LayoutParentPageInsertion,Spread=>LayoutSpreadsDelta=>LayoutSpreadInsertion,Page=>LayoutPagesDelta=>LayoutPageInsertion,Frame=>PageFramesDelta=>PageFrameInsertion,Layer=>PageLayersDelta=>PageLayerInsertion,PageOverride=>PageOverridesDelta=>PageOverrideInsertion,QuestionDictionaryEntry=>LayoutDataEntriesDelta=>LayoutDataEntryInsertion);

use super::positional_chain::PositionalChainSource;
/// 📍 Typed original positional deltas expose extents and supplied coordinates only.
pub trait BorrowedPositionalDelta{fn target_len(&self)->Option<usize>;fn rows_len(&self)->usize;fn row_index(&self,index:usize)->usize;}
macro_rules! positional_delta{($($delta:ty),+)=>{$(impl BorrowedPositionalDelta for $delta{fn target_len(&self)->Option<usize>{self.len}fn rows_len(&self)->usize{self.rows.len()}fn row_index(&self,i:usize)->usize{self.rows[i].index}})+};}
positional_delta!(TextStyleRunsDelta,PageGuidesDelta);
/// 🔢 Paid indices select actual earlier story/page positional patches by constant-time native borrow.
pub struct BorrowedPositionalChain<'a,D,O>{pub held_len:usize,pub indices:&'a [usize],pub owner:&'a O,pub delta:for<'b>fn(&'b O,usize)->&'b D}
impl<D:BorrowedPositionalDelta,O> PositionalChainSource for BorrowedPositionalChain<'_,D,O>{fn held_len(&self)->usize{self.held_len}fn steps_len(&self)->usize{self.indices.len()}fn target_len(&self,s:usize)->Option<usize>{(self.delta)(self.owner,self.indices[s]).target_len()}fn rows_len(&self,s:usize)->usize{(self.delta)(self.owner,self.indices[s]).rows_len()}fn row_index(&self,s:usize,i:usize)->usize{(self.delta)(self.owner,self.indices[s]).row_index(i)}}
