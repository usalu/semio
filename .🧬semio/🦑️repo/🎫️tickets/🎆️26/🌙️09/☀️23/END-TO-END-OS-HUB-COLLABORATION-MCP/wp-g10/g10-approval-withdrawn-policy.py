"""🪦️ G10 one-off codemod: the shell approval lane withdraws what it published (cancel, timeout, superseded)."""
import pathlib
P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🦀️.rs")
t = P.read_text()
old = '''    fn resolve_by_shell(&self, request: &ApprovalRequest<'_>) -> Result<ApprovalResolution, &'static str> {
        let Some(bridge) = self.bridge.as_ref().and_then(|slot| slot.get()) else { return Err("this gateway is serving no /bridge — no OS shell can be asked") };
        let Some(connection) = crate::ui::active_shell_connection(bridge) else { return Err("a /bridge is running but no OS shell is attached to it") };
        let frame = crate::bridge::GatewayToShell::ApprovalRequested { approval_id: request.approval_handle.to_string(), summary: request.shell_summary(self.shell_timeout_ms) };
        if !bridge.send_to(connection, frame) {
            return Err("the attached shell's connection closed while the approval was being published");
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(self.shell_timeout_ms);
        loop {'''
new = '''    /// 🐚️ Asks the most recently connected shell, and takes back what it published whenever the answer
    /// stops mattering there: `ApprovalWithdrawn{cancelled}` when the agent's call is cancelled,
    /// `{timed_out}` when the countdown runs out, and `{superseded}` when a newer shell connects — which
    /// is then asked instead, with the time that remains. A shell never keeps a decidable affordance
    /// for a request nobody is waiting on.
    fn resolve_by_shell(&self, request: &ApprovalRequest<'_>) -> Result<ApprovalResolution, &'static str> {
        let Some(bridge) = self.bridge.as_ref().and_then(|slot| slot.get()) else { return Err("this gateway is serving no /bridge — no OS shell can be asked") };
        let Some(mut connection) = crate::ui::active_shell_connection(bridge) else { return Err("a /bridge is running but no OS shell is attached to it") };
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(self.shell_timeout_ms);
        let publish = |connection| {
            let remaining_ms = u64::try_from(deadline.saturating_duration_since(std::time::Instant::now()).as_millis()).unwrap_or(u64::MAX).max(1);
            bridge.send_to(connection, crate::bridge::GatewayToShell::ApprovalRequested { approval_id: request.approval_handle.to_string(), summary: request.shell_summary(remaining_ms) })
        };
        let withdraw = |connection, reason| {
            let _ = bridge.send_to(connection, crate::bridge::GatewayToShell::ApprovalWithdrawn { approval_id: request.approval_handle.to_string(), reason });
        };
        if !publish(connection) {
            return Err("the attached shell's connection closed while the approval was being published");
        }
        loop {'''
assert t.count(old) == 1
t = t.replace(old, new)
old2 = '''            if crate::notify::active_request_cancel_requested() {
                return Ok(ApprovalResolution::Cancelled);
            }
            if std::time::Instant::now() >= deadline {
                return Err("the attached OS shell did not answer the approval request in time");
            }
            std::thread::sleep(std::time::Duration::from_millis(SHELL_APPROVAL_POLL_INTERVAL_MS));'''
new2 = '''            if crate::notify::active_request_cancel_requested() {
                withdraw(connection, crate::bridge::ApprovalWithdrawal::Cancelled);
                return Ok(ApprovalResolution::Cancelled);
            }
            if std::time::Instant::now() >= deadline {
                withdraw(connection, crate::bridge::ApprovalWithdrawal::TimedOut);
                return Err("the attached OS shell did not answer the approval request in time");
            }
            if let Some(newest) = crate::ui::active_shell_connection(bridge).filter(|newest| *newest != connection) {
                withdraw(connection, crate::bridge::ApprovalWithdrawal::Superseded);
                connection = newest;
                if !publish(connection) {
                    return Err("the newer shell's connection closed while the approval was being moved to it");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(SHELL_APPROVAL_POLL_INTERVAL_MS));'''
assert t.count(old2) == 1
t = t.replace(old2, new2)
P.write_text(t)
print("ok")
