//! 🧪️ Unmounted actual scheduler cancellation origins, without timing or source-text checks.
use super::*;
use semio_framework_value::ValueRefusalKind;
use std::sync::atomic::AtomicUsize;

struct AuthoritySource{reads:Arc<AtomicUsize>}
impl AsyncPackSource for AuthoritySource{
 fn len(&self)->u64{1}
 async fn read_at(&self,_:u64,_:usize)->Result<Vec<u8>,PackError>{self.reads.fetch_add(1,Ordering::SeqCst);std::future::pending().await}
}

#[test]
fn actual_scheduler_pre_cancel_and_pending_read_keep_canceled_authority(){
 let reads=Arc::new(AtomicUsize::new(0));
 let scheduler=ReadScheduler::new(AuthoritySource{reads:reads.clone()});
 let request=ReadRequest{range:ByteRange{offset:0,len:1},priority:LoadPriority::Critical};
 let canceled=CancellationToken::new();canceled.cancel();
 let refusal=semio_framework_async::block_on(scheduler.read(request,&canceled)).unwrap_err();
 assert_eq!(refusal.refusal_kind(),Some(ValueRefusalKind::Canceled));
 assert_eq!(reads.load(Ordering::SeqCst),0);
 let token=CancellationToken::new();
 let mut pending=Box::pin(scheduler.read(request,&token));
 let mut context=Context::from_waker(Waker::noop());
 assert!(pending.as_mut().poll(&mut context).is_pending());
 assert!(pending.as_mut().poll(&mut context).is_pending());
 assert_eq!(reads.load(Ordering::SeqCst),1);
 token.cancel();
 let refusal=semio_framework_async::block_on(pending).unwrap_err();
 assert_eq!(refusal.refusal_kind(),Some(ValueRefusalKind::Canceled));
 assert!(matches!(refusal,PackError::ValueRefusal(_)));
 eprintln!("[DEBUG] Pack scheduler both observed token producers retained Canceled");
}
