import sys
p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs"
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    n = s.count(old)
    if n != count:
        sys.exit(f"anchor count {n}!={count}: {old[:90]!r}")
    s = s.replace(old, new)
rep('''    async fn run_job_on_worker(&self, kind: &str, input: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {''',
    '''    async fn run_job_on_worker(&self, kind: &str, input: Vec<u8>, cancel: semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {''')
rep('''        let pool = plugin_host_worker_pool();
        let cancel = semio_framework_job::root_cancel_token();
        let operation = semio_framework_job::OperationId(self.actor.0);''', '''        let pool = plugin_host_worker_pool();
        let operation = semio_framework_job::OperationId(self.actor.0);''')
for kind in ["semio.io-run", "semio.io-sniff", "semio.mutation-plan", "semio.migrate", "semio.compose"]:
    for arg in ["input", "request.to_vec()"]:
        old = f'self.run_job_on_worker("{kind}", {arg}).await'
        if s.count(old) == 1:
            s = s.replace(old, f'self.run_job_on_worker("{kind}", {arg}, semio_framework_job::root_cancel_token()).await')
rep('''    pub async fn infer(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.infer", request.to_vec()).await''', '''    pub async fn infer(&self, request: &[u8], cancel: &semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.infer", request.to_vec(), cancel.child_now()).await''')
rep('''    pub async fn infer(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let request_text''', '''    pub async fn infer(&self, request: &[u8], cancel: &semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        let request_text''')
rep('''self.infer_with_visited(request, &mut Vec::new()).await;''', '''self.infer_with_visited(request, &mut Vec::new(), cancel).await;''')
rep('''    async fn infer_with_visited(&self, request: &[u8], visited: &mut Vec<String>) -> Result<Vec<u8>, PluginHostError> {''', '''    async fn infer_with_visited(&self, request: &[u8], visited: &mut Vec<String>, cancel: &semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {''')
rep('''Box::pin(self.infer_with_visited(&dependency_request_bytes, visited)).await?;''', '''Box::pin(self.infer_with_visited(&dependency_request_bytes, visited, cancel)).await?;''')
rep('''        let result = handle.infer(&request).await?;''', '''        let result = handle.infer(&request, cancel).await?;''')
if "run_job_on_worker(\"semio.io-run\", input).await" in s or s.count("root_cancel_token()).await") != 5:
    sys.exit(f"callers not all rewritten: {s.count('root_cancel_token()).await')}")
open(p, "w", encoding="utf-8").write(s)
t = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️guest-cold-relay/🦀️.rs"
u = open(t, encoding="utf-8").read()
old = "handle.infer(&oversized).await"
if u.count(old) != 1: sys.exit("test anchor")
open(t, "w", encoding="utf-8").write(u.replace(old, "handle.infer(&oversized, &semio_framework_job::root_cancel_token()).await"))
print("ok")
