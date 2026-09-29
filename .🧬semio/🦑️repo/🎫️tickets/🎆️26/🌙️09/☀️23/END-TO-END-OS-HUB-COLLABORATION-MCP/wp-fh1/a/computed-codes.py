"""🧯️ FH1 family A — the remaining computed framework codes become literals the census reads: helpers take a
`FaultCode`, `code()` tables gain a `fault_code()` twin of literal arms right beside them, and matches over
diagnostics yield the `FaultCode` itself."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply, regex
M = "🧰️framework/🛍️products/💻️os/🔨️modules/"
P = M + "🔌️plugin/🦀️.rs"
regex("", P, r'self\.reject\("(app\.intent\.[^"]+)"', r'self.reject(FaultCode::new("\1")', 6)
regex("", P, r'transaction_fault\((FaultOrigin::[A-Za-z]+), "(transaction\.[^"]+)"', r'transaction_fault(\1, FaultCode::new("\2")', 21)
regex("", M + "🔌️plugin/🪟️window/🎚️config/🦀️.rs", r'reject\(typed, "(window-config\.[^"]+)"', r'reject(typed, FaultCode::new("\1")', 2)
apply("", [
    (P, "        fn reject(&mut self, code: &'static str, detail: &'static str) -> UiCommandJsonStep {", "        fn reject(&mut self, code: FaultCode, detail: &'static str) -> UiCommandJsonStep {"),
    (P, "            self.fault = Some(Fault::new(FaultOrigin::Framework, FaultCode::new(code), detail));", "            self.fault = Some(Fault::new(FaultOrigin::Framework, code, detail));"),
    (P, '                            protocol_fault = Some("interactive-job.checkpoint");', '                            protocol_fault = Some(FaultCode::new("interactive-job.checkpoint"));'),
    (P, '                            protocol_fault = Some("interactive-job.output-envelope");', '                            protocol_fault = Some(FaultCode::new("interactive-job.output-envelope"));'),
    (P, '''                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(code), format!("framework route '{verb}' returned an invalid retained checkpoint")));''', '''                        return Err(Fault::new(FaultOrigin::Framework, code, format!("framework route '{verb}' returned an invalid retained checkpoint")));'''),
    (P, '''                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(code), format!("framework route '{verb}' did not preserve its admitted envelope")));''', '''                    return Err(Fault::new(FaultOrigin::Framework, code, format!("framework route '{verb}' did not preserve its admitted envelope")));'''),
    (P, '''        fn transaction_fault(origin: FaultOrigin, code: &'static str, message: impl Into<String>) -> Fault {
            Fault::new(origin, FaultCode::new(code), message)''', '''        fn transaction_fault(origin: FaultOrigin, code: FaultCode, message: impl Into<String>) -> Fault {
            Fault::new(origin, code, message)'''),
    (P, ".map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))", ".map_err(|refusal| Fault::new(FaultOrigin::Framework, refusal.fault_code(), refusal.message()))", 2),
    ("🧰️framework/🔨️modules/🎠️kernel/🦀️.rs", '''    /// 🏷️ The stable fault code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "file-import.envelope",
            Self::Chunk => "file-import.chunk",
            Self::Gap => "file-import.gap",
        }
    }
''', '''    /// 🏷️ The stable fault code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "file-import.envelope",
            Self::Chunk => "file-import.chunk",
            Self::Gap => "file-import.gap",
        }
    }

    /// 🧯️ The fault code a dispatch refuses with — [`Self::code`] as the literal the fault census reads.
    pub fn fault_code(self) -> FaultCode {
        match self {
            Self::Envelope => FaultCode::new("file-import.envelope"),
            Self::Chunk => FaultCode::new("file-import.chunk"),
            Self::Gap => FaultCode::new("file-import.gap"),
        }
    }
'''),
    (M + "🔌️plugin/⏯️tool-run/🦀️.rs", '''.map_err(|rejection| Fault::new(FaultOrigin::Framework, FaultCode::new(rejection.code()), "tool run trace page does not belong to the current run generation"))?;''', '''.map_err(|rejection| Fault::new(FaultOrigin::Framework, match rejection { ToolRunRejection::Stale => FaultCode::new("toolRun.stale"), ToolRunRejection::Busy => FaultCode::new("toolRun.busy"), ToolRunRejection::Illegal => FaultCode::new("toolRun.illegal") }, "tool run trace page does not belong to the current run generation"))?;'''),
    (M + "🏪️store/🔄️sync/🦀️.rs", "code: crate::os_dsl::FaultCode::new(status.code()),", "code: status.fault_code(),"),
    (M + "🏪️store/🔄️sync/🦀️.rs", '''    /// 🏷️ The schema's status code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Reconnecting => "reconnecting",
            Self::LinkExpired => "link-expired",
            Self::AccessRevoked => "access-revoked",
        }
    }
''', '''    /// 🏷️ The schema's status code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Reconnecting => "reconnecting",
            Self::LinkExpired => "link-expired",
            Self::AccessRevoked => "access-revoked",
        }
    }

    /// 🧯️ [`Self::code`] as the fault code a shell renders by (the literal the fault census reads).
    pub fn fault_code(self) -> crate::os_dsl::FaultCode {
        match self {
            Self::Linked => crate::os_dsl::FaultCode::new("linked"),
            Self::Reconnecting => crate::os_dsl::FaultCode::new("reconnecting"),
            Self::LinkExpired => crate::os_dsl::FaultCode::new("link-expired"),
            Self::AccessRevoked => crate::os_dsl::FaultCode::new("access-revoked"),
        }
    }
'''),
    (M + "🔌️plugin/🪟️window/🎚️config/🦀️.rs", '''        let reject = |typed: Box<O::Mutation>, code: &'static str, message: &str| RejectedWindowConfigEmission {
            mutation: WindowConfigMutation { window_id: authority.window_id.clone(), window_kind_id, mutation: typed },
            fault: Fault::new(FaultOrigin::Framework, FaultCode::new(code), message),''', '''        let reject = |typed: Box<O::Mutation>, code: FaultCode, message: &str| RejectedWindowConfigEmission {
            mutation: WindowConfigMutation { window_id: authority.window_id.clone(), window_kind_id, mutation: typed },
            fault: Fault::new(FaultOrigin::Framework, code, message),'''),
    (M + "🔌️plugin/🪟️window/🎚️config/🦀️.rs", '''        let code = match diagnostic {
            WindowConfigPackLoadDiagnostic::EnvelopeIdentity => "window-config.pack-envelope",
            WindowConfigPackLoadDiagnostic::Pack => "window-config.pack",
            WindowConfigPackLoadDiagnostic::TypedState => "window-config.typed-state",
            WindowConfigPackLoadDiagnostic::History => "window-config.history",
            WindowConfigPackLoadDiagnostic::InnerIdentity => "window-config.inner-identity",
            WindowConfigPackLoadDiagnostic::Replay => "window-config.replay",
            WindowConfigPackLoadDiagnostic::Capacity => "window-config.capacity",
            WindowConfigPackLoadDiagnostic::Stale => "window-config.stale",
            WindowConfigPackLoadDiagnostic::Cancelled => "window-config.cancelled",
            WindowConfigPackLoadDiagnostic::Retirement => "window-config.retirement",
        };
        Fault::new(FaultOrigin::Framework, FaultCode::new(code), "retained exact window config Pack load was rejected")''', '''        let code = match diagnostic {
            WindowConfigPackLoadDiagnostic::EnvelopeIdentity => FaultCode::new("window-config.pack-envelope"),
            WindowConfigPackLoadDiagnostic::Pack => FaultCode::new("window-config.pack"),
            WindowConfigPackLoadDiagnostic::TypedState => FaultCode::new("window-config.typed-state"),
            WindowConfigPackLoadDiagnostic::History => FaultCode::new("window-config.history"),
            WindowConfigPackLoadDiagnostic::InnerIdentity => FaultCode::new("window-config.inner-identity"),
            WindowConfigPackLoadDiagnostic::Replay => FaultCode::new("window-config.replay"),
            WindowConfigPackLoadDiagnostic::Capacity => FaultCode::new("window-config.capacity"),
            WindowConfigPackLoadDiagnostic::Stale => FaultCode::new("window-config.stale"),
            WindowConfigPackLoadDiagnostic::Cancelled => FaultCode::new("window-config.cancelled"),
            WindowConfigPackLoadDiagnostic::Retirement => FaultCode::new("window-config.retirement"),
        };
        Fault::new(FaultOrigin::Framework, code, "retained exact window config Pack load was rejected")'''),
])
