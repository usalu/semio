"""🪦️ G10 one-off codemod: the React shell retires a withdrawn approval affordance (hook + affordance + panel, en + de)."""
import pathlib
E = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements")
def edit(path, pairs):
    t = path.read_text()
    for old, new in pairs:
        assert t.count(old) == 1, (path.name, old[:70], t.count(old))
        t = t.replace(old, new)
    path.write_text(t)
edit(E / "🔗️AgentBridge/🟦️.tsx", [
('''  type ApprovalDecision,''', '''  type ApprovalDecision,
  type ApprovalWithdrawal,'''),
('''  | { readonly kind: "approval"; readonly id: string; readonly summary: string; readonly state: "pending" | "resolved"; readonly decision: ApprovalDecision | null; readonly atMs: number }''',
 '''  /** ⛩️ `withdrawn` is the gateway taking the request back before anyone here decided it — the call was
   * cancelled, the countdown ran out, or a newer shell now carries it (`withdrawal` says which). */
  | { readonly kind: "approval"; readonly id: string; readonly summary: string; readonly state: "pending" | "resolved" | "withdrawn"; readonly decision: ApprovalDecision | null; readonly withdrawal: ApprovalWithdrawal | null; readonly atMs: number }'''),
('''            decidedSession: { label: { normal: "Approved for this session", beginner: "You allowed it for this session" } },''', '''            decidedSession: { label: { normal: "Approved for this session", beginner: "You allowed it for this session" } },
            withdrawnCancelled: { label: { normal: "Withdrawn — the agent's request was cancelled", beginner: "No longer needed — the request was stopped" } },
            withdrawnTimedOut: { label: { normal: "Withdrawn — nobody decided in time", beginner: "No longer needed — the time ran out" } },
            withdrawnSuperseded: { label: { normal: "Withdrawn — moved to your newer window", beginner: "Moved to your newer window" } },'''),
('''            decidedSession: { label: { normal: "Für diese Sitzung genehmigt", beginner: "Du hast es für diese Sitzung erlaubt" } },''', '''            decidedSession: { label: { normal: "Für diese Sitzung genehmigt", beginner: "Du hast es für diese Sitzung erlaubt" } },
            withdrawnCancelled: { label: { normal: "Zurückgezogen — die Anfrage des Agenten wurde abgebrochen", beginner: "Nicht mehr nötig — die Anfrage wurde gestoppt" } },
            withdrawnTimedOut: { label: { normal: "Zurückgezogen — niemand hat rechtzeitig entschieden", beginner: "Nicht mehr nötig — die Zeit ist abgelaufen" } },
            withdrawnSuperseded: { label: { normal: "Zurückgezogen — in dein neueres Fenster verschoben", beginner: "In dein neueres Fenster verschoben" } },'''),
('''          setConversation((current) => appendConversationEntry(current, { kind: "approval", id: frame.approvalId, summary: frame.summary, state: "pending", decision: null, atMs: Date.now() }));''',
 '''          setConversation((current) => appendConversationEntry(current, { kind: "approval", id: frame.approvalId, summary: frame.summary, state: "pending", decision: null, withdrawal: null, atMs: Date.now() }));'''),
('''          setConversation((current) => updateConversationEntry(current, frame.approvalId, (entry) => (entry.kind === "approval" ? { ...entry, state: "resolved", decision: frame.decision } : entry)));
          break;
        }''', '''          setConversation((current) => updateConversationEntry(current, frame.approvalId, (entry) => (entry.kind === "approval" ? { ...entry, state: "resolved", decision: frame.decision } : entry)));
          break;
        }
        case "approvalWithdrawn": {
          setPendingApprovals((current) => current.filter((approval) => approval.approvalId !== frame.approvalId));
          setConversation((current) => updateConversationEntry(current, frame.approvalId, (entry) => (entry.kind === "approval" && entry.state === "pending" ? { ...entry, state: "withdrawn", withdrawal: frame.reason } : entry)));
          break;
        }'''),
])
edit(E / "🤖️AgentApprovals/🟦️.tsx", [
('''  readonly state: "pending" | "resolved";
  readonly decision: ApprovalDecision | null;''', '''  readonly state: "pending" | "resolved" | "withdrawn";
  readonly decision: ApprovalDecision | null;
  /** 🪦️ Why the gateway withdrew the request; set exactly when `state` is `withdrawn`. */
  readonly withdrawal?: ApprovalWithdrawal | null;'''),
('''export function AgentApprovalAffordance({ approvalId, summary, requestedAtMs, state, decision, onDecision }: AgentApprovalAffordanceProps): ReactElement {''',
 '''export function AgentApprovalAffordance({ approvalId, summary, requestedAtMs, state, decision, withdrawal = null, onDecision }: AgentApprovalAffordanceProps): ReactElement {'''),
('''    session: useLabel(agentUiLabel("os.agent.approvals.decidedSession")),
  };''', '''    session: useLabel(agentUiLabel("os.agent.approvals.decidedSession")),
  };
  const withdrawnLabels: Readonly<Record<ApprovalWithdrawal, string>> = {
    cancelled: useLabel(agentUiLabel("os.agent.approvals.withdrawnCancelled")),
    timed_out: useLabel(agentUiLabel("os.agent.approvals.withdrawnTimedOut")),
    superseded: useLabel(agentUiLabel("os.agent.approvals.withdrawnSuperseded")),
  };'''),
('''data-semio-agent-approval-state={pending ? "pending" : (decision ?? "resolved")}''', '''data-semio-agent-approval-state={pending ? "pending" : state === "withdrawn" ? "withdrawn" : (decision ?? "resolved")} data-semio-agent-approval-withdrawal={withdrawal ?? ""}'''),
('''        <p data-semio-agent-approval-decision={decision ?? ""} className="text-2xs font-medium text-muted-foreground">
          {decision ? decidedLabels[decision] : ""}
        </p>''', '''        <p role={state === "withdrawn" ? "status" : undefined} data-semio-agent-approval-decision={decision ?? ""} className="text-2xs font-medium text-muted-foreground">
          {state === "withdrawn" && withdrawal ? withdrawnLabels[withdrawal] : decision ? decidedLabels[decision] : ""}
        </p>'''),
])
edit(E / "💬️AgentChatPanel/🟦️.tsx", [
('''state={entry.state} decision={entry.decision} onDecision={onResolveApproval} />''', '''state={entry.state} decision={entry.decision} withdrawal={entry.withdrawal} onDecision={onResolveApproval} />'''),
])
print("ok")
