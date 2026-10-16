"""F3: RuntimeMailbox<R: MailboxRuntime> (plan 5.3 I2) with the narrow runtime surface the mailbox really uses, plus I3 (close_world3d_dynamic_step moves to AppPresenter)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
R = os.environ['R']; T = os.environ['T']; V = '️'
P = lambda base, name: base + f'/{name.replace("@", V)}/🦀{V}.rs'
rd = lambda p: open(p, encoding='utf8').read()
w = lambda p, s: open(p, 'w', encoding='utf8').write(s)
def sub1(t, a, b, n=1):
    assert t.count(a) == n, (a[:80], t.count(a)); return t.replace(a, b)
def subre(t, pat, rep, n=None, flags=0):
    found = len(re.findall(pat, t, flags))
    assert found > 0 and (n is None or found == n), (pat, found); return re.sub(pat, rep, t, flags=flags)

pr = P(R, '🎨@presentation-raster'); mb = P(R, '📮@mailbox-impl'); ra = R + f'/🛎{V}frame-actions/🔁{V}runtime-apply/🦀{V}.rs'
fb = P(R, '🧾@frame-build'); fp = P(R, '🖥@frame-presentation'); ar = P(R, '🧠@app-runtime'); fa = P(R, '🛎@frame-actions')
oh = P(T, '🏠@os-host'); wa = P(T, '🪟@winit-app'); fj = P(T, '🧵@frame-job')

# ---- presentation-raster: generic state + trait
t = rd(pr)
t = sub1(t, 'pub(crate) runtime: Mutex<AppRuntime>,', 'pub(crate) runtime: Mutex<R>,')
t = sub1(t, 'pub(crate) struct RuntimeMailboxInner {', 'pub(crate) struct RuntimeMailboxInner<R> {')
t = sub1(t, 'impl RuntimeMailboxInner {\n    pub(crate) fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<\'_, AppRuntime>> {', 'impl<R> RuntimeMailboxInner<R> {\n    pub(crate) fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<\'_, R>> {')
trait_text = '''/// 🔌️ Everything the runtime mailbox needs of the runtime it guards (plan section 5.3, inversion I2): the interaction checkout, the two pending
/// frame cursors and the resize entry. The mailbox is generic over it so the transport crate never names the application runtime above it.
pub(crate) trait MailboxRuntime: Send + 'static {
    fn interaction(&self) -> Option<&AppInteractionState>;
    fn interaction_mut(&mut self) -> Option<&mut AppInteractionState>;
    fn interaction_available(&self) -> bool;
    fn check_out_interaction(&mut self, site: &'static str) -> Option<AppInteractionState>;
    fn return_interaction(&mut self, interaction: AppInteractionState);
    fn resize(&mut self, logical_width: f32, logical_height: f32, scale_factor: f32);
    fn pending_frame_deferred(&self) -> &Option<FrameDeferredCursor>;
    fn pending_frame_deferred_mut(&mut self) -> &mut Option<FrameDeferredCursor>;
    fn frame_actions_mut(&mut self) -> &mut FrameActionOwners;
    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal(&self) -> &Option<FrameMaintenanceRefusal>;
    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal_mut(&mut self) -> &mut Option<FrameMaintenanceRefusal>;
}

/// 🪪️ Submits owned work without holding the original runtime or a mutex guard across suspension.
pub(crate) type RuntimeHandle<R> = std::sync::Weak<RuntimeMailboxInner<R>>;

pub(crate) struct RuntimeMailbox<R>(pub(crate) Arc<RuntimeMailboxInner<R>>);

impl<R> Clone for RuntimeMailbox<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
'''
t = sub1(t, '#[derive(Clone)]\npub(crate) struct RuntimeMailbox(pub(crate) Arc<RuntimeMailboxInner>);\n', trait_text)
for name in ['FrameMaintenanceExecutionEnvelope', 'FrameDeferredExecutionEnvelope']:
    t = sub1(t, f'pub(crate) struct {name} {{\n    pub(crate) mailbox: RuntimeMailbox,', f'pub(crate) struct {name}<R> {{\n    pub(crate) mailbox: RuntimeMailbox<R>,')
    t = sub1(t, f'impl {name} {{\n    pub(crate) fn new(mailbox: RuntimeMailbox,', f'impl<R> {name}<R> {{\n    pub(crate) fn new(mailbox: RuntimeMailbox<R>,')
    t = sub1(t, f'impl Drop for {name} {{', f'impl<R> Drop for {name}<R> {{')
for kind in ['Maintenance', 'Deferred']:
    g = f'Frame{kind}ExecutionGuard'; e = f'Frame{kind}ExecutionEnvelope'
    t = sub1(t, f'struct {g} {{\n    mailbox: RuntimeMailbox,', f'struct {g}<R> {{\n    mailbox: RuntimeMailbox<R>,')
    t = sub1(t, f'impl {g} {{', f'impl<R> {g}<R> {{')
    t = sub1(t, f'impl Drop for {g} {{', f'impl<R> Drop for {g}<R> {{')
    t = sub1(t, f'pub(crate) fn begin(mut self) -> Option<{g}> {{', f'pub(crate) fn begin(mut self) -> Option<{g}<R>> {{')
t = sub1(t, 'pub(crate) struct RuntimeDispatchReturn{mailbox:RuntimeMailbox,', 'pub(crate) struct RuntimeDispatchReturn<R>{mailbox:RuntimeMailbox<R>,')
t = sub1(t, 'impl RuntimeDispatchReturn{', 'impl<R> RuntimeDispatchReturn<R>{')
t = sub1(t, 'impl Drop for RuntimeDispatchReturn{', 'impl<R> Drop for RuntimeDispatchReturn<R>{')
w(pr, t)

# ---- mailbox-impl
t = rd(mb)
t = sub1(t, 'impl RuntimeMailbox {', 'impl<R: MailboxRuntime> RuntimeMailbox<R> {')
t = sub1(t, 'pub(crate) fn new(runtime: AppRuntime) -> Self {', 'pub(crate) fn new(runtime: R) -> Self {')
t = sub1(t, 'pub(crate) fn downgrade(&self) -> AppHandle {', 'pub(crate) fn downgrade(&self) -> RuntimeHandle<R> {')
t = sub1(t, "std::sync::MutexGuard<'_, AppRuntime>>", "std::sync::MutexGuard<'_, R>>")
t = sub1(t, 'if runtime.pending_frame_maintenance_refusal.is_some(){return false}', 'if runtime.pending_frame_maintenance_refusal().is_some(){return false}')
t = sub1(t, 'runtime.interaction_available()&&runtime.pending_frame_deferred.is_none()', 'runtime.interaction_available()&&runtime.pending_frame_deferred().is_none()')
t = subre(t, r'runtime\.interaction\.as_mut\(\)', 'runtime.interaction_mut()')
t = subre(t, r'runtime\.interaction\.as_ref\(\)', 'runtime.interaction()')
t = sub1(t, 'self.try_lock().ok()?.interaction.as_mut()?.text_fault.take()', 'self.try_lock().ok()?.interaction_mut()?.text_fault.take()')
t = sub1(t, 'self.try_lock().ok()?.interaction.as_mut()?.frame_fault.take()', 'self.try_lock().ok()?.interaction_mut()?.frame_fault.take()')
t = sub1(t, 'runtime.pending_frame_deferred.is_none() {', 'runtime.pending_frame_deferred().is_none() {')
t = sub1(t, 'runtime.pending_frame_deferred = Some(cursor);', '*runtime.pending_frame_deferred_mut() = Some(cursor);')
t = sub1(t, 'if let Some(cursor) = runtime.pending_frame_deferred.as_mut() {', 'if let Some(cursor) = runtime.pending_frame_deferred_mut().as_mut() {')
t = sub1(t, 'runtime.pending_frame_deferred = None;', '*runtime.pending_frame_deferred_mut() = None;')
t = sub1(t, 'if !runtime.frame_actions.close_step() {', 'if !runtime.frame_actions_mut().close_step() {')
t = sub1(t, 'completion.apply.close_returned_interaction_step(&mut runtime,cx)', 'completion.apply.close_returned_interaction_step(&mut *runtime,cx)')
t = sub1(t, 'pub(crate) fn update_frame_inputs(&self, runtime: &AppRuntime) {', 'pub(crate) fn update_frame_inputs(&self, runtime: &R) {')
t = sub1(t, '''        if !runtime.interaction_available() {
            return;
        }
        *self.0.frame_inputs.lock().expect("runtime frame inputs lock") = FrameBuildInputs { wheel_zoom_deadline_ms: runtime.wheel_zoom_deadline_ms, now_ms: app_now_ms() };''', '''        let Some(interaction) = runtime.interaction() else {
            return;
        };
        *self.0.frame_inputs.lock().expect("runtime frame inputs lock") = FrameBuildInputs { wheel_zoom_deadline_ms: interaction.wheel_zoom_deadline_ms, now_ms: app_now_ms() };''')
# I3: the world3d dynamic close step becomes an AppPresenter method taking the mailbox
t, method = mv.cut_method(t, 'close_world3d_dynamic_step')
assert method.rstrip().endswith('}'), method[-80:]
for private in ['fn close_world_cursor_wake_step']:
    assert t.count('    ' + private) == 1, private
    t = t.replace('    ' + private, '    pub(crate) ' + private)
submit = '''    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn submit_interaction<F, Fut>(runtime: &mut R, handle: &RuntimeHandle<R>, key: Option<&'static str>, work: F) -> bool
    where
        F: FnOnce(AppInteractionState) -> Fut,
        Fut: Future<Output = AppInteractionState> + Send + 'static,
    {
        let Some(interaction) = runtime.check_out_interaction("submit-interaction") else { return false };
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            runtime.return_interaction(interaction);
            return false;
        };
        if !mailbox.reserve_interaction_future() {
            runtime.return_interaction(interaction);
            return false;
        }
        mailbox.spawn_interaction_reserved(key, work(interaction));
        true
    }
'''
anchor = '    pub(crate) fn downgrade(&self) -> RuntimeHandle<R> {'
t = sub1(t, anchor, submit + '\n' + anchor)
w(mb, t)
body = sub1(method, "pub(crate) fn close_world3d_dynamic_step(&self,presenter:&mut AppPresenter,cx:&mut semio_framework_job::StepContext<'_>)", "pub(crate) fn close_world3d_dynamic_step(&mut self,mailbox:&AppMailbox,cx:&mut semio_framework_job::StepContext<'_>)")
body = body.replace('self.', 'mailbox.').replace('presenter.gpu', 'self.gpu')
tfp = rd(fp)
tfp = tfp.rstrip('\n') + '\n\n//#region 📮' + V + "RuntimeMailbox\nimpl AppPresenter {\n    /// 🌍️ Plan section 5.3, inversion I3: the world3d dynamic-resource close step needs the presenter's GPU and the mailbox's runtime, so it lives with the presenter and borrows the mailbox.\n" + body + '\n}\n//#endregion 📮' + V + 'RuntimeMailbox\n'
tfp = sub1(tfp, '    pub(crate) runtime: RuntimeMailbox,', '    pub(crate) runtime: AppMailbox,')
w(fp, tfp)

# ---- runtime-apply: generic over the runtime
t = rd(ra)
for sig in ['fn close_returned_interaction_step', 'fn restore_interaction_only', 'fn start_dispatch', 'fn start_frame_deferred', 'fn apply_step']:
    assert t.count(sig + '(') == 1, sig
    t = t.replace(sig + '(', sig + '<R: MailboxRuntime>(')
t = subre(t, r'runtime: ?&mut AppRuntime', 'runtime: &mut R', 5)
t = subre(t, r'handle: ?&AppHandle', 'handle: &RuntimeHandle<R>', 3)
t = sub1(t, 'if runtime.pending_frame_maintenance_refusal.is_none() {', 'if runtime.pending_frame_maintenance_refusal().is_none() {')
t = sub1(t, 'runtime.pending_frame_maintenance_refusal = mailbox.try_take_abandoned_frame_maintenance();', '*runtime.pending_frame_maintenance_refusal_mut() = mailbox.try_take_abandoned_frame_maintenance();')
t = sub1(t, 'if let Some(mut refusal) = runtime.pending_frame_maintenance_refusal.take() {', 'if let Some(mut refusal) = runtime.pending_frame_maintenance_refusal_mut().take() {')
t = subre(t, r'runtime\.pending_frame_maintenance_refusal = Some\(refusal\);', '*runtime.pending_frame_maintenance_refusal_mut() = Some(refusal);', 5)
t = sub1(t, 'runtime.submit_interaction(&handle, Some("plugin-boot"),', 'RuntimeMailbox::submit_interaction(runtime, &handle, Some("plugin-boot"),')
w(ra, t)

# ---- frame-build: aliases, submit_interaction moves to the mailbox
t = rd(fb)
t = sub1(t, 'pub(crate) type AppHandle = std::sync::Weak<RuntimeMailboxInner>;', 'pub(crate) type AppHandle = RuntimeHandle<AppRuntime>;\n\n/// 📮️ The runtime mailbox of the application runtime.\npub(crate) type AppMailbox = RuntimeMailbox<AppRuntime>;')
t, sm = mv.cut_method(t, 'submit_interaction')
t = sub1(t, 'pub(crate) fn discard_frame_input_candidate(runtime: &RuntimeMailbox,', 'pub(crate) fn discard_frame_input_candidate(runtime: &AppMailbox,')
w(fb, t)

# ---- app-runtime implements the trait
t = rd(ar)
impl = '''impl MailboxRuntime for AppRuntime {
    fn interaction(&self) -> Option<&AppInteractionState> {
        self.interaction.as_ref()
    }

    fn interaction_mut(&mut self) -> Option<&mut AppInteractionState> {
        self.interaction.as_mut()
    }

    fn interaction_available(&self) -> bool {
        AppRuntime::interaction_available(self)
    }

    fn check_out_interaction(&mut self, site: &'static str) -> Option<AppInteractionState> {
        AppRuntime::check_out_interaction(self, site)
    }

    fn return_interaction(&mut self, interaction: AppInteractionState) {
        AppRuntime::return_interaction(self, interaction)
    }

    fn resize(&mut self, logical_width: f32, logical_height: f32, scale_factor: f32) {
        AppRuntime::resize(self, logical_width, logical_height, scale_factor)
    }

    fn pending_frame_deferred(&self) -> &Option<FrameDeferredCursor> {
        &self.pending_frame_deferred
    }

    fn pending_frame_deferred_mut(&mut self) -> &mut Option<FrameDeferredCursor> {
        &mut self.pending_frame_deferred
    }

    fn frame_actions_mut(&mut self) -> &mut FrameActionOwners {
        &mut self.frame_actions
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal(&self) -> &Option<FrameMaintenanceRefusal> {
        &self.pending_frame_maintenance_refusal
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal_mut(&mut self) -> &mut Option<FrameMaintenanceRefusal> {
        &mut self.pending_frame_maintenance_refusal
    }
}

'''
anchor = 'impl AppRuntime {\n'
assert t.count(anchor) == 1
t = t.replace(anchor, impl + anchor, 1)
t = sub1(t, 'pub(crate) fn frame_after_input_step(&mut self, runtime: &RuntimeMailbox,', 'pub(crate) fn frame_after_input_step(&mut self, runtime: &AppMailbox,')
w(ar, t)

# ---- frame-job / os-host / winit-app type uses
t = rd(fj); t = subre(t, r'crate::RuntimeMailbox\b', 'crate::AppMailbox', 5); w(fj, t)
t = rd(oh)
t = sub1(t, 'use crate::{AppPresenter, RuntimeMailbox};', 'use crate::{AppMailbox, AppPresenter};')
t = subre(t, r'(?<![\w:])RuntimeMailbox(?=[,>\s)])(?!::)', 'AppMailbox')
t = sub1(t, 'if !runtime.close_world3d_dynamic_step(presenter,cx)? {', 'if !presenter.close_world3d_dynamic_step(runtime,cx)? {')
w(oh, t)
t = rd(wa)
t = sub1(t, '    use crate::RuntimeMailbox;', '    use crate::AppMailbox;')
t = sub1(t, '            runtime: RuntimeMailbox,', '            runtime: AppMailbox,')
w(wa, t)
print('f3 done')
