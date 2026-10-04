//! 🧪️ Unmounted actual retry behavior is governed by explicitly injected provider policy.
use super::*;
use semio_framework_pack_error::PackRetryDisposition;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::sync::atomic::AtomicUsize;

struct PolicyTransport{attempts:Arc<AtomicUsize>,retry:PackRetryDisposition}
impl RangeTransport for PolicyTransport{
 async fn fetch_range(&self,_:RangeRequest)->Result<RangeResponse,PackError>{
  self.attempts.fetch_add(1,Ordering::SeqCst);
  Err(PackError::Io{error:ValueError::new(ValueRefusalKind::WorkLimit,"same intentionally misleading permanent cancellation prose"),retry:self.retry})
 }
}

#[test]
fn actual_http_retries_only_explicit_transport_disposition(){
 for(retry,expected_attempts)in[(PackRetryDisposition::Never,1),(PackRetryDisposition::Transient,3)]{
  let attempts=Arc::new(AtomicUsize::new(0));
  let pool=Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative,3)));
  let runtime=RetryRuntime::native(pool.clone());
  let source=HttpPackSource::with_retry_policy("https://example.test/authority.pack".into(),PolicyTransport{attempts:attempts.clone(),retry},RetryPolicy{max_retries:2,initial_backoff:Duration::ZERO,max_backoff:Duration::ZERO},runtime);
  let result=semio_framework_async::block_on(source.read_at(0,1));
  pool.shutdown();
  let refusal=result.unwrap_err();
  assert_eq!(refusal.refusal_kind(),Some(ValueRefusalKind::WorkLimit));
  assert_eq!(attempts.load(Ordering::SeqCst),expected_attempts);
  assert!(matches!(refusal,PackError::Io{retry:actual,..}if actual==retry));
 }
 eprintln!("[DEBUG] Pack HTTP identical cause/prose followed distinct explicit retry policy");
}
