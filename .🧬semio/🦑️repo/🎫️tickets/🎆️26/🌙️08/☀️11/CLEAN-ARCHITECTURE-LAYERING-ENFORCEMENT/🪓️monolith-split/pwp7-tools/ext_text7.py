import sys; sys.path.insert(0,'.')
from bundle7 import *
CTXG="<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>"
LEDG="<A: ArtifactApp>"
LP="time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>"
LPU="_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>"
LPM="_time_travel: &mut TimeTravelLedger<A>, _tool_runs: &mut ToolRunLedger<A>"
IDENT="identity: &mut EntityIdentityAuthority<'_>"
CXP="cx: &mut semio_framework_job::StepContext<'_>"
def extension_file(W, emoji, tests_mod):
    t=[]
    t.append('''//! %s️ The neutral extension point of the mounted artifact instance. A feature module (history editing, tool runs, …)
//! contributes command routes, document refusals, maintenance turns and framework bodies through [`InstanceExtension`];
//! the instance consults the registered list `X` its bundles carry and never names the feature itself.
//!
//! 🧩️ Hooks come in three kinds. The static ones read only the action id. The ledger hooks read the two feature ledgers
//! (`time_travel`, `tool_runs`) the instance owns and are passed explicitly. The context hooks run over [`ExtensionCtx`]
//! or [`ExtensionRef`], the borrows the two implementations need, and build their own use-case bundle from it.
use super::*;
use std::marker::PhantomData;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
'''%emoji)
    t.append(extension_structs(W))
    t.append('')
    t.append('''/// 🧱️ One feature's contribution to every mounted instance. Every hook defaults to "no contribution", so an extension
/// overrides only what it owns. A registered list is a right-nested tuple `(First, (Second, ()))`: earlier entries are
/// consulted first, the first claimant of a route, refusal or body wins, and every entry's maintenance turn runs in order.
pub(crate) trait InstanceExtension {
    /// 🧭️ Whether this extension owns the dispatch of `action`.
    fn claims_action(_action: &str) -> bool {
        false
    }

    /// 📜️ Whether `action` needs the document's history, so a head-only pure evaluation refuses it.
    fn reads_history(_action: &str) -> bool {
        false
    }

    /// ⏳️ Whether `action` stays admitted while the document is still loading.
    fn admitted_while_loading(_action: &str) -> bool {
        false
    }

    /// 🚦️ Dispatches one action this extension claims.
    async fn dispatch_action%s(_ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, _args: Option<&DslValue>, _meta: &ActionMeta, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        Err(plugin_sdk_fault(format!("no instance extension claims action '{action}'")))
    }

    /// 🔒️ The refusal of a framework-reserved `action` this extension currently freezes.
    fn reserved_action_refusal%s(%s, _action: &str) -> Option<Fault> {
        None
    }

    /// 🛑️ The refusal of a `verb` that would emit artifact mutations while this extension holds them.
    fn mutation_refusal%s(%s, _verb: &str) -> Option<Fault> {
        None
    }

    /// ❄️ Whether this extension currently freezes every local change to the document.
    fn freezes_document%s(%s) -> bool {
        false
    }

    /// 🚫️ The refusal of a `verb` that would change the document (artifact mutations or child emits) while it is frozen.
    fn document_refusal%s(%s, _verb: &str) -> Option<Fault> {
        None
    }

    /// 🔀️ The refusal of a foreign transaction `txn_id` that would change the document while it is frozen.
    fn transaction_refusal%s(%s, _txn_id: &str) -> Option<Fault> {
        None
    }

    /// 🎚️ Observes one committed window-config publication.
    fn window_config_published%s(%s) {}

    /// 🏃️ Whether this extension owes maintenance work.
    fn has_pending_work%s(_ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        false
    }

    /// 🧾️ Whether this extension holds an original receipt its recipient has not taken yet.
    fn holds_receipt%s(%s) -> bool {
        false
    }

    /// 🪦️ Whether this extension retains nothing a close still has to retire.
    fn terminal_is_empty%s(%s) -> bool {
        true
    }

    /// 🔁️ Drives one maintenance turn under the caller's current retained grant; `true` ends the instance's maintenance
    /// turn here.
    async fn drive_turn%s(_ctx: &mut ExtensionCtx<'_, A, M, X>, _retained: &mut RetainedCloneGrant, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        Ok(false)
    }

    /// 🖼️ The framework body this extension renders for `body_key`.
    fn render_body%s(_ctx: &mut ExtensionCtx<'_, A, M, X>, _body_key: &str, _view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        None
    }

    /// 🖌️ Decorates the app's own rendered body for `body_key` after it is retained.
    fn after_app_render%s(_ctx: &mut ExtensionCtx<'_, A, M, X>, _body_key: &str, _view_state: &ViewModel) -> Result<(), Fault> {
        Ok(())
    }
}

/// 🫙️ The empty list: no route, no refusal, no work.
impl InstanceExtension for () {}

/// 🔗️ A registered list: `First` is consulted before every entry of `Rest`.
impl<First: InstanceExtension, Rest: InstanceExtension> InstanceExtension for (First, Rest) {
    fn claims_action(action: &str) -> bool {
        First::claims_action(action) || Rest::claims_action(action)
    }

    fn reads_history(action: &str) -> bool {
        First::reads_history(action) || Rest::reads_history(action)
    }

    fn admitted_while_loading(action: &str) -> bool {
        First::admitted_while_loading(action) || Rest::admitted_while_loading(action)
    }

    async fn dispatch_action%s(ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if First::claims_action(action) {
            First::dispatch_action(ctx, action, args, meta, identity, cx).await
        } else {
            Rest::dispatch_action(ctx, action, args, meta, identity, cx).await
        }
    }

    fn reserved_action_refusal%s(%s, action: &str) -> Option<Fault> {
        First::reserved_action_refusal(time_travel, tool_runs, action).or_else(|| Rest::reserved_action_refusal(time_travel, tool_runs, action))
    }

    fn mutation_refusal%s(%s, verb: &str) -> Option<Fault> {
        First::mutation_refusal(time_travel, tool_runs, verb).or_else(|| Rest::mutation_refusal(time_travel, tool_runs, verb))
    }

    fn freezes_document%s(%s) -> bool {
        First::freezes_document(time_travel, tool_runs) || Rest::freezes_document(time_travel, tool_runs)
    }

    fn document_refusal%s(%s, verb: &str) -> Option<Fault> {
        First::document_refusal(time_travel, tool_runs, verb).or_else(|| Rest::document_refusal(time_travel, tool_runs, verb))
    }

    fn transaction_refusal%s(%s, txn_id: &str) -> Option<Fault> {
        First::transaction_refusal(time_travel, tool_runs, txn_id).or_else(|| Rest::transaction_refusal(time_travel, tool_runs, txn_id))
    }

    fn window_config_published%s(time_travel: &mut TimeTravelLedger<A>, tool_runs: &mut ToolRunLedger<A>) {
        First::window_config_published(&mut *time_travel, &mut *tool_runs);
        Rest::window_config_published(time_travel, tool_runs);
    }

    fn has_pending_work%s(ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        First::has_pending_work(ctx) || Rest::has_pending_work(ctx)
    }

    fn holds_receipt%s(%s) -> bool {
        First::holds_receipt(time_travel, tool_runs) || Rest::holds_receipt(time_travel, tool_runs)
    }

    fn terminal_is_empty%s(%s) -> bool {
        First::terminal_is_empty(time_travel, tool_runs) && Rest::terminal_is_empty(time_travel, tool_runs)
    }

    async fn drive_turn%s(ctx: &mut ExtensionCtx<'_, A, M, X>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        *retained = cx.retained_grant();
        if First::drive_turn(ctx, retained, identity, cx).await? {
            return Ok(true);
        }
        Rest::drive_turn(ctx, retained, identity, cx).await
    }

    fn render_body%s(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        match First::render_body(ctx, body_key, view_state) {
            Some(body) => Some(body),
            None => Rest::render_body(ctx, body_key, view_state),
        }
    }

    fn after_app_render%s(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        First::after_app_render(ctx, body_key, view_state)?;
        Rest::after_app_render(ctx, body_key, view_state)
    }
}
'''%(CTXG,LEDG,LPU,LEDG,LPU,LEDG,LPU,LEDG,LPU,LEDG,LPU,LEDG,LPM,CTXG,LEDG,LPU,LEDG,LPU,CTXG,CTXG,CTXG,
     CTXG,LEDG,LP,LEDG,LP,LEDG,LP,LEDG,LP,LEDG,LP,LEDG,CTXG,LEDG,LP,LEDG,LP,CTXG,CTXG,CTXG))
    t.append(tests_mod)
    return '\n'.join(t)
if __name__=='__main__':
    W=build(); field_sets(W)
    print(extension_file(W,'🐀','#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n')[:200])
