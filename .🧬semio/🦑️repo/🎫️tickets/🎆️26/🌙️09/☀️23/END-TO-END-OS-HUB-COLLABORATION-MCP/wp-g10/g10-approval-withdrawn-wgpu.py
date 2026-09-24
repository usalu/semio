"""🪦️ G10 one-off codemod: the wgpu shell decodes `ApprovalWithdrawn` (tag 11) and retires the affordance, saying why (en + de)."""
import pathlib
E = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements")
def edit(path, pairs):
    t = path.read_text()
    for old, new in pairs:
        assert t.count(old) == 1, (path.name, old[:70], t.count(old))
        t = t.replace(old, new)
    path.write_text(t)
edit(E / "🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs", [
('''impl ApprovalDecision {
    pub fn to_tag(self) -> u8 {''', '''/// 🪦️ Why the gateway withdrew an approval request — the wgpu twin of `🌉️mcp/🧵️bridge`'s
/// `ApprovalWithdrawal`, same tags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalWithdrawal {
    Cancelled,
    TimedOut,
    Superseded,
}

impl ApprovalWithdrawal {
    pub fn to_tag(self) -> u8 {
        match self {
            ApprovalWithdrawal::Cancelled => 0,
            ApprovalWithdrawal::TimedOut => 1,
            ApprovalWithdrawal::Superseded => 2,
        }
    }

    pub fn from_tag(tag: u8) -> Result<Self, BridgeFrameFault> {
        match tag {
            0 => Ok(ApprovalWithdrawal::Cancelled),
            1 => Ok(ApprovalWithdrawal::TimedOut),
            2 => Ok(ApprovalWithdrawal::Superseded),
            other => Err(BridgeFrameFault::UnknownTag(other)),
        }
    }
}

impl ApprovalDecision {
    pub fn to_tag(self) -> u8 {'''),
('''/// 📤️ Gateway→Shell frames, tags `0..9` in SSOT declaration order.''', '''/// 📤️ Gateway→Shell frames, tags `0..11` in SSOT declaration order.'''),
('''    AgentReply {
        reply_id: String,
        in_reply_to: Option<String>,
        text: String,
        complete: bool,
    },
}''', '''    AgentReply {
        reply_id: String,
        in_reply_to: Option<String>,
        text: String,
        complete: bool,
    },
    /// 🪦️ The approval request is withdrawn — its call was cancelled, it timed out, or a newer shell
    /// now carries it — so this shell retires the affordance and says why.
    ApprovalWithdrawn {
        approval_id: String,
        reason: ApprovalWithdrawal,
    },
}'''),
('''            10 => GatewayToShell::AgentReply { reply_id: reader.read_string()?, in_reply_to: reader.read_option_string()?, text: reader.read_string()?, complete: reader.read_bool()? },
            other => return Err(BridgeFrameFault::UnknownTag(other)),''', '''            10 => GatewayToShell::AgentReply { reply_id: reader.read_string()?, in_reply_to: reader.read_option_string()?, text: reader.read_string()?, complete: reader.read_bool()? },
            11 => GatewayToShell::ApprovalWithdrawn { approval_id: reader.read_string()?, reason: ApprovalWithdrawal::from_tag(reader.read_u8()?)? },
            other => return Err(BridgeFrameFault::UnknownTag(other)),'''),
('''                wire::write_string(&mut buf, text);
                wire::write_bool(&mut buf, *complete);
            }
        }
        buf
    }
}
//#endregion 🔖️GatewayToShell''', '''                wire::write_string(&mut buf, text);
                wire::write_bool(&mut buf, *complete);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                wire::write_u8(&mut buf, 11);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, reason.to_tag());
            }
        }
        buf
    }
}
//#endregion 🔖️GatewayToShell'''),
('''pub enum AgentApprovalState {
    Pending,
    Resolved,
}''', '''pub enum AgentApprovalState {
    Pending,
    Resolved,
    /// 🪦️ The gateway took the request back before anyone here decided it.
    Withdrawn(ApprovalWithdrawal),
}'''),
('''            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                self.pending_approvals.retain(|approval| approval.approval_id != approval_id);
                self.resolve_conversation_approval(&approval_id, decision);
            }''', '''            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                self.pending_approvals.retain(|approval| approval.approval_id != approval_id);
                self.resolve_conversation_approval(&approval_id, decision);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                self.pending_approvals.retain(|approval| approval.approval_id != approval_id);
                if let Some(AgentConversationEntry::Approval { state, .. }) = self.conversation.iter_mut().find(|entry| entry.id() == approval_id) {
                    if matches!(state, AgentApprovalState::Pending) {
                        *state = AgentApprovalState::Withdrawn(reason);
                    }
                }
            }'''),
])
edit(E / "🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs", [
('''//#endregion 🌐️Labels

//#region 🔖️Layout''', '''/// 🪦️ Why a request left without a decision — React's `os.agent.approvals.withdrawn*`.
pub fn approvals_withdrawal_label(reason: crate::agent_bridge::ApprovalWithdrawal, locale: Locale) -> String {
    match reason {
        crate::agent_bridge::ApprovalWithdrawal::Cancelled => agent_label("Withdrawn — the agent's request was cancelled", "Zurückgezogen — die Anfrage des Agenten wurde abgebrochen", locale),
        crate::agent_bridge::ApprovalWithdrawal::TimedOut => agent_label("Withdrawn — nobody decided in time", "Zurückgezogen — niemand hat rechtzeitig entschieden", locale),
        crate::agent_bridge::ApprovalWithdrawal::Superseded => agent_label("Withdrawn — moved to your newer window", "Zurückgezogen — in dein neueres Fenster verschoben", locale),
    }
}
//#endregion 🌐️Labels

//#region 🔖️Layout'''),
])
edit(E / "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", [
('''            (AgentApprovalState::Resolved, None) => None,
        },''', '''            (AgentApprovalState::Resolved, None) => None,
            (AgentApprovalState::Withdrawn(reason), _) => Some(crate::agent_approvals::approvals_withdrawal_label(*reason, locale)),
        },'''),
('''        AgentConversationEntry::Approval { state, .. } => match state {
            AgentApprovalState::Pending => "pending",
            AgentApprovalState::Resolved => "resolved",
        },''', '''        AgentConversationEntry::Approval { state, .. } => match state {
            AgentApprovalState::Pending => "pending",
            AgentApprovalState::Resolved => "resolved",
            AgentApprovalState::Withdrawn(_) => "withdrawn",
        },'''),
])
print("ok")
