from ph import *
c="semio-framework-text-splice-native"
N="🧬️native/🦀️.rs";A="🧬️native/🧵️advance.rs"
E(c,N,"fn get(&self,i:usize)->&Vec<char>{match i{0=>&self.a,1=>&self.b,2=>&self.c,3=>&self.d,4=>&self.e,5=>&self.f,6=>&self.g,7=>&self.h,_=>unreachable!()}}","fn get(&self,i:usize)->Result<&Vec<char>,ValueError>{match i{0=>Ok(&self.a),1=>Ok(&self.b),2=>Ok(&self.c),3=>Ok(&self.d),4=>Ok(&self.e),5=>Ok(&self.f),6=>Ok(&self.g),7=>Ok(&self.h),_=>Err(invalid())}}")
E(c,N,"fn get_mut(&mut self,i:usize)->&mut Vec<char>{match i{0=>&mut self.a,1=>&mut self.b,2=>&mut self.c,3=>&mut self.d,4=>&mut self.e,5=>&mut self.f,6=>&mut self.g,7=>&mut self.h,_=>unreachable!()}}","fn get_mut(&mut self,i:usize)->Result<&mut Vec<char>,ValueError>{match i{0=>Ok(&mut self.a),1=>Ok(&mut self.b),2=>Ok(&mut self.c),3=>Ok(&mut self.d),4=>Ok(&mut self.e),5=>Ok(&mut self.f),6=>Ok(&mut self.g),7=>Ok(&mut self.h),_=>Err(invalid())}}")
E(c,A,"owners.scalars.get(c.field)","owners.scalars.get(c.field)?",3)
E(c,A,"owners.scalars.get_mut(c.field)","owners.scalars.get_mut(c.field)?",2)
E(c,A,"fn output_mut(result:&mut OriginalTextSpliceComposition,field:usize)->&mut String{let OriginalTextSpliceComposition::Composed(s)=result else{unreachable!()};match field{0=>&mut s.deleted,1=>&mut s.insert,2=>&mut s.before,3=>&mut s.after,_=>unreachable!()}}","fn output_mut(result:&mut OriginalTextSpliceComposition,field:usize)->Result<&mut String,ValueError>{let OriginalTextSpliceComposition::Composed(s)=result else{return Err(invalid())};match field{0=>Ok(&mut s.deleted),1=>Ok(&mut s.insert),2=>Ok(&mut s.before),3=>Ok(&mut s.after),_=>Err(invalid())}}")
E(c,A,"output_mut(owners.result.as_mut().ok_or_else(invalid)?,c.field)","output_mut(owners.result.as_mut().ok_or_else(invalid)?,c.field)?",2)
if __name__=="__main__":build()
