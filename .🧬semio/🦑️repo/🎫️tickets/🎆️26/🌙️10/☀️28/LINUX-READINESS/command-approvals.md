# Command Approval Settings

The active session explicitly supplies sandbox_mode danger-full-access, approval policy never, and enabled network. Commands in this turn have executed without escalation or confirmation. Local user config marks /home/deck/semio trusted; neither inspected user nor project config had an explicit approval_policy or sandbox_mode assignment. Session-supplied settings are the authoritative evidence for this running session; no global config was changed.

OpenAI Docs distinguishes sandbox access from command approval policy. The never policy disables command approval prompts; full access combines danger-full-access with never. An agent-authored conversational confirmation can still be an unnecessary workflow choice rather than an execution restriction. Earlier individual prompt events are not available in this turn, so their exact cause cannot be established from current settings alone. Continue authorized repo work without routine confirmation.

Official source: [Agent approvals and security](https://learn.chatgpt.com/docs/agent-approvals-security), fetched2026-10-29.
