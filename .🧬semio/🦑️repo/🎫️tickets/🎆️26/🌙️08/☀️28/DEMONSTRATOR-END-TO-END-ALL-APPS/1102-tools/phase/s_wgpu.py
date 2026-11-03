from ph import *
c="semio-framework-os-renderer-wgpu"
S="🚪️native-io/🎟️source/🦀️.rs"
W="self.failure=Some(RendererSourceIoFailureKind::WrongResult);return}"
E(c,S,'''let reservation=&mut self.path_reservation;let output=&mut self.published_path;let invocation=&mut self.invocation;
  let call=self.caller.dispatch_step(|cx|invocation.supply_selected_path(reservation.as_mut().unwrap(),output,cx));''','''let Some(reservation)=self.path_reservation.as_mut()else{'''+W+''';let output=&mut self.published_path;let invocation=&mut self.invocation;
  let call=self.caller.dispatch_step(|cx|invocation.supply_selected_path(reservation,output,cx));''')
E(c,S,"let maximum=match self.mode{RendererSourceIoMode::Contiguous{maximum}=>maximum,_=>unreachable!()};","let RendererSourceIoMode::Contiguous{maximum}=self.mode else{"+W+";")
E(c,S,"let offset=self.contiguous.as_ref().unwrap().retained_length()as u64;let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.restart_read_page(","let Some(contiguous)=self.contiguous.as_ref()else{"+W+";let offset=contiguous.retained_length()as u64;let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.restart_read_page(")
E(c,S,"matches!(self.refusals[index].as_ref().unwrap().message,std::borrow::Cow::Borrowed(_))","self.refusals[index].as_ref().is_some_and(|refusal|matches!(refusal.message,std::borrow::Cow::Borrowed(_)))")
E(c,S,"let release=match &refusal.as_ref().unwrap().message{std::borrow::Cow::Owned(text)=>text.capacity(),_=>0};","let release=match refusal.as_ref().map(|refusal|&refusal.message){Some(std::borrow::Cow::Owned(text))=>text.capacity(),_=>0};")
if __name__=="__main__":build()
